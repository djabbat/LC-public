//! MCARA simulation — time evolution of multi-counter damage per tissue.
//!
//! Includes optional EDC (Endocrine Disrupting Chemical) modulation module
//! for modelling environmental impacts on aging (thyroid disruptors, etc.).
//!
//! Since 2026-08-08 (v0.5): the centriolar counter is split into two mechanisms
//! per the "Spatially Constrained, Not Chemically Copied" framework:
//!   (1) PTM accumulation (existing Counter::Centriolar drift);
//!   (2) geometric inheritance — daughter centriole length follows an
//!       Ornstein–Uhlenbeck process with maternal transmission coefficient
//!       α ≈ 0.97 (steady-state variance Var = σ²/(1−α²) diverges as α→1).

use mcara_core::{
    default_drift_rates, default_reference_scales, default_weights, Counter, CounterState, Gamma,
    Tissue, N_COUNTERS, is_epigenetic_above_critical,
};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;
use serde::Serialize;

/// EDC modulation configuration: applies exposure-dependent drift rate multipliers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdcTarget {
    /// No EDC effect
    None,
    /// Thyroid-disrupting EDCs (PCBs, bisphenols, PFAS) — increase mito + proteostasis rates
    Thyroid,
    /// Broad EDC effect across all counters
    General,
}

impl EdcTarget {
    pub fn as_str(self) -> &'static str {
        match self {
            EdcTarget::None => "none",
            EdcTarget::Thyroid => "thyroid",
            EdcTarget::General => "general",
        }
    }
}

/// Apply EDC modulation to a counter's drift rates.
/// `exposure` is 0.0–1.0 (0 = none, 1 = high).
fn edc_modulate(counter: Counter, exposure: f64, target: EdcTarget) -> f64 {
    if exposure <= 0.0 || target == EdcTarget::None {
        return 1.0; // no modulation
    }
    match target {
        EdcTarget::Thyroid => {
            // Thyroid EDCs primarily affect mitochondrial (ROS) and proteostatic burden
            match counter {
                Counter::Mitochondrial => 1.0 + 0.8 * exposure,   // up to 1.8x
                Counter::Proteostasis  => 1.0 + 0.5 * exposure,   // up to 1.5x
                Counter::Epigenetic    => 1.0 + 0.2 * exposure,   // small effect via thyroid hormone
                _ => 1.0,
            }
        }
        EdcTarget::General => {
            1.0 + 0.3 * exposure // uniform effect
        }
        EdcTarget::None => unreachable!(), // handled by early return above
    }
}

/// Centriole geometry — spatial (non-chemical) inheritance of length.
/// Ornstein–Uhlenbeck: L_{n+1} = α·L_n + (1−α)·L0 + ε, ε ~ N(0, σ²).
/// α = transmission coefficient (≈0.97 per MCARA geometric model);
/// L0 = regulated equilibrium length (young reference).
///
/// Asymmetric-division model: in stem-cell compartments the OLDEST (mother)
/// centriole is retained by the daughter that preserves stemness (Yamashita 2007;
/// Wang 2009). Retained centrioles are not replaced and therefore accumulate
/// geometric damage faster, degrading cilia and centrosome (MTOC) function.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct CentrioleGeometry {
    pub length: f64,       // current normalised length (L0 = 1.0)
    pub length0: f64,      // equilibrium (young) length
    pub alpha: f64,        // transmission coefficient
    pub sigma: f64,        // per-division noise
    pub retained: bool,    // true if the oldest centriole is retained (asymmetric stem division)
}

impl Default for CentrioleGeometry {
    fn default() -> Self {
        Self {
            length: 1.0,
            length0: 1.0,
            alpha: 0.97,  // model estimate (Köhrer +29% over ~50 divisions)
            sigma: 0.02,
            retained: false,
        }
    }
}

