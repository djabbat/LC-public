// Макробиом — питание, среда, INFOGEST
//
// INFOGEST — стандартизированный протокол in vitro пищеварения
// (Brodkorb et al. 2019, Nature Protocols, DOI: 10.1038/s41596-018-0119-1)
//
// Три фазы пищеварения:
//   1. Оральная фаза (амилаза, pH 7.0, 2 мин, 37°C)
//   2. Желудочная фаза (пепсин, pH 3.0, 2 ч, 37°C)
//   3. Кишечная фаза (панкреатин, желчные кислоты, pH 7.0, 2 ч, 37°C)
//
// Результат: расщепление нутриентов → всасывание → метаболизм → влияние на старение

use crate::Fraction;
use crate::provenance::Source;

/// Источник: INFOGEST протокол
pub const INFOGEST_SOURCE: Source = Source::DOI(
    "10.1038/s41596-018-0119-1",
    "Brodkorb 2019 — INFOGEST static in vitro simulation of gastrointestinal food digestion"
);

/// Конфигурация питания (макробиом)
#[derive(Debug, Clone)]
pub struct DietConfig {
    pub name: &'static str,
    /// Калорийность (ккал/день)
    pub calories: f64,
    /// Белки (г/день)
    pub protein_g: f64,
    /// Жиры (г/день)
    pub fat_g: f64,
    /// Углеводы (г/день)
    pub carbs_g: f64,
    /// Клетчатка (г/день)
    pub fiber_g: f64,
    /// Множитель ROS (1.0 = стандарт)
    pub ros_multiplier: Fraction,
    /// Множитель повреждения протеостаза
    pub proteo_multiplier: Fraction,
}

impl Default for DietConfig {
    fn default() -> Self {
        // Стандартная западная диета (~2500 ккал)
        Self {
            name: "Western diet",
            calories: 2500.0,
            protein_g: 90.0,
            fat_g: 100.0,
            carbs_g: 300.0,
            fiber_g: 15.0,
            ros_multiplier: 1.0,
            proteo_multiplier: 1.0,
        }
    }
}

/// Предустановленные диеты
impl DietConfig {
    /// Средиземноморская диета
    pub fn mediterranean() -> Self {
        Self {
            name: "Mediterranean",
            calories: 2200.0,
            protein_g: 80.0,
            fat_g: 70.0,  // оливковое масло
            carbs_g: 250.0,
            fiber_g: 30.0,
            ros_multiplier: 0.7,  // меньше окислительный стресс
            proteo_multiplier: 0.8, // меньше агрегация
        }
    }

    /// Калорийное ограничение (CR, -30%)
    pub fn caloric_restriction() -> Self {
        Self {
            name: "CR 30%",
            calories: 1750.0,
            protein_g: 70.0,
            fat_g: 70.0,
            carbs_g: 200.0,
            fiber_g: 25.0,
            ros_multiplier: 0.5,
            proteo_multiplier: 0.6,
        }
    }

    /// Высокожировая диета (Western fast food)
    pub fn high_fat() -> Self {
        Self {
            name: "High-fat",
            calories: 3500.0,
            protein_g: 100.0,
            fat_g: 180.0,
            carbs_g: 350.0,
            fiber_g: 10.0,
            ros_multiplier: 1.5,
            proteo_multiplier: 1.4,
        }
    }
}

/// Результаты INFOGEST-симуляции пищеварения
#[derive(Debug, Clone)]
pub struct DigestionResult {
    /// Время транзита (часы)
    pub transit_time_h: f64,
    /// Усвоено белков (%)
    pub protein_absorbed: Fraction,
    /// Усвоено жиров (%)
    pub fat_absorbed: Fraction,
    /// Усвоено углеводов (%)
    pub carbs_absorbed: Fraction,
    /// Продукция короткоцепочечных жирных кислот (SCFA, мМ)
    pub scfa_production: f64,
    /// Ферментировано клетчатки (%)
    pub fiber_fermented: Fraction,
}

impl DigestionResult {
    /// Симуляция INFOGEST (упрощённая)
    pub fn simulate(diet: &DietConfig) -> Self {
        // Оральная фаза: амилаза расщепляет ~5% углеводов
        let oral_carbs = diet.carbs_g * 0.05;

        // Желудочная фаза: пепсин расщепляет ~15% белков
        let gastric_protein = diet.protein_g * 0.15;

        // Кишечная фаза: основное переваривание
        let intestinal_protein = diet.protein_g * 0.70;
        let intestinal_fat = diet.fat_g * 0.85;
        let intestinal_carbs = diet.carbs_g * 0.80;

        // Клетчатка → SCFA (микробиом)
        let fiber_fermented = 0.60; // 60% клетчатки ферментируется
        let scfa = diet.fiber_g * fiber_fermented * 0.5; // ~0.5 мМ SCFA на г клетчатки

        Self {
            transit_time_h: 24.0,
            protein_absorbed: (gastric_protein + intestinal_protein) / diet.protein_g.max(1.0),
            fat_absorbed: intestinal_fat / diet.fat_g.max(1.0),
            carbs_absorbed: (oral_carbs + intestinal_carbs) / diet.carbs_g.max(1.0),
            scfa_production: scfa,
            fiber_fermented,
        }
    }

    /// Влияние пищеварения на ROS (через митохондрии)
    pub fn ros_impact(&self, diet: &DietConfig) -> Fraction {
        // Больше калорий и жиров → больше ROS
        let cal_factor = (diet.calories / 2000.0).min(2.0) as Fraction;
        let fat_factor = (diet.fat_g / 70.0).min(2.0) as Fraction;
        cal_factor * fat_factor * diet.ros_multiplier
    }

    /// Влияние на протеостаз
    pub fn proteo_impact(&self, diet: &DietConfig) -> Fraction {
        // Больше белка → больше агрегация (если аутофагия не справляется)
        let protein_factor = (diet.protein_g / 80.0).min(2.0) as Fraction;
        protein_factor * diet.proteo_multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mediterranean_diet_lower_ros() {
        let med = DietConfig::mediterranean();
        let western = DietConfig::default();
        let dig_med = DigestionResult::simulate(&med);
        let dig_west = DigestionResult::simulate(&western);
        assert!(dig_med.ros_impact(&med) < dig_west.ros_impact(&western));
    }

    #[test]
    fn cr_diet_lowest_ros() {
        let cr = DietConfig::caloric_restriction();
        let dig = DigestionResult::simulate(&cr);
        assert!(dig.ros_impact(&cr) < 0.6);
    }

    #[test]
    fn high_fat_highest_ros() {
        let hf = DietConfig::high_fat();
        let dig = DigestionResult::simulate(&hf);
        assert!(dig.ros_impact(&hf) > 1.0);
    }

    #[test]
    fn infogest_source_is_valid() {
        let cite = INFOGEST_SOURCE.cite();
        assert!(cite.contains("Brodkorb"));
        assert!(cite.contains("INFOGEST"));
    }
}
