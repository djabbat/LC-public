//! B3: theoretical fixed point exposed and used consistently in both conventions.
//! THEORY §7 row B3 + §3.3 + datasets/MIGRATION_NOTES §2.

use approx::assert_abs_diff_eq;
use biosense_simulator::{ChiZeIndex, VelocityConvention,
                         consts::{V_STAR, V_STAR_ARTICLE, V_STAR_PYTHON}};

#[test]
fn b3_fixed_point_is_45631_python() {
    let v_star = ChiZeIndex::fixed_point();
    assert_abs_diff_eq!(v_star, 0.45631, epsilon = 1e-6);
    assert_abs_diff_eq!(v_star, V_STAR, epsilon = 1e-12);
    assert_abs_diff_eq!(v_star, V_STAR_PYTHON, epsilon = 1e-12);
}

#[test]
fn b3_fixed_point_article_is_minus_008738() {
    let v_star_a = ChiZeIndex::fixed_point_for(VelocityConvention::Article);
    let expected = 2.0 * 0.45631 - 1.0;
    assert_abs_diff_eq!(v_star_a, expected, epsilon = 1e-12);
    assert_abs_diff_eq!(v_star_a, V_STAR_ARTICLE, epsilon = 1e-12);
}

#[test]
fn b3_chi_at_fixed_point_is_one_both_conventions() {
    let chi_python = ChiZeIndex::per_modality_with_convention(V_STAR_PYTHON, VelocityConvention::Python);
    let chi_article = ChiZeIndex::per_modality_with_convention(V_STAR_ARTICLE, VelocityConvention::Article);
    assert_abs_diff_eq!(chi_python, 1.0, epsilon = 1e-12);
    assert_abs_diff_eq!(chi_article, 1.0, epsilon = 1e-12);
}

#[test]
fn b3_chi_decreases_as_v_moves_away() {
    // Python convention
    let v_p = V_STAR_PYTHON;
    let chi_at = ChiZeIndex::per_modality_with_convention(v_p, VelocityConvention::Python);
    let chi_off = ChiZeIndex::per_modality_with_convention(v_p - 0.1, VelocityConvention::Python);
    assert!(chi_at > chi_off);

    // Article convention
    let v_a = V_STAR_ARTICLE;
    let chi_at_a = ChiZeIndex::per_modality_with_convention(v_a, VelocityConvention::Article);
    let chi_off_a = ChiZeIndex::per_modality_with_convention(v_a + 0.3, VelocityConvention::Article);
    assert!(chi_at_a > chi_off_a);
}

#[test]
fn b3_convention_consistency() {
    // The same physical state evaluated under either convention should yield
    // the same χ_Ze. Take v_python = 0.6 → v_article = 2·0.6 − 1 = 0.2.
    let v_p = 0.6;
    let v_a = 2.0 * v_p - 1.0;
    let chi_p = ChiZeIndex::per_modality_with_convention(v_p, VelocityConvention::Python);
    let chi_a = ChiZeIndex::per_modality_with_convention(v_a, VelocityConvention::Article);
    // Python denom = max(0.45631, 0.54369) = 0.54369; |0.6 − 0.45631| = 0.14369; χ = 1 − 0.14369/0.54369 ≈ 0.7358
    // Article denom = max(|−0.08738 − (−1)|, |1 − (−0.08738)|) = max(0.91262, 1.08738) = 1.08738
    //                |0.2 − (−0.08738)| = 0.28738; χ = 1 − 0.28738/1.08738 ≈ 0.7358
    assert_abs_diff_eq!(chi_p, chi_a, epsilon = 1e-6);
}
