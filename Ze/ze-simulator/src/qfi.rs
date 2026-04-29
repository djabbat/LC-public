//! QFI lower bound from Lemma E (THEORY.md §5).
//!
//! `F_Q ≥ 8·C₀·(β·I·τ)²·(1 − β·I·τ)`  for `β·I·τ ≤ BTAU_LIMIT`.
//! Optimisation over τ at the leading order yields `F_Q,max ∝ |dτ_Ze/dt|`.

use crate::{Result, ZeError, consts::BTAU_LIMIT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QfiBound {
    pub c0: f64,
    pub beta: f64,
    pub impedance: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QfiResult {
    pub f_q_lower_bound: f64,
    pub regime: &'static str,
    pub tau_used: f64,
}

impl QfiBound {
    pub fn new(c0: f64, beta: f64, impedance: f64) -> Result<Self> {
        if c0 <= 0.0 || !c0.is_finite() {
            return Err(ZeError::InvalidParameter {
                field: "c0", value: c0, reason: "must be positive and finite",
            });
        }
        if beta <= 0.0 || !beta.is_finite() {
            return Err(ZeError::InvalidParameter {
                field: "beta", value: beta, reason: "must be positive and finite",
            });
        }
        if impedance < 0.0 || !impedance.is_finite() {
            return Err(ZeError::InvalidParameter {
                field: "impedance", value: impedance, reason: "must be non-negative and finite",
            });
        }
        Ok(Self { c0, beta, impedance })
    }

    /// Lower bound at a specified `τ`. The cubic factor is `(1 − x)`; for `x ≥ 1`
    /// the simulator refuses (extrapolation).
    pub fn at(&self, tau: f64) -> Result<QfiResult> {
        let x = self.beta * self.impedance * tau;
        if x > BTAU_LIMIT {
            return Err(ZeError::ExtrapolationRefused { value: x, limit: BTAU_LIMIT });
        }
        let bound = 8.0 * self.c0 * x * x * (1.0 - x);
        Ok(QfiResult {
            f_q_lower_bound: bound.max(0.0),
            regime: "stationary_finite_tau",
            tau_used: tau,
        })
    }

    /// Maximum of the lower bound over `τ` ∈ [0, BTAU_LIMIT/(β·I)].
    /// Closed form: `f(x) = 8·C₀·x²·(1−x)` is maximised at `x = 2/3`,
    /// value `8·C₀·(4/9)·(1/3) = 32·C₀/27`. Converting back to `τ`: `τ* = 2/(3·β·I)`.
    pub fn at_optimal_tau(&self) -> Result<QfiResult> {
        if self.impedance == 0.0 {
            return Ok(QfiResult {
                f_q_lower_bound: 0.0,
                regime: "stationary_optimal_tau",
                tau_used: 0.0,
            });
        }
        let tau_star = 2.0 / (3.0 * self.beta * self.impedance);
        let bound = 32.0 * self.c0 / 27.0;
        Ok(QfiResult {
            f_q_lower_bound: bound,
            regime: "stationary_optimal_tau",
            tau_used: tau_star,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn closed_form_optimum() {
        let q = QfiBound::new(1.0, 1.0, 0.5).unwrap();
        let opt = q.at_optimal_tau().unwrap();
        assert_abs_diff_eq!(opt.f_q_lower_bound, 32.0 / 27.0, epsilon = 1e-12);
        assert_abs_diff_eq!(opt.tau_used, 2.0 / (3.0 * 0.5), epsilon = 1e-12);
    }

    #[test]
    fn at_specified_tau_matches_formula() {
        let q = QfiBound::new(1.0, 1.0, 0.5).unwrap();
        let tau = 0.7;
        let x = 0.5 * tau;
        let expected = 8.0 * x * x * (1.0 - x);
        let result = q.at(tau).unwrap();
        assert_abs_diff_eq!(result.f_q_lower_bound, expected, epsilon = 1e-12);
    }

    #[test]
    fn refuses_beyond_btau_limit() {
        let q = QfiBound::new(1.0, 1.0, 1.0).unwrap();
        let res = q.at(1.5);
        assert!(matches!(res, Err(ZeError::ExtrapolationRefused { .. })));
    }
}
