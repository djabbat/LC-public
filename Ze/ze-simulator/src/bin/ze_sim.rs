//! `ze_sim` CLI. JSON-on-stdout for every successful subcommand;
//! non-zero exit + JSON-on-stderr for errors.

use clap::{Parser, Subcommand};
use serde_json::json;
use std::process::ExitCode;
use ze_simulator::{
    chsh::{ChshDeformation, ChshOptimizer},
    correlation::CorrelationDecay,
    impedance::Distribution,
    proper_time::{IntegratorMethod, ProperTimeIntegrator},
    qfi::QfiBound,
    consts::CHSH_DEFORMATION_CONSTANT,
};

#[derive(Parser)]
#[command(name = "ze_sim", about = "Ze Theory reference simulator (CLI).")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Compute KL divergence between two distributions.
    Impedance {
        #[arg(long, value_delimiter = ',')]
        real: Vec<f64>,
        #[arg(long, value_delimiter = ',')]
        model: Vec<f64>,
    },
    /// Integrate τ_Ze with constant impedance.
    ProperTime {
        #[arg(long, default_value_t = 1.0)]
        alpha: f64,
        #[arg(long, default_value_t = 0.5)]
        i: f64,
        #[arg(long, default_value_t = 10.0)]
        t_max: f64,
        #[arg(long, default_value_t = 1e-3)]
        dt: f64,
        #[arg(long, default_value_t = 1.0)]
        tau_0: f64,
        #[arg(long, default_value = "rk4")]
        method: String,
    },
    /// Compute optimal CHSH for a given δ.
    Chsh {
        #[arg(long, default_value_t = 0.0)]
        delta: f64,
        #[arg(long, default_value = "planar-grid")]
        optimizer: String,
        #[arg(long, default_value_t = 1024)]
        n: usize,
    },
    /// Single-point C(τ).
    Correlation {
        #[arg(long, default_value_t = 1.0)]
        c0: f64,
        #[arg(long, default_value_t = 1.0)]
        beta: f64,
        #[arg(long, default_value_t = 0.5)]
        i: f64,
        #[arg(long, default_value_t = 0.5)]
        tau: f64,
    },
    /// Single-point QFI lower bound.
    Qfi {
        #[arg(long, default_value_t = 1.0)]
        c0: f64,
        #[arg(long, default_value_t = 1.0)]
        beta: f64,
        #[arg(long, default_value_t = 0.5)]
        i: f64,
        #[arg(long)]
        tau: Option<f64>, // if absent, use optimal τ
    },
    /// QFI sweep across an impedance grid.
    QfiSweep {
        #[arg(long, default_value_t = 1.0)]
        c0: f64,
        #[arg(long, default_value_t = 1.0)]
        beta: f64,
        #[arg(long, default_value_t = 0.01)]
        i_min: f64,
        #[arg(long, default_value_t = 1.0)]
        i_max: f64,
        #[arg(long, default_value_t = 50)]
        n: usize,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.cmd {
        Cmd::Impedance { real, model } => cmd_impedance(real, model),
        Cmd::ProperTime { alpha, i, t_max, dt, tau_0, method } => {
            cmd_proper_time(alpha, i, t_max, dt, tau_0, &method)
        }
        Cmd::Chsh { delta, optimizer, n } => cmd_chsh(delta, &optimizer, n),
        Cmd::Correlation { c0, beta, i, tau } => cmd_correlation(c0, beta, i, tau),
        Cmd::Qfi { c0, beta, i, tau } => cmd_qfi(c0, beta, i, tau),
        Cmd::QfiSweep { c0, beta, i_min, i_max, n } => cmd_qfi_sweep(c0, beta, i_min, i_max, n),
    };
    match result {
        Ok(j) => {
            println!("{}", j);
            ExitCode::SUCCESS
        }
        Err(j) => {
            eprintln!("{}", j);
            ExitCode::from(1)
        }
    }
}

fn cmd_impedance(real: Vec<f64>, model: Vec<f64>) -> Result<serde_json::Value, serde_json::Value> {
    let pr = Distribution::new(real).map_err(err)?;
    let pm = Distribution::new(model).map_err(err)?;
    let i = pr.kl_to(&pm).map_err(err)?;
    Ok(json!({"impedance": i}))
}

