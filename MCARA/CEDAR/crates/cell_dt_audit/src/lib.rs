//! CEDAR pipeline-audit discipline — transfer of the Noepedia Demon Core
//! principles (Demon Core Harvest 2026-10-16, DCT01–DCT20) to the CEDAR
//! computational pipeline audit.
//!
//! The CEDAR pipeline (calibration → sensitivity → validation → datasets)
//! must obey the same executive discipline as the ignorant demon:
//!
//! 1. **Non-destructive stage status / TRACE CONTINUITY** — re-running or
//!    archiving a pipeline stage never erases the prior record. A stage's
//!    status is an addressable assertion with an `epoch`; the history is kept.
//! 2. **STOP / NEED** — a stage that cannot proceed lawfully (missing
//!    calibration data, conflicting constraints) returns a classified
//!    `StageStopCard` (GO/ASK/BLOCK/NEED); it never fabricates a result.
//! 3. **Transaction membrane** — a pipeline run cannot use a parameter/rule it
//!    introduces in the same run to authorize its own output (a calibration
//!    cannot validate itself with its own posterior without a fresh run).
//! 4. **Stage dependency policy lives in the network** — a downstream stage's
//!    validity depends on an upstream stage's status through an addressable
//!    policy; without an applicable policy the pipeline BLOCKs (NEED).
//! 5. **Composition semantics** — multiple calibration witnesses (datasets)
//!    compose as ALTERNATIVE or JOINT; multiplicity ≠ redundancy.
//! 6. **Tamper-evident audit journal** — the append-only audit trail is signed
//!    (HMAC-SHA256); compaction/archiving are legitimate re-signing events,
//!    silent rewrites fail verification.
//!
//! Master document: `docs/DEMON_CORE_TRANSFER_2026-10-16.md`.

pub mod journal;
pub mod policy;
pub mod status;
pub mod stop_card;

pub use journal::{AuditJournal, AuditJournalEntry, JournalGuard};
pub use policy::{
    evaluate_composition, CompositionPolicy, CompositionVerdict, JustificationComposition,
    StageDependencyPolicy,
};
pub use status::{AuditRecord, HistoryStatus, StageStatus};
pub use stop_card::{NeedClass, RegimeState, StageStopCard};

use serde::{Deserialize, Serialize};

/// The stages of the CEDAR computational pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineStage {
    /// Bayesian MCMC calibration of free parameters (tau_protection, pi_0).
    Calibration,
    /// Sensitivity analysis of the D(t) weight vector.
    Sensitivity,
    /// Validation of model outputs against reference datasets.
    Validation,
    /// Reference-dataset preparation / ingestion.
    Datasets,
    /// Simulation run (cell_dt_core → aging engine).
    Simulation,
}

impl PipelineStage {
    /// Which upstream stages a stage depends on by default (declared edge).
    pub fn upstream_dependencies(self) -> &'static [PipelineStage] {
        match self {
            PipelineStage::Calibration => &[PipelineStage::Datasets],
            PipelineStage::Sensitivity => &[PipelineStage::Calibration],
            PipelineStage::Validation => &[PipelineStage::Calibration, PipelineStage::Sensitivity],
            PipelineStage::Datasets => &[],
            PipelineStage::Simulation => &[PipelineStage::Calibration, PipelineStage::Validation],
        }
    }

    /// Canonical string form (snake_case, matches the serde wire format).
    pub fn as_str(self) -> &'static str {
        match self {
            PipelineStage::Calibration => "calibration",
            PipelineStage::Sensitivity => "sensitivity",
            PipelineStage::Validation => "validation",
            PipelineStage::Datasets => "datasets",
            PipelineStage::Simulation => "simulation",
        }
    }
}

/// The pipeline-audit orchestrator: records stage runs non-destructively,
/// checks dependency policies, and produces STOP CARDS instead of fabricating.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PipelineAudit {
    /// Non-destructive stage records (history preserved).
    pub records: Vec<AuditRecord>,
    /// The append-only audit journal (tamper-evident).
    pub journal: AuditJournal,
}

impl PipelineAudit {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a stage run as a NEW assertion (non-destructive MARK):
    /// the previous status for this stage remains in `history`; the current
    /// epistemic status becomes the new value with `epoch += 1`.
    pub fn mark_stage(
        &mut self,
        stage: PipelineStage,
        status: StageStatus,
        author: &str,
        scope: &[&str],
    ) -> AuditRecord {
        let record =
            AuditRecord::new(stage.as_str(), status, author, scope, self.current_epoch(stage));
        self.records.push(record.clone());
        self.journal.append_stage_run(&record);
        record
    }

