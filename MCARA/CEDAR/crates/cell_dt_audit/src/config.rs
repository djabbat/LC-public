//! Data-driven pipeline policies (Demon Core Transfer, step 4 — 2026-10-16).
//!
//! The dependency graph and the dependency policies are declared in a config
//! file (JSON), not hardcoded. Policies match on a TYPED `StatusTransition`
//! (from → to), so a policy with the wrong transition type never matches —
//! no string comparison, no silent renames.
//!
//! Example config: `crates/cell_dt_audit/config/cedar_pipeline.json`.

use crate::status::StageStatus;
use crate::PipelineStage;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A typed status transition `from -> to` (UPPERCASE wire form).
///
/// Serializes to the human-friendly string `"SUPPORTED -> SUPPORTED"`;
/// deserializes from that string OR from `{"from": .., "to": ..}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusTransition {
    pub from: StageStatus,
    pub to: StageStatus,
}

impl Serialize for StatusTransition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_str())
    }
}

impl<'de> Deserialize<'de> for StatusTransition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Repr {
            Str(String),
            Obj { from: StageStatus, to: StageStatus },
        }
        match Repr::deserialize(deserializer)? {
            Repr::Str(s) => Self::parse(&s).map_err(serde::de::Error::custom),
            Repr::Obj { from, to } => Ok(Self { from, to }),
        }
    }
}

impl StatusTransition {
    pub fn new(from: StageStatus, to: StageStatus) -> Self {
        Self { from, to }
    }

    /// Canonical string form, e.g. `SUPPORTED -> SUPPORTED`.
    pub fn as_str(self) -> String {
        format!("{} -> {}", self.from.as_str(), self.to.as_str())
    }

    /// Parse `"SUPPORTED -> SUPPORTED"` into a typed transition.
    pub fn parse(s: &str) -> Result<Self, String> {
        let mut parts = s.split("->");
        let from = parts.next().ok_or("missing 'from' in transition")?.trim();
        let to = parts.next().ok_or("missing 'to' in transition")?.trim();
        if parts.next().is_some() {
            return Err(format!("malformed transition: '{s}' (too many arrows)"));
        }
        Ok(Self {
            from: StageStatus::try_from(from)?,
            to: StageStatus::try_from(to)?,
        })
    }
}

/// A data-driven dependency policy (ROLE = POLICY).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DependencyPolicyConfig {
    pub policy_id: String,
    pub name: String,
    pub upstream_stage: String,
    pub upstream_transition: StatusTransition,
    pub downstream_stage: String,
    #[serde(default = "default_role")]
    pub role: String,
}

fn default_role() -> String {
    "POLICY".into()
}

/// A declared dependency edge: `downstream` depends on `upstreams`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub downstream: String,
    pub upstreams: Vec<String>,
}

/// The whole data-driven policy file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub version: u32,
    /// Declared dependency edges (replaces the hardcoded default graph when set).
    #[serde(default)]
    pub graph: Vec<DependencyEdge>,
    /// Declared dependency policies.
    #[serde(default)]
    pub policies: Vec<DependencyPolicyConfig>,
}

