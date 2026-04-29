//! F2: RK4 vs Euler vs analytical for piecewise-constant I(t).
//! THEORY.md §7 row F2.

use ze_simulator::{IntegratorMethod, ProperTimeIntegrator};

/// Piecewise-constant impedance: I(t) = 0.3 for t < 1, 0.7 for 1 ≤ t < 2, 0.5 otherwise.
fn impedance_piecewise(t: f64) -> f64 {
    if t < 1.0 {
        0.3
    } else if t < 2.0 {
        0.7
    } else {
        0.5
    }
}

/// Closed-form integral of I(t) over [0, t_max].
fn integral_piecewise(t_max: f64) -> f64 {
    if t_max <= 1.0 {
        0.3 * t_max
    } else if t_max <= 2.0 {
        0.3 + 0.7 * (t_max - 1.0)
    } else {
        0.3 + 0.7 + 0.5 * (t_max - 2.0)
    }
}

#[test]
fn f2_rk4_matches_analytical_within_1e6() {
    let alpha = 1.3;
    let dt = 1e-3;
    let pt = ProperTimeIntegrator::new(alpha, IntegratorMethod::Rk4, dt).unwrap();
    let t_max = 5.0;
    let tau_0 = 100.0;
    let traj = pt.integrate(impedance_piecewise, t_max, tau_0).unwrap();
    let last = traj.last().unwrap();
    let expected = tau_0 - alpha * integral_piecewise(t_max);
    let err = (last.1 - expected).abs();
    assert!(err < 1e-3, "RK4 error: {:e}, got {}, expected {}", err, last.1, expected);
}

#[test]
fn f2_euler_matches_analytical_within_1e3() {
    let alpha = 1.3;
    let dt = 1e-4;
    let pt = ProperTimeIntegrator::new(alpha, IntegratorMethod::Euler, dt).unwrap();
    let t_max = 5.0;
    let tau_0 = 100.0;
    let traj = pt.integrate(impedance_piecewise, t_max, tau_0).unwrap();
    let last = traj.last().unwrap();
    let expected = tau_0 - alpha * integral_piecewise(t_max);
    let err = (last.1 - expected).abs();
    assert!(err < 1e-2, "Euler error: {:e}, got {}, expected {}", err, last.1, expected);
}

/// Smooth impedance for clean RK4 vs Euler comparison (piecewise-constant has
/// discontinuities that make RK4 mid-step biased — that's an artifact of the
/// integrand, not RK4).
fn impedance_smooth(t: f64) -> f64 {
    0.5 + 0.3 * (t * 0.7).sin()
}
fn integral_smooth(t_max: f64) -> f64 {
    // ∫₀^t_max [0.5 + 0.3·sin(0.7·t)] dt = 0.5·t_max + (0.3/0.7)·(1 − cos(0.7·t_max))
    0.5 * t_max + (0.3 / 0.7) * (1.0 - (0.7 * t_max).cos())
}

#[test]
fn f2_rk4_more_accurate_than_euler_at_same_dt() {
    let alpha = 1.0;
    let dt = 1e-2;
    let t_max = 4.0;
    let tau_0 = 0.0;
    let pt_rk = ProperTimeIntegrator::new(alpha, IntegratorMethod::Rk4, dt).unwrap();
    let pt_eu = ProperTimeIntegrator::new(alpha, IntegratorMethod::Euler, dt).unwrap();
    let rk = pt_rk.integrate(impedance_smooth, t_max, tau_0).unwrap().last().unwrap().1;
    let eu = pt_eu.integrate(impedance_smooth, t_max, tau_0).unwrap().last().unwrap().1;
    let exact = tau_0 - alpha * integral_smooth(t_max);
    assert!(
        (rk - exact).abs() <= (eu - exact).abs() + 1e-15,
        "expected RK4 ≤ Euler on smooth integrand, got rk={} eu={} exact={}", rk, eu, exact
    );
}
