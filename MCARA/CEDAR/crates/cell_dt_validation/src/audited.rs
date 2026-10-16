//! Audited pipeline runs — bridge between the `cell_dt_audit` discipline
//! (Demon Core Transfer 2026-10-16) and the real CEDAR pipeline stages.
//!
//! Wraps `ValidationSuite` and calibration/sensitivity runs in the demon-core
//! regime so that real pipeline execution is lawfully bounded:
//!
//! - **Dependency policy:** validation may be promoted to SUPPORTED only if an
//!   applicable policy exists for each upstream stage (calibration, sensitivity);
//!   otherwise the run BLOCKs with a classified STOP CARD (NEED / MISSING_RULE).
//! - **Transaction membrane:** a validation run cannot introduce the acceptance
//!   threshold it uses to authorize its own SUPPORTED status.
//! - **Non-destructive MARK:** every run appends an epoch-scoped audit record;
//!   the prior run history is preserved (TRACE CONTINUITY).
//!
//! The pipeline never fabricates a downstream consequence; it returns a STOP
//! CARD instead.

use cell_dt_audit::policy::StageDependencyPolicy;
use cell_dt_audit::status::StageStatus;
use cell_dt_audit::stop_card::{NeedClass, StageStopCard};
use cell_dt_audit::{PipelineAudit, PipelineStage};
use crate::validation::{ValidationResult, ValidationSuite};

/// Outcome of an audited validation run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStatus {
    /// All biomarkers acceptable → SUPPORTED.
    Accepted,
    /// Some acceptable, some not → CONTESTED (both sides preserved).
    PartiallyAccepted,
    /// None acceptable → REFUTED.
    Rejected,
    /// Empty suite → OPEN.
    Inconclusive,
}

impl ValidationStatus {
    fn to_stage_status(self) -> StageStatus {
        match self {
            Self::Accepted => StageStatus::Supported,
            Self::PartiallyAccepted => StageStatus::Contested,
            Self::Rejected => StageStatus::Refuted,
            Self::Inconclusive => StageStatus::Open,
        }
    }
}

/// Audited CEDAR pipeline: wraps real stage runs in the demon-core regime.
#[derive(Debug, Clone)]
pub struct AuditedPipeline {
    pub audit: PipelineAudit,
    /// The externally-declared acceptance threshold (R² > this).
    pub validation_threshold_r2: f64,
}

impl Default for AuditedPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditedPipeline {
    /// Threshold 0.75 matches `ValidationResult::is_acceptable()`.
    pub fn new() -> Self {
        Self {
            audit: PipelineAudit::new(),
            validation_threshold_r2: 0.75,
        }
    }

    /// Run a validation suite through the audit discipline.
    ///
    /// 1. Dependency policy: every upstream stage must have an applicable
    ///    policy, or the run BLOCKs (NEED / MISSING_RULE).
    /// 2. Membrane: `introduced_threshold` must be false — the acceptance
    ///    threshold is external, not invented by this run.
    /// 3. Status from the suite: Accepted / PartiallyAccepted / Rejected /
    ///    Inconclusive → Supported / Contested / Refuted / Open.
    /// 4. Non-destructive record (epoch-scoped) + journal entry.
    #[allow(clippy::result_large_err)] // a STOP CARD is deliberately information-rich
    pub fn run_validation(
        &mut self,
        suite: &ValidationSuite,
        author: &str,
        policies: &[StageDependencyPolicy],
        introduced_threshold: bool,
    ) -> Result<ValidationStatus, StageStopCard> {
        // 1. Dependency policy (lawful BLOCK if missing).
        self.audit
            .check_dependency(PipelineStage::Validation, StageStatus::Supported, policies)?;

        // 2. Transaction membrane: the run cannot authorize itself with a
        //    threshold it introduces in the same run.
        if introduced_threshold {
            return Err(StageStopCard::new(
                "validation / membrane",
                "promote validation to SUPPORTED",
                "acceptance threshold introduced by this run",
                "externally-declared acceptance threshold",
                NeedClass::MissingRule,
                false,
                None,
            ));
        }

        // 3. Derive the status from the suite (no fabrication: each status
        //    follows from the actual results).
        let status = self.status_from_suite(suite);

        // 4. Non-destructive record.
        self.audit.mark_stage(
            PipelineStage::Validation,
            status.to_stage_status(),
            author,
            &[&format!("r2>{:.2}", self.validation_threshold_r2)],
        );
        Ok(status)
    }

