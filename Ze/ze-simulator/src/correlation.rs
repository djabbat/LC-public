//! Correlation decay and LGI K(τ).
//!
//! THEORY.md §4. `C(τ) = C₀·exp(−β·I·τ)`, `K(τ) = 2C(τ) − C(2τ)`.
//! Refuses to extrapolate beyond `β·I·τ > BTAU_LIMIT`.

use crate::{Result, ZeError, consts::BTAU_LIMIT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CorrelationDecay {
    pub c0: f64,
    pub beta: f64,
    pub impedance: f64,
}

impl CorrelationDecay {
    pub fn new(c0: f64, beta: f64, impedance: f64) -> Result<Self> {
        if c0 <= 0.0 || !c0.is_finite() {
            return Err(ZeError::InvalidParameter {
                field: "c0",
                value: c0,
                reason: "must be positive and finite",
            });
        }
        if beta <= 0.0 || !beta.is_finite() {
            return Err(ZeError::InvalidParameter {
                field: "beta",
                value: beta,
                reason: "must be positive and finite",
            });
        }
        if impedance < 0.0 || !impedance.is_finite() {
            return Err(ZeError::InvalidParameter {
                field: "impedance",
                value: impedance,
                reason: "must be non-negative and finite",
            });
        }
        Ok(Self { c0, beta, impedance })
    }

    /// `C(τ)`. Refuses if `β·I·τ > BTAU_LIMIT`.
    pub fn at(&self, tau: f64) -> Result<f64> {
        let x = self.beta * self.impedance * tau;
        if x > BTAU_LIMIT {
            return Err(ZeError::ExtrapolationRefused { value: x, limit: BTAU_LIMIT });
        }
        Ok(self.c0 * (-x).exp())
    }

    /// `K(τ) = 2C(τ) − C(2τ)`. Refuses if `2τ` puts us past BTAU_LIMIT.
    pub fn lgi_k(&self, tau: f64) -> Result<f64> {
        let c_tau = self.at(tau)?;
        let c_2tau = self.at(2.0 * tau)?;
        Ok(2.0 * c_tau - c_2tau)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn decay_matches_closed_form() {
        let cd = CorrelationDecay::new(1.0, 1.0, 0.5).unwrap();
        for &tau in &[0.0_f64, 0.1, 0.5, 1.0, 1.99] {
            let expected = (-(0.5_f64 * tau)).exp();
            let actual = cd.at(tau).unwrap();
            assert_abs_diff_eq!(actual, expected, epsilon = 1e-12);
        }
    }

    #[test]
    fn refuses_beyond_btau_limit() {
        let cd = CorrelationDecay::new(1.0, 1.0, 1.0).unwrap();
        let res = cd.at(1.5);
        assert!(matches!(res, Err(ZeError::ExtrapolationRefused { .. })));
    }

    #[test]
    fn k_consistent_with_c() {
        let cd = CorrelationDecay::new(1.0, 1.0, 0.2).unwrap();
        let tau = 1.0;
        let c1 = cd.at(tau).unwrap();
        let c2 = cd.at(2.0 * tau).unwrap();
        let k = cd.lgi_k(tau).unwrap();
        assert_abs_diff_eq!(k, 2.0 * c1 - c2, epsilon = 1e-12);
    }
}
