// Уровень #2: 5 счётчиков MCAOA
//
// L_tissue = Σ w_i · f_i(D_i)
// f_i(x) = 1 / (1 + exp(-k_i · (x - x_crit)))  — сигмоида

use crate::{Fraction, Time, Divisions};
use crate::provenance::Source;

pub mod telomere;
pub mod mitochondrial;
pub mod epigenetic;
pub mod proteostatic;
pub mod coupling;
pub mod extension;

pub use coupling::GammaMatrix;

/// Типы счётчиков
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterType {
    Telomere = 2,
    Mitochondrial = 3,
    Epigenetic = 4,
    Proteostatic = 5,
    // Слот для #6 (piRNA) — зарезервирован, не активен в v1.0
}

/// Параметры счётчика
#[derive(Debug, Clone, Copy)]
pub struct CounterParams {
    /// Интенсивность от делений
    pub alpha: Fraction,
    /// Интенсивность от времени
    pub beta: Fraction,
    /// Референсное число делений
    pub n_star: Divisions,
    /// Референсное время (годы)
    pub tau_star: Time,
    /// Крутизна сигмоиды
    pub k: Fraction,
    /// Средняя точка сигмоиды (полумаксимальное бремя)
    pub x_crit: Fraction,
    /// Источник параметров (PMID, DOI, оценка, постулат)
    pub source: Source,
}

/// Состояние счётчика
#[derive(Debug, Clone)]
pub struct CounterState {
    pub counter_type: CounterType,
    pub damage: Fraction,
    pub params: CounterParams,
    pub divisions: Divisions,
}

impl CounterState {
    pub fn new(ct: CounterType, params: CounterParams) -> Self {
        Self {
            counter_type: ct,
            damage: 0.0,
            params,
            divisions: 0.0,
        }
    }

    /// Обновить повреждение счётчика
    pub fn update(&mut self, dt: Time, division_rate: Fraction) {
        let dn = division_rate * dt;
        self.divisions += dn;

        // D_i = D_i0 + alpha * (n/n*) + beta * (t/tau*)
        let d_damage = self.params.alpha * (dn / self.params.n_star)
            + self.params.beta * (dt / self.params.tau_star);

        self.damage = (self.damage + d_damage).min(2.0); // допускаем >1 для сильно повреждённых
    }

    /// Сигмоидальная трансформация: повреждение → вклад в бремя
    pub fn burden(&self) -> Fraction {
        let x = self.damage;
        let k = self.params.k;
        let x0 = self.params.x_crit;
        1.0 / (1.0 + (-k * (x - x0)).exp())
    }
}

/// Параметры счётчиков — делегированы в подмодули
impl CounterParams {
    pub fn telomere() -> Self { telomere::default_params() }
    pub fn mitochondrial() -> Self { mitochondrial::default_params() }
    pub fn epigenetic() -> Self { epigenetic::default_params() }
    pub fn proteostatic() -> Self { proteostatic::default_params() }
}

/// Агрегатор L_tissue: взвешенная сумма бремен счётчиков
pub fn l_tissue_aggregator(counters: &[CounterState], weights: &[Fraction]) -> Fraction {
    assert_eq!(counters.len(), weights.len(), "Каждому счётчику — свой вес");
    let total: Fraction = counters.iter()
        .zip(weights.iter())
        .map(|(c, &w)| w * c.burden())
        .sum();
    total.min(1.0)
}

/// Константа: агрегатор L_tissue (для совместимости)
pub static L_TISSUE_AGGREGATOR: &str = "MCAOA v1.0 — 5 counters";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_counter_starts_at_zero() {
        let c = CounterState::new(CounterType::Telomere, CounterParams::telomere());
        assert!((c.damage - 0.0).abs() < 1e-10);
    }

    #[test]
    fn damage_grows_monotonically() {
        let mut c = CounterState::new(CounterType::Telomere, CounterParams::telomere());
        let before = c.damage;
        c.update(1.0, 1.0);
        assert!(c.damage > before);
    }

    #[test]
    fn burden_is_sigmoidal() {
        let mut c = CounterState::new(CounterType::Telomere, CounterParams::telomere());
        // При damage = 0 → burden ≈ 1/(1+exp(2.5)) ≈ 0.076
        let b0 = c.burden();
        assert!(b0 < 0.1);

        // При damage = x_crit = 0.5 → burden = 0.5
        c.damage = 0.5;
        let b_half = c.burden();
        assert!((b_half - 0.5).abs() < 1e-10);
    }

    #[test]
    fn l_tissue_is_weighted_sum() {
        let telo = CounterParams::telomere();
        let mito = CounterParams::mitochondrial();
        let counters = vec![
            CounterState::new(CounterType::Telomere, telo),
            CounterState::new(CounterType::Mitochondrial, mito),
        ];
        let weights = vec![0.5, 0.5];
        let l = l_tissue_aggregator(&counters, &weights);
        assert!(l >= 0.0 && l <= 1.0);
    }
}
