//! χ_Ze index (THEORY §3.3, §3.4).
//!
//! Convention-aware: the formula
//!   `χ_Ze = 1 − |v_obs − v*| / max(v*, 1 − v*)`
//! is well-defined only relative to a velocity-scale convention. See
//! `VelocityConvention` and `datasets/MIGRATION_NOTES.md §2`.

use crate::{BioSenseError, Result, VelocityConvention, consts::V_STAR_PYTHON};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChiZeWeights {
    pub eeg: f64,
    pub hrv: f64,
    pub resp: f64,
    pub sleep: f64,
}

impl Default for ChiZeWeights {
    fn default() -> Self {
        Self { eeg: 0.30, hrv: 0.30, resp: 0.20, sleep: 0.20 }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChiZeBreakdown {
    pub eeg: f64,
    pub hrv: f64,
    pub resp: f64,
    pub sleep: f64,
    pub composite: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct ChiZeIndex {
    pub weights: ChiZeWeights,
}

impl ChiZeIndex {
    pub fn new(weights: ChiZeWeights) -> Result<Self> {
        let s = weights.eeg + weights.hrv + weights.resp + weights.sleep;
        if (s - 1.0).abs() > 1e-6 {
            return Err(BioSenseError::InvalidParameter {
                field: "weights",
                value: s,
                reason: "must sum to 1.0",
            });
        }
        for (n, v) in [("eeg", weights.eeg), ("hrv", weights.hrv),
                       ("resp", weights.resp), ("sleep", weights.sleep)] {
            if v < 0.0 || v > 1.0 || !v.is_finite() {
                return Err(BioSenseError::InvalidParameter {
                    field: Box::leak(n.to_string().into_boxed_str()),
                    value: v,
                    reason: "must be in [0, 1]",
                });
            }
        }
        Ok(Self { weights })
    }

    /// `χ_Ze(modality) = 1 − |v_obs − v*| / max(v*, 1 − v*)` on the **Python scale**.
    /// Use `per_modality_with_convention` when you have an article-scale `v_obs`.
    pub fn per_modality(v_obs: f64) -> f64 {
        Self::per_modality_with_convention(v_obs, VelocityConvention::Python)
    }

    /// Convention-aware per-modality χ_Ze.
    /// For `Python` convention: domain `v_obs ∈ [0, 1]`, `v* = V_STAR_PYTHON = 0.45631`.
    /// For `Article` convention: domain `v_obs ∈ [−1, +1]`, `v* = V_STAR_ARTICLE ≈ −0.08738`.
    /// The result is always normalised to `χ ∈ [0, 1]`, peak = 1 at `v_obs = v*`.
    pub fn per_modality_with_convention(v_obs: f64, convention: VelocityConvention) -> f64 {
        let v_star = convention.v_star();
        let (lo, hi) = match convention {
            VelocityConvention::Python => (0.0_f64, 1.0_f64),
            VelocityConvention::Article => (-1.0_f64, 1.0_f64),
        };
        // Maximum possible distance from v_star within the domain:
        let denom = (v_star - lo).abs().max((hi - v_star).abs());
        let raw = 1.0 - (v_obs - v_star).abs() / denom;
        raw.clamp(0.0, 1.0)
    }

    pub fn composite(&self, eeg: f64, hrv: f64, resp: f64, sleep: f64) -> f64 {
        let w = self.weights;
        (w.eeg * eeg + w.hrv * hrv + w.resp * resp + w.sleep * sleep).clamp(0.0, 1.0)
    }

    pub fn breakdown(&self, v_eeg: f64, v_hrv: f64, v_resp: f64, v_sleep: f64) -> ChiZeBreakdown {
        let eeg = Self::per_modality(v_eeg);
        let hrv = Self::per_modality(v_hrv);
        let resp = Self::per_modality(v_resp);
        let sleep = Self::per_modality(v_sleep);
        let composite = self.composite(eeg, hrv, resp, sleep);
        ChiZeBreakdown { eeg, hrv, resp, sleep, composite }
    }

    pub fn fixed_point() -> f64 {
        V_STAR_PYTHON
    }

    pub fn fixed_point_for(convention: VelocityConvention) -> f64 {
        convention.v_star()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn at_v_star_score_is_one_python() {
        let chi = ChiZeIndex::per_modality(V_STAR_PYTHON);
        assert_abs_diff_eq!(chi, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn at_v_star_score_is_one_article() {
        let chi = ChiZeIndex::per_modality_with_convention(
            VelocityConvention::Article.v_star(),
            VelocityConvention::Article,
        );
        assert_abs_diff_eq!(chi, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn far_from_v_star_score_is_zero() {
        let chi_max = ChiZeIndex::per_modality(1.0);
        assert!(chi_max < 0.20);
        let chi_min = ChiZeIndex::per_modality(0.0);
        assert!(chi_min >= 0.0);
    }

    #[test]
    fn convention_consistency_at_v_star() {
        // Both conventions should report χ = 1 exactly at their respective v*.
        let chi_p = ChiZeIndex::per_modality_with_convention(
            VelocityConvention::Python.v_star(),
            VelocityConvention::Python,
        );
        let chi_a = ChiZeIndex::per_modality_with_convention(
            VelocityConvention::Article.v_star(),
            VelocityConvention::Article,
        );
        assert_abs_diff_eq!(chi_p, 1.0, epsilon = 1e-12);
        assert_abs_diff_eq!(chi_a, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn weights_must_sum_to_one() {
        let bad = ChiZeWeights { eeg: 0.5, hrv: 0.5, resp: 0.5, sleep: 0.5 };
        assert!(ChiZeIndex::new(bad).is_err());
    }
}
