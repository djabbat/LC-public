//! B5: exacerbation classifier — lower χ_Ze → higher risk; AUC ≥ 0.5 on synthetic.
//! THEORY §7 row B5 + §5.

use biosense_simulator::{ExacerbCoeffs, ExacerbationModel};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::Rng;

#[test]
fn b5_lower_chi_higher_risk() {
    let m = ExacerbationModel::new(ExacerbCoeffs::default());
    let high = m.risk(70.0, true, 0.85, 0.84).unwrap();
    let low = m.risk(70.0, true, 0.40, 0.42).unwrap();
    assert!(low.risk_30d > high.risk_30d);
}

#[test]
fn b5_falling_chi_higher_risk() {
    let m = ExacerbationModel::new(ExacerbCoeffs::default());
    let stable = m.risk(70.0, false, 0.55, 0.55).unwrap();
    let falling = m.risk(70.0, false, 0.55, 0.70).unwrap(); // dropped 0.7→0.55
    assert!(falling.risk_30d > stable.risk_30d);
}

#[test]
fn b5_auc_ge_half_on_synthetic() {
    // Generate cohort: low-χ subjects label=1, high-χ subjects label=0
    // (with random age/sex/Δχ). Predicted risk should rank 1s above 0s on average.
    let mut rng = StdRng::seed_from_u64(20_260_428);
    let m = ExacerbationModel::new(ExacerbCoeffs::default());
    let n = 1000;
    let mut scores: Vec<(f64, u8)> = Vec::with_capacity(n);
    for _ in 0..n {
        let label: u8 = if rng.gen::<f64>() < 0.5 { 1 } else { 0 };
        let chi = if label == 1 {
            rng.gen_range(0.30..0.55)
        } else {
            rng.gen_range(0.55..0.85)
        };
        let chi_7d = chi + rng.gen_range(-0.05..0.05);
        let age = rng.gen_range(50.0..80.0);
        let male = rng.gen::<f64>() < 0.5;
        let r = m.risk(age, male, chi, chi_7d).unwrap();
        scores.push((r.risk_30d, label));
    }

    // Compute AUC by counting concordant pairs.
    let mut concordant = 0_u64;
    let mut discordant = 0_u64;
    for i in 0..n {
        for j in (i + 1)..n {
            if scores[i].1 != scores[j].1 {
                let (pos_score, neg_score) = if scores[i].1 == 1 {
                    (scores[i].0, scores[j].0)
                } else {
                    (scores[j].0, scores[i].0)
                };
                if pos_score > neg_score {
                    concordant += 1;
                } else if pos_score < neg_score {
                    discordant += 1;
                }
            }
        }
    }
    let total = (concordant + discordant) as f64;
    let auc = concordant as f64 / total;
    assert!(auc > 0.7, "expected AUC > 0.7 on synthetic, got {}", auc);
}
