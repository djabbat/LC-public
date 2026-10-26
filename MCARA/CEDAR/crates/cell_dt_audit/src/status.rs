//! Non-destructive stage status — CEDAR pipeline-audit mirror of the Auditor
//! Core `HISTORY_STATUS`/`CURRENT_EPISTEMIC_STATUS` separation and the
//! non-destructive MARK (DCT11–DCT12).
//!
//! A pipeline stage run is an addressable assertion with an `epoch`. Marking a
//! stage with a new status NEVER erases the prior record; the current status is
//! the last applicable assertion and the history is preserved as provenance.

use serde::{Deserialize, Serialize};

/// Stage-status ladder — mirror of the Aletheia Evidentiary Ladder
/// (GRADE / Murad / CEBM / TRL mapping, see `EVIDENCE.md` of CEDAR).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum StageStatus {
    /// Stage configured, not yet run.
    Proposed,
    /// Ran once with plausible output.
    Supported,
    /// Ran under an explicit verified procedure.
    Tested,
    /// Independently reproduced (≥2 witnesses).
    Replicated,
    /// Output contradicted by evidence.
    Refuted,
    /// Supported positions incompatible; both preserved.
    Contested,
    /// Insufficient evidence / missing inputs.
    Open,
    /// Valid only under stated conditions.
    ValidInContext,
    /// Replaced by a newer formulation.
    Superseded,
    /// Not actively maintained.
    Stale,
}

impl TryFrom<&str> for StageStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "PROPOSED" => Self::Proposed,
            "SUPPORTED" => Self::Supported,
            "TESTED" => Self::Tested,
            "REPLICATED" => Self::Replicated,
            "REFUTED" => Self::Refuted,
            "CONTESTED" => Self::Contested,
            "OPEN" => Self::Open,
            "VALID_IN_CONTEXT" | "VALIDINCONTEXT" => Self::ValidInContext,
            "SUPERSEDED" => Self::Superseded,
            "STALE" => Self::Stale,
            other => return Err(format!("unknown StageStatus: {other}")),
        })
    }
}

impl StageStatus {
    /// Canonical UPPERCASE wire form (matches the serde representation).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "PROPOSED",
            Self::Supported => "SUPPORTED",
            Self::Tested => "TESTED",
            Self::Replicated => "REPLICATED",
            Self::Refuted => "REFUTED",
            Self::Contested => "CONTESTED",
            Self::Open => "OPEN",
            Self::ValidInContext => "VALID_IN_CONTEXT",
            Self::Superseded => "SUPERSEDED",
            Self::Stale => "STALE",
        }
    }
}

/// History-status of a committed stage run: the committed fact is stable; it
/// may be compacted/archived/superseded but never made unaddressable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HistoryStatus {
    Committed,
    Archived,
    Compacted,
    Superseded,
}

/// One addressable stage-run record (non-destructive MARK).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditRecord {
    pub record_id: String,
    pub stage: String,
    pub status: StageStatus,
    pub scope: Vec<String>,
    /// Monotonic epoch; prior records for the same stage remain in history.
    pub epoch: u64,
    pub author: String,
    pub history_status: HistoryStatus,
}

impl AuditRecord {
    pub fn new(
        stage: impl Into<String>,
        status: StageStatus,
        author: &str,
        scope: &[&str],
        epoch: u64,
    ) -> Self {
        let stage = stage.into();
        Self {
            record_id: format!("{}-{}-{}", stage, epoch, hex_fingerprint(&format!("{author}{epoch}"))),
            stage,
            status,
            scope: scope.iter().map(|s| s.to_string()).collect(),
            epoch,
            author: author.to_string(),
            history_status: HistoryStatus::Committed,
        }
    }
}

/// Small stable fingerprint for record ids (FNV-1a, 8 hex chars) — ids are
/// for provenance addressing, not cryptography.
fn hex_fingerprint(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:08x}", hash)
}
