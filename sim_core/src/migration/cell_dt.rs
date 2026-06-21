/// Миграция из cell_dt_core — компоненты клеточного деления
///
/// Оригинал: MCAOA/CDATA/crates/cell_dt_core/src/components/
/// Мигрировано: 2026-06-21
///
/// Компоненты:
/// - inflammaging: воспалительное старение
/// - asymmetric_division: асимметричное деление
/// - mitochondrial: митохондриальная дисфункция
/// - tissue_specific: ткане-специфичные модули
/// - aging_engine: движок старения

use crate::Fraction;

/// Воспалительное старение (inflammaging)
pub mod inflammaging {
    use super::Fraction;

    /// Модель inflammaging: SASP → хроническое воспаление → ускорение старения
    pub fn sasp_contribution(age: Fraction, senescent_fraction: Fraction) -> Fraction {
        // SASP растёт с долей сенесцентных клеток и возрастом
        (senescent_fraction * (1.0 + 0.1 * age)).min(1.0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sasp_increases_with_age() {
            let young = sasp_contribution(20.0, 0.05);
            let old = sasp_contribution(80.0, 0.15);
            assert!(old > young);
        }
    }
}

/// Асимметричное деление центриоли
pub mod asymmetric_division {
    use super::Fraction;

    /// Вероятность асимметричного наследования
    /// PMID 17255513 (Yamashita 2007)
    pub fn asymmetry_probability(age: Fraction, baseline: Fraction) -> Fraction {
        // С возрастом асимметрия снижается
        (baseline * (1.0 - 0.3 * age / 120.0)).max(0.5)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn asymmetry_declines_with_age() {
            let young = asymmetry_probability(20.0, 0.65);
            let old = asymmetry_probability(80.0, 0.65);
            assert!(old < young);
        }

        #[test]
        fn asymmetry_never_below_0_5() {
            let very_old = asymmetry_probability(200.0, 0.65);
            assert!(very_old >= 0.5);
        }
    }
}

/// Митохондриальная дисфункция
pub mod mitochondrial {
    use super::Fraction;

    /// ROS production как функция возраста и митохондриальной массы
    pub fn ros_level(age: Fraction, mito_mass: Fraction) -> Fraction {
        (0.1 + 0.005 * age) * (2.0 - mito_mass).max(0.5)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn ros_increases_with_age() {
            assert!(ros_level(80.0, 1.0) > ros_level(20.0, 1.0));
        }
    }
}

/// Ткане-специфичные модули
pub mod tissue_specific {
    use super::Fraction;

    /// Ткане-специфичный вес счётчика
    pub fn tissue_weight(
        tissue_renewal_days: Fraction,
        counter_type: &str,
    ) -> Fraction {
        match counter_type {
            "telomere" if tissue_renewal_days < 30.0 => 0.5,
            "telomere" => 0.1,
            "mitochondrial" if tissue_renewal_days > 365.0 => 0.55,
            "mitochondrial" => 0.15,
            "epigenetic" if tissue_renewal_days > 90.0 => 0.3,
            "epigenetic" => 0.15,
            "proteostatic" if tissue_renewal_days < 10.0 => 0.4,
            "proteostatic" => 0.15,
            _ => 0.2,
        }
    }
}

/// Движок старения
pub mod aging_engine {
    use crate::Fraction;

    /// Гомпертцовская кривая смертности: h(t) = h₀·exp(γ·t)
    pub fn gompertz(h0: Fraction, gamma: Fraction, t: Fraction) -> Fraction {
        h0 * (gamma * t).exp()
    }

    /// Ожидаемая продолжительность жизни из Гомпертца
    pub fn life_expectancy(h0: Fraction, gamma: Fraction) -> Fraction {
        // Приближение: E[T] ≈ (1/γ)·(ln(γ/h0) − 0.577)
        let euler = 0.5772156649;
        (1.0 / gamma) * ((gamma / h0).ln() - euler)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn gompertz_increases_exponentially() {
            let h20 = gompertz(0.0001, 0.08, 20.0);
            let h80 = gompertz(0.0001, 0.08, 80.0);
            assert!(h80 > h20 * 10.0); // экспоненциальный рост
        }
    }
}
