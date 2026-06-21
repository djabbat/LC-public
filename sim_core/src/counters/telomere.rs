/// Счётчик #2: Теломерный — укорочение теломер, лимит Хейфлика
/// PMID 24138928 (Horvath 2013), Hayflick 1965
/// Источник: PMID 24138928 — эпигенетические часы и теломеры

use crate::counters::{CounterParams, CounterState, CounterType};
use crate::provenance::sources;

pub fn default_params() -> CounterParams {
    CounterParams {
        alpha: 0.02 / 50.0,
        beta: 0.002,
        n_star: 50.0,
        tau_star: 0.019, // ~1 неделя turnover теломерных повторов
        k: 5.0,
        x_crit: 0.5,
        source: sources::TELOMERE_SHORTENING,
    }
}

pub fn new_state() -> CounterState {
    CounterState::new(CounterType::Telomere, default_params())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telomere_default_damage_is_zero() {
        let c = new_state();
        assert!((c.damage - 0.0).abs() < 1e-10);
    }

    #[test]
    fn telomere_has_hayflick_n_star() {
        let p = default_params();
        assert!((p.n_star - 50.0).abs() < 1e-10);
    }
}
