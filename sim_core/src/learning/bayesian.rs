/// Байесовский контур самообучения — ядро Organismal Aging
///
/// P(θ | data) ∝ P(data | θ) · P(θ)
///
/// Метод: MCMC (No-U-Turn Sampler) через Python (PyMC) или Rust (linfa)
/// В v1.0: простая байесовская линейная регрессия для обновления параметров
///
/// Цикл: гипотеза → ARGUS-LP → результат → обновление → новая гипотеза

use crate::{Fraction, Time};
use crate::provenance::Source;

/// Параметр модели с априорным распределением
#[derive(Debug, Clone)]
pub struct ModelParameter {
    pub name: String,
    pub value: Fraction,
    pub prior_mean: Fraction,
    pub prior_std: Fraction,   // неопределённость априорного
    pub posterior_mean: Fraction,
    pub posterior_std: Fraction,
    pub source: Source,
    pub last_updated: Time,
}

/// Результат эксперимента для обновления
#[derive(Debug, Clone)]
pub struct ExperimentResult {
    pub hypothesis: String,
    pub predicted_value: Fraction,
    pub observed_value: Fraction,
    pub std_error: Fraction,
    pub source: String,         // "ARGUS-LP", "INFOGEST", "literature"
    pub timestamp: Time,
}

/// Байесовский контур
#[derive(Debug, Clone)]
pub struct BayesianLoop {
    pub parameters: Vec<ModelParameter>,
    pub history: Vec<ExperimentResult>,
    pub total_updates: u64,
    pub convergence_threshold: Fraction,
}

impl BayesianLoop {
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
            history: Vec::new(),
            total_updates: 0,
            convergence_threshold: 0.01,
        }
    }

    /// Зарегистрировать параметр модели
    pub fn register_parameter(
        &mut self,
        name: &str,
        initial_value: Fraction,
        prior_std: Fraction,
        source: Source,
    ) {
        self.parameters.push(ModelParameter {
            name: name.into(),
            value: initial_value,
            prior_mean: initial_value,
            prior_std,
            posterior_mean: initial_value,
            posterior_std: prior_std,
            source,
            last_updated: 0.0,
        });
    }

    /// Байесовское обновление одного параметра
    /// Использует conjugate prior: Normal-Normal model
    /// μ_post = (μ_prior/σ²_prior + x/σ²_obs) / (1/σ²_prior + 1/σ²_obs)
    /// σ²_post = 1 / (1/σ²_prior + 1/σ²_obs)
    pub fn update_parameter(
        &mut self,
        param_index: usize,
        observed_value: Fraction,
        observation_std: Fraction,
        timestamp: Time,
    ) {
        if param_index >= self.parameters.len() {
            return;
        }

        let p = &mut self.parameters[param_index];

        let prior_precision = 1.0 / (p.prior_std * p.prior_std).max(1e-10);
        let obs_precision = 1.0 / (observation_std * observation_std).max(1e-10);

        let post_precision = prior_precision + obs_precision;
        p.posterior_std = (1.0 / post_precision.max(1e-10)).sqrt();
        p.posterior_mean = (prior_precision * p.prior_mean + obs_precision * observed_value)
            / post_precision.max(1e-10);

        // Обновить текущее значение
        p.value = p.posterior_mean;
        p.last_updated = timestamp;
    }

    /// Запустить один цикл обучения
    pub fn run_cycle(
        &mut self,
        result: ExperimentResult,
        param_indices: &[usize],
        timestamp: Time,
    ) {
        self.history.push(result.clone());
        self.total_updates += 1;

        for &i in param_indices {
            self.update_parameter(i, result.observed_value, result.std_error, timestamp);
        }
    }

    /// Проверить сходимость: все параметры имеют posterior_std < порог
    pub fn has_converged(&self) -> bool {
        if self.parameters.is_empty() {
            return false;
        }
        self.parameters.iter().all(|p| p.posterior_std < self.convergence_threshold)
    }

    /// Сгенерировать новую гипотезу: какой параметр больше всего нуждается в уточнении
    pub fn suggest_next_experiment(&self) -> Option<(usize, String)> {
        self.parameters.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.posterior_std.partial_cmp(&b.posterior_std).unwrap())
            .map(|(i, p)| (i, format!("Измерить {} (σ={:.4})", p.name, p.posterior_std)))
    }

    /// Аудит: вывести все параметры с их неопределённостями
    pub fn audit(&self) -> Vec<String> {
        let mut lines = vec!["=== БАЙЕСОВСКИЙ АУДИТ ===".into()];
        lines.push(format!("Параметров: {}, обновлений: {}", self.parameters.len(), self.total_updates));
        for p in &self.parameters {
            lines.push(format!(
                "  {}: {:.4} ± {:.4} (prior σ={:.4}, source={})",
                p.name, p.posterior_mean, p.posterior_std, p.prior_std,
                p.source.cite()
            ));
        }
        if self.has_converged() {
            lines.push("✅ Модель сошлась".into());
        } else {
            lines.push("⏳ Модель не сошлась — требуется больше данных".into());
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provenance::sources;

    #[test]
    fn bayesian_update_reduces_uncertainty() {
        let mut bl = BayesianLoop::new();
        bl.register_parameter("eta_time", 0.010, 0.005, sources::CENTRIOLE_ENTROPY_POSTULATE);

        let before_std = bl.parameters[0].posterior_std;

        let result = ExperimentResult {
            hypothesis: "Test eta_time".into(),
            predicted_value: 0.010,
            observed_value: 0.008,
            std_error: 0.002,
            source: "test".into(),
            timestamp: 1.0,
        };
        bl.run_cycle(result, &[0], 1.0);

        let after_std = bl.parameters[0].posterior_std;
        assert!(after_std < before_std, "Bayesian update must reduce uncertainty");
    }

    #[test]
    fn convergence_is_detected() {
        let mut bl = BayesianLoop::new();
        bl.register_parameter("test", 0.5, 0.1, sources::COUNTER_PARAMS_ESTIMATE);

        // Много обновлений с маленькой ошибкой → сходимость
        for i in 0..50 {
            let result = ExperimentResult {
                hypothesis: format!("update {}", i),
                predicted_value: 0.5,
                observed_value: 0.5,
                std_error: 0.001,
                source: "test".into(),
                timestamp: i as f64,
            };
            bl.run_cycle(result, &[0], i as f64);
        }

        assert!(bl.has_converged(), "After 50 precise updates, model should converge");
    }

    #[test]
    fn suggests_most_uncertain_parameter() {
        let mut bl = BayesianLoop::new();
        bl.register_parameter("certain", 0.5, 0.01, sources::COUNTER_PARAMS_ESTIMATE);
        bl.register_parameter("uncertain", 0.5, 0.5, sources::COUNTER_PARAMS_ESTIMATE);

        let suggestion = bl.suggest_next_experiment();
        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().1.contains("uncertain"));
    }

    #[test]
    fn empty_loop_does_not_converge() {
        let bl = BayesianLoop::new();
        assert!(!bl.has_converged());
    }
}
