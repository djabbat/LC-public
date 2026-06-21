/// Счётчик #5: Протеостатический — агрегация белков, шапероны, PPIA
/// PMID 40738832 (Catic 2026), Maneix 2024 (DOI 10.1038/s41556-024-01387-x)
/// tau_star = 0.027 года (~10 дней turnover PPIA)

use crate::counters::{CounterParams, CounterState, CounterType};
use crate::provenance::sources;

pub fn default_params() -> CounterParams {
    CounterParams {
        alpha: 0.001,
        beta: 0.0004,
        n_star: 50.0,
        tau_star: 0.027, // ~10 дней turnover PPIA
        k: 8.0,
        x_crit: 0.5,
        source: sources::PROTEOSTASIS_HSC,
    }
}

pub fn new_state() -> CounterState {
    CounterState::new(CounterType::Proteostatic, default_params())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proteostasis_has_ppia_turnover() {
        let p = default_params();
        assert!((p.tau_star - 0.027).abs() < 0.001);
    }

    #[test]
    fn proteostasis_beta_is_low() {
        let p = default_params();
        assert!(p.beta < 0.001);
    }
}
