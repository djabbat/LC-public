use crate::error::ApiError;
use axum::{Json, Router, routing::{get, post}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ze_simulator::{
    chsh::{ChshDeformation, ChshOptimizer, s_qm},
    consts::CHSH_DEFORMATION_CONSTANT,
    correlation::CorrelationDecay,
    impedance::Distribution,
    proper_time::{IntegratorMethod, ProperTimeIntegrator},
    qfi::QfiBound,
};

pub fn router() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/impedance", post(impedance))
        .route("/api/proper_time", post(proper_time))
        .route("/api/chsh", post(chsh))
        .route("/api/correlation", post(correlation))
        .route("/api/qfi", post(qfi))
        .route("/api/qfi_sweep", post(qfi_sweep))
}

async fn healthz() -> Json<Value> {
    Json(json!({"status": "ok", "version": env!("CARGO_PKG_VERSION")}))
}

#[derive(Deserialize)]
struct ImpedanceReq {
    real: Vec<f64>,
    model: Vec<f64>,
}

#[derive(Serialize)]
struct ImpedanceResp { impedance: f64 }

async fn impedance(Json(req): Json<ImpedanceReq>) -> Result<Json<ImpedanceResp>, ApiError> {
    let p = Distribution::new(req.real)?;
    let q = Distribution::new(req.model)?;
    let i = p.kl_to(&q)?;
    Ok(Json(ImpedanceResp { impedance: i }))
}

#[derive(Deserialize)]
struct ProperTimeReq {
    alpha: f64,
    i: f64,
    t_max: f64,
    dt: Option<f64>,
    tau_0: Option<f64>,
    method: Option<String>,
}

#[derive(Serialize)]
struct ProperTimeResp { trajectory: Vec<(f64, f64)> }

async fn proper_time(Json(req): Json<ProperTimeReq>) -> Result<Json<ProperTimeResp>, ApiError> {
    let dt = req.dt.unwrap_or(1e-3);
    let tau_0 = req.tau_0.unwrap_or(0.0);
    let method = match req.method.as_deref().unwrap_or("rk4") {
        "rk4" => IntegratorMethod::Rk4,
        "euler" => IntegratorMethod::Euler,
        other => return Err(ApiError::bad_request("E_BAD_METHOD", format!("unknown method `{}`", other))),
    };
    let pt = ProperTimeIntegrator::new(req.alpha, method, dt)?;
    let traj = pt.integrate(|_| req.i, req.t_max, tau_0)?;
    Ok(Json(ProperTimeResp { trajectory: traj }))
}

#[derive(Deserialize)]
struct ChshReq {
    delta: f64,
    optimizer: Option<String>,
    n: Option<usize>,
}

#[derive(Serialize)]
struct ChshResp {
    s_qm: f64,
    s_ze: f64,
    predicted_linear: f64,
    delta: f64,
    deformation_const: f64,
    warning: Option<&'static str>,
}

async fn chsh(Json(req): Json<ChshReq>) -> Result<Json<ChshResp>, ApiError> {
    let n = req.n.unwrap_or(1024);
    let optimizer = match req.optimizer.as_deref().unwrap_or("planar-grid") {
        "planar-grid" => ChshOptimizer::PlanarGrid { n },
        "grid" => ChshOptimizer::Grid { n },
        other => return Err(ApiError::bad_request("E_BAD_OPTIMIZER", format!("unknown optimizer `{}`", other))),
    };
    let chsh = ChshDeformation::new(req.delta)?;
    let s_ze_val = chsh.s_optimal(optimizer)?;
    let predicted = s_qm() + req.delta * CHSH_DEFORMATION_CONSTANT;
    Ok(Json(ChshResp {
        s_qm: s_qm(),
        s_ze: s_ze_val,
        predicted_linear: predicted,
        delta: req.delta,
        deformation_const: CHSH_DEFORMATION_CONSTANT,
        warning: if req.delta > 0.0 { Some("exceeds_tsirelson_bound") } else { None },
    }))
}

#[derive(Deserialize)]
struct CorrelationReq {
    c0: f64,
    beta: f64,
    i: f64,
    tau_grid: Vec<f64>,
}

#[derive(Serialize)]
struct CorrelationResp { c: Vec<f64> }

async fn correlation(Json(req): Json<CorrelationReq>) -> Result<Json<CorrelationResp>, ApiError> {
    let cd = CorrelationDecay::new(req.c0, req.beta, req.i)?;
    let mut out = Vec::with_capacity(req.tau_grid.len());
    for &tau in &req.tau_grid {
        out.push(cd.at(tau)?);
    }
    Ok(Json(CorrelationResp { c: out }))
}

#[derive(Deserialize)]
struct QfiReq {
    c0: f64,
    beta: f64,
    i: f64,
    tau: Option<f64>,
}

#[derive(Serialize)]
struct QfiResp {
    f_q_lower_bound: f64,
    regime: &'static str,
    tau_used: f64,
}

async fn qfi(Json(req): Json<QfiReq>) -> Result<Json<QfiResp>, ApiError> {
    let q = QfiBound::new(req.c0, req.beta, req.i)?;
    let r = match req.tau {
        Some(tau) => q.at(tau)?,
        None => q.at_optimal_tau()?,
    };
    Ok(Json(QfiResp {
        f_q_lower_bound: r.f_q_lower_bound,
        regime: r.regime,
        tau_used: r.tau_used,
    }))
}

#[derive(Deserialize)]
struct QfiSweepReq {
    c0: f64,
    beta: f64,
    i_grid: Vec<f64>,
}

#[derive(Serialize)]
struct QfiSweepResp {
    f_q: Vec<f64>,
    dtau_dt_abs: Vec<f64>,
}

async fn qfi_sweep(Json(req): Json<QfiSweepReq>) -> Result<Json<QfiSweepResp>, ApiError> {
    let mut fq = Vec::with_capacity(req.i_grid.len());
    let mut dtau = Vec::with_capacity(req.i_grid.len());
    for &i in &req.i_grid {
        let q = QfiBound::new(req.c0, req.beta, i)?;
        let r = q.at_optimal_tau()?;
        fq.push(r.f_q_lower_bound);
        dtau.push(i); // α = 1 by convention
    }
    Ok(Json(QfiSweepResp { f_q: fq, dtau_dt_abs: dtau }))
}