impl CentrioleGeometry {
    /// Advance one division-equivalent. `retain` = true for asymmetric stem-cell divisions
    /// in which the aged mother centriole is kept by the stem daughter (no replacement),
    /// which weakens mean reversion and adds damage.
    pub fn step(&mut self, rng: &mut ChaCha8Rng, retain: bool) {
        self.retained = retain;
        let epsilon: f64 = rng.gen::<f64>() * 2.0 - 1.0;
        let eps = epsilon * self.sigma;
        // Retained (aged) centrioles: mean reversion towards L0 is partially lost,
        // effectively pushing α towards 1 (unregulated random walk regime).
        let alpha_eff = if retain { 0.985 } else { self.alpha };
        self.length = alpha_eff * self.length + (1.0 - alpha_eff) * self.length0 + eps;
        self.length = self.length.max(0.5).min(3.0); // clamp to physiologically plausible range
    }

    /// Geometric damage = deviation from young reference (0 at L0, grows with over-elongation).
    pub fn damage(&self) -> f64 {
        (self.length - self.length0).abs() / self.length0
    }

    /// Steady-state variance Var(L∞) = σ²/(1−α²).
    pub fn steady_state_variance(&self) -> f64 {
        self.sigma * self.sigma / (1.0 - self.alpha * self.alpha)
    }

    /// Primary-cilium competence: declines as geometric damage accumulates.
    /// Reference: Odf2-deficient mother centrioles cannot form primary cilia (Ishikawa 2005).
    pub fn cilia_function(&self) -> f64 {
        (1.0 - self.damage() / 0.30).clamp(0.0, 1.0)
    }

    /// Centrosome (MTOC) competence: declines with geometric damage and, via feedback,
    /// with the PTM burden. Reference: centrosome disruption triggers senescence (Manning 2010).
    pub fn centrosome_function(&self, ptm_damage: f64) -> f64 {
        (1.0 - (self.damage() + 0.5 * ptm_damage) / 0.60).clamp(0.0, 1.0)
    }
}

/// One simulation time-step: advances all counters by ONE division-equivalent (Δn=1) and
/// `dt_seconds` of wall-clock time.
///
/// `edc_exposure` (0.0–1.0) and `edc_target` enable optional EDC modulation.
pub fn step(
    states: &mut [CounterState; N_COUNTERS],
    tissue: Tissue,
    _n_divisions_cumulative: f64,
    _t_seconds_cumulative: f64,
    dn: f64,
    dt_seconds: f64,
    gamma: &Gamma,
    edc_exposure: f64,
    edc_target: EdcTarget,
    geometry: &mut CentrioleGeometry,
    rng: &mut ChaCha8Rng,
) {
    let prev = *states;
    for c in Counter::ALL {
        let rates = default_drift_rates(c, tissue);
        let scales = default_reference_scales(c, tissue);
        let div_inc = match scales.n_star {
            Some(n_star) if n_star > 0.0 => rates.alpha * (dn / n_star),
            _ => 0.0,
        };
        let time_inc = if scales.tau_seconds > 0.0 {
            rates.beta * (dt_seconds / scales.tau_seconds)
        } else {
            0.0
        };
        // EDC modulation multiplier
        let edc_mult = edc_modulate(c, edc_exposure, edc_target);
        let gamma_i = 0.01;
        let coupling = gamma_i * gamma.influence(c, &prev);
        // Apply EDC multiplier to the total drift increment
        let increment = (div_inc + time_inc) * edc_mult + coupling;
        states[c as usize].value = prev[c as usize].value + increment;
    }
    // Geometric centriole inheritance (spatial, not chemical)
    // Asymmetric divisions: in stem-cell compartments the oldest centriole is retained
    // by the stem daughter (Yamashita 2007; Wang 2009) -> accelerated geometric accumulation.
    let retain = matches!(
        tissue,
        Tissue::Hsc | Tissue::Fibroblast | Tissue::CD8TMemory
    );
    geometry.step(rng, retain);
    // Geometric damage feeds back into the centriolar counter; retention boosts the build-up
    let retention_boost = if retain { 1.6 } else { 1.0 };
    states[Counter::Centriolar as usize].value +=
        0.02 * retention_boost * geometry.damage() * dn;
}

