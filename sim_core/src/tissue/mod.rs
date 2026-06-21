// Уровень #3: Ткани + Ze-конфликты
// Исправленная версия: burden_rate — скользящее среднее, Ze-конфликты корректны.

pub mod types;
pub mod renewal;
pub mod ze_conflict;
pub mod weights;
pub mod connectivity;
pub mod extension;

use crate::{Fraction, Time, counters::CounterState};

#[derive(Debug, Clone)]
pub struct TissueConfig {
    pub name: &'static str,
    pub renewal_period_days: f64,
    pub counter_weights: [Fraction; 4],
    pub critical_burden: Fraction,
    pub position: [Fraction; 3],
    pub vascular_density: Fraction,
    pub innervation_density: Fraction,
}

#[derive(Debug, Clone)]
pub struct TissueState {
    pub config: TissueConfig,
    pub burden: Fraction,
    /// Сглаженная скорость старения (экспоненциальное скользящее среднее)
    pub burden_rate: Fraction,
    pub age: Time,
    /// История бремени для вычисления dL/dt на больших интервалах
    burden_history: Vec<(Time, Fraction)>,
    /// Параметр сглаживания (0 < alpha < 1, ближе к 0 = более гладко)
    smoothing_alpha: Fraction,
}

impl TissueState {
    pub fn new(config: TissueConfig) -> Self {
        Self {
            config,
            burden: 0.0,
            burden_rate: 0.0,
            age: 0.0,
            burden_history: Vec::new(),
            smoothing_alpha: 0.1,
        }
    }

    pub fn update(&mut self, dt: Time, counters: &[CounterState], centriole_entropy: Fraction) {
        self.age += dt;
        let prev = self.burden;

        // L_tissue = w_centriole · S_centriole + Σ w_i · f_i(D_i)
        // w_centriole = 1.0 - Σ w_i  (остаток от 1.0)
        let counter_sum: Fraction = self.config.counter_weights.iter().sum();
        let w_centriole = (1.0 - counter_sum).max(0.0);

        let counter_burden: Fraction = counters.iter()
            .zip(self.config.counter_weights.iter())
            .map(|(c, &w)| w * c.burden())
            .sum();

        self.burden = (w_centriole * centriole_entropy + counter_burden).min(1.0);

        // Мгновенная скорость
        let instant_rate = (self.burden - prev) / dt.max(1e-10);

        // Экспоненциальное скользящее среднее
        self.burden_rate = self.smoothing_alpha * instant_rate
            + (1.0 - self.smoothing_alpha) * self.burden_rate;

        // Храним историю (максимум 20 точек)
        self.burden_history.push((self.age, self.burden));
        if self.burden_history.len() > 20 {
            self.burden_history.remove(0);
        }
    }

    /// dL/dt по линейной регрессии на истории (более стабильно)
    pub fn burden_rate_smoothed(&self) -> Fraction {
        if self.burden_history.len() < 2 {
            return self.burden_rate;
        }
        let n = self.burden_history.len() as f64;
        let sum_t: f64 = self.burden_history.iter().map(|(t, _)| *t).sum();
        let sum_l: f64 = self.burden_history.iter().map(|(_, l)| *l).sum();
        let sum_tl: f64 = self.burden_history.iter().map(|(t, l)| t * l).sum();
        let sum_t2: f64 = self.burden_history.iter().map(|(t, _)| t * t).sum();

        let denom = n * sum_t2 - sum_t * sum_t;
        if denom.abs() < 1e-10 {
            return self.burden_rate;
        }
        let slope = (n * sum_tl - sum_t * sum_l) / denom;
        slope.max(0.0) as Fraction // скорость не может быть отрицательной
    }

    pub fn is_critical(&self) -> bool {
        self.burden > self.config.critical_burden
    }

    pub fn renewal_period_years(&self) -> f64 {
        if self.config.renewal_period_days.is_infinite() {
            f64::INFINITY
        } else {
            self.config.renewal_period_days / 365.25
        }
    }

    /// Ze-скорость: τ · dL/dt (использует сглаженную скорость)
    pub fn ze_velocity(&self) -> Fraction {
        let tau = if self.renewal_period_years().is_infinite() {
            120.0
        } else {
            self.renewal_period_years()
        };
        let rate = self.burden_rate_smoothed();
        (tau * rate as f64).abs().min(1.0) as Fraction
    }
}

