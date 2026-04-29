//! Ze velocity from binary symbol stream (THEORY §2.3).
//!
//! Two equivalent conventions (see `datasets/MIGRATION_NOTES.md §2`):
//!   - `from_signal`           — article-style `(N_T − N_S)/(N_T + N_S) ∈ [−1, +1]`
//!   - `frequency_switch`      — Python-style `switches/(N − 1) ∈ [0, 1]`
//! With v_python = (v_article + 1) / 2 by `convert`.

use crate::{BioSenseError, Result, VelocityConvention};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PredictorKind {
    /// Predict next symbol = current symbol (identity).
    Identity,
    /// Predict next symbol = NOT current symbol.
    Flip,
}

pub struct ZeVelocity;

impl ZeVelocity {
    /// **Article convention** (docx §2.1, Definition 1):
    ///   `v = (N_T − N_S) / (N_T + N_S) ∈ [−1, +1]`
    ///   `N_T` = incorrect predictions, `N_S` = correct predictions.
    pub fn from_signal(symbols: &[u8], predictor: PredictorKind) -> Result<f64> {
        if symbols.len() < 2 {
            return Err(BioSenseError::StreamTooShort { len: symbols.len(), min: 2 });
        }
        for (i, &s) in symbols.iter().enumerate() {
            if s > 1 {
                return Err(BioSenseError::InvalidSymbol { index: i, value: s });
            }
        }
        let mut n_t = 0_u64;
        let mut n_s = 0_u64;
        for w in symbols.windows(2) {
            let cur = w[0];
            let nxt = w[1];
            let pred = match predictor {
                PredictorKind::Identity => cur,
                PredictorKind::Flip => 1 - cur,
            };
            if pred == nxt {
                n_s += 1;
            } else {
                n_t += 1;
            }
        }
        let total = (n_t + n_s) as f64;
        if total == 0.0 {
            return Err(BioSenseError::StreamTooShort { len: symbols.len(), min: 2 });
        }
        Ok((n_t as f64 - n_s as f64) / total)
    }

    /// **Python convention** (`eeg_ze_processor.py::ze_velocity`):
    ///   `v = N_switches / (N − 1) ∈ [0, 1]`
    ///   where `N_switches = #{i : symbols[i] != symbols[i-1]}`.
    /// Predictor-free — depends only on the symbol stream itself.
    pub fn frequency_switch(symbols: &[u8]) -> Result<f64> {
        if symbols.len() < 2 {
            return Err(BioSenseError::StreamTooShort { len: symbols.len(), min: 2 });
        }
        for (i, &s) in symbols.iter().enumerate() {
            if s > 1 {
                return Err(BioSenseError::InvalidSymbol { index: i, value: s });
            }
        }
        let mut switches = 0_u64;
        for w in symbols.windows(2) {
            if w[0] != w[1] {
                switches += 1;
            }
        }
        Ok(switches as f64 / (symbols.len() - 1) as f64)
    }

    /// Compute velocity under the chosen convention.
    pub fn under(symbols: &[u8], convention: VelocityConvention) -> Result<f64> {
        match convention {
            VelocityConvention::Python => Self::frequency_switch(symbols),
            VelocityConvention::Article => Self::from_signal(symbols, PredictorKind::Identity),
        }
    }

    /// Convert between the two conventions.
    /// `python = (article + 1) / 2`, `article = 2·python − 1`.
    pub fn convert(v: f64, from: VelocityConvention, to: VelocityConvention) -> f64 {
        if from == to {
            return v;
        }
        match (from, to) {
            (VelocityConvention::Article, VelocityConvention::Python) => (v + 1.0) / 2.0,
            (VelocityConvention::Python, VelocityConvention::Article) => 2.0 * v - 1.0,
            _ => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn identity_predictor_recovers_2p_minus_1() {
        // Constant signal: identity predictor always correct → v = -1
        let s = vec![0_u8; 1000];
        let v = ZeVelocity::from_signal(&s, PredictorKind::Identity).unwrap();
        assert_abs_diff_eq!(v, -1.0, epsilon = 1e-12);
    }

    #[test]
    fn alternating_with_identity() {
        // 0,1,0,1,... identity always wrong → v = +1
        let s: Vec<u8> = (0..1000).map(|i| (i % 2) as u8).collect();
        let v = ZeVelocity::from_signal(&s, PredictorKind::Identity).unwrap();
        assert_abs_diff_eq!(v, 1.0, epsilon = 1e-12);
    }
}
