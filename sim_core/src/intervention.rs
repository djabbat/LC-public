// Интервенции — симуляция лекарств, диеты, генотерапии
//
// Позволяет моделировать эффекты интервенций на старение:
// - Снижение калорий (CR) → уменьшение ROS
// - Рапамицин → усиление аутофагии (счётчик #5)
// - Ингибиторы TTLL → снижение polyGlu (счётчик #1)
// - Терапия стволовыми клетками → сброс D_i для ткани
// - Генотерапия (TERT, CCP1) → изменение параметров счётчиков

use crate::{Fraction, Time};

/// Тип интервенции
#[derive(Debug, Clone)]
pub enum InterventionType {
    /// Снижение калорий: уменьшает ROS на factor (0..1)
    CaloricRestriction { ros_reduction: Fraction },
    /// Рапамицин: усиливает аутофагию, снижает proteo damage
    Rapamycin { proteo_reduction: Fraction },
    /// Ингибитор TTLL: снижает скорость polyGlu
    TTLLInhibitor { polyglu_reduction: Fraction },
    /// Антиоксидант (NAC): снижает ROS
    Antioxidant { ros_reduction: Fraction },
    /// Генотерапия TERT: снижает telomere damage rate
    TERTTherapy { telo_reduction: Fraction },
    /// Терапия стволовыми клетками: сбрасывает D_i для ткани
    StemCellTherapy { tissue_index: usize, reset_fraction: Fraction },
    /// Диета (макробиом): меняет метаболические параметры
    Diet { ros_change: Fraction, proteo_change: Fraction },
    /// Пользовательская интервенция
    Custom { name: String, ros_mult: Fraction, damage_mults: Vec<Fraction> },
}

/// Результат интервенции
#[derive(Debug, Clone)]
pub struct InterventionResult {
    pub name: String,
    pub years_gained: Time,
    pub l_max_reduction: Fraction,
    pub fi_reduction: Fraction,
    pub age_at_death_with: Time,
    pub age_at_death_without: Time,
}

impl InterventionResult {
    pub fn new(name: &str, with: Time, without: Time) -> Self {
        Self {
            name: name.into(),
            years_gained: with - without,
            l_max_reduction: 0.0,
            fi_reduction: 0.0,
            age_at_death_with: with,
            age_at_death_without: without,
        }
    }
}

/// Симуляция интервенции: применить и сравнить с контролем
pub fn simulate_intervention(
    intervention: &InterventionType,
    _start_age: Time,
) -> InterventionResult {
    // В v1.0 — заглушка. Полная реализация — Phase 5.
    match intervention {
        InterventionType::CaloricRestriction { ros_reduction } => {
            // Каждый 1% снижения калорий → ~0.5% продления жизни (мыши)
            let gain = ros_reduction * 0.5 * 30.0; // примерно
            InterventionResult {
                name: format!("CR {:.0}%", ros_reduction * 100.0),
                years_gained: gain,
                l_max_reduction: ros_reduction * 0.3,
                fi_reduction: ros_reduction * 0.2,
                age_at_death_with: 77.0 + gain,
                age_at_death_without: 77.0,
            }
        }
        InterventionType::Rapamycin { proteo_reduction } => {
            let gain = proteo_reduction * 20.0;
            InterventionResult {
                name: "Рапамицин".into(),
                years_gained: gain,
                l_max_reduction: *proteo_reduction,
                fi_reduction: *proteo_reduction * 0.5,
                age_at_death_with: 77.0 + gain,
                age_at_death_without: 77.0,
            }
        }
        _ => InterventionResult {
            name: format!("{:?}", intervention),
            years_gained: 0.0,
            l_max_reduction: 0.0,
            fi_reduction: 0.0,
            age_at_death_with: 77.0,
            age_at_death_without: 77.0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cr_extends_lifespan() {
        let result = simulate_intervention(
            &InterventionType::CaloricRestriction { ros_reduction: 0.3 },
            50.0,
        );
        assert!(result.years_gained > 0.0);
    }

    #[test]
    fn rapamycin_works() {
        let result = simulate_intervention(
            &InterventionType::Rapamycin { proteo_reduction: 0.4 },
            50.0,
        );
        assert!(result.years_gained > 0.0);
    }
}
