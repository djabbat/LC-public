//! Exacerbation risk classifier (THEORY §5).

use crate::{BioSenseError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExacerbCoeffs {
    pub b0: f64,
    pub b_age: f64,
    pub b_sex: f64,
    pub b_chi: f64,
    pub b_dchi: f64,
}

impl Default for ExacerbCoeffs {
    fn default() -> Self {
        Self { b0: -0.4, b_age: 0.025, b_sex: 0.10, b_chi: -2.5, b_dchi: -1.8 }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RiskResult {
    pub risk_30d: f64,
    pub logit: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct ExacerbationModel {
    pub coeffs: ExacerbCoeffs,
}

impl ExacerbationModel {
    pub fn new(coeffs: ExacerbCoeffs) -> Self {
        Self { coeffs }
    }

    pub fn risk(&self, age: f64, sex_male: bool, chi_now: f64, chi_7d_ago: f64) -> Result<RiskResult> {
        if !(0.0..=1.0).contains(&chi_now) || !(0.0..=1.0).contains(&chi_7d_ago) {
            return Err(BioSenseError::InvalidParameter {
                field: "chi_ze",
                value: chi_now,
                reason: "must be in [0, 1]",
            });
        }
        if age < 0.0 || age > 130.0 {
            return Err(BioSenseError::InvalidParameter {
                field: "age",
                value: age,
                reason: "0 ≤ age ≤ 130",
            });
        }
        let c = &self.coeffs;
        let dchi = chi_now - chi_7d_ago;
        let sex = if sex_male { 1.0 } else { 0.0 };
        let logit = c.b0 + c.b_age * age + c.b_sex * sex + c.b_chi * chi_now + c.b_dchi * dchi;
        let risk = 1.0 / (1.0 + (-logit).exp());
        Ok(RiskResult { risk_30d: risk, logit })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_chi_ze_higher_risk() {
        let m = ExacerbationModel::new(ExacerbCoeffs::default());
        let high = m.risk(70.0, true, 0.8, 0.78).unwrap();
        let low = m.risk(70.0, true, 0.4, 0.42).unwrap();
        assert!(low.risk_30d > high.risk_30d);
    }

    #[test]
    fn falling_chi_ze_higher_risk() {
        let m = ExacerbationModel::new(ExacerbCoeffs::default());
        let stable = m.risk(70.0, false, 0.6, 0.60).unwrap();
        let falling = m.risk(70.0, false, 0.6, 0.70).unwrap(); // dropped from 0.7 → 0.6
        assert!(falling.risk_30d > stable.risk_30d);
    }
}
