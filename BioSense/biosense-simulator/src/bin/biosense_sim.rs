//! `biosense_sim` CLI — JSON in/out.

use clap::{Parser, Subcommand};
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde_json::json;
use std::process::ExitCode;
use biosense_simulator::{
    BridgeParams, CdataBridge, ChiZeIndex, ChiZeWeights,
    DpBudget, ExacerbCoeffs, ExacerbationModel, KAnonymity,
    PredictiveInfo, PredictorKind, ZeVelocity,
};

#[derive(Parser)]
#[command(name = "biosense_sim", about = "BioSense reference simulator (CLI).")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Velocity {
        #[arg(long, value_delimiter = ',')]
        symbols: Vec<u8>,
        #[arg(long, default_value = "identity")]
        predictor: String,
    },
    PredInfo {
        #[arg(long)]
        p: Option<f64>,
        #[arg(long, value_delimiter = ',')]
        symbols: Option<Vec<u8>>,
    },
    ChiZe {
        #[arg(long)]
        eeg: f64,
        #[arg(long)]
        hrv: f64,
        #[arg(long)]
        resp: f64,
        #[arg(long)]
        sleep: f64,
    },
    Bridge {
        #[arg(long)]
        d: f64,
    },
    Exacerbation {
        #[arg(long)]
        age: f64,
        #[arg(long, default_value = "F")]
        sex: String,
        #[arg(long)]
        chi_now: f64,
        #[arg(long)]
        chi_7d_ago: f64,
    },
    PrivacyDp {
        #[arg(long, default_value_t = 2.0)]
        eps: f64,
        #[arg(long, default_value_t = 1e-5)]
        delta: f64,
        #[arg(long, default_value_t = 0.3)]
        sensitivity: f64,
        #[arg(long)]
        x: f64,
        #[arg(long, default_value_t = 20_260_428)]
        seed: u64,
    },
    PrivacyCompose {
        #[arg(long, default_value_t = 2.0)]
        eps: f64,
        #[arg(long, default_value_t = 1e-5)]
        delta: f64,
        #[arg(long)]
        n: usize,
    },
    FixedPoint,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.cmd {
        Cmd::Velocity { symbols, predictor } => cmd_velocity(symbols, &predictor),
        Cmd::PredInfo { p, symbols } => cmd_pred_info(p, symbols),
        Cmd::ChiZe { eeg, hrv, resp, sleep } => cmd_chi_ze(eeg, hrv, resp, sleep),
        Cmd::Bridge { d } => cmd_bridge(d),
        Cmd::Exacerbation { age, sex, chi_now, chi_7d_ago } => {
            cmd_exacerbation(age, &sex, chi_now, chi_7d_ago)
        }
        Cmd::PrivacyDp { eps, delta, sensitivity, x, seed } => {
            cmd_privacy_dp(eps, delta, sensitivity, x, seed)
        }
        Cmd::PrivacyCompose { eps, delta, n } => cmd_privacy_compose(eps, delta, n),
        Cmd::FixedPoint => Ok(json!({"v_star": ChiZeIndex::fixed_point()})),
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

fn err(e: biosense_simulator::BioSenseError) -> serde_json::Value {
    json!({"error": {"code": "E_BIOSENSE", "message": e.to_string()}})
}

fn cmd_velocity(symbols: Vec<u8>, predictor: &str) -> Result<serde_json::Value, serde_json::Value> {
    let p = match predictor {
        "identity" => PredictorKind::Identity,
        "flip" => PredictorKind::Flip,
        other => return Err(json!({"error": {"code": "E_BAD_PRED", "message": format!("unknown predictor `{}`", other)}})),
    };
    let v = ZeVelocity::from_signal(&symbols, p).map_err(err)?;
    Ok(json!({"v": v}))
}

fn cmd_pred_info(p: Option<f64>, symbols: Option<Vec<u8>>) -> Result<serde_json::Value, serde_json::Value> {
    if let Some(p) = p {
        let i = PredictiveInfo::closed_form(p).map_err(err)?;
        return Ok(json!({"i_pred": i, "method": "closed_form"}));
    }
    if let Some(s) = symbols {
        let i = PredictiveInfo::estimate(&s, 64).map_err(err)?;
        return Ok(json!({"i_pred": i, "method": "numerical"}));
    }
    Err(json!({"error": {"code": "E_INPUT", "message": "need --p or --symbols"}}))
}

fn cmd_chi_ze(eeg: f64, hrv: f64, resp: f64, sleep: f64) -> Result<serde_json::Value, serde_json::Value> {
    let chi = ChiZeIndex::new(ChiZeWeights::default()).map_err(err)?;
    let bd = chi.breakdown(eeg, hrv, resp, sleep);
    Ok(json!({
        "composite": bd.composite,
        "per_modality": {"eeg": bd.eeg, "hrv": bd.hrv, "resp": bd.resp, "sleep": bd.sleep},
        "v_star": ChiZeIndex::fixed_point()
    }))
}

fn cmd_bridge(d: f64) -> Result<serde_json::Value, serde_json::Value> {
    let b = CdataBridge::new(BridgeParams::default()).map_err(err)?;
    let a = b.activity(d);
    let chi = b.chi_ze_from_a(a);
    Ok(json!({"d": d, "a": a, "chi_ze": chi}))
}

fn cmd_exacerbation(age: f64, sex: &str, chi_now: f64, chi_7d_ago: f64) -> Result<serde_json::Value, serde_json::Value> {
    let m = ExacerbationModel::new(ExacerbCoeffs::default());
    let male = matches!(sex, "M" | "m" | "male" | "Male" | "1");
    let r = m.risk(age, male, chi_now, chi_7d_ago).map_err(err)?;
    Ok(json!({"risk_30d": r.risk_30d, "logit": r.logit}))
}

fn cmd_privacy_dp(eps: f64, delta: f64, sens: f64, x: f64, seed: u64) -> Result<serde_json::Value, serde_json::Value> {
    let dp = DpBudget::new(eps, delta, sens).map_err(err)?;
    let mut rng = StdRng::seed_from_u64(seed);
    let xn = dp.laplace_noise(x, &mut rng);
    Ok(json!({"x": x, "x_noised": xn, "noise": xn - x}))
}

fn cmd_privacy_compose(eps: f64, delta: f64, n: usize) -> Result<serde_json::Value, serde_json::Value> {
    let dp = DpBudget::new(eps, delta, 1.0).map_err(err)?;
    Ok(json!({"eps_total_naive": dp.naive_cumulative_eps(n), "n_releases": n}))
}
