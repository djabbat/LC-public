//! Impedance = KL divergence between actual and modeled distributions.
//!
//! THEORY.md §2.1.

use crate::{ZeError, Result, consts::{LOG_EPS, NORM_TOL}};
use serde::{Serialize, Deserialize};

/// A finite-support probability distribution. Validated and normalised on construction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distribution {
    probs: Vec<f64>,
}

impl Distribution {
    /// Construct from raw probabilities. Re-normalises if the sum is within `NORM_TOL`,
    /// otherwise fails with `ZeError::Normalization`. Negative entries are rejected.
    pub fn new(mut probs: Vec<f64>) -> Result<Self> {
        if probs.is_empty() {
            return Err(ZeError::InvalidParameter {
                field: "probs",
                value: 0.0,
                reason: "distribution must be non-empty",
            });
        }
        for (i, &p) in probs.iter().enumerate() {
            if !p.is_finite() || p < 0.0 {
                return Err(ZeError::InvalidParameter {
                    field: "probs",
                    value: p,
                    reason: if p < 0.0 { "negative entry" } else { "non-finite entry" },
                });
            }
            if p < 0.0 {
                let _ = i; // unreachable but suppress unused
            }
        }
        let sum: f64 = probs.iter().sum();
        if sum <= 0.0 || (sum - 1.0).abs() > NORM_TOL.max(1e-6) {
            // allow re-normalisation if sum is positive and within a generous tolerance,
            // but a wildly off sum is a user error.
            if sum <= 0.0 || (sum - 1.0).abs() > 1e-3 {
                return Err(ZeError::Normalization { sum, tol: NORM_TOL });
            }
        }
        for p in probs.iter_mut() {
            *p /= sum;
        }
        Ok(Self { probs })
    }

    pub fn probs(&self) -> &[f64] {
        &self.probs
    }

    pub fn len(&self) -> usize {
        self.probs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.probs.is_empty()
    }

    /// `I(self ‖ model) = Σᵢ pᵢ · log(pᵢ / qᵢ)` (natural log).
    /// Returns `Err(InfiniteKl)` if `model[i] == 0` while `self[i] > 0` (the formal +∞ case).
    pub fn kl_to(&self, model: &Distribution) -> Result<f64> {
        if self.probs.len() != model.probs.len() {
            return Err(ZeError::LengthMismatch {
                real_len: self.probs.len(),
                model_len: model.probs.len(),
            });
        }
        let mut acc = 0.0_f64;
        for (i, (&p, &q)) in self.probs.iter().zip(model.probs.iter()).enumerate() {
            if p <= 0.0 {
                continue; // 0 · log(0/q) = 0 by convention
            }
            if q <= 0.0 {
                return Err(ZeError::InfiniteKl { index: i });
            }
            let ratio = p / q.max(LOG_EPS);
            acc += p * ratio.ln();
        }
        Ok(acc)
    }
}

/// Quantum relative entropy `S(ρ ‖ σ) = Tr[ρ (log ρ − log σ)]` for **real symmetric**
/// density matrices. Complex Hermitian density matrices are deferred (see OPEN_PROBLEMS §1.3).
///
/// The matrices are assumed positive semi-definite with trace 1; this is the caller's responsibility.
pub fn relative_entropy(
    rho: &ndarray::Array2<f64>,
    sigma: &ndarray::Array2<f64>,
) -> Result<f64> {
    if rho.shape() != sigma.shape() {
        return Err(ZeError::LengthMismatch {
            real_len: rho.nrows(),
            model_len: sigma.nrows(),
        });
    }
    if rho.nrows() != rho.ncols() || sigma.nrows() != sigma.ncols() {
        return Err(ZeError::InvalidParameter {
            field: "matrix",
            value: rho.nrows() as f64,
            reason: "must be square",
        });
    }

    // Eigen-decompose each (real symmetric); compute log on eigenbasis.
    let log_rho = matrix_log_symmetric(rho)?;
    let log_sigma = matrix_log_symmetric(sigma)?;
    let diff = &log_rho - &log_sigma;
    let prod = rho.dot(&diff);
    Ok(prod.diag().iter().sum::<f64>())
}

