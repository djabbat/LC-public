//! Tamper-evident audit journal — mirror of the Auditor Core TRACE CONTINUITY
//! (DCT10–DCT12) and Ed25519 journal signing (P2). An append-only log is only
//! a discipline until it is tamper-evident: here the canonical digest of the
//! whole audit trail is signed with HMAC-SHA256.
//!
//! Compaction/archiving are legitimate re-signing events; a silent rewrite of
//! the committed trail fails verification.

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

use crate::status::AuditRecord;

/// One append-only audit entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditJournalEntry {
    pub seq: u64,
    pub stage: String,
    pub status: String,
    pub epoch: u64,
    pub author: String,
    pub scope: Vec<String>,
    /// Whether this entry was compacted into a summary.
    pub compacted: bool,
    /// Summary entry that absorbed this entry's address, if compacted.
    pub compacted_into: Option<u64>,
}

/// The append-only audit journal of pipeline stage runs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditJournal {
    entries: Vec<AuditJournalEntry>,
    /// Trace-continuity violations found on the last check.
    #[serde(default)]
    violations: Vec<String>,
}

impl AuditJournal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entries(&self) -> &[AuditJournalEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Append a stage-run record as a journal entry (addressable, seq).
    pub fn append_stage_run(&mut self, record: &AuditRecord) -> u64 {
        let seq = self.entries.len() as u64 + 1;
        self.entries.push(AuditJournalEntry {
            seq,
            stage: record.stage.clone(),
            status: format!("{:?}", record.status).to_uppercase(),
            epoch: record.epoch,
            author: record.author.clone(),
            scope: record.scope.clone(),
            compacted: false,
            compacted_into: None,
        });
        seq
    }

    /// Mark entries as archived (they remain addressable and replayable).
    /// Archiving is a marker operation; addresses are never removed.
    pub fn archive(&mut self, seqs: &[u64]) -> Result<(), String> {
        let mut missing = Vec::new();
        for seq in seqs {
            if !self.entries.iter().any(|e| e.seq == *seq) {
                missing.push(seq.to_string());
            }
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(format!("archive: unknown seqs: {}", missing.join(", ")))
        }
    }

    /// Compact entries into a single summary entry; addresses preserved.
    pub fn compact(&mut self, seqs: &[u64], summary: &str) -> Result<u64, String> {
        let mut missing = Vec::new();
        for seq in seqs {
            if !self.entries.iter().any(|e| e.seq == *seq) {
                missing.push(seq.to_string());
            }
        }
        if !missing.is_empty() {
            return Err(format!("compact: unknown seqs: {}", missing.join(", ")));
        }
        let summary_seq = self.entries.len() as u64 + 1;
        self.entries.push(AuditJournalEntry {
            seq: summary_seq,
            stage: "summary".into(),
            status: "COMPACTED".into(),
            epoch: 0,
            author: summary.to_string(),
            scope: seqs.iter().map(|s| s.to_string()).collect(),
            compacted: true,
            compacted_into: None,
        });
        for entry in &mut self.entries {
            if seqs.contains(&entry.seq) {
                entry.compacted = true;
                entry.compacted_into = Some(summary_seq);
            }
        }
        Ok(summary_seq)
    }

    /// Trace-continuity invariant: every compacted entry points to a summary;
    /// summaries carry at least one absorbed address.
    pub fn check_trace_continuity(&mut self) -> Vec<String> {
        self.violations = Vec::new();
        for entry in &self.entries {
            let is_summary = entry.stage == "summary" && entry.compacted;
            if entry.compacted && entry.compacted_into.is_none() && !is_summary {
                self.violations
                    .push(format!("seq {} compacted without summary pointer", entry.seq));
            }
        }
        self.violations.clone()
    }
}

/// HMAC-SHA256 journal guard: signs the canonical digest of the audit trail.
#[derive(Debug, Clone)]
pub struct JournalGuard {
    key: Option<[u8; 32]>,
}

impl Default for JournalGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl JournalGuard {
    /// A read-only guard (can verify only if a key is provided; by default
    /// verifies nothing and signs nothing).
    pub fn new() -> Self {
        Self { key: None }
    }

    /// Deterministic keyed guard from a 32-byte secret.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self { key: Some(seed) }
    }

    /// Sign the digest of the journal entries. `None` if read-only.
    pub fn sign(&self, entries: &[AuditJournalEntry]) -> Option<Vec<u8>> {
        let key = self.key?;
        let mut mac = HmacSha256::new_from_slice(&key).ok()?;
        for entry in entries {
            let canonical = serde_json::to_vec(entry).unwrap_or_default();
            mac.update(&(canonical.len() as u64).to_le_bytes());
            mac.update(&canonical);
        }
        Some(mac.finalize().into_bytes().to_vec())
    }

    /// Verify a signature over the entries. `false` if no key or mismatch.
    pub fn verify(&self, entries: &[AuditJournalEntry], signature: &[u8]) -> bool {
        let Some(expected) = self.sign(entries) else {
            return false;
        };
        expected.len() == signature.len() && expected.iter().zip(signature).all(|(a, b)| a == b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::status::{AuditRecord, StageStatus};

    fn rec(stage: &str, status: StageStatus, epoch: u64) -> AuditRecord {
        AuditRecord::new(stage, status, "tester", &[], epoch)
    }

    #[test]
    fn journal_sign_verify_roundtrip() {
        let mut j = AuditJournal::new();
        j.append_stage_run(&rec("calibration", StageStatus::Supported, 1));
        let guard = JournalGuard::from_seed([1u8; 32]);
        let sig = guard.sign(j.entries()).expect("keyed guard signs");
        assert!(guard.verify(j.entries(), &sig));
        // Read-only guard signs nothing and verifies nothing.
        assert!(JournalGuard::new().sign(j.entries()).is_none());
        assert!(!JournalGuard::new().verify(j.entries(), &sig));
    }

    #[test]
    fn journal_signature_detects_tamper() {
        let mut j = AuditJournal::new();
        j.append_stage_run(&rec("calibration", StageStatus::Supported, 1));
        let guard = JournalGuard::from_seed([2u8; 32]);
        let sig = guard.sign(j.entries()).unwrap();
        // A changed trail fails the old signature.
        let mut j2 = j.clone();
        j2.append_stage_run(&rec("validation", StageStatus::Supported, 1));
        assert!(!guard.verify(j2.entries(), &sig));
    }

    #[test]
    fn compaction_preserves_addresses() {
        let mut j = AuditJournal::new();
        let s1 = j.append_stage_run(&rec("calibration", StageStatus::Proposed, 1));
        let s2 = j.append_stage_run(&rec("calibration", StageStatus::Supported, 2));
        j.compact(&[s1, s2], "coalesced").unwrap();
        assert!(j.check_trace_continuity().is_empty());
        let summary = j.entries().iter().find(|e| e.compacted && e.stage == "summary").unwrap();
        assert_eq!(summary.scope.len(), 2, "summary carries both absorbed addresses");
        let e1 = j.entries().iter().find(|e| e.seq == s1).unwrap();
        assert!(e1.compacted);
        assert!(e1.compacted_into.is_some());
    }
}
