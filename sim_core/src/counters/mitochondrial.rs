/// Счётчик #3: Митохондриальный — АФК, mtDNA, дыхательная цепь
/// PMID 39651989 (Yang 2025), PMID 22252130 (Youle 2012 — митофагия)
/// tau_star = 0.082 года (~30 дней turnover mtDNA через митофагию)

use crate::counters::{CounterParams, CounterState, CounterType};
use crate::provenance::sources;

pub fn default_params() -> CounterParams {
    CounterParams {
        alpha: 0.000,
        beta: 0.001,
        n_star: 50.0,
        tau_star: 0.082, // ~30 дней (Youle 2012, PMID 22252130)
        k: 8.0,
        x_crit: 0.5,
        source: sources::MITOCHONDRIAL_ROS,
    }
}

pub fn new_state() -> CounterState {
    CounterState::new(CounterType::Mitochondrial, default_params())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mito_is_time_driven() {
        let p = default_params();
        assert!((p.alpha - 0.0).abs() < 1e-10); // α ≈ 0 для митохондрий
    }

    #[test]
    fn mito_tau_is_30_days() {
        let p = default_params();
        assert!((p.tau_star - 0.082).abs() < 0.001);
    }
}
