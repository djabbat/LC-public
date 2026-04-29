//! F4: C(τ) matches `C₀ exp(−β·I·τ)` to 1e-9.
//! THEORY.md §7 row F4.

use approx::assert_abs_diff_eq;
use ze_simulator::CorrelationDecay;

#[test]
fn f4a_closed_form_exact() {
    let c0 = 0.7;
    let beta = 1.3;
    let i = 0.4;
    let cd = CorrelationDecay::new(c0, beta, i).unwrap();
    for &tau in &[0.0, 0.05, 0.1, 0.5, 1.0, 1.5] {
        let actual = cd.at(tau).unwrap();
        let expected = c0 * (-beta * i * tau).exp();
        assert_abs_diff_eq!(actual, expected, epsilon = 1e-12);
    }
}

#[test]
fn f4b_lgi_consistency() {
    let cd = CorrelationDecay::new(1.0, 1.0, 0.3).unwrap();
    let tau = 0.6;
    let c1 = cd.at(tau).unwrap();
    let c2 = cd.at(2.0 * tau).unwrap();
    let k = cd.lgi_k(tau).unwrap();
    assert_abs_diff_eq!(k, 2.0 * c1 - c2, epsilon = 1e-12);
}

#[test]
fn f4c_decreasing_in_tau() {
    let cd = CorrelationDecay::new(1.0, 1.0, 0.3).unwrap();
    let mut last = f64::INFINITY;
    for &tau in &[0.0, 0.1, 0.5, 1.0, 1.5] {
        let c = cd.at(tau).unwrap();
        assert!(c <= last + 1e-15, "non-monotone: c({}) = {} > {}", tau, c, last);
        last = c;
    }
}