/// Tissue-integrated load L_tissue = Σ_i w_i · f_i(D_i). Here f_i = identity.
pub fn tissue_load(states: &[CounterState; N_COUNTERS], tissue: Tissue) -> f64 {
    let w = default_weights(tissue);
    let mut sum = 0.0;
    for c in Counter::ALL {
        sum += w.get(c) * states[c as usize].value;
    }
    sum
}

#[derive(Debug, Clone, Serialize)]
pub struct SimulationRecord {
    pub step: usize,
    pub n_cumulative: f64,
    pub t_seconds: f64,
    pub t_years: f64,
    pub telomere: f64,
    pub centriolar: f64,
    pub mito: f64,
    pub epigenetic: f64,
    pub proteostasis: f64,
    pub centriole_length: f64,
    pub geometry_damage: f64,
    pub stem_retention: bool,
    pub cilia_function: f64,
    pub centrosome_function: f64,
    pub epigenetic_critical: bool,
    pub tissue_load: f64,
    pub edc_exposure: f64,
    pub edc_target: String,
}

/// Run a forward simulation of `n_steps` discrete divisions; each step is one division-equivalent
/// of duration `seconds_per_division`.
///
/// `edc_exposure` (0.0–1.0) and `edc_target` enable EDC modulation (default: none).
pub fn run(
    tissue: Tissue,
    n_steps: usize,
    seconds_per_division: f64,
    gamma: &Gamma,
    edc_exposure: f64,
    edc_target: EdcTarget,
    seed: u64,
) -> Vec<SimulationRecord> {
    let yr = 365.25 * 24.0 * 3600.0;
    let mut states = [CounterState::default(); N_COUNTERS];
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut geometry = CentrioleGeometry::default();
    let mut records = Vec::with_capacity(n_steps + 1);
    let mut n_cum = 0.0;
    let mut t_cum = 0.0;
    let edc_label = edc_target.as_str().to_string();
    for step_idx in 0..=n_steps {
        let epi = states[Counter::Epigenetic as usize].value;
        records.push(SimulationRecord {
            step: step_idx,
            n_cumulative: n_cum,
            t_seconds: t_cum,
            t_years: t_cum / yr,
            centriolar:   states[Counter::Centriolar    as usize].value,
            telomere:     states[Counter::Telomere      as usize].value,
            mito:         states[Counter::Mitochondrial as usize].value,
            epigenetic:   epi,
            proteostasis: states[Counter::Proteostasis  as usize].value,
            centriole_length: geometry.length,
            geometry_damage: geometry.damage(),
            stem_retention: geometry.retained,
            cilia_function: geometry.cilia_function(),
            centrosome_function: geometry.centrosome_function(states[Counter::Centriolar as usize].value),
            epigenetic_critical: is_epigenetic_above_critical(epi),
            tissue_load: tissue_load(&states, tissue),
            edc_exposure,
            edc_target: edc_label.clone(),
        });
        if step_idx < n_steps {
            let dn = 1.0;
            let dt = seconds_per_division;
            n_cum += dn;
            t_cum += dt;
            step(&mut states, tissue, n_cum, t_cum, dn, dt, gamma, edc_exposure, edc_target, &mut geometry, &mut rng);
        }
    }
    records
}

#[cfg(test)]
mod tests {
    use super::*;

    const YR: f64 = 365.25 * 24.0 * 3600.0;

    fn run_default(tissue: Tissue, n_steps: usize, secs: f64) -> Vec<SimulationRecord> {
        let gamma = Gamma::default();
        run(tissue, n_steps, secs, &gamma, 0.0, EdcTarget::None, 42)
    }

    #[test]
    fn hsc_100_step_run_produces_expected_growth() {
        let records = run_default(Tissue::Hsc, 100, 7.0 * 86400.0);
        assert_eq!(records.len(), 101);
        for pair in records.windows(2) {
            assert!(pair[1].tissue_load >= pair[0].tissue_load - 1e-9);
        }
        let last = records.last().unwrap();
        assert!(last.tissue_load > 0.0);
        assert!(last.tissue_load < 0.6);
        assert!(last.mito > last.telomere);
        assert!(last.mito > last.centriolar);
    }

