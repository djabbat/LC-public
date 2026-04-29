//! Canonical symbolisation rules — extracted from archived Python pipelines
//! (see `_archive/v1_pre_2026-04-28/src/eeg_ze_processor.py:binarize` and
//! `ze_hrv.py`).
//!
//! Three rules:
//!   1. `median_threshold` — for arbitrary continuous signal: `1 if x > median(x)`.
//!   2. `lf_hf_state`     — for HRV: `1 if LF/HF > 1+δ; 0 if < 1-δ; carry-over otherwise`.
//!   3. `derivative_sign` — for respiration: `1 if dx/dt > 0` (inspiration vs expiration).
//!
//! The article (Tkemaladze 2026 §2.1.1) states the signal is mapped to a binary
//! Markov chain. The three rules implement that mapping for three modalities.

/// Median-threshold binarisation. THEORY §1.
/// For a vector `x`, returns `b` of same length where `b[i] = 1 if x[i] > median(x) else 0`.
pub fn median_threshold(x: &[f64]) -> Vec<u8> {
    if x.is_empty() {
        return Vec::new();
    }
    let mut sorted: Vec<f64> = x.iter().copied().filter(|v| v.is_finite()).collect();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len();
    if n == 0 {
        return vec![0; x.len()];
    }
    let median = if n % 2 == 0 {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    };
    x.iter().map(|&v| if v > median { 1_u8 } else { 0_u8 }).collect()
}

/// LF/HF binarisation with hysteresis. THEORY §3.5 + PARAMETERS §3.
/// `δ = hyst` (default 0.10). Carry-over rule for the dead-band.
pub fn lf_hf_state(lf_over_hf: &[f64], hyst: f64) -> Vec<u8> {
    let mut out = Vec::with_capacity(lf_over_hf.len());
    let upper = 1.0 + hyst;
    let lower = 1.0 - hyst;
    let mut state = 0_u8;
    for &r in lf_over_hf {
        if r > upper {
            state = 1;
        } else if r < lower {
            state = 0;
        }
        // else: carry over — hysteresis prevents flutter
        out.push(state);
    }
    out
}

/// Sign-of-derivative binarisation. For respiration: 1 = inspiration, 0 = expiration.
pub fn derivative_sign(x: &[f64]) -> Vec<u8> {
    if x.len() < 2 {
        return vec![0; x.len()];
    }
    let mut out = vec![0_u8; x.len()];
    for i in 1..x.len() {
        out[i] = if x[i] > x[i - 1] { 1 } else { 0 };
    }
    // first sample uses second's direction
    out[0] = out[1];
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_threshold_balanced() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let b = median_threshold(&x);
        // median = 3 → x>3 → [4,5] → bits [0,0,0,1,1]
        assert_eq!(b, vec![0, 0, 0, 1, 1]);
    }

    #[test]
    fn lf_hf_carry_over_in_deadband() {
        // First clearly sympathetic (>1.10), then deadband, then clearly parasympathetic
        let r = vec![1.5, 1.0, 0.6];
        let s = lf_hf_state(&r, 0.10);
        assert_eq!(s, vec![1, 1, 0]);
    }

    #[test]
    fn derivative_sign_simple() {
        let x = vec![0.0, 1.0, 0.5, 2.0, 1.5];
        let s = derivative_sign(&x);
        // x[1]>x[0] → 1; x[2]<x[1] → 0; x[3]>x[2] → 1; x[4]<x[3] → 0; first = second
        assert_eq!(s, vec![1, 1, 0, 1, 0]);
    }
}
