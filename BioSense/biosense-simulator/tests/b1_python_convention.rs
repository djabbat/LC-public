//! B1-python: `frequency_switch` reproduces archived Python `ze_velocity` exactly.
//! For symmetric Markov chain with flip prob `p`, expect `v_python = p`.

use approx::assert_abs_diff_eq;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::Rng;
use biosense_simulator::{ZeVelocity, consts::RNG_SEED};

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
fn b1_python_v_matches_p() {
    let n = 200_000;
    for &p in &[0.1_f64, 0.3, 0.5, 0.7, 0.9] {
        let s = synthesise_markov(n, p, RNG_SEED + (p * 1000.0) as u64);
        let v = ZeVelocity::frequency_switch(&s).unwrap();
        assert_abs_diff_eq!(v, p, epsilon = 0.005);
    }
}

#[test]
fn b1_convention_relationship() {
    let n = 100_000;
    let p = 0.6;
    let s = synthesise_markov(n, p, RNG_SEED);
    let v_python = ZeVelocity::frequency_switch(&s).unwrap();
    let v_article = ZeVelocity::from_signal(&s, biosense_simulator::PredictorKind::Identity).unwrap();
    // python = (article + 1) / 2  (within numerical noise).
    let predicted_python = (v_article + 1.0) / 2.0;
    assert_abs_diff_eq!(v_python, predicted_python, epsilon = 1e-10);
}