    /// Record a calibration run's status: converged (R-hat < 1.05) AND
    /// acceptable training R² → SUPPORTED; not converged → CONTESTED;
    /// converged but weak fit → OPEN.
    pub fn record_calibration(
        &mut self,
        r_hat_converged: bool,
        r2_training: f64,
        author: &str,
    ) -> StageStatus {
        let status = if r_hat_converged && r2_training > self.validation_threshold_r2 {
            StageStatus::Supported
        } else if !r_hat_converged {
            StageStatus::Contested
        } else {
            StageStatus::Open
        };
        self.audit
            .mark_stage(PipelineStage::Calibration, status, author, &["mcmc"])
            .status
    }

    /// Record a sensitivity run: stable under ±20% perturbation → SUPPORTED.
    pub fn record_sensitivity(&mut self, stable: bool, author: &str) -> StageStatus {
        let status = if stable {
            StageStatus::Supported
        } else {
            StageStatus::Contested
        };
        self.audit
            .mark_stage(PipelineStage::Sensitivity, status, author, &["w-perturbation"])
            .status
    }

    /// A full audited run in order: datasets → calibration → sensitivity →
    /// validation. If calibration or sensitivity did not reach SUPPORTED, the
    /// run returns the STOP CARD instead of fabricating validation.
    #[allow(clippy::result_large_err)] // a STOP CARD is deliberately information-rich
    pub fn run_full_pipeline(
        &mut self,
        r_hat_converged: bool,
        r2_training: f64,
        sensitivity_stable: bool,
        suite: &ValidationSuite,
        author: &str,
        policies: &[StageDependencyPolicy],
    ) -> Result<ValidationStatus, StageStopCard> {
        self.audit.mark_stage(
            PipelineStage::Datasets,
            StageStatus::Replicated,
            author,
            &["reference-datasets"],
        );
        self.record_calibration(r_hat_converged, r2_training, author);
        self.record_sensitivity(sensitivity_stable, author);
        // Validation may only run if calibration+sensitivity are SUPPORTED and
        // an applicable policy exists — otherwise a lawful STOP CARD.
        self.run_validation(suite, author, policies, false)
    }

    fn status_from_suite(&self, suite: &ValidationSuite) -> ValidationStatus {
        if suite.results.is_empty() {
            return ValidationStatus::Inconclusive;
        }
        let n_ok = suite
            .results
            .iter()
            .filter(|r| r.is_acceptable())
            .count();
        if n_ok == suite.results.len() {
            ValidationStatus::Accepted
        } else if n_ok == 0 {
            ValidationStatus::Rejected
        } else {
            ValidationStatus::PartiallyAccepted
        }
    }
}

