/// Счётчик #4: Эпигенетический — метилирование ДНК, Cdc42, полярность
/// PMID 22560076 (Florian 2012 — Cdc42/HSC), PMID 35029144 (Belsky 2022 — DunedinPACE)
/// tau_star = 3.6 лет (DunedinPACE — темп старения)

use crate::counters::{CounterParams, CounterState, CounterType};
use crate::provenance::sources;

pub fn default_params() -> CounterParams {
    CounterParams {
        alpha: 0.001,
        beta: 0.030,
        n_star: 50.0,
        tau_star: 3.6, // DunedinPACE (Belsky 2022, PMID 35029144)
        k: 8.0,
        x_crit: 0.5,
        source: sources::EPIGENETIC_CLOCK,
    }
}

pub fn new_state() -> CounterState {
    CounterState::new(CounterType::Epigenetic, default_params())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epigenetic_is_time_driven() {
        let p = default_params();
        assert!(p.beta > p.alpha * 10.0); // β >> α
    }

    #[test]
    fn epigenetic_tau_is_dunedinpace() {
        let p = default_params();
        assert!((p.tau_star - 3.6).abs() < 0.01);
    }
}
