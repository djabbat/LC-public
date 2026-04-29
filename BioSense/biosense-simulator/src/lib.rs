//! BioSense reference simulator.
//!
//! Five canonical computations (CONCEPT §3):
//!   - `velocity`           — Ze velocity v from binary symbol stream.
//!   - `pred_info`          — predictive information I_pred (closed form + numeric).
//!   - `chi_ze`             — composite χ_Ze index over four modalities.
//!   - `bridge`              — CDATA bridge `A(D)` and `χ_Ze(A)`.
//!   - `exacerbation`       — 30-day risk classifier.
//!
//! Plus a 5-layer privacy stack (`privacy`) that wraps every released aggregate.
//!
//! Authoritative derivations and tests are in `THEORY.md §7` (B1–B6).

pub mod velocity;
pub mod pred_info;
pub mod chi_ze;
pub mod bridge;
pub mod exacerbation;
pub mod privacy;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BioSenseError {
    #[error("invalid parameter `{field}` = {value}: {reason}")]
    InvalidParameter { field: &'static str, value: f64, reason: &'static str },
    #[error("symbol stream empty or too short: len={len}, need ≥ {min}")]
    StreamTooShort { len: usize, min: usize },
    #[error("symbol out of range at index {index}: {value} (must be 0 or 1)")]
    InvalidSymbol { index: usize, value: u8 },
    #[error("Markov rate p={p} out of allowed range ({min}, {max})")]
    MarkovRateOutOfRange { p: f64, min: f64, max: f64 },
    #[error("dp budget exceeded: cumulative ε={cum} > cap={cap}")]
    DpBudgetExceeded { cum: f64, cap: f64 },
    #[error("k-anonymity violated: cohort_size={n} < k={k}")]
    KAnonymityViolated { n: usize, k: usize },
}

pub type Result<T> = std::result::Result<T, BioSenseError>;

pub use velocity::{PredictorKind, ZeVelocity};
// Re-export convention type at crate root for convenient `use biosense_simulator::VelocityConvention`.
pub use pred_info::PredictiveInfo;
pub use chi_ze::{ChiZeIndex, ChiZeWeights, ChiZeBreakdown};
pub use bridge::{BridgeParams, CdataBridge};
pub use exacerbation::{ExacerbCoeffs, ExacerbationModel, RiskResult};
pub use privacy::{DpBudget, KAnonymity, SecAgg};

/// Numerical-safety constants (PARAMETERS §6).
pub mod consts {
    pub const LOG_EPS: f64 = 1e-30;
    pub const MARKOV_P_MIN: f64 = 0.02;
    pub const MARKOV_P_MAX: f64 = 0.98;
    pub const RNG_SEED: u64 = 20_260_428;
    /// Theoretical fixed point on the Python switching-frequency scale (∈ [0, 1]).
    /// This is the value that produced the article's published numbers.
    pub const V_STAR_PYTHON: f64 = 0.45631;
    /// Same fixed point on the docx Definition-1 scale (∈ [−1, +1]).
    /// Affinely equivalent: v_article = 2·v_python − 1.
    pub const V_STAR_ARTICLE: f64 = 2.0 * V_STAR_PYTHON - 1.0; // ≈ −0.08738
    /// Backward-compat alias — points to the Python-scale value (canonical for χ_Ze).
    pub const V_STAR: f64 = V_STAR_PYTHON;
    /// Empirical upper bound for χ_Ze in living systems (Tkemaladze 2024).
    /// Reference value, not used in canonical computation.
    /// (Source: archived `eeg_ze_processor.py:48`, ported during 2026-04-28 audit.)
    pub const CHI_MAX_LIVING: f64 = 0.839;
    /// Default initial Ze proper-time budget (arbitrary units; ratio is invariant).
    /// (Source: archived `ze_cdata_bridge.py::tau_z_from_damage`.)
    pub const TAU_Z_0_DEFAULT: f64 = 1000.0;
}

/// Velocity / χ_Ze scaling convention. See `datasets/MIGRATION_NOTES.md §2`.
///
/// - `Python` — `v = switches / (N − 1) ∈ [0, 1]`. Reproduces archived Python numbers.
/// - `Article` — `v = (N_T − N_S) / (N_T + N_S) ∈ [−1, +1]`. Per docx Definition 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VelocityConvention {
    Python,
    Article,
}

impl Default for VelocityConvention {
    fn default() -> Self {
        // Canonical default: Python convention, matches archived analyses.
        VelocityConvention::Python
    }
}

impl VelocityConvention {
    /// Theoretical fixed point under this convention.
    pub fn v_star(&self) -> f64 {
        match self {
            VelocityConvention::Python => consts::V_STAR_PYTHON,
            VelocityConvention::Article => consts::V_STAR_ARTICLE,
        }
    }
}
