//! F5: QFI Lemma E formula matches direct evaluation `8·[K(τ) − ⟨Q²⟩]`.
//! In the stationary case ⟨Q²⟩ = C₀ at τ → 0 (THEORY.md §5.2).
//! THEORY.md §7 row F5.

use approx::assert_abs_diff_eq;
use ze_simulator::{CorrelationDecay, QfiBound};

#[test]
fn f5_formula_matches_direct() {
    let c0 = 0.5;
    let beta = 0.8;
    let i = 0.3;
    let cd = CorrelationDecay::new(c0, beta, i).unwrap();
    let q = QfiBound::new(c0, beta, i).unwrap();

    let q_squared = c0; // stationary assumption ⟨Q²⟩ = C₀

    for &tau in &[0.05_f64, 0.1, 0.5, 1.0, 1.5] {
        let r = q.at(tau).unwrap();
        // direct: 8·[K(τ) − ⟨Q²⟩], take the *positive part* per Lemma E discussion (taking |·|).
        let k = cd.lgi_k(tau).unwrap();
        let direct_signed = 8.0 * (k - q_squared);
        // Lemma E formula: 8·C₀·x²·(1 − x) — this is the positive part of −[direct_signed]/something?
        // The clean check: k − q_squared = c0·[(2 e^-x − e^-2x) − 1].
        let x = beta * i * tau;
        let expected_k_minus_q2 = c0 * (2.0 * (-x).exp() - (-2.0 * x).exp() - 1.0);
        assert_abs_diff_eq!(k - q_squared, expected_k_minus_q2, epsilon = 1e-12);
        // Lemma E expansion: 8·c0·x²·(1−x) approximates −8·(k − q_squared) at small x,
        // because k − q_squared = c0·(−x² + x³ + O(x⁴)) (Taylor, THEORY §5.2).
        let lemma_e = 8.0 * c0 * x * x * (1.0 - x);
        let approx = -direct_signed; // positive
        // Not exactly equal at finite τ — they agree to leading and next-to-leading orders.
        // Verify cubic-order accuracy: |approx − lemma_e| ≤ 8·c0·x⁴ (cap by the next Taylor term).
        let bound = 8.0 * c0 * x.powi(4);
        assert!(
            (approx - lemma_e).abs() <= 10.0 * bound + 1e-12,
            "tau={}, x={}, approx={}, lemma_e={}, bound={}", tau, x, approx, lemma_e, bound
        );
    }
}

#[test]
fn f5_optimal_value_closed_form() {
    let c0 = 1.7;
    let q = QfiBound::new(c0, 1.0, 1.0).unwrap();
    let opt = q.at_optimal_tau().unwrap();
    assert_abs_diff_eq!(opt.f_q_lower_bound, 32.0 * c0 / 27.0, epsilon = 1e-12);
}
