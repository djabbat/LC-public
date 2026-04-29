use crate::error::ApiError;
use axum::{Json, Router, routing::{get, post}};
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use biosense_simulator::{
    BridgeParams, CdataBridge, ChiZeIndex, ChiZeWeights,
    DpBudget, ExacerbCoeffs, ExacerbationModel,
    PredictiveInfo, PredictorKind, VelocityConvention, ZeVelocity,
};

pub fn router() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/velocity", post(velocity))
        .route("/api/pred_info", post(pred_info))
        .route("/api/chi_ze", post(chi_ze))
        .route("/api/bridge", post(bridge))
        .route("/api/exacerbation", post(exacerbation))
        .route("/api/privacy/dp", post(privacy_dp))
        .route("/api/privacy/compose", post(privacy_compose))
}

async fn healthz() -> Json<Value> {
    Json(json!({"status": "ok", "version": env!("CARGO_PKG_VERSION")}))
}

#[derive(Deserialize)]
struct VelocityReq {
    symbols: Vec<u8>,
    predictor: Option<String>,
    convention: Option<VelocityConvention>,
}

async fn velocity(Json(req): Json<VelocityReq>) -> Result<Json<Value>, ApiError> {
    let convention = req.convention.unwrap_or_default();
    let v = match convention {
        VelocityConvention::Python => ZeVelocity::frequency_switch(&req.symbols)?,
        VelocityConvention::Article => {
            let p = match req.predictor.as_deref().unwrap_or("identity") {
                "identity" => PredictorKind::Identity,
                "flip" => PredictorKind::Flip,
                other => return Err(ApiError::bad_request("E_BAD_PRED",
                                                          format!("unknown predictor `{}`", other))),
            };
            ZeVelocity::from_signal(&req.symbols, p)?
        }
    };
    Ok(Json(json!({"v": v, "convention": convention})))
}

#[derive(Deserialize)]
struct PredInfoReq { p: Option<f64>, symbols: Option<Vec<u8>> }

async fn pred_info(Json(req): Json<PredInfoReq>) -> Result<Json<Value>, ApiError> {
    if let Some(p) = req.p {
        let i = PredictiveInfo::closed_form(p)?;
        return Ok(Json(json!({"i_pred": i, "method": "closed_form"})));
    }
    if let Some(s) = req.symbols {
        let i = PredictiveInfo::estimate(&s, 64)?;
        return Ok(Json(json!({"i_pred": i, "method": "numerical"})));
    }
    Err(ApiError::bad_request("E_INPUT", "need p or symbols"))
}

#[derive(Deserialize)]
struct ChiZeReq {
    eeg: f64, hrv: f64, resp: f64, sleep: f64,
    weights: Option<ChiZeWeights>,
    convention: Option<VelocityConvention>,
}

async fn chi_ze(Json(req): Json<ChiZeReq>) -> Result<Json<Value>, ApiError> {
    let w = req.weights.unwrap_or_default();
    let chi = ChiZeIndex::new(w)?;
    let convention = req.convention.unwrap_or_default();
    let eeg = ChiZeIndex::per_modality_with_convention(req.eeg, convention);
    let hrv = ChiZeIndex::per_modality_with_convention(req.hrv, convention);
    let resp = ChiZeIndex::per_modality_with_convention(req.resp, convention);
    let sleep = ChiZeIndex::per_modality_with_convention(req.sleep, convention);
    let composite = chi.composite(eeg, hrv, resp, sleep);
    Ok(Json(json!({
        "composite": composite,
        "per_modality": {"eeg": eeg, "hrv": hrv, "resp": resp, "sleep": sleep},
        "v_star": ChiZeIndex::fixed_point_for(convention),
        "convention": convention
    })))
}

#[derive(Deserialize)]
struct BridgeReq { d: f64, params: Option<BridgeParams> }

async fn bridge(Json(req): Json<BridgeReq>) -> Result<Json<Value>, ApiError> {
    let b = CdataBridge::new(req.params.unwrap_or_default())?;
    let a = b.activity(req.d);
    let chi = b.chi_ze_from_a(a);
    Ok(Json(json!({"d": req.d, "a": a, "chi_ze": chi})))
}

#[derive(Deserialize)]
struct ExacerbReq {
    age: f64,
    sex: String,
    chi_now: f64,
    chi_7d_ago: f64,
    coeffs: Option<ExacerbCoeffs>,
}

async fn exacerbation(Json(req): Json<ExacerbReq>) -> Result<Json<Value>, ApiError> {
    let m = ExacerbationModel::new(req.coeffs.unwrap_or_default());
    let male = matches!(req.sex.as_str(), "M" | "m" | "male" | "Male" | "1");
    let r = m.risk(req.age, male, req.chi_now, req.chi_7d_ago)?;
    Ok(Json(json!({"risk_30d": r.risk_30d, "logit": r.logit})))
}

#[derive(Deserialize)]
struct DpReq {
    x: f64, eps: f64, delta: f64, sensitivity: f64, seed: Option<u64>,
}

async fn privacy_dp(Json(req): Json<DpReq>) -> Result<Json<Value>, ApiError> {
    let dp = DpBudget::new(req.eps, req.delta, req.sensitivity)?;
    let mut rng = StdRng::seed_from_u64(req.seed.unwrap_or(20_260_428));
    let xn = dp.laplace_noise(req.x, &mut rng);
    Ok(Json(json!({"x": req.x, "x_noised": xn, "noise": xn - req.x})))
}

#[derive(Deserialize)]
struct ComposeReq { eps: f64, delta: f64, n: usize }

async fn privacy_compose(Json(req): Json<ComposeReq>) -> Result<Json<Value>, ApiError> {
    let dp = DpBudget::new(req.eps, req.delta, 1.0)?;
    Ok(Json(json!({"eps_total_naive": dp.naive_cumulative_eps(req.n), "n_releases": req.n})))
}

#[derive(Serialize)]
#[allow(dead_code)]
struct UnusedHelper;