fn cmd_proper_time(
    alpha: f64,
    i: f64,
    t_max: f64,
    dt: f64,
    tau_0: f64,
    method: &str,
) -> Result<serde_json::Value, serde_json::Value> {
    let m = match method {
        "rk4" => IntegratorMethod::Rk4,
        "euler" => IntegratorMethod::Euler,
        other => return Err(json!({"error": {"code": "E_BAD_METHOD", "message": format!("unknown method `{}`", other)}})),
    };
    let pt = ProperTimeIntegrator::new(alpha, m, dt).map_err(err)?;
    let traj = pt.integrate(|_| i, t_max, tau_0).map_err(err)?;
    Ok(json!({"trajectory": traj}))
}

fn cmd_chsh(delta: f64, optimizer: &str, n: usize) -> Result<serde_json::Value, serde_json::Value> {
    let chsh = ChshDeformation::new(delta).map_err(err)?;
    let opt = match optimizer {
        "planar-grid" => ChshOptimizer::PlanarGrid { n },
        "grid" => ChshOptimizer::Grid { n },
        other => return Err(json!({"error": {"code": "E_BAD_OPTIMIZER", "message": format!("unknown optimizer `{}`", other)}})),
    };
    let s_ze = chsh.s_optimal(opt).map_err(err)?;
    let s_qm = ze_simulator::chsh::s_qm();
    let predicted = s_qm + delta * CHSH_DEFORMATION_CONSTANT;
    Ok(json!({
        "s_qm": s_qm,
        "s_ze": s_ze,
        "predicted_linear": predicted,
        "delta": delta,
        "deformation_const": CHSH_DEFORMATION_CONSTANT,
        "warning": if delta > 0.0 { Some("exceeds_tsirelson_bound") } else { None }
    }))
}

fn cmd_correlation(c0: f64, beta: f64, i: f64, tau: f64) -> Result<serde_json::Value, serde_json::Value> {
    let cd = CorrelationDecay::new(c0, beta, i).map_err(err)?;
    let c = cd.at(tau).map_err(err)?;
    Ok(json!({"c": c, "tau": tau}))
}

fn cmd_qfi(c0: f64, beta: f64, i: f64, tau: Option<f64>) -> Result<serde_json::Value, serde_json::Value> {
    let q = QfiBound::new(c0, beta, i).map_err(err)?;
    let r = match tau {
        Some(t) => q.at(t).map_err(err)?,
        None => q.at_optimal_tau().map_err(err)?,
    };
    Ok(json!({"f_q_lower_bound": r.f_q_lower_bound, "regime": r.regime, "tau_used": r.tau_used}))
}

fn cmd_qfi_sweep(c0: f64, beta: f64, i_min: f64, i_max: f64, n: usize) -> Result<serde_json::Value, serde_json::Value> {
    if n < 2 || i_min <= 0.0 || i_max <= i_min {
        return Err(json!({"error": {"code": "E_BAD_SWEEP", "message": "need n ≥ 2, 0 < i_min < i_max"}}));
    }
    let mut grid = Vec::with_capacity(n);
    let mut fq = Vec::with_capacity(n);
    let mut dtau = Vec::with_capacity(n); // |dτ/dt| with α=1: equals impedance
    let log_min = i_min.ln();
    let log_max = i_max.ln();
    for k in 0..n {
        let frac = k as f64 / (n - 1) as f64;
        let i = (log_min + frac * (log_max - log_min)).exp();
        let q = QfiBound::new(c0, beta, i).map_err(err)?;
        let r = q.at_optimal_tau().map_err(err)?;
        grid.push(i);
        fq.push(r.f_q_lower_bound);
        dtau.push(i); // |dτ/dt| = α·I, with α = 1 by convention
    }
    Ok(json!({"i_grid": grid, "f_q": fq, "dtau_dt_abs": dtau}))
}

fn err(e: ze_simulator::ZeError) -> serde_json::Value {
    json!({"error": {"code": "E_ZE", "message": e.to_string()}})
}
