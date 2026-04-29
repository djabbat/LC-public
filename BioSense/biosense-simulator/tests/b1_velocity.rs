//! B1: velocity for symmetric Markov chain matches `2p − 1`.
//! THEORY §7 row B1.

use approx::assert_abs_diff_eq;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::Rng;
use biosense_simulator::{PredictorKind, ZeVelocity, consts::RNG_SEED};

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
fn b1_v_matches_2p_minus_1() {
    let n = 200_000;
    for &p in &[0.1_f64, 0.3, 0.5, 0.7, 0.9] {
        let s = synthesise_markov(n, p, RNG_SEED + (p * 1000.0) as u64);
        // For an identity predictor on a symmetric Markov with flip prob p:
        //   P(correct) = P(no flip) = 1 - p
        //   v = 1 - 2(1-p) = 2p - 1
        let v = ZeVelocity::from_signal(&s, PredictorKind::Identity).unwrap();
        let expected = 2.0 * p - 1.0;
        assert_abs_diff_eq!(v, expected, epsilon = 0.01);
    }
}
