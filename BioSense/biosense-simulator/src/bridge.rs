//! CDATA bridge (THEORY §4).

use crate::{BioSenseError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BridgeParams {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub g0: f64,
    pub g1: f64,
}

impl Default for BridgeParams {
    fn default() -> Self {
        Self { a: 0.05, b: 1.20, c: 0.40, g0: 0.95, g1: 1.10 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CdataBridge {
    pub params: BridgeParams,
}

impl CdataBridge {
    pub fn new(params: BridgeParams) -> Result<Self> {
        let p = &params;
        if p.b <= 0.0 || p.c < 0.0 || p.g1 <= 0.0 {
            return Err(BioSenseError::InvalidParameter {
                field: "bridge",
                value: 0.0,
                reason: "b > 0, c ≥ 0, g1 > 0 required",
            });
        }
        if p.a < 0.0 || p.g0 <= 0.0 || p.g0 > 1.0 {
            return Err(BioSenseError::InvalidParameter {
                field: "bridge",
                value: 0.0,
                reason: "a ≥ 0 and 0 < g0 ≤ 1 required",
            });
        }
        Ok(Self { params })
    }

    pub fn activity(&self, d: f64) -> f64 {
        let p = self.params;
        p.a + p.b * d + p.c * d * d
    }

    pub fn chi_ze_from_a(&self, a_val: f64) -> f64 {
        let p = self.params;
        (p.g0 - p.g1 * a_val).clamp(0.0, 1.0)
    }

    pub fn chi_ze_from_d(&self, d: f64) -> f64 {
        self.chi_ze_from_a(self.activity(d))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monotone_in_d() {
        let b = CdataBridge::new(BridgeParams::default()).unwrap();
        let mut last = f64::INFINITY;
        for &d in &[0.0_f64, 0.1, 0.3, 0.5, 0.7, 0.9] {
            let chi = b.chi_ze_from_d(d);
            assert!(chi <= last + 1e-12, "non-monotone: {} > {}", chi, last);
            last = chi;
        }
    }

    #[test]
    fn limits() {
        let b = CdataBridge::new(BridgeParams::default()).unwrap();
        let chi_zero = b.chi_ze_from_d(0.0);
        let chi_one = b.chi_ze_from_d(1.0);
        assert!(chi_zero > chi_one);
        assert!(chi_zero >= 0.0 && chi_zero <= 1.0);
        assert!(chi_one >= 0.0 && chi_one <= 1.0);
    }
}
