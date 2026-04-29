//! Predictive information for binary Markov chains (THEORY §2.2).

use crate::{BioSenseError, Result, consts::{MARKOV_P_MAX, MARKOV_P_MIN}};

pub struct PredictiveInfo;

impl PredictiveInfo {
    /// Closed form for symmetric binary Markov chain with rate `p`:
    ///   I_pred(τ=1) = log 2 − H[p]
    /// where `H[p] = −p ln p − (1−p) ln(1−p)`.
    pub fn closed_form(p: f64) -> Result<f64> {
        if !(MARKOV_P_MIN..=MARKOV_P_MAX).contains(&p) {
            return Err(BioSenseError::MarkovRateOutOfRange {
                p, min: MARKOV_P_MIN, max: MARKOV_P_MAX,
            });
        }
        let h = -p * p.ln() - (1.0 - p) * (1.0 - p).ln();
        Ok((2.0_f64).ln() - h)
    }

    /// Numerical estimator: per-window mutual information between past and present.
    /// Uses plug-in entropy estimator over `window` consecutive bigrams.
    pub fn estimate(symbols: &[u8], window: usize) -> Result<f64> {
        if symbols.len() < window + 2 {
            return Err(BioSenseError::StreamTooShort {
                len: symbols.len(),
                min: window + 2,
            });
        }
        // Counts of (prev, next) bigrams.
        let mut counts = [[0_u64; 2]; 2];
        for w in symbols.windows(2) {
            counts[w[0] as usize][w[1] as usize] += 1;
        }
        let total = (symbols.len() - 1) as f64;
        let mut h_joint = 0.0_f64;
        let mut p_prev = [0.0_f64; 2];
        let mut p_next = [0.0_f64; 2];
        for i in 0..2 {
            for j in 0..2 {
                let p = counts[i][j] as f64 / total;
                if p > 0.0 {
                    h_joint -= p * p.ln();
                }
                p_prev[i] += p;
                p_next[j] += p;
            }
        }
        let mut h_prev = 0.0_f64;
        let mut h_next = 0.0_f64;
        for i in 0..2 {
            if p_prev[i] > 0.0 {
                h_prev -= p_prev[i] * p_prev[i].ln();
            }
            if p_next[i] > 0.0 {
                h_next -= p_next[i] * p_next[i].ln();
            }
        }
        // I(prev; next) = H(prev) + H(next) − H(prev, next)
        Ok(h_prev + h_next - h_joint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn closed_form_at_half_is_zero() {
        let i = PredictiveInfo::closed_form(0.5).unwrap();
        assert_abs_diff_eq!(i, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn closed_form_monotone_away_from_half() {
        let a = PredictiveInfo::closed_form(0.3).unwrap();
        let b = PredictiveInfo::closed_form(0.1).unwrap();
        assert!(b > a, "expected I_pred(0.1) > I_pred(0.3); got {} <= {}", b, a);
    }
}
