//! B6: DP noise statistics + naive composition + k-anonymity.
//! THEORY §7 row B6 + §6.

use biosense_simulator::{DpBudget, KAnonymity, SecAgg};
use rand::SeedableRng;
use rand::rngs::StdRng;

#[test]
fn b6_dp_zero_mean_in_expectation() {
    let dp = DpBudget::new(2.0, 1e-5, 0.3).unwrap();
    let mut rng = StdRng::seed_from_u64(20_260_428);
    let n = 200_000;
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    for _ in 0..n {
        let v = dp.laplace_noise(0.0, &mut rng);
        sum += v;
        sum_sq += v * v;
    }
    let mean = sum / n as f64;
    let var = sum_sq / n as f64 - mean * mean;
    // Laplace(0, b) has mean 0, variance 2 b^2; here b = 0.3/2.0 = 0.15
    let expected_var = 2.0 * 0.15_f64.powi(2);
    assert!(mean.abs() < 0.02, "mean = {}", mean);
    assert!((var - expected_var).abs() / expected_var < 0.05,
            "var = {}, expected {}", var, expected_var);
}

#[test]
fn b6_naive_composition_linear() {
    let dp = DpBudget::new(2.0, 1e-5, 0.3).unwrap();
    assert_eq!(dp.naive_cumulative_eps(1), 2.0);
    assert_eq!(dp.naive_cumulative_eps(10), 20.0);
    assert_eq!(dp.naive_cumulative_eps(100), 200.0);
}

#[test]
fn b6_k_anonymity_threshold() {
    let ka = KAnonymity::new(7).unwrap();
    assert!(!ka.safe_to_release(0));
    assert!(!ka.safe_to_release(6));
    assert!(ka.safe_to_release(7));
    assert!(ka.safe_to_release(100));
    let small: Vec<u32> = (0..5).collect();
    assert!(ka.enforce(&small).is_err());
    let big: Vec<u32> = (0..10).collect();
    assert!(ka.enforce(&big).is_ok());
}

#[test]
fn b6_secagg_threshold() {
    let s = SecAgg::new(3).unwrap();
    assert!(!s.ready(2));
    assert!(s.ready(3));
}

#[test]
fn b6_dp_rejects_invalid_params() {
    assert!(DpBudget::new(0.0, 1e-5, 0.3).is_err());
    assert!(DpBudget::new(-1.0, 1e-5, 0.3).is_err());
    assert!(DpBudget::new(2.0, 1.0, 0.3).is_err());
    assert!(DpBudget::new(2.0, 1e-5, 0.0).is_err());
}
