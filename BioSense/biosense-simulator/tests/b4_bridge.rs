//! B4: CDATA bridge — A monotone in D, χ_Ze monotone (decreasing) in A; limits.
//! THEORY §7 row B4 + §4.

use biosense_simulator::{BridgeParams, CdataBridge};

#[test]
fn b4_a_monotone_in_d() {
    let b = CdataBridge::new(BridgeParams::default()).unwrap();
    let mut last = f64::NEG_INFINITY;
    for &d in &[0.0_f64, 0.1, 0.3, 0.5, 0.7, 0.9, 1.0] {
        let a = b.activity(d);
        assert!(a >= last - 1e-12, "A non-monotone: {} < {}", a, last);
        last = a;
    }
}

#[test]
fn b4_chi_decreases_as_d_grows() {
    let b = CdataBridge::new(BridgeParams::default()).unwrap();
    let mut last = f64::INFINITY;
    for &d in &[0.0_f64, 0.1, 0.3, 0.5, 0.7, 0.9, 1.0] {
        let chi = b.chi_ze_from_d(d);
        assert!(chi <= last + 1e-12, "χ non-monotone: {} > {}", chi, last);
        last = chi;
    }
}

#[test]
fn b4_limits() {
    let b = CdataBridge::new(BridgeParams::default()).unwrap();
    let chi_zero = b.chi_ze_from_d(0.0);
    let chi_one = b.chi_ze_from_d(1.0);
    assert!(chi_zero > chi_one);
    // χ ∈ [0, 1] always
    for &d in &[0.0_f64, 0.5, 1.0] {
        let c = b.chi_ze_from_d(d);
        assert!(c >= 0.0 && c <= 1.0);
    }
}

#[test]
fn b4_chi_zero_near_g0() {
    let p = BridgeParams::default();
    let b = CdataBridge::new(p).unwrap();
    // At D = 0: A = a; χ = g0 - g1 a
    let expected = (p.g0 - p.g1 * p.a).clamp(0.0, 1.0);
    let actual = b.chi_ze_from_d(0.0);
    assert!((actual - expected).abs() < 1e-12);
}
