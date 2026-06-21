/// Прогноз смертности через max(L_tissue)
/// Гомпертц: h(t) = h₀ · exp(γ · L_max(t) / L_crit)
/// В v1.0: смерть при L_max ≥ 0.99 или age > max_lifespan
pub use super::aging_curve::gompertz_hazard;