impl PolicyConfig {
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("invalid PolicyConfig JSON: {e}"))
    }

    pub fn from_file(path: &str) -> Result<Self, String> {
        let json = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read policy config {path}: {e}"))?;
        Self::from_json(&json)
    }

    /// Upstream stages declared for a stage (from the config graph).
    pub fn upstreams_of(&self, stage: PipelineStage) -> Vec<PipelineStage> {
        self.graph
            .iter()
            .find(|e| e.downstream == stage.as_str())
            .map(|e| {
                e.upstreams
                    .iter()
                    .filter_map(|s| PipelineStage::try_from(s.as_str()).ok())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Does an applicable policy exist for this upstream transition?
    pub fn has_applicable_policy(
        &self,
        upstream: PipelineStage,
        transition: StatusTransition,
        downstream: PipelineStage,
    ) -> bool {
        self.policies.iter().any(|p| {
            p.upstream_stage == upstream.as_str()
                && p.upstream_transition == transition
                && p.downstream_stage == downstream.as_str()
        })
    }

    /// Convert the declared policies to the legacy string-based view
    /// (for callers that still use `StageDependencyPolicy`).
    pub fn as_legacy_policies(&self) -> Vec<crate::policy::StageDependencyPolicy> {
        self.policies
            .iter()
            .map(|p| crate::policy::StageDependencyPolicy {
                policy_id: p.policy_id.clone(),
                name: p.name.clone(),
                upstream_stage: p.upstream_stage.clone(),
                upstream_transition: p.upstream_transition.as_str(),
                downstream_stage: p.downstream_stage.clone(),
                role: p.role.clone(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = r#"{
        "version": 1,
        "graph": [
            {"downstream": "validation", "upstreams": ["calibration", "sensitivity"]},
            {"downstream": "simulation", "upstreams": ["calibration", "validation"]}
        ],
        "policies": [
            {"policy_id": "p-cal-v", "name": "calibration_must_be_supported",
             "upstream_stage": "calibration", "upstream_transition": "SUPPORTED -> SUPPORTED",
             "downstream_stage": "validation"},
            {"policy_id": "p-sens-v", "name": "sensitivity_must_be_supported",
             "upstream_stage": "sensitivity", "upstream_transition": "SUPPORTED -> SUPPORTED",
             "downstream_stage": "validation"}
        ]
    }"#;

    #[test]
    fn parses_config_and_resolves_graph() {
        let cfg = PolicyConfig::from_json(VALID).expect("valid config");
        assert_eq!(cfg.version, 1);
        let ups = cfg.upstreams_of(PipelineStage::Validation);
        assert_eq!(ups, vec![PipelineStage::Calibration, PipelineStage::Sensitivity]);
        assert!(cfg.upstreams_of(PipelineStage::Datasets).is_empty());
    }

    #[test]
    fn typed_transition_roundtrip() {
        let t = StatusTransition::parse("SUPPORTED -> CONTESTED").unwrap();
        assert_eq!(t, StatusTransition::new(StageStatus::Supported, StageStatus::Contested));
        assert_eq!(t.as_str(), "SUPPORTED -> CONTESTED");
        assert!(StatusTransition::parse("SUPPORTED -> BOGUS").is_err());
        assert!(StatusTransition::parse("SUPPORTED").is_err());
    }

    #[test]
    fn policy_matches_typed_transition_only() {
        let cfg = PolicyConfig::from_json(VALID).unwrap();
        // Exact transition → applicable.
        assert!(cfg.has_applicable_policy(
            PipelineStage::Calibration,
            StatusTransition::new(StageStatus::Supported, StageStatus::Supported),
            PipelineStage::Validation,
        ));
        // Different typed transition → NOT applicable (no string-lenient match).
        assert!(!cfg.has_applicable_policy(
            PipelineStage::Calibration,
            StatusTransition::new(StageStatus::Contested, StageStatus::Supported),
            PipelineStage::Validation,
        ));
        // Wrong downstream → NOT applicable.
        assert!(!cfg.has_applicable_policy(
            PipelineStage::Calibration,
            StatusTransition::new(StageStatus::Supported, StageStatus::Supported),
            PipelineStage::Simulation,
        ));
    }

    #[test]
    fn legacy_view_roundtrips() {
        let cfg = PolicyConfig::from_json(VALID).unwrap();
        let legacy = cfg.as_legacy_policies();
        assert_eq!(legacy.len(), 2);
        assert_eq!(legacy[0].upstream_transition, "SUPPORTED -> SUPPORTED");
        assert_eq!(legacy[0].role, "POLICY");
    }

    #[test]
    fn invalid_json_is_error() {
        assert!(PolicyConfig::from_json("{not json").is_err());
    }
}
