//! Stage dependency policies & composition semantics — mirror of the Demon
//! Core dependency-policy-in-the-network (DCT05–DCT07) and justification
//! composition (DCT19–DCT20).
//!
//! Downstream stage validity is determined by an addressable policy, not by a
//! core primitive; if no applicable policy exists, the pipeline BLOCKs with
//! `NEED / MISSING_RULE`. Multiple calibration witnesses compose as
//! ALTERNATIVE or JOINT; multiplicity ≠ redundancy.

use crate::stop_card::NeedClass;
use crate::stop_card::StageStopCard;
use serde::{Deserialize, Serialize};

/// An addressable dependency policy: how an upstream stage's status transition
/// propagates to a downstream stage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageDependencyPolicy {
    pub policy_id: String,
    pub name: String,
    pub upstream_stage: String,
    /// e.g. "SUPPORTED -> CONTESTED"
    pub upstream_transition: String,
    pub downstream_stage: String,
    /// ROLE = "POLICY".
    pub role: String,
}

impl StageDependencyPolicy {
    pub fn new(
        name: &str,
        upstream_stage: &str,
        upstream_transition: &str,
        downstream_stage: &str,
    ) -> Self {
        Self {
            policy_id: format!("policy-{}", upstream_stage),
            name: name.to_string(),
            upstream_stage: upstream_stage.to_string(),
            upstream_transition: upstream_transition.to_string(),
            downstream_stage: downstream_stage.to_string(),
            role: "POLICY".into(),
        }
    }
}

/// How multiple justifications compose (DCT19–DCT20).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum JustificationComposition {
    /// Any one complete supported justification suffices (e.g., independent
    /// datasets).
    Alternative,
    /// All listed justifications are required together.
    Joint,
}

/// Addressable composition policy for a result's justifications.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompositionPolicy {
    pub policy_id: String,
    /// The result/record this policy governs.
    pub result_id: String,
    pub composition: JustificationComposition,
    pub justification_ids: Vec<String>,
}

/// Outcome of evaluating a result's survival under a composition policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CompositionVerdict {
    Supported,
    NotSupported,
    /// No composition policy applies — lawful STOP (NEED / MISSING_RULE).
    PolicyMissing,
}

/// Evaluate whether a result survives a challenge under its composition policy.
/// `witness_supported[i]` is the current status of `policy.justification_ids[i]`.
///
/// - ALTERNATIVE: any one currently supported witness suffices.
/// - JOINT: all listed witnesses are required together.
pub fn evaluate_composition(
    policy: &CompositionPolicy,
    witness_supported: &[bool],
) -> CompositionVerdict {
    if witness_supported.len() != policy.justification_ids.len() {
        return CompositionVerdict::PolicyMissing;
    }
    match policy.composition {
        JustificationComposition::Alternative => {
            if witness_supported.iter().any(|&b| b) {
                CompositionVerdict::Supported
            } else {
                CompositionVerdict::NotSupported
            }
        }
        JustificationComposition::Joint => {
            if witness_supported.iter().all(|&b| b) {
                CompositionVerdict::Supported
            } else {
                CompositionVerdict::NotSupported
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alternative_survives_when_one_witness_remains() {
        let p = CompositionPolicy {
            policy_id: "p1".into(),
            result_id: "calibration".into(),
            composition: JustificationComposition::Alternative,
            justification_ids: vec!["ROS".into(), "telomere".into()],
        };
        assert_eq!(evaluate_composition(&p, &[false, true]), CompositionVerdict::Supported);
        assert_eq!(evaluate_composition(&p, &[false, false]), CompositionVerdict::NotSupported);
    }

    #[test]
    fn joint_requires_all_witnesses() {
        let p = CompositionPolicy {
            policy_id: "p2".into(),
            result_id: "calibration".into(),
            composition: JustificationComposition::Joint,
            justification_ids: vec!["ROS".into(), "telomere".into()],
        };
        assert_eq!(evaluate_composition(&p, &[false, true]), CompositionVerdict::NotSupported);
        assert_eq!(evaluate_composition(&p, &[true, true]), CompositionVerdict::Supported);
    }

    #[test]
    fn mismatched_witnesses_is_policy_missing() {
        let p = CompositionPolicy {
            policy_id: "p3".into(),
            result_id: "r".into(),
            composition: JustificationComposition::Alternative,
            justification_ids: vec!["a".into(), "b".into()],
        };
        assert_eq!(evaluate_composition(&p, &[true]), CompositionVerdict::PolicyMissing);
    }
}

/// Convenience: STOP CARD for a missing dependency policy.
pub fn missing_policy_stop_card(
    where_: &str,
    transition: &str,
) -> StageStopCard {
    StageStopCard::new(
        where_,
        "propagate upstream stage status downstream",
        &format!("upstream transition: {transition}"),
        "applicable stage dependency policy",
        NeedClass::MissingRule,
        false,
        None,
    )
}