/// Build a small `ValidationSuite` from raw biomarker fits (matches
/// `Calibrator::calculate_r2` / `calculate_rmse` outputs).
pub fn suite_from_fits(
    biomarker_name: &str,
    r_squared: f64,
    rmse: f64,
    mae: f64,
    n_points: usize,
) -> ValidationSuite {
    let mut suite = ValidationSuite::default();
    suite.add_result(ValidationResult {
        biomarker_name: biomarker_name.to_string(),
        r_squared,
        rmse,
        mae,
        n_points,
    });
    suite
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_policies() -> Vec<StageDependencyPolicy> {
        vec![
            StageDependencyPolicy::new(
                "calibration_must_be_supported",
                "calibration",
                "SUPPORTED -> SUPPORTED",
                "validation",
            ),
            StageDependencyPolicy::new(
                "sensitivity_must_be_supported",
                "sensitivity",
                "SUPPORTED -> SUPPORTED",
                "validation",
            ),
        ]
    }

    #[test]
    fn full_pipeline_run_audits_validation_non_destructively() {
        let mut p = AuditedPipeline::new();
        let suite = suite_from_fits("CHIP", 0.79, 0.05, 0.04, 5);

        let outcome = p
            .run_full_pipeline(true, 0.82, true, &suite, "ci", &default_policies())
            .expect("converged calibration + stable sensitivity + policies → validation runs");
        assert_eq!(outcome, ValidationStatus::Accepted);

        // Second run appends a new epoch; history keeps both runs.
        p.run_full_pipeline(true, 0.80, true, &suite, "ci", &default_policies())
            .unwrap();
        let history = p.audit.history_of(PipelineStage::Validation);
        assert_eq!(history.len(), 2, "prior validation runs must be kept");
        assert_eq!(history[0].epoch, 1);
        assert_eq!(history[1].epoch, 2);
        assert_eq!(
            p.audit.current_status(PipelineStage::Validation),
            Some(StageStatus::Supported)
        );
        // Journal is append-only and signed.
        let guard = cell_dt_audit::journal::JournalGuard::from_seed([9u8; 32]);
        let sig = p.audit.sign_journal(&guard).unwrap();
        assert!(p.audit.verify_journal(&guard, &sig));
    }

    #[test]
    fn validation_blocks_when_upstream_not_supported() {
        // Calibration contested (not converged) → validation must BLOCK with a
        // classified STOP CARD (NEED / MISSING_RULE), not fabricate.
        let mut p = AuditedPipeline::new();
        let suite = suite_from_fits("CHIP", 0.79, 0.05, 0.04, 5);
        p.record_calibration(false, 0.82, "ci"); // not converged → CONTESTED

        let err = p
            .run_validation(&suite, "ci", &default_policies(), false)
            .unwrap_err();
        assert_eq!(err.stop_class, NeedClass::MissingRule);
        assert!(!err.can_ask_outside);
        assert_eq!(
            p.audit.current_status(PipelineStage::Validation),
            None,
            "validation must NOT be recorded when it could not run lawfully"
        );
    }

    #[test]
    fn validation_membrane_rejects_self_introduced_threshold() {
        let mut p = AuditedPipeline::new();
        p.record_calibration(true, 0.82, "ci");
        p.record_sensitivity(true, "ci");
        let suite = suite_from_fits("CHIP", 0.79, 0.05, 0.04, 5);
        // The run tries to introduce its own acceptance threshold → banned.
        let err = p
            .run_validation(&suite, "ci", &default_policies(), true)
            .unwrap_err();
        assert_eq!(err.stop_class, NeedClass::MissingRule);
        assert_eq!(
            p.audit.current_status(PipelineStage::Validation),
            None
        );
    }

    #[test]
    fn status_reflects_actual_results_no_fabrication() {
        // Weak fits → REFUTED; mixed → CONTESTED; empty → OPEN.
        let mut p = AuditedPipeline::new();
        p.record_calibration(true, 0.82, "ci");
        p.record_sensitivity(true, "ci");

        let weak = suite_from_fits("CHIP", 0.5, 0.2, 0.15, 5);
        let out = p
            .run_validation(&weak, "ci", &default_policies(), false)
            .unwrap();
        assert_eq!(out, ValidationStatus::Rejected);

        let mut mixed = ValidationSuite::default();
        mixed.add_result(ValidationResult {
            biomarker_name: "CHIP".into(),
            r_squared: 0.8,
            rmse: 0.05,
            mae: 0.04,
            n_points: 5,
        });
        mixed.add_result(ValidationResult {
            biomarker_name: "ROS".into(),
            r_squared: 0.6,
            rmse: 0.1,
            mae: 0.08,
            n_points: 4,
        });
        let out = p
            .run_validation(&mixed, "ci", &default_policies(), false)
            .unwrap();
        assert_eq!(out, ValidationStatus::PartiallyAccepted);
    }
}
