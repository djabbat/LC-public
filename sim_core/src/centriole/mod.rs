// Уровень #1: Центриоль — накопитель энтропии
//
// Центриоль — структура с ограниченным самообновлением.
// Рабочая гипотеза: потенциальный накопитель энтропии.

pub mod entropy;
pub mod replication;
pub mod division;
pub mod polyglutamylation;

use crate::{Fraction, Time, Divisions};

/// Состояние центриоли
#[derive(Debug, Clone)]
pub struct CentrioleState {
    /// Энтропия центриоли [0, 1]
    pub entropy: Fraction,
    /// Накопленное число делений
    pub divisions: Divisions,
    /// Уровень полиглутамилирования [0, 1]
    pub polyglu: Fraction,
}

/// Скорости накопления энтропии
#[derive(Debug, Clone)]
pub struct EntropyRates {
    /// Интенсивность от делений (на одно референсное деление)
    pub eta_div: Fraction,
    /// Интенсивность от времени (на год)
    pub eta_time: Fraction,
    /// Интенсивность от АФК
    pub eta_ros: Fraction,
    /// Интенсивность ошибок репликации
    pub eta_rep: Fraction,
    /// Источник параметров
    pub source: crate::provenance::Source,
}

impl Default for EntropyRates {
    fn default() -> Self {
        Self {
            eta_div: 0.02 / 50.0,  // 0.0004 на референсное деление
            eta_time: 0.010,        // 0.010 в год → S(85)≈0.86
            eta_ros: 0.01,
            eta_rep: 0.001,
            source: crate::provenance::sources::CENTRIOLE_ENTROPY_POSTULATE,
        }
    }
}

impl CentrioleState {
    /// Создать новое состояние центриоли (зигота)
    pub fn new() -> Self {
        Self {
            entropy: 0.01,     // минимальная энтропия при рождении
            divisions: 0.0,
            polyglu: 0.0,
        }
    }

    /// Обновить состояние центриоли за шаг dt (в годах)
    /// n_ref — референсное число делений (Хейфлик)
    /// ros — текущий уровень АФК [0, 1]
    /// division_rate — частота делений (делений/год)
    pub fn update(
        &mut self,
        dt: Time,
        n_ref: Divisions,
        ros: Fraction,
        division_rate: Fraction,
        rates: &EntropyRates,
    ) {
        // Число делений за шаг
        let dn = division_rate * dt;
        self.divisions += dn;

        // Прирост энтропии
        let d_entropy = rates.eta_div * (dn / n_ref)
            + rates.eta_time * dt
            + rates.eta_ros * ros * dt
            + rates.eta_rep * dn;

        self.entropy = (self.entropy + d_entropy).min(1.0);

        // polyGlu пропорционально энтропии
        self.polyglu = self.entropy;
    }

    /// Центриоль достигла критического состояния?
    pub fn is_critical(&self, threshold: Fraction) -> bool {
        self.entropy > threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_centriole_has_minimal_entropy() {
        let c = CentrioleState::new();
        assert!((c.entropy - 0.01).abs() < 1e-10);
        assert_eq!(c.divisions, 0.0);
    }

    #[test]
    fn entropy_grows_monotonically() {
        let mut c = CentrioleState::new();
        let rates = EntropyRates::default();
        let before = c.entropy;
        c.update(1.0, 50.0, 0.1, 1.0, &rates);
        assert!(c.entropy > before);
    }

    #[test]
    fn entropy_never_exceeds_one() {
        let mut c = CentrioleState::new();
        let rates = EntropyRates::default();
        for _ in 0..10000 {
            c.update(1.0, 50.0, 0.5, 10.0, &rates);
        }
        assert!(c.entropy <= 1.0);
    }

    #[test]
    fn divisions_accumulate() {
        let mut c = CentrioleState::new();
        let rates = EntropyRates::default();
        c.update(1.0, 50.0, 0.0, 5.0, &rates);
        assert!((c.divisions - 5.0).abs() < 1e-10);
    }
}
