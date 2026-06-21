// Уровень #3: Ткани + Ze-конфликты
//
// 8 базовых тканей с периодами самообновления, весами счётчиков,
// и межтканевыми конфликтами Z_conflict(i,j).

use crate::{Fraction, Time, counters::CounterState};

/// Конфигурация ткани
#[derive(Debug, Clone)]
pub struct TissueConfig {
    pub name: &'static str,
    /// Период самообновления (дни); f64::INFINITY для постмитотических
    pub renewal_period_days: f64,
    /// Веса счётчиков #2–#5 (центриоль учитывается отдельно)
    pub counter_weights: [Fraction; 4],
    /// Критическое бремя ткани (L_crit)
    pub critical_burden: Fraction,
    /// 3D-координаты центра ткани (нормализованные [0,1])
    pub position: [Fraction; 3],
    /// Сосудистая плотность [0,1]
    pub vascular_density: Fraction,
    /// Плотность иннервации [0,1]
    pub innervation_density: Fraction,
}

/// Состояние ткани во время симуляции
#[derive(Debug, Clone)]
pub struct TissueState {
    pub config: TissueConfig,
    /// Текущее бремя старения L(t)
    pub burden: Fraction,
    /// Скорость изменения бремени dL/dt
    pub burden_rate: Fraction,
    /// Возраст ткани (годы)
    pub age: Time,
}

impl TissueState {
    pub fn new(config: TissueConfig) -> Self {
        Self {
            config,
            burden: 0.0,
            burden_rate: 0.0,
            age: 0.0,
        }
    }

    /// Обновить состояние ткани на основе счётчиков
    pub fn update(&mut self, dt: Time, counters: &[CounterState]) {
        self.age += dt;

        let prev_burden = self.burden;

        // L_tissue = Σ w_i · f_i(D_i)
        self.burden = counters.iter()
            .zip(self.config.counter_weights.iter())
            .map(|(c, &w)| w * c.burden())
            .sum::<Fraction>()
            .min(1.0);

        // Скорость изменения
        self.burden_rate = (self.burden - prev_burden) / dt.max(1e-10);
    }

    /// Ткань в критическом состоянии?
    pub fn is_critical(&self) -> bool {
        self.burden > self.config.critical_burden
    }

    /// Период самообновления в годах
    pub fn renewal_period_years(&self) -> f64 {
        if self.config.renewal_period_days.is_infinite() {
            f64::INFINITY
        } else {
            self.config.renewal_period_days / 365.25
        }
    }

    /// Ze-скорость: |τ · dL/dt|
    pub fn ze_velocity(&self) -> Fraction {
        let tau = if self.renewal_period_years().is_infinite() {
            120.0 // для постмитотических — lifespan
        } else {
            self.renewal_period_years()
        };
        (tau * self.burden_rate as f64).abs().min(1.0) as Fraction
    }
}

/// Межтканевой Ze-конфликт
#[derive(Debug, Clone)]
pub struct ZeConflict {
    pub tissue_i: usize,
    pub tissue_j: usize,
    /// Значение конфликта
    pub value: Fraction,
    /// Сила связи C_ij
    pub coupling: Fraction,
}

impl ZeConflict {
    /// Вычислить Z_conflict(i, j)
    pub fn compute(
        tissue_i: &TissueState,
        tissue_j: &TissueState,
        coupling: Fraction,
    ) -> Self {
        let tau_i = if tissue_i.renewal_period_years().is_infinite() { 120.0 }
            else { tissue_i.renewal_period_years() };
        let tau_j = if tissue_j.renewal_period_years().is_infinite() { 120.0 }
            else { tissue_j.renewal_period_years() };

        let v_i = tau_i * tissue_i.burden_rate as f64;
        let v_j = tau_j * tissue_j.burden_rate as f64;

        let value = ((v_i - v_j).abs() * coupling as f64).min(1.0) as Fraction;

        ZeConflict {
            tissue_i: 0, // будет установлено извне
            tissue_j: 0,
            value,
            coupling,
        }
    }

    /// Конфликт превышает критический порог?
    pub fn is_critical(&self, z_crit: Fraction) -> bool {
        self.value > z_crit
    }
}

/// 8 базовых тканей человека
pub fn human_tissue_configs() -> Vec<TissueConfig> {
    vec![
        TissueConfig {
            name: "Эпидермис",
            renewal_period_days: 28.0,
            counter_weights: [0.50, 0.15, 0.15, 0.10],
            critical_burden: 0.60,
            position: [0.5, 0.5, 0.98],
            vascular_density: 0.3,
            innervation_density: 0.7,
        },
        TissueConfig {
            name: "Кишечный эпителий",
            renewal_period_days: 5.0,
            counter_weights: [0.20, 0.15, 0.15, 0.40],
            critical_burden: 0.60,
            position: [0.5, 0.4, 0.5],
            vascular_density: 0.8,
            innervation_density: 0.6,
        },
        TissueConfig {
            name: "Гепатоциты",
            renewal_period_days: 300.0,
            counter_weights: [0.10, 0.25, 0.15, 0.35],
            critical_burden: 0.60,
            position: [0.7, 0.6, 0.5],
            vascular_density: 0.9,
            innervation_density: 0.3,
        },
        TissueConfig {
            name: "Нейроны",
            renewal_period_days: f64::INFINITY,
            counter_weights: [0.00, 0.55, 0.20, 0.20],
            critical_burden: 0.55,
            position: [0.5, 0.8, 0.5],
            vascular_density: 0.7,
            innervation_density: 1.0,
        },
        TissueConfig {
            name: "Гемопоэтические стволовые",
            renewal_period_days: 90.0,
            counter_weights: [0.20, 0.15, 0.30, 0.15],
            critical_burden: 0.60,
            position: [0.5, 0.3, 0.5],
            vascular_density: 1.0,
            innervation_density: 0.2,
        },
        TissueConfig {
            name: "Кардиомиоциты",
            renewal_period_days: 73000.0, // ~0.5%/год
            counter_weights: [0.00, 0.55, 0.20, 0.20],
            critical_burden: 0.50,
            position: [0.6, 0.55, 0.5],
            vascular_density: 0.9,
            innervation_density: 0.8,
        },
        TissueConfig {
            name: "Эндотелий",
            renewal_period_days: 1095.0, // ~3 года
            counter_weights: [0.20, 0.20, 0.20, 0.20],
            critical_burden: 0.60,
            position: [0.5, 0.5, 0.5],
            vascular_density: 1.0,
            innervation_density: 0.5,
        },
        TissueConfig {
            name: "Костная ткань",
            renewal_period_days: 3650.0, // ~10 лет
            counter_weights: [0.15, 0.15, 0.15, 0.15],
            critical_burden: 0.60,
            position: [0.5, 0.1, 0.5],
            vascular_density: 0.4,
            innervation_density: 0.3,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_8_human_tissues() {
        let configs = human_tissue_configs();
        assert_eq!(configs.len(), 8);
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
    fn ze_conflict_computes() {
        let cfg1 = human_tissue_configs()[0].clone(); // эпидермис, 28 дней
        let cfg2 = human_tissue_configs()[3].clone(); // нейроны, ∞
        let mut t1 = TissueState::new(cfg1);
        let mut t2 = TissueState::new(cfg2);

        // Искусственно задаём разные скорости старения
        t1.burden_rate = 0.01;
        t2.burden_rate = 0.001;

        let z = ZeConflict::compute(&t1, &t2, 1.0);
        assert!(z.value > 0.0);
    }
}
