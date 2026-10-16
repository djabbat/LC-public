//! FIRST RUN — CEDAR pipeline-audit end-to-end walk (Demon Core transfer,
//! 2026-10-16). Mirrors the DAIS `demon_first_run` scenario but for the CEDAR
//! computational pipeline (calibration → sensitivity → validation → datasets):
//!
//!   1. datasets stage recorded → calibration PROPOSED→SUPPORTED (epochs,
//!      non-destructive history)
//!   2. transaction membrane: calibration cannot authorize itself with a
//!      parameter it introduces
//!   3. validation promotion without an applicable dependency policy → lawful
//!      BLOCK + STOP CARD (NEED / MISSING_RULE)
//!   4. dependency policy present → validation promotion passes
//!   5. composition semantics: two calibration datasets, one challenged —
//!      ALTERNATIVE survives, JOINT does not
//!   6. audit journal signed (HMAC-SHA256), tamper detected, compaction
//!      preserves addresses (TRACE CONTINUITY)
//!
//! Run: `cargo test -p cell_dt_audit`

use cell_dt_audit::journal::JournalGuard;
use cell_dt_audit::policy::{
    evaluate_composition, CompositionPolicy, JustificationComposition, CompositionVerdict,
    StageDependencyPolicy,
};
use cell_dt_audit::status::StageStatus;
use cell_dt_audit::stop_card::{NeedClass, RegimeState};
use cell_dt_audit::{PipelineAudit, PipelineStage};

#[test]
fn ced_pipeline_audit_first_run() {
    let mut audit = PipelineAudit::new();

    // ── 1. Non-destructive stage status ────────────────────────────────
    // Datasets ingested.
    audit.mark_stage(
        PipelineStage::Datasets,
        StageStatus::Replicated, // ROS + telomere + CHIP VAF + MCAI datasets
        "cedar-pipeline",
        &["reference-datasets-v3.2.3"],
    );
    // Calibration: first run PROPOSED, then recalibrated SUPPORTED.
    audit.mark_stage(
        PipelineStage::Calibration,
        StageStatus::Proposed,
        "mcmc",
        &["tau_protection, pi_0"],
    );
    audit.mark_stage(
        PipelineStage::Calibration,
        StageStatus::Supported,
        "mcmc",
        &["posterior R-hat < 1.05"],
    );

    // Current status = SUPPORTED (epoch 2); history keeps PROPOSED (epoch 1).
    assert_eq!(
        audit.current_status(PipelineStage::Calibration),
        Some(StageStatus::Supported)
    );
    let cal_history = audit.history_of(PipelineStage::Calibration);
    assert_eq!(cal_history.len(), 2, "prior run must be kept (non-destructive)");
    assert_eq!(cal_history[0].epoch, 1);
    assert_eq!(cal_history[1].epoch, 2);

    // ── 2. Transaction membrane ─────────────────────────────────────────
    // A calibration run introduces its own posterior mean and then tries to
    // authorize its own SUPPORTED status with that same posterior — banned.
    let err = audit
        .membrane_check(
            PipelineStage::Calibration,
            "posterior_tau_protection",      // the authorizing param
            &["posterior_tau_protection"],   // the param this run introduces
        )
        .unwrap_err();
    assert_eq!(err.to_string().contains("transaction membrane"), true);

    // An externally sourced param (from committed N-1) is allowed.
    assert!(audit
        .membrane_check(
            PipelineStage::Calibration,
            "literature_alpha", // from literature, committed earlier
            &["posterior_tau_protection"],
        )
        .is_ok());

    // ── 3. Dependency policy: validation promotion, NO policy → BLOCK ───
    let err = audit.check_dependency(
        PipelineStage::Validation,
        StageStatus::Supported,
        &[], // no policies registered
    );
    let card = err.unwrap_err();
    assert_eq!(card.stop_class, NeedClass::MissingRule);
    assert!(!card.can_ask_outside, "NEED, not ASK");
    assert_eq!(card.where_.contains("Validation"), true);
    // The pipeline does not fabricate the downstream consequence.
    assert_eq!(audit.current_status(PipelineStage::Validation), None);

    // ── 4. Dependency policy present → promotion passes ─────────────────
    let policies = vec![
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
    ];
    audit.mark_stage(
        PipelineStage::Sensitivity,
        StageStatus::Supported,
        "sensitivity",
        &["w-perturbation ±20% stable"],
    );
    // Still blocked: calibration policy applies, sensitivity policy applies,
    // but validation's dependency list is [calibration, sensitivity] — both
    // policies are present now.
    assert!(audit
        .check_dependency(
            PipelineStage::Validation,
            StageStatus::Supported,
            &policies,
        )
        .is_ok());
    audit.mark_stage(
        PipelineStage::Validation,
        StageStatus::Supported,
        "validation",
        &["R² > 0.75"],
    );
    assert_eq!(
        audit.current_status(PipelineStage::Validation),
        Some(StageStatus::Supported)
    );

    // ── 5. Composition semantics (DCT19–DCT20) ──────────────────────────
    // Two calibration datasets; one later challenged.
    let alt = CompositionPolicy {
        policy_id: "c-alternative".into(),
        result_id: "calibration".into(),
        composition: JustificationComposition::Alternative,
        justification_ids: vec!["ROS".into(), "telomere".into()],
    };
    // ROS dataset challenged (false), telomere still supported (true).
    assert_eq!(
        evaluate_composition(&alt, &[false, true]),
        CompositionVerdict::Supported
    );
    let joint = CompositionPolicy {
        policy_id: "c-joint".into(),
        result_id: "calibration".into(),
        composition: JustificationComposition::Joint,
        justification_ids: vec!["ROS".into(), "telomere".into()],
    };
    // same graph shape, different policy → different verdict
    assert_eq!(
        evaluate_composition(&joint, &[false, true]),
        CompositionVerdict::NotSupported
    );

    // ── 6. Tamper-evident audit journal ─────────────────────────────────
    let guard = JournalGuard::from_seed([7u8; 32]);
    let sig = audit.sign_journal(&guard).expect("keyed guard signs");
    assert!(audit.verify_journal(&guard, &sig), "honest trail verifies");

    // Any change to the committed trail invalidates the old signature.
    let mut grown = audit.clone();
    grown.mark_stage(
        PipelineStage::Simulation,
        StageStatus::Proposed,
        "sim",
        &["new run"],
    );
    assert!(
        !grown.verify_journal(&guard, &sig),
        "old signature must not cover a changed trail (tamper-evident)"
    );

    // Compaction preserves addresses.
    let seqs: Vec<u64> = audit.journal.entries().iter().map(|e| e.seq).take(2).collect();
    audit.journal.compact(&seqs, "coalesced early runs").unwrap();
    assert!(audit.journal.check_trace_continuity().is_empty());
    let sig2 = audit.sign_journal(&guard).expect("re-sign after compaction");
    assert!(audit.verify_journal(&guard, &sig2), "re-signed trail verifies");

    // ── Regime sanity ───────────────────────────────────────────────────
    assert_ne!(RegimeState::Need, RegimeState::Go);
    assert_eq!(NeedClass::MissingRule, NeedClass::MissingRule);
}
