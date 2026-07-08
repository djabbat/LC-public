//! MCAOA simulation — time evolution of multi-counter damage per tissue.
//!
//! Includes optional EDC (Endocrine Disrupting Chemical) modulation module
//! for modelling environmental impacts on aging (thyroid disruptors, etc.).

use mcoa_core::{
    default_drift_rates, default_reference_scales, default_weights, Counter, CounterState, Gamma,
    Tissue, N_COUNTERS,
};
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
) -> Vec<SimulationRecord> {
    let yr = 365.25 * 24.0 * 3600.0;
    let mut states = [CounterState::default(); N_COUNTERS];
    let mut records = Vec::with_capacity(n_steps + 1);
    let mut n_cum = 0.0;
    let mut t_cum = 0.0;
    let edc_label = edc_target.as_str().to_string();
    for step_idx in 0..=n_steps {
        records.push(SimulationRecord {
            step: step_idx,
            n_cumulative: n_cum,
            t_seconds: t_cum,
            t_years: t_cum / yr,
            centriolar:   states[Counter::Centriolar    as usize].value,
            telomere:     states[Counter::Telomere      as usize].value,
            mito:         states[Counter::Mitochondrial as usize].value,
            epigenetic:   states[Counter::Epigenetic    as usize].value,
            proteostasis: states[Counter::Proteostasis  as usize].value,
            tissue_load: tissue_load(&states, tissue),
            edc_exposure,
            edc_target: edc_label.clone(),
        });
        if step_idx < n_steps {
            let dn = 1.0;
            let dt = seconds_per_division;
            n_cum += dn;
            t_cum += dt;
            step(&mut states, tissue, n_cum, t_cum, dn, dt, gamma, edc_exposure, edc_target);
        }
    }
    records
}

#[cfg(test)]
mod tests {
    use super::*;

    const YR: f64 = 365.25 * 24.0 * 3600.0;

    #[test]
    fn hsc_100_step_run_produces_expected_growth() {
        let gamma = Gamma::default();
        let records = run(Tissue::Hsc, 100, 7.0 * 86400.0, &gamma, 0.0, EdcTarget::None);
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
        let gamma = Gamma::default();
        let records = run(Tissue::Neuron, 50, 30.0 * 86400.0, &gamma, 0.0, EdcTarget::None);
        let first = &records[0];
        let last = records.last().unwrap();
        assert!(last.centriolar >= first.centriolar);
    }

    #[test]
    fn edc_thyroid_accelerates_mito_and_proteostasis() {
        let gamma = Gamma::default();
        let baseline = run(Tissue::Hsc, 200, 7.0 * 86400.0, &gamma, 0.0, EdcTarget::None);
        let edc = run(Tissue::Hsc, 200, 7.0 * 86400.0, &gamma, 0.8, EdcTarget::Thyroid);
        let b_last = baseline.last().unwrap();
        let e_last = edc.last().unwrap();
        // EDC should increase mito and proteostasis
        assert!(e_last.mito > b_last.mito, "EDC should increase mito damage");
        assert!(e_last.proteostasis > b_last.proteostasis, "EDC should increase proteostasis damage");
        // Tissue load should be higher with EDC
        assert!(e_last.tissue_load > b_last.tissue_load, "EDC should increase tissue load");
    }
}
