//! Proper-time integration: `dτ_Ze/dt = −α·I(t)` (THEORY.md §2.2).
//!
//! Two methods: explicit Euler (baseline) and classical Runge–Kutta 4 (canonical).

use crate::{Result, ZeError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IntegratorMethod {
    Rk4,
    Euler,
}

impl Default for IntegratorMethod {
    fn default() -> Self {
        IntegratorMethod::Rk4
    }
}

#[derive(Debug, Clone)]
pub struct ProperTimeIntegrator {
    pub alpha: f64,
    pub method: IntegratorMethod,
    pub dt: f64,
}

impl ProperTimeIntegrator {
    pub fn new(alpha: f64, method: IntegratorMethod, dt: f64) -> Result<Self> {
        if alpha <= 0.0 || !alpha.is_finite() {
            return Err(ZeError::InvalidParameter {
                field: "alpha",
                value: alpha,
                reason: "must be positive and finite",
            });
        }
        if dt <= 0.0 || !dt.is_finite() {
            return Err(ZeError::InvalidParameter {
                field: "dt",
                value: dt,
                reason: "must be positive and finite",
            });
        }
        Ok(Self { alpha, method, dt })
    }

    /// Integrate `dτ/dt = −α·I(t)` over `[0, t_max]` with initial value `tau_0`.
    /// Returns sampled trajectory `[(t_i, τ_i)]` at every step.
    pub fn integrate<F>(&self, i_of_t: F, t_max: f64, tau_0: f64) -> Result<Vec<(f64, f64)>>
    where
        F: Fn(f64) -> f64,
    {
        if t_max <= 0.0 || !t_max.is_finite() {
            return Err(ZeError::InvalidParameter {
                field: "t_max",
                value: t_max,
                reason: "must be positive and finite",
            });
        }
        let n_steps = (t_max / self.dt).ceil() as usize;
        let mut traj = Vec::with_capacity(n_steps + 1);
        traj.push((0.0, tau_0));
        let mut t = 0.0_f64;
        let mut tau = tau_0;
        for _ in 0..n_steps {
            let h = self.dt.min(t_max - t);
            tau = match self.method {
                IntegratorMethod::Euler => self.euler_step(&i_of_t, t, tau, h),
                IntegratorMethod::Rk4 => self.rk4_step(&i_of_t, t, tau, h),
            };
            t += h;
            traj.push((t, tau));
        }
        Ok(traj)
    }

    fn deriv<F>(&self, i_of_t: &F, t: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        -self.alpha * i_of_t(t)
    }

    fn euler_step<F>(&self, i_of_t: &F, t: f64, tau: f64, h: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        tau + h * self.deriv(i_of_t, t)
    }

    fn rk4_step<F>(&self, i_of_t: &F, t: f64, tau: f64, h: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let k1 = self.deriv(i_of_t, t);
        let k2 = self.deriv(i_of_t, t + h / 2.0);
        let k3 = self.deriv(i_of_t, t + h / 2.0);
        let k4 = self.deriv(i_of_t, t + h);
        // Note: derivative is autonomous in tau (does not depend on tau itself), so k_i are τ-independent
        tau + (h / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analytical_constant_i_rk4() {
        let alpha = 1.0;
        let i = 0.5_f64;
        let dt = 1e-3_f64;
        let pt = ProperTimeIntegrator::new(alpha, IntegratorMethod::Rk4, dt).unwrap();
        let traj = pt.integrate(|_| i, 5.0, 10.0).unwrap();
        let last = traj.last().unwrap();
        let expected = 10.0 - alpha * i * 5.0;
        assert!((last.1 - expected).abs() < 1e-9, "got {}, expected {}", last.1, expected);
    }

    #[test]
    fn analytical_constant_i_euler() {
        let alpha = 1.0;
        let i = 0.5_f64;
        let dt = 1e-4_f64;
        let pt = ProperTimeIntegrator::new(alpha, IntegratorMethod::Euler, dt).unwrap();
        let traj = pt.integrate(|_| i, 5.0, 10.0).unwrap();
        let last = traj.last().unwrap();
        let expected = 10.0 - alpha * i * 5.0;
        assert!((last.1 - expected).abs() < 1e-3, "got {}, expected {}", last.1, expected);
    }
}
