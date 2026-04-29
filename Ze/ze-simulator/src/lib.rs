//! Ze Theory reference simulator.
//!
//! Five canonical quantities, each in its own module:
//!   - `impedance` — KL divergence as a Distribution method.
//!   - `proper_time` — RK4/Euler integration of `dτ/dt = −α·I(t)`.
//!   - `chsh` — quadratic CHSH deformation and S-value optimization.
//!   - `correlation` — exponential decay `C(τ) = C₀·exp(−β·I·τ)` plus LGI K(τ).
//!   - `qfi` — lower-bound QFI from Lemma E (THEORY.md §5).
//!
//! Authoritative derivations and tests are referenced from `THEORY.md §7`.

pub mod impedance;
pub mod proper_time;
pub mod chsh;
pub mod correlation;
pub mod qfi;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZeError {
    #[error("distribution does not normalize within {tol:?}: sum = {sum}")]
    Normalization { sum: f64, tol: f64 },

    #[error("distribution lengths differ: real={real_len}, model={model_len}")]
    LengthMismatch { real_len: usize, model_len: usize },

    #[error("model distribution has zero where real is non-zero (i={index}): KL is +∞")]
    InfiniteKl { index: usize },

    #[error("regime out of validity: β·I·τ = {value} > {limit} (extrapolation refused)")]
    ExtrapolationRefused { value: f64, limit: f64 },

    #[error("invalid parameter `{field}` = {value}: {reason}")]
    InvalidParameter {
        field: &'static str,
        value: f64,
        reason: &'static str,
    },

    #[error("optimizer failed to converge: {reason}")]
    OptimizerFailed { reason: String },
}

pub type Result<T> = std::result::Result<T, ZeError>;

pub use impedance::Distribution;
pub use proper_time::{IntegratorMethod, ProperTimeIntegrator};
pub use chsh::{ChshAngles, ChshDeformation, ChshOptimizer};
pub use correlation::CorrelationDecay;
pub use qfi::{QfiBound, QfiResult};

/// Numerical-safety constants. PARAMETERS.md §5.
pub mod consts {
    pub const LOG_EPS: f64 = 1e-30;
    pub const NORM_TOL: f64 = 1e-12;
    pub const BTAU_LIMIT: f64 = 1.0;
    pub const RNG_SEED: u64 = 20_260_428;
    pub const TSIRELSON: f64 = 2.828_427_124_746_190_2; // 2 * sqrt(2)
    pub const CHSH_DEFORMATION_CONSTANT: f64 = 1.7478;
}
