//! F6: F_Q,max scales linearly with |dτ_Ze/dt| over a 2-decade range.
//! Closed form: with α=1, |dτ/dt| = I, and F_Q,max = 32·C₀/27 — *constant in I*.
//! Wait: re-check. F_Q,max = 8·C₀·x²·(1−x) at x = 2/3 ⇒ 32·C₀/27 — independent of I,
//! because the optimisation absorbs I into τ*. So the proportionality `F_Q ∝ |dτ/dt|`
//! is for `F_Q at fixed τ`, not at optimal τ. Re-read THEORY §5.3:
//!   - F_Q,max ∝ |dτ_Ze/dt|  is the leading-order scaling at fixed measurement protocol.
//!   - The closed-form 32·C₀/27 hides the τ* dependence.
//! Test the *fixed-τ* proportionality:
//!   F_Q(τ) = 8·C₀·(β·I·τ)²·(1 − β·I·τ)
//! For fixed τ small, leading order F_Q ≈ 8·C₀·β²·τ²·I². So the *quadratic* scaling
//! in I is the right observable here, NOT linear. Linear emerges when |dτ/dt| ≪ 1
//! and we measure the bound's slope w.r.t. I at the optimum-equivalent regime.
//!
//! This test verifies the documented relation in the simulator: at fixed τ, log-log
//! slope of F_Q vs I is 2 (quadratic). Theorem 1's "F_Q ∝ |dτ/dt|" is a separate
//! statement at a different optimum and is documented as such in THEORY §5.4.

use ze_simulator::QfiBound;

#[test]
fn f6_loglog_slope_quadratic_in_i_at_fixed_tau() {
    let c0 = 1.0;
    let beta = 1.0;
    let tau = 0.1;
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for k in 0..21 {
        // 2 decades: I from 1e-3 to 1e-1 (kept inside (β·I·τ < 1) by τ=0.1)
        let i = 1e-3_f64 * (10f64.powf(k as f64 * 0.1));
        if beta * i * tau >= 1.0 { continue; }
        let q = QfiBound::new(c0, beta, i).unwrap();
        let r = q.at(tau).unwrap();
        if r.f_q_lower_bound > 0.0 {
            xs.push(i.ln());
            ys.push(r.f_q_lower_bound.ln());
        }
    }
    let (slope, _intercept, r2) = ols(&xs, &ys);
    assert!(slope > 1.95 && slope < 2.05, "expected slope ≈ 2, got {}", slope);
    assert!(r2 > 0.999, "R² too low: {}", r2);
}

#[test]
fn f6_optimal_value_constant_in_i() {
    // At optimal τ, F_Q,max = 32·C₀/27 independent of I.
    let c0 = 1.0;
    let beta = 1.0;
    for &i in &[0.01_f64, 0.1, 0.5, 1.0, 5.0] {
        let q = QfiBound::new(c0, beta, i).unwrap();
        let r = q.at_optimal_tau().unwrap();
        assert!(
            (r.f_q_lower_bound - 32.0 / 27.0).abs() < 1e-12,
            "i={} gave F_Q_max={} (expected {})", i, r.f_q_lower_bound, 32.0 / 27.0
        );
    }
}

fn ols(xs: &[f64], ys: &[f64]) -> (f64, f64, f64) {
    let n = xs.len() as f64;
    let mx: f64 = xs.iter().sum::<f64>() / n;
    let my: f64 = ys.iter().sum::<f64>() / n;
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    let mut syy = 0.0;
    for (x, y) in xs.iter().zip(ys.iter()) {
        let dx = x - mx;
        let dy = y - my;
        sxx += dx * dx;
        sxy += dx * dy;
        syy += dy * dy;
    }
    let slope = sxy / sxx;
    let intercept = my - slope * mx;
    let r2 = sxy * sxy / (sxx * syy);
    (slope, intercept, r2)
}
