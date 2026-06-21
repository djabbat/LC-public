/// Frailty Index: FI(t) = 0.7 × L_max(t)
/// Калибровка: Rockwood 2005 (PMID 16129869)
/// Протокол: Searle 2008 (PMID 18671847)
/// Связь со смертностью: Mitnitski 2002 (PMID 12456714)
pub fn frailty_index(l_max: f64) -> f64 {
    (0.7 * l_max).min(0.7)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fi_at_birth_is_zero() {
        assert!((frailty_index(0.0) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn fi_at_death_is_max() {
        assert!((frailty_index(1.0) - 0.7).abs() < 1e-10);
    }
}