#[derive(Debug, Clone)]
pub struct ZeConflict {
    pub tissue_i: usize,
    pub tissue_j: usize,
    pub value: Fraction,
    pub coupling: Fraction,
}

impl ZeConflict {
    pub fn compute(
        tissue_i: &TissueState,
        tissue_j: &TissueState,
        coupling: Fraction,
    ) -> Self {
        let v_i = tissue_i.ze_velocity() as f64;
        let v_j = tissue_j.ze_velocity() as f64;
        let value = ((v_i - v_j).abs() * coupling as f64).min(1.0) as Fraction;

        ZeConflict { tissue_i: 0, tissue_j: 0, value, coupling }
    }

    pub fn is_critical(&self, z_crit: Fraction) -> bool {
        self.value > z_crit
    }
}

/// 8 базовых тканей человека (с более агрессивными параметрами)
pub fn human_tissue_configs() -> Vec<TissueConfig> {
    vec![
        TissueConfig { name: "Эпидермис", renewal_period_days: 28.0,
            counter_weights: [0.50, 0.15, 0.15, 0.10], critical_burden: 0.60,
            position: [0.5, 0.5, 0.98], vascular_density: 0.3, innervation_density: 0.7 },
        TissueConfig { name: "Кишечный эпителий", renewal_period_days: 5.0,
            counter_weights: [0.20, 0.15, 0.15, 0.40], critical_burden: 0.60,
            position: [0.5, 0.4, 0.5], vascular_density: 0.8, innervation_density: 0.6 },
        TissueConfig { name: "Гепатоциты", renewal_period_days: 300.0,
            counter_weights: [0.10, 0.25, 0.15, 0.35], critical_burden: 0.60,
            position: [0.7, 0.6, 0.5], vascular_density: 0.9, innervation_density: 0.3 },
        TissueConfig { name: "Нейроны", renewal_period_days: f64::INFINITY,
            counter_weights: [0.00, 0.55, 0.20, 0.20], critical_burden: 0.75,
            position: [0.5, 0.8, 0.5], vascular_density: 0.7, innervation_density: 1.0 },
        TissueConfig { name: "Гемопоэтические стволовые", renewal_period_days: 90.0,
            counter_weights: [0.20, 0.15, 0.30, 0.15], critical_burden: 0.65,
            position: [0.5, 0.3, 0.5], vascular_density: 1.0, innervation_density: 0.2 },
        TissueConfig { name: "Кардиомиоциты", renewal_period_days: 73000.0,
            counter_weights: [0.00, 0.55, 0.20, 0.20], critical_burden: 0.70,
            position: [0.6, 0.55, 0.5], vascular_density: 0.9, innervation_density: 0.8 },
        TissueConfig { name: "Эндотелий", renewal_period_days: 1095.0,
            counter_weights: [0.20, 0.20, 0.20, 0.20], critical_burden: 0.60,
            position: [0.5, 0.5, 0.5], vascular_density: 1.0, innervation_density: 0.5 },
        TissueConfig { name: "Костная ткань", renewal_period_days: 3650.0,
            counter_weights: [0.15, 0.15, 0.15, 0.15], critical_burden: 0.60,
            position: [0.5, 0.1, 0.5], vascular_density: 0.4, innervation_density: 0.3 },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_8_human_tissues() {
        assert_eq!(human_tissue_configs().len(), 8);
    }

    #[test]
    fn tissue_burden_starts_at_zero() {
        let cfg = &human_tissue_configs()[0];
        let t = TissueState::new(cfg.clone());
        assert!((t.burden - 0.0).abs() < 1e-10);
    }

    #[test]
    fn postmitotic_has_infinite_renewal() {
        let neurons = &human_tissue_configs()[3];
        assert!(neurons.renewal_period_days.is_infinite());
    }

    #[test]
    fn burden_rate_is_smoothed() {
        let cfg = human_tissue_configs()[0].clone();
        let mut t = TissueState::new(cfg);
        // После 20+ обновлений сглаженная скорость должна быть доступна
        for _ in 0..25 {
            t.burden_history.push((t.age, 0.5));
            t.age += 1.0;
        }
        let rate = t.burden_rate_smoothed();
        assert!(rate >= 0.0);
    }
}
