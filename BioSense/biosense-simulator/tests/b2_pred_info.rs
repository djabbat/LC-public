//! B2: `I_pred` numerical estimator vs closed form.
//! THEORY §7 row B2.

use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::Rng;
use biosense_simulator::{PredictiveInfo, consts::RNG_SEED};

fn synthesise_markov(n: usize, p: f64, seed: u64) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut out = Vec::with_capacity(n);
    let mut prev = 0_u8;
    out.push(prev);
    for _ in 1..n {
        if rng.gen::<f64>() < p {
            prev = 1 - prev;
        }
        out.push(prev);
    }
    out
}

#[test]
fn b2_estimator_matches_closed_form() {
    let n = 1_000_000;
    for &p in &[0.1_f64, 0.3, 0.5] {
        let closed = PredictiveInfo::closed_form(p).unwrap();
        let s = synthesise_markov(n, p, RNG_SEED + (p * 1000.0) as u64);
        let est = PredictiveInfo::estimate(&s, 64).unwrap();
        // Plug-in estimator on symmetric Markov: I(prev; next) = log 2 - H[p]
        // (same as closed form).
        let err = (est - closed).abs();
        assert!(err < 1e-2, "p={}, closed={}, est={}, err={}", p, closed, est, err);
    }
}

#[test]
fn b2_at_p_half_is_zero() {
    let n = 200_000;
    let s = synthesise_markov(n, 0.5, RNG_SEED);
    let est = PredictiveInfo::estimate(&s, 64).unwrap();
    assert!(est.abs() < 5e-3, "expected ≈0, got {}", est);
}
