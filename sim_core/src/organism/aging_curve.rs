/// Кривая старения: L_tissue(t), FI(t), T_death
/// Гомпертцовская модель: h(t) = h₀ · exp(γ · t)
/// В v1.0: вычисляется в organism/mod.rs (step)
pub fn gompertz_hazard(h0: f64, gamma: f64, t: f64) -> f64 {
    h0 * (gamma * t).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gompertz_increases_with_age() {
        let h_young = gompertz_hazard(0.0001, 0.08, 20.0);
        let h_old = gompertz_hazard(0.0001, 0.08, 80.0);
        assert!(h_old > h_young);
    }
}
