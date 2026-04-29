//! 5-layer privacy stack (THEORY §6).
//! Implements: layer 2 (minimisation), layer 3 (k-anonymity), layer 4 (DP), layer 5 (SecAgg interface).
//! Layer 1 (de-identification) is procedural, not implemented in software.

use crate::{BioSenseError, Result};
use rand::RngCore;

/// Inverse-CDF sampling for Laplace(0, scale).
fn laplace_sample(scale: f64, rng: &mut impl RngCore) -> f64 {
    // u ~ Uniform(0,1) excluding 0
    let bits = rng.next_u64();
    // Make a double in (0, 1)
    let u = ((bits >> 11) as f64 + 1.0) / ((1u64 << 53) as f64 + 1.0);
    let v = u - 0.5;
    let sign = if v >= 0.0 { 1.0 } else { -1.0 };
    -scale * sign * (1.0 - 2.0 * v.abs()).ln()
}

#[derive(Debug, Clone, Copy)]
pub struct DpBudget {
    pub eps: f64,
    pub delta: f64,
    pub sensitivity: f64,
}

impl DpBudget {
    pub fn new(eps: f64, delta: f64, sensitivity: f64) -> Result<Self> {
        if eps <= 0.0 || !eps.is_finite() {
            return Err(BioSenseError::InvalidParameter {
                field: "eps",
                value: eps,
                reason: "must be > 0",
            });
        }
        if !(0.0..1.0).contains(&delta) {
            return Err(BioSenseError::InvalidParameter {
                field: "delta",
                value: delta,
                reason: "must be in [0, 1)",
            });
        }
        if sensitivity <= 0.0 || !sensitivity.is_finite() {
            return Err(BioSenseError::InvalidParameter {
                field: "sensitivity",
                value: sensitivity,
                reason: "must be > 0",
            });
        }
        Ok(Self { eps, delta, sensitivity })
    }

    /// Add Laplace noise scaled by sensitivity / ε.
    pub fn laplace_noise(&self, x: f64, rng: &mut impl RngCore) -> f64 {
        let scale = self.sensitivity / self.eps;
        x + laplace_sample(scale, rng)
    }

    /// Cumulative ε under naive sequential composition: `n · ε`.
    /// (RDP composition is tighter; this is the conservative upper bound.)
    pub fn naive_cumulative_eps(&self, n: usize) -> f64 {
        n as f64 * self.eps
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KAnonymity {
    pub k: usize,
}

impl KAnonymity {
    pub fn new(k: usize) -> Result<Self> {
        if k < 2 {
            return Err(BioSenseError::InvalidParameter {
                field: "k",
                value: k as f64,
                reason: "must be ≥ 2",
            });
        }
        Ok(Self { k })
    }

    pub fn safe_to_release(&self, cohort_size: usize) -> bool {
        cohort_size >= self.k
    }

    pub fn enforce<T>(&self, items: &[T]) -> Result<()> {
        if items.len() < self.k {
            return Err(BioSenseError::KAnonymityViolated { n: items.len(), k: self.k });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SecAgg {
    pub min_participants: usize,
}

impl SecAgg {
    pub fn new(min_participants: usize) -> Result<Self> {
        if min_participants < 2 {
            return Err(BioSenseError::InvalidParameter {
                field: "min_participants",
                value: min_participants as f64,
                reason: "must be ≥ 2",
            });
        }
        Ok(Self { min_participants })
    }

    pub fn ready(&self, n: usize) -> bool {
        n >= self.min_participants
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn dp_noise_centered_zero_in_expectation() {
        let dp = DpBudget::new(2.0, 1e-5, 0.3).unwrap();
        let mut rng = StdRng::seed_from_u64(crate::consts::RNG_SEED);
        let n = 50_000;
        let mut sum = 0.0;
        for _ in 0..n {
            sum += dp.laplace_noise(0.0, &mut rng);
        }
        let mean = sum / n as f64;
        assert!(mean.abs() < 0.05, "mean = {}", mean);
    }

    #[test]
    fn k_anonymity_blocks_small_cohorts() {
        let ka = KAnonymity::new(7).unwrap();
        assert!(!ka.safe_to_release(5));
        assert!(ka.safe_to_release(7));
    }
}