fn matrix_log_symmetric(m: &ndarray::Array2<f64>) -> Result<ndarray::Array2<f64>> {
    use ndarray::Array2;
    let n = m.nrows();
    // Naive Jacobi eigendecomposition for small matrices is enough for our use case.
    let (eigvals, eigvecs) = jacobi_eigendecomp(m)?;
    let mut log_diag = Array2::<f64>::zeros((n, n));
    for i in 0..n {
        let lam = eigvals[i];
        if lam <= 0.0 {
            return Err(ZeError::InvalidParameter {
                field: "eigenvalue",
                value: lam,
                reason: "matrix is not positive definite",
            });
        }
        log_diag[(i, i)] = lam.ln();
    }
    Ok(eigvecs.dot(&log_diag).dot(&eigvecs.t()))
}

fn jacobi_eigendecomp(
    m: &ndarray::Array2<f64>,
) -> Result<(Vec<f64>, ndarray::Array2<f64>)> {
    use ndarray::Array2;
    let n = m.nrows();
    let mut a = m.clone();
    let mut v = Array2::<f64>::eye(n);
    let max_sweeps = 100;
    let tol = 1e-14;
    for _ in 0..max_sweeps {
        let mut off = 0.0_f64;
        for p in 0..n {
            for q in (p + 1)..n {
                off += a[(p, q)].powi(2);
            }
        }
        if off < tol {
            break;
        }
        for p in 0..n {
            for q in (p + 1)..n {
                let app = a[(p, p)];
                let aqq = a[(q, q)];
                let apq = a[(p, q)];
                if apq.abs() < 1e-18 {
                    continue;
                }
                let theta = (aqq - app) / (2.0 * apq);
                let t = if theta >= 0.0 {
                    1.0 / (theta + (1.0 + theta * theta).sqrt())
                } else {
                    1.0 / (theta - (1.0 + theta * theta).sqrt())
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;
                // rotate
                a[(p, p)] = app - t * apq;
                a[(q, q)] = aqq + t * apq;
                a[(p, q)] = 0.0;
                a[(q, p)] = 0.0;
                for r in 0..n {
                    if r != p && r != q {
                        let arp = a[(r, p)];
                        let arq = a[(r, q)];
                        a[(r, p)] = c * arp - s * arq;
                        a[(p, r)] = a[(r, p)];
                        a[(r, q)] = s * arp + c * arq;
                        a[(q, r)] = a[(r, q)];
                    }
                    let vrp = v[(r, p)];
                    let vrq = v[(r, q)];
                    v[(r, p)] = c * vrp - s * vrq;
                    v[(r, q)] = s * vrp + c * vrq;
                }
            }
        }
    }
    let eigvals: Vec<f64> = (0..n).map(|i| a[(i, i)]).collect();
    Ok((eigvals, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kl_self_is_zero() {
        let p = Distribution::new(vec![0.25, 0.25, 0.25, 0.25]).unwrap();
        let kl = p.kl_to(&p).unwrap();
        assert!(kl.abs() < 1e-12, "KL(p, p) = {}", kl);
    }

    #[test]
    fn kl_positive_for_distinct() {
        let p = Distribution::new(vec![0.5, 0.5]).unwrap();
        let q = Distribution::new(vec![0.7, 0.3]).unwrap();
        let kl = p.kl_to(&q).unwrap();
        assert!(kl > 0.0, "expected positive, got {}", kl);
    }

    #[test]
    fn kl_asymmetric() {
        let p = Distribution::new(vec![0.4, 0.6]).unwrap();
        let q = Distribution::new(vec![0.7, 0.3]).unwrap();
        let pq = p.kl_to(&q).unwrap();
        let qp = q.kl_to(&p).unwrap();
        assert!((pq - qp).abs() > 1e-6);
    }
}
