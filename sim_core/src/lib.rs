// Organismal Aging — Self-learning 4D organism simulator
// Copyright 2026 Jaba Tqemaladze, Georgia Longevity Alliance
// License: Apache-2.0

pub mod centriole;
pub mod counters;
pub mod tissue;
pub mod organism;
pub mod species;
pub mod microbiome;
pub mod macrobiome;
pub mod learning;
pub mod spatial;
pub mod provenance;
pub mod intervention;
pub mod migration;

pub mod prelude;

/// Версия симулятора
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Тип для времени симуляции (в годах)
pub type Time = f64;

/// Тип для безразмерных величин [0, 1]
pub type Fraction = f64;

/// Тип для счётчиков делений
pub type Divisions = f64;

/// Результат шага симуляции
#[derive(Debug, Clone)]
pub struct SimulationStep {
    pub time: Time,
    pub centriole_entropy: Fraction,
    pub tissue_burdens: Vec<Fraction>,
    pub ze_conflicts: Vec<Vec<Fraction>>,
    pub frailty_index: Fraction,
    pub is_alive: bool,
    pub events: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn simulation_step_creation() {
        let step = SimulationStep {
            time: 0.0,
            centriole_entropy: 0.01,
            tissue_burdens: vec![0.0; 8],
            ze_conflicts: vec![vec![0.0; 8]; 8],
            frailty_index: 0.0,
            is_alive: true,
            events: vec![],
        };
        assert!(step.is_alive);
        assert!((step.centriole_entropy - 0.01).abs() < 1e-10);
    }
}
