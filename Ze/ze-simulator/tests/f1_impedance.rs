//! F1: Impedance is a genuine KL divergence.
//! THEORY.md §7 row F1.

use approx::assert_abs_diff_eq;
use ze_simulator::Distribution;

#[test]
fn f1a_self_divergence_zero() {
    for probs in [
        vec![1.0],
        vec![0.5, 0.5],
        vec![0.25, 0.25, 0.25, 0.25],
        vec![0.1, 0.2, 0.3, 0.4],
        vec![0.001, 0.999],
    ] {
        let p = Distribution::new(probs).unwrap();
        let kl = p.kl_to(&p).unwrap();
        assert_abs_diff_eq!(kl, 0.0, epsilon = 1e-12);
    }
}

#[test]
fn f1b_distinct_distributions_positive() {
    let p = Distribution::new(vec![0.1, 0.4, 0.5]).unwrap();
    let q = Distribution::new(vec![0.4, 0.4, 0.2]).unwrap();
    let kl = p.kl_to(&q).unwrap();
    assert!(kl > 1e-6, "expected KL > 0, got {}", kl);
}

#[test]
fn f1c_asymmetry() {
    let p = Distribution::new(vec![0.1, 0.4, 0.5]).unwrap();
    let q = Distribution::new(vec![0.4, 0.4, 0.2]).unwrap();
    let pq = p.kl_to(&q).unwrap();
    let qp = q.kl_to(&p).unwrap();
    assert!((pq - qp).abs() > 1e-3, "expected asymmetry, got pq={} qp={}", pq, qp);
}

#[test]
fn f1d_nonnegativity_under_perturbation() {
    let p = Distribution::new(vec![0.5, 0.5]).unwrap();
    for &eps in &[0.0, 0.01, 0.05, 0.1, 0.2, 0.4] {
        let q = Distribution::new(vec![0.5 + eps, 0.5 - eps]).unwrap();
        let kl = p.kl_to(&q).unwrap();
        assert!(kl >= -1e-15, "kl < 0: {} at eps={}", kl, eps);
    }
}
