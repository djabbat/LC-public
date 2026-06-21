// Самообучение — Байесовское обновление параметров
// Цикл: гипотеза → ARGUS-LP/INFOGEST → результат → обновление модели.
// P(θ | data) ∝ P(data | θ) · P(θ)
// Метод: Normal-Normal conjugate (аналитическое решение)

pub mod bayesian;
pub mod hypothesis;
pub mod experiment;
pub mod feedback;

pub use bayesian::BayesianLoop;
pub use bayesian::ExperimentResult;
pub use bayesian::ModelParameter;
pub use hypothesis::HypothesisGenerator;
pub use experiment::ExperimentPlan;
pub use feedback::FeedbackLoop;
