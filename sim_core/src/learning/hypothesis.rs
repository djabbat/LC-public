/// Генерация гипотез — какие параметры тестировать
/// В v1.0: заглушка. Реализация — Phase 5.
pub struct HypothesisGenerator {
    pub target_counters: Vec<usize>,
    pub intervention_type: String,
    pub expected_effect: f64,
}
