/// Травмы как внешние возмущения модели
/// D_i(t + t_trauma) = D_i(t) + ΔD_trauma
/// L_tissue пересчитывается с новыми D_i
/// В v1.0: заглушка. Полная реализация — Phase 3.
pub struct Trauma {
    pub name: &'static str,
    pub affected_tissues: Vec<usize>,
    pub damage_delta: Vec<f64>,
}
