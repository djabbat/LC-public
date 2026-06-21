/// Миграция из mcoa_core — ReferenceScales, DriftRates, Gamma
/// Совместимость с Axiom M2 (dimensional consistency)
///
/// Оригинал: MCAOA/crates/mcoa_core/src/lib.rs
/// Мигрировано: 2026-06-21

use crate::{Fraction, Time, Divisions};

/// Референсные шкалы для счётчика i в конкретной ткани
/// Аксиома M2: n_star и tau ДОЛЖНЫ быть установлены a priori
#[derive(Debug, Clone, Copy)]
pub struct ReferenceScales {
    /// Референсное число делений (None для постмитотических — α→0)
    pub n_star: Option<Divisions>,
    /// Референсное время в годах
    pub tau_years: Time,
}

/// Скорости дрейфа счётчика (безразмерные)
#[derive(Debug, Clone, Copy)]
pub struct DriftRates {
    pub alpha: Fraction,  // интенсивность от делений
    pub beta: Fraction,   // интенсивность от времени
}

/// Axiom M2: dimensional consistency
/// D = D₀ + α·(n/n_star) + β·(t/tau)
pub fn independent_drift(
    d0: Fraction,
    n: Divisions,
    t_years: Time,
    rates: DriftRates,
    scales: ReferenceScales,
) -> Fraction {
    let div_term = match scales.n_star {
        Some(n_star) if n_star > 0.0 => rates.alpha * (n / n_star),
        _ => 0.0, // постмитотические: α → 0
    };
    let time_term = if scales.tau_years > 0.0 {
        rates.beta * (t_years / scales.tau_years)
    } else {
        0.0
    };
    d0 + div_term + time_term
}

/// Референсные шкалы по умолчанию для счётчика и ткани
pub fn default_scales(counter: &str, tissue: &str) -> ReferenceScales {
    match (counter, tissue) {
        ("telomere", "fibroblast") => ReferenceScales { n_star: Some(50.0), tau_years: 1.0 },
        ("telomere", "hsc")        => ReferenceScales { n_star: Some(200.0), tau_years: 1.0 },
        ("telomere", "neuron")     => ReferenceScales { n_star: None, tau_years: 1.0 },
        ("centriole", "hsc")       => ReferenceScales { n_star: Some(65.0), tau_years: 0.5 },
        ("centriole", "neuron")    => ReferenceScales { n_star: None, tau_years: 1.0 },
        ("mitochondrial", "neuron")=> ReferenceScales { n_star: None, tau_years: 0.082 },
        ("mitochondrial", _)       => ReferenceScales { n_star: None, tau_years: 0.082 },
        ("epigenetic", _)          => ReferenceScales { n_star: None, tau_years: 3.6 },
        ("proteostatic", _)        => ReferenceScales { n_star: Some(80.0), tau_years: 1.0 },
        _                          => ReferenceScales { n_star: Some(50.0), tau_years: 1.0 },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_mitotic_alpha_is_zero() {
        let scales = ReferenceScales { n_star: None, tau_years: 1.0 };
        let rates = DriftRates { alpha: 0.015, beta: 0.01 };
        let d = independent_drift(0.0, 9999.0, 1.0, rates, scales);
        assert!((d - 0.01).abs() < 1e-10, "α must be ignored for post-mitotic");
    }

    #[test]
    fn dimensional_consistency() {
        // n = n_star, t = tau → D = α + β
        let scales = ReferenceScales { n_star: Some(50.0), tau_years: 1.0 };
        let rates = DriftRates { alpha: 0.02, beta: 0.005 };
        let d = independent_drift(0.0, 50.0, 1.0, rates, scales);
        assert!((d - 0.025).abs() < 1e-10);
    }
}
