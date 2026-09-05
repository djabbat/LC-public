//! Component-age Shannon entropy (Entropy in Aging roadmap, §6, §13)
//!
//! Implements the three-component mixture model from the roadmap
//! (Barardo et al., 2026), computed from the zygote (t = 0) where H(0) = 0.
//!
//! - Class A: fast-turnover molecules/proteins (exponential transition, tau_A)
//! - Class B: renewing cells (exponential transition, tau_B)
//! - Class C: non-renewable accumulated components (uniform on [0,t],
//!   weight w_C(t) = w_C0 + alpha * t)
//!
//!     H(t) = - sum_i p_i(t) * log2(p_i(t))
//!
//! Working genes (Thesis 3 / §6.4), epigenetic drift from birth:
//!     G(t) = G0 * exp(-gamma * t),  G0 = 20000, gamma = 0.0012 /yr
//!
//! Tissue-specific parameters (0-120 years) as in §13.2.
//! Pure std (f64::log2) — no external numeric dependency required.

/// Number of 1-year age bins (full human lifespan).
pub const L_BINS: usize = 120;

/// Global mixture weights (classes A and B).
pub const W_A: f64 = 0.55;
pub const W_B: f64 = 0.35;

/// Working-gene baseline and epigenetic drift rate (§6.4).
pub const G0: f64 = 20_000.0;
pub const GAMMA: f64 = 0.0012;

/// Tissue-specific model parameters (§13.2).
#[derive(Debug, Clone, Copy)]
pub struct TissueParams {
    /// Turnover time (yr) of fast-turnover molecules (class A).
    pub tau_a: f64,
    /// Turnover time (yr) of renewing cells (class B).
    pub tau_b: f64,
    /// Initial weight of non-renewable components (class C) at birth.
    pub w_c0: f64,
    /// Linear accumulation rate of class C weight (yr^-1).
    pub alpha: f64,
}

/// Named tissue parameter set, in the same order as §6.3 / §13.2.
pub const TISSUES: [(&str, TissueParams); 7] = [
    (
        "blood",
        TissueParams { tau_a: 0.3, tau_b: 0.1, w_c0: 0.05, alpha: 0.001 },
    ),
    (
        "skin",
        TissueParams { tau_a: 0.5, tau_b: 0.5, w_c0: 0.07, alpha: 0.002 },
    ),
    (
        "liver",
        TissueParams { tau_a: 1.0, tau_b: 2.0, w_c0: 0.10, alpha: 0.003 },
    ),
    (
        "muscle",
        TissueParams { tau_a: 1.5, tau_b: 5.0, w_c0: 0.15, alpha: 0.008 },
    ),
    (
        "heart",
        TissueParams { tau_a: 1.5, tau_b: 6.0, w_c0: 0.15, alpha: 0.010 },
    ),
    (
        "brain",
        TissueParams { tau_a: 2.0, tau_b: 8.0, w_c0: 0.20, alpha: 0.015 },
    ),
    (
        "average",
        TissueParams { tau_a: 0.5, tau_b: 8.0, w_c0: 0.10, alpha: 0.004 },
    ),
];

/// Return parameters for a named tissue, falling back to `average`.
pub fn tissue(name: &str) -> TissueParams {
    TISSUES
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, p)| *p)
        .unwrap_or(TISSUES[6].1)
}

/// Shannon entropy (bits) of the component-age distribution at age `t` (yr).
///
/// Mirrors the reference Python implementation in §13.2, including the
/// delta-mass term `exp(-lambda * t)` added to the current age bin for the
/// original cohort that has never been renewed.
pub fn h(t: f64, p: &TissueParams) -> f64 {
    if t < 0.0 || t > L_BINS as f64 {
        return f64::NAN; // out-of-range age
    }
    let la = 1.0 / p.tau_a;
    let lb = 1.0 / p.tau_b;
    let idx = t.floor() as usize;

    let mut dist = vec![0.0f64; L_BINS];
    for i in 0..L_BINS {
        let fi = i as f64;
        let mut pa = if fi < t { la * (-la * fi).exp() } else { 0.0 };
        let mut pb = if fi < t { lb * (-lb * fi).exp() } else { 0.0 };
        let pc = if fi <= t { 1.0 / (t + 1.0) } else { 0.0 };
        if i == idx {
            pa += (-la * t).exp();
            pb += (-lb * t).exp();
        }
        let wc = p.w_c0 + p.alpha * t;
        let ws = W_A + W_B + wc;
        dist[i] = (W_A / ws) * pa + (W_B / ws) * pb + (wc / ws) * pc;
    }

    let sum: f64 = dist.iter().sum();
    let mut entropy = 0.0;
    for &x in &dist {
        let q = x / sum;
        if q > 0.0 {
            entropy -= q * q.log2();
        }
    }
    entropy.max(0.0)
}

/// Number of working genes at age `t` (§6.4): G0 * exp(-gamma * t).
pub fn g(t: f64) -> f64 {
    G0 * (-GAMMA * t).exp()
}

/// Multiscale Entropic Aging Index (MEAI) — Z-score vs. a reference
/// population (§1.4, §6.1).
pub fn meai(h: f64, mu: f64, sigma: f64) -> f64 {
    (h - mu) / sigma
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zygote_has_zero_entropy() {
        let p = tissue("average");
        let h0 = h(0.0, &p);
        assert!((h0 - 0.0).abs() < 1e-9, "H(0) must be 0, got {h0}");
    }

    #[test]
    fn brain_h120_matches_roadmap() {
        // Roadmap §13.4: H(120) brain = 6.2922 bits.
        let p = tissue("brain");
        let h120 = h(120.0, &p);
        assert!((h120 - 6.2922).abs() < 0.01, "brain H(120) = {h120}");
    }

    #[test]
    fn blood_h120_matches_roadmap() {
        // Roadmap §13.4: H(120) blood = 0.4963 bits.
        let p = tissue("blood");
        let h120 = h(120.0, &p);
        assert!((h120 - 0.4963).abs() < 0.01, "blood H(120) = {h120}");
    }

    #[test]
    fn working_genes_decline() {
        let g90 = g(90.0);
        let g120 = g(120.0);
        assert!((g90 - 17_953.0).abs() < 10.0, "G(90) = {g90}");
        assert!((g120 - 17_318.0).abs() < 10.0, "G(120) = {g120}");
        assert!(g120 < g90);
    }

    #[test]
    fn entropy_non_decreasing_in_senescence() {
        let p = tissue("brain");
        for t in 40..119 {
            let a = h(t as f64, &p);
            let b = h(t as f64 + 1.0, &p);
            assert!(a <= b + 1e-9, "H not monotone at t={t}");
        }
    }

    #[test]
    fn meai_is_zscore() {
        assert!((meai(2.0, 1.0, 0.5) - 2.0).abs() < 1e-12);
    }
}
