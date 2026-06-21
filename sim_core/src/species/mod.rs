// Видовая параметризация
//
// Organismal Aging поддерживает любые виды — от одноклеточных до млекопитающих.
// Каждый вид — набор параметров, переопределяющих базовую модель.

use crate::Fraction;
use crate::tissue::TissueConfig;

/// Конфигурация вида
#[derive(Debug, Clone)]
pub struct SpeciesConfig {
    pub name: &'static str,
    pub max_lifespan_years: f64,
    pub has_centrioles: bool,
    pub n_ref: f64, // референсное число делений (Хейфлик)
    pub tissue_configs: Vec<TissueConfig>,
    pub ros_baseline: Fraction,
    pub division_rate_baseline: Fraction, // средняя частота делений (1/год)
}

/// Предустановленные виды

pub fn human() -> SpeciesConfig {
    SpeciesConfig {
        name: "Homo sapiens",
        max_lifespan_years: 120.0,
        has_centrioles: true,
        n_ref: 50.0,
        tissue_configs: crate::tissue::human_tissue_configs(),
        ros_baseline: 0.1,
        division_rate_baseline: 1.0,
    }
}

pub fn mouse() -> SpeciesConfig {
    SpeciesConfig {
        name: "Mus musculus",
        max_lifespan_years: 3.0,
        has_centrioles: true,
        n_ref: 20.0, // мышиный Хейфлик ~20
        tissue_configs: vec![
            // Упрощённая модель мыши — те же 8 тканей, но ускоренные
            TissueConfig {
                name: "Эпидермис", renewal_period_days: 14.0,
                counter_weights: [0.50, 0.15, 0.15, 0.10],
                critical_burden: 0.60,
                position: [0.5, 0.5, 0.98],
                vascular_density: 0.3,
                innervation_density: 0.7,
            },
            TissueConfig {
                name: "Кишечный эпителий", renewal_period_days: 3.0,
                counter_weights: [0.20, 0.15, 0.15, 0.40],
                critical_burden: 0.60,
                position: [0.5, 0.4, 0.5],
                vascular_density: 0.8,
                innervation_density: 0.6,
            },
            TissueConfig {
                name: "Гепатоциты", renewal_period_days: 200.0,
                counter_weights: [0.10, 0.25, 0.15, 0.35],
                critical_burden: 0.60,
                position: [0.7, 0.6, 0.5],
                vascular_density: 0.9,
                innervation_density: 0.3,
            },
            TissueConfig {
                name: "Нейроны", renewal_period_days: f64::INFINITY,
                counter_weights: [0.00, 0.55, 0.20, 0.20],
                critical_burden: 0.55,
                position: [0.5, 0.8, 0.5],
                vascular_density: 0.7,
                innervation_density: 1.0,
            },
            TissueConfig {
                name: "HSC", renewal_period_days: 60.0,
                counter_weights: [0.20, 0.15, 0.30, 0.15],
                critical_burden: 0.60,
                position: [0.5, 0.3, 0.5],
                vascular_density: 1.0,
                innervation_density: 0.2,
            },
            TissueConfig {
                name: "Кардиомиоциты", renewal_period_days: 36500.0,
                counter_weights: [0.00, 0.55, 0.20, 0.20],
                critical_burden: 0.50,
                position: [0.6, 0.55, 0.5],
                vascular_density: 0.9,
                innervation_density: 0.8,
            },
            TissueConfig {
                name: "Эндотелий", renewal_period_days: 730.0,
                counter_weights: [0.20, 0.20, 0.20, 0.20],
                critical_burden: 0.60,
                position: [0.5, 0.5, 0.5],
                vascular_density: 1.0,
                innervation_density: 0.5,
            },
            TissueConfig {
                name: "Кость", renewal_period_days: 1825.0,
                counter_weights: [0.15, 0.15, 0.15, 0.15],
                critical_burden: 0.60,
                position: [0.5, 0.1, 0.5],
                vascular_density: 0.4,
                innervation_density: 0.3,
            },
        ],
        ros_baseline: 0.15, // у мышей выше метаболизм → больше АФК
        division_rate_baseline: 5.0, // быстрее делятся
    }
}

pub fn celegans() -> SpeciesConfig {
    SpeciesConfig {
        name: "Caenorhabditis elegans",
        max_lifespan_years: 21.0 / 365.25, // ~3 недели
        has_centrioles: true, // в делящихся клетках
        n_ref: 20.0,
        tissue_configs: vec![
            // C. elegans — 3 ткани
            TissueConfig {
                name: "Кишечник", renewal_period_days: 1.0,
                counter_weights: [0.30, 0.20, 0.20, 0.30],
                critical_burden: 0.60,
                position: [0.5, 0.5, 0.5],
                vascular_density: 0.0, // нет сосудистой системы
                innervation_density: 0.5,
            },
            TissueConfig {
                name: "Нейроны", renewal_period_days: f64::INFINITY,
                counter_weights: [0.00, 0.60, 0.20, 0.20],
                critical_burden: 0.55,
                position: [0.5, 0.7, 0.5],
                vascular_density: 0.0,
                innervation_density: 1.0,
            },
            TissueConfig {
                name: "Гонады", renewal_period_days: 2.0,
                counter_weights: [0.30, 0.20, 0.30, 0.20],
                critical_burden: 0.60,
                position: [0.5, 0.3, 0.5],
                vascular_density: 0.0,
                innervation_density: 0.3,
            },
        ],
        ros_baseline: 0.1,
        division_rate_baseline: 20.0,
    }
}

/// Одноклеточный организм (E. coli)
/// Нет центриолей → Уровень #1 отключён
/// Нет тканей → одна «клетка»
pub fn unicellular(name: &'static str, lifespan_hours: f64, has_centrioles: bool) -> SpeciesConfig {
    SpeciesConfig {
        name,
        max_lifespan_years: lifespan_hours / (24.0 * 365.25),
        has_centrioles,
        n_ref: if has_centrioles { 30.0 } else { 0.0 },
        tissue_configs: vec![
            TissueConfig {
                name: "Клетка",
                renewal_period_days: lifespan_hours / 24.0,
                counter_weights: [0.25, 0.25, 0.25, 0.25],
                critical_burden: 0.60,
                position: [0.5, 0.5, 0.5],
                vascular_density: 0.0,
                innervation_density: 0.0,
            },
        ],
        ros_baseline: 0.2,
        division_rate_baseline: 365.25 * 24.0 / lifespan_hours, // делений в год
    }
}

// Реэкспорт для удобства
pub use human as Human;
pub use mouse as Mouse;
pub use celegans as Celegans;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_has_centrioles() {
        assert!(human().has_centrioles);
    }

    #[test]
    fn human_has_8_tissues() {
        assert_eq!(human().tissue_configs.len(), 8);
    }

    #[test]
    fn mouse_lifespan_is_3_years() {
        assert!((mouse().max_lifespan_years - 3.0).abs() < 0.1);
    }

    #[test]
    fn celegans_lifespan_is_3_weeks() {
        let w = celegans();
        assert!(w.max_lifespan_years < 0.1); // ~0.057 года
    }

    #[test]
    fn unicellular_no_centrioles() {
        let e_coli = unicellular("E. coli", 0.5, false);
        assert!(!e_coli.has_centrioles);
        assert_eq!(e_coli.tissue_configs.len(), 1);
    }
}
