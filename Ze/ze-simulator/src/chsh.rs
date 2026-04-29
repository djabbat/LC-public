//! CHSH with Ze quadratic deformation.
//!
//! THEORY.md §3. Two settings per side: `(a, a')` for Alice, `(b, b')` for Bob.
//! Standard QM: `S_QM = 2√2 ≈ 2.8284`. Ze: `S_Ze = 2√2 + δ·1.7478`.
//! The constant `1.7478` is verified, not hard-coded — F3 must reproduce it from optimisation.

use crate::{Result, ZeError, consts::TSIRELSON};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChshAngles {
    /// Spherical (theta, phi) on S² for each of the four measurement settings.
    pub a: (f64, f64),
    pub a_prime: (f64, f64),
    pub b: (f64, f64),
    pub b_prime: (f64, f64),
}

#[derive(Debug, Clone, Copy)]
pub struct ChshDeformation {
    pub delta: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum ChshOptimizer {
    /// Brute-force grid in spherical coordinates. `n` is the number of θ-samples;
    /// φ uses `n` samples too. Total quadruple count is `n^8` — keep `n` ≤ 24.
    Grid { n: usize },
    /// Restricted optimisation in the *single canonical CHSH plane*: each direction
    /// is parameterised by one polar angle in the X-Z plane. `n` is the angle samples.
    /// This recovers the standard 2√2 result; F3 uses this on `n ≥ 1024`.
    PlanarGrid { n: usize },
}

impl ChshDeformation {
    pub fn new(delta: f64) -> Result<Self> {
        if !delta.is_finite() {
            return Err(ZeError::InvalidParameter {
                field: "delta",
                value: delta,
                reason: "must be finite",
            });
        }
        Ok(Self { delta })
    }

    /// Ze-deformed correlation: `E_Ze(a, b) = −(a · b) + δ · [(a · b)² − 1/3]`.
    pub fn correlation(&self, a: [f64; 3], b: [f64; 3]) -> f64 {
        let ab = dot3(a, b);
        -ab + self.delta * (ab.powi(2) - 1.0 / 3.0)
    }

    /// CHSH expression at a given quadruple of measurement directions:
    /// `S = E(a, b) + E(a, b') + E(a', b) − E(a', b')`.
    /// (One of several equivalent CHSH sign conventions.)
    pub fn s_value_unit(&self, a: [f64; 3], a2: [f64; 3], b: [f64; 3], b2: [f64; 3]) -> f64 {
        self.correlation(a, b)
            + self.correlation(a, b2)
            + self.correlation(a2, b)
            - self.correlation(a2, b2)
    }

    /// Optimise `S_Ze` over measurement directions.
    pub fn s_optimal(&self, optimizer: ChshOptimizer) -> Result<f64> {
        match optimizer {
            ChshOptimizer::Grid { n } => self.optimize_grid(n),
            ChshOptimizer::PlanarGrid { n } => self.optimize_planar(n),
        }
    }

    fn optimize_planar(&self, n: usize) -> Result<f64> {
        if n < 8 {
            return Err(ZeError::InvalidParameter {
                field: "n",
                value: n as f64,
                reason: "PlanarGrid requires n ≥ 8",
            });
        }
        // In the X-Z plane: a = (sin θ_a, 0, cos θ_a), etc.
        let angles: Vec<f64> = (0..n).map(|k| 2.0 * std::f64::consts::PI * (k as f64) / (n as f64)).collect();
        let best: f64 = angles
            .par_iter()
            .map(|&ta| {
                let mut local_best = f64::NEG_INFINITY;
                for &ta2 in &angles {
                    for &tb in &angles {
                        for &tb2 in &angles {
                            let a = [ta.sin(), 0.0, ta.cos()];
                            let a2 = [ta2.sin(), 0.0, ta2.cos()];
                            let b = [tb.sin(), 0.0, tb.cos()];
                            let b2 = [tb2.sin(), 0.0, tb2.cos()];
                            let s = self.s_value_unit(a, a2, b, b2);
                            if s > local_best {
                                local_best = s;
                            }
                        }
                    }
                }
                local_best
            })
            .reduce(|| f64::NEG_INFINITY, f64::max);
        Ok(best)
    }

    fn optimize_grid(&self, n: usize) -> Result<f64> {
        if n < 4 || n > 24 {
            return Err(ZeError::InvalidParameter {
                field: "n",
                value: n as f64,
                reason: "Grid in S² requires 4 ≤ n ≤ 24 (cost is n^8)",
            });
        }
        let directions: Vec<[f64; 3]> = sphere_grid(n);
        let dirs = &directions;
        let m = dirs.len();
        // Parallelize over the outermost index.
        let best: f64 = (0..m)
            .into_par_iter()
            .map(|i| {
                let mut local_best = f64::NEG_INFINITY;
                let a = dirs[i];
                for j in 0..m {
                    let a2 = dirs[j];
                    for k in 0..m {
                        let b = dirs[k];
                        for l in 0..m {
                            let b2 = dirs[l];
                            let s = self.s_value_unit(a, a2, b, b2);
                            if s > local_best {
                                local_best = s;
                            }
                        }
                    }
                }
                local_best
            })
            .reduce(|| f64::NEG_INFINITY, f64::max);
        Ok(best)
    }
}

fn sphere_grid(n: usize) -> Vec<[f64; 3]> {
    let mut out = Vec::with_capacity(n * n);
    for i in 0..n {
        let theta = std::f64::consts::PI * (i as f64 + 0.5) / (n as f64);
        for j in 0..n {
            let phi = 2.0 * std::f64::consts::PI * (j as f64) / (n as f64);
            let s = theta.sin();
            out.push([s * phi.cos(), s * phi.sin(), theta.cos()]);
        }
    }
    out
}

fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Convenience: standard QM CHSH maximum.
pub fn s_qm() -> f64 {
    TSIRELSON
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn delta_zero_recovers_qm() {
        let chsh = ChshDeformation::new(0.0).unwrap();
        let s = chsh.s_optimal(ChshOptimizer::PlanarGrid { n: 1024 }).unwrap();
        assert_abs_diff_eq!(s, TSIRELSON, epsilon = 1e-3);
    }

    #[test]
    fn delta_increases_s() {
        let chsh0 = ChshDeformation::new(0.0).unwrap();
        let chsh1 = ChshDeformation::new(0.1).unwrap();
        let s0 = chsh0.s_optimal(ChshOptimizer::PlanarGrid { n: 256 }).unwrap();
        let s1 = chsh1.s_optimal(ChshOptimizer::PlanarGrid { n: 256 }).unwrap();
        assert!(s1 > s0);
    }
}