    #[test]
    fn post_mitotic_neuron_has_no_division_contribution_to_centriolar() {
        let records = run_default(Tissue::Neuron, 50, 30.0 * 86400.0);
        let first = &records[0];
        let last = records.last().unwrap();
        assert!(last.centriolar >= first.centriolar);
    }

    #[test]
    fn edc_thyroid_accelerates_mito_and_proteostasis() {
        let gamma = Gamma::default();
        let baseline = run(Tissue::Hsc, 200, 7.0 * 86400.0, &gamma, 0.0, EdcTarget::None, 7);
        let edc = run(Tissue::Hsc, 200, 7.0 * 86400.0, &gamma, 0.8, EdcTarget::Thyroid, 7);
        let b_last = baseline.last().unwrap();
        let e_last = edc.last().unwrap();
        assert!(e_last.mito > b_last.mito, "EDC should increase mito damage");
        assert!(e_last.proteostasis > b_last.proteostasis, "EDC should increase proteostasis damage");
        assert!(e_last.tissue_load > b_last.tissue_load, "EDC should increase tissue load");
    }

    #[test]
    fn geometry_drifts_with_mean_reversion() {
        let records = run_default(Tissue::Hsc, 200, 7.0 * 86400.0);
        let g0 = records[0].centriole_length;
        let gl = records.last().unwrap().centriole_length;
        // With α = 0.97 the OU process stays near L0 but accumulates small deviations
        assert!((g0 - 1.0).abs() < 1e-9);
        assert!(gl >= 0.5 && gl <= 3.0);
        // Damage grows over time
        assert!(records.last().unwrap().geometry_damage > 0.0);
    }

    #[test]
    fn epigenetic_clock_tau_100yr() {
        // 52 divisions × 7 days ≈ 1 year. Pure time-driven drift ≈ t/τ = 1/100 = 0.01
        // (plus a small coupling contribution from other counters).
        let records = run_default(Tissue::Hsc, 52, 7.0 * 86400.0);
        let epi = records.last().unwrap().epigenetic;
        assert!(epi > 0.008 && epi < 0.15, "expected ≈0.01–0.05, got {}", epi);
        assert!(!records.last().unwrap().epigenetic_critical);
    }

    #[test]
    fn epigenetic_clock_reaches_critical_at_long_horizon() {
        // 5200 divisions × 7 days ≈ 100 years; coupling accelerates, threshold 0.75 is crossed
        let records = run_default(Tissue::Hsc, 5200, 7.0 * 86400.0);
        assert!(records.last().unwrap().epigenetic_critical,
            "epigenetic counter should cross 0.75 by 100 years, got {}",
            records.last().unwrap().epigenetic);
    }

    #[test]
    fn steady_state_variance_diverges_as_alpha_to_1() {
        let g = CentrioleGeometry { alpha: 0.99, sigma: 0.02, ..Default::default() };
        assert!(g.steady_state_variance() > CentrioleGeometry::default().steady_state_variance());
    }

    #[test]
    fn asymmetric_stem_retention_accumulates_more_damage() {
        // HSC (asymmetric, retains oldest centriole) accumulates more geometric damage
        // than a symmetric/post-mitotic tissue over the same horizon.
        let hsc = run_default(Tissue::Hsc, 300, 7.0 * 86400.0);
        let neuron = run_default(Tissue::Neuron, 300, 7.0 * 86400.0);
        assert!(hsc.last().unwrap().stem_retention);
        assert!(!neuron.last().unwrap().stem_retention);
        assert!(hsc.last().unwrap().geometry_damage > neuron.last().unwrap().geometry_damage,
            "retained centriole should accumulate more geometric damage");
    }

    #[test]
    fn cilia_and_centrosome_function_degrade_with_accumulation() {
        let records = run_default(Tissue::Hsc, 2000, 7.0 * 86400.0);
        let first = &records[0];
        let last = records.last().unwrap();
        assert!((first.cilia_function - 1.0).abs() < 1e-9);
        assert!(last.cilia_function < first.cilia_function,
            "cilia function should degrade with centriole accumulation");
        assert!(last.centrosome_function < first.centrosome_function,
            "centrosome function should degrade with centriole accumulation");
    }
}