    /// Current status of a stage (last applicable assertion), if any.
    pub fn current_status(&self, stage: PipelineStage) -> Option<StageStatus> {
        self.records
            .iter()
            .rev()
            .find(|r| r.stage == stage.as_str())
            .map(|r| r.status)
    }

    /// Current epoch for a stage (number of prior assertions + 1).
    fn current_epoch(&self, stage: PipelineStage) -> u64 {
        self.records
            .iter()
            .filter(|r| r.stage == stage.as_str())
            .count() as u64
            + 1
    }

    /// History of a stage's status assertions (never erased).
    pub fn history_of(&self, stage: PipelineStage) -> Vec<&AuditRecord> {
        self.records
            .iter()
            .filter(|r| r.stage == stage.as_str())
            .collect()
    }

    /// Transaction membrane: a stage run cannot use a parameter/rule it
    /// introduces in the same run to authorize its own output.
    ///
    /// `authorizing_param` is the param/rule the run claims as its authorizer;
    /// `introduced_params` are the params this run itself introduces. If the
    /// run authorizes itself with a param it introduced → SELF_AUTHORIZATION.
    pub fn membrane_check(
        &self,
        stage: PipelineStage,
        authorizing_param: &str,
        introduced_params: &[&str],
    ) -> Result<(), MembraneError> {
        if introduced_params.contains(&authorizing_param) {
            return Err(MembraneError::SelfAuthorization {
                stage,
                param: authorizing_param.to_string(),
            });
        }
        Ok(())
    }

    /// Stage dependency policy: a stage may be promoted to `wanted` only if an
    /// applicable upstream policy exists for each declared dependency.
    ///
    /// Without an applicable policy the pipeline must not invent the
    /// downstream consequence — it BLOCKs and returns a STOP CARD (NEED).
    #[allow(clippy::result_large_err)] // a STOP CARD is deliberately information-rich
    pub fn check_dependency(
        &self,
        stage: PipelineStage,
        wanted: StageStatus,
        policies: &[StageDependencyPolicy],
    ) -> Result<(), StageStopCard> {
        for upstream in stage.upstream_dependencies() {
            let up_status = self.current_status(*upstream);
            let Some(up_status) = up_status else {
                return Err(stage_missing_upstream(stage, *upstream));
            };
            let transition = format!("{} -> {}", up_status.as_str(), wanted.as_str());
            let applicable = policies.iter().any(|p| {
                p.upstream_stage == upstream.as_str()
                    && p.upstream_transition == transition
                    && p.downstream_stage == stage.as_str()
            });
            if !applicable {
                return Err(StageStopCard::new(
                    format!("{:?} promotion", stage),
                    &format!("promote {:?} to {:?}", stage, wanted),
                    &format!("upstream {:?} = {:?}", upstream, up_status),
                    &format!("applicable dependency policy for '{transition}'"),
                    NeedClass::MissingRule,
                    false,
                    None,
                ));
            }
        }
        Ok(())
    }

    /// Sign the audit trail (HMAC-SHA256) and verify it.
    pub fn sign_journal(&self, guard: &JournalGuard) -> Option<Vec<u8>> {
        guard.sign(self.journal.entries())
    }

    pub fn verify_journal(&self, guard: &JournalGuard, signature: &[u8]) -> bool {
        guard.verify(self.journal.entries(), signature)
    }
}

/// Transaction-membrane violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MembraneError {
    SelfAuthorization {
        stage: PipelineStage,
        param: String,
    },
}

impl std::fmt::Display for MembraneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SelfAuthorization { stage, param } => write!(
                f,
                "transaction membrane: stage {:?} authorizes itself with parameter/rule '{param}' \
                 it introduces in the same run (rule-set for run N must come from committed N-1)",
                stage
            ),
        }
    }
}

impl std::error::Error for MembraneError {}

/// Build the canonical STOP CARD when an upstream stage was never recorded.
fn stage_missing_upstream(stage: PipelineStage, upstream: PipelineStage) -> StageStopCard {
    StageStopCard::new(
        format!("{:?} requires {:?}", stage, upstream),
        &format!("run {:?} with valid upstream", stage),
        "upstream stage never recorded",
        &format!("upstream {:?} run record", upstream),
        NeedClass::MissingFact,
        false,
        None,
    )
}
