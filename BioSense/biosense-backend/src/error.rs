use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
}

impl ApiError {
    pub fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self { status: StatusCode::BAD_REQUEST, code, message: message.into() }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = json!({"error": {"code": self.code, "message": self.message}});
        (self.status, Json(body)).into_response()
    }
}

impl From<biosense_simulator::BioSenseError> for ApiError {
    fn from(e: biosense_simulator::BioSenseError) -> Self {
        let code = match &e {
            biosense_simulator::BioSenseError::InvalidParameter { .. } => "E_INVALID_PARAM",
            biosense_simulator::BioSenseError::StreamTooShort { .. } => "E_STREAM_SHORT",
            biosense_simulator::BioSenseError::InvalidSymbol { .. } => "E_BAD_SYMBOL",
            biosense_simulator::BioSenseError::MarkovRateOutOfRange { .. } => "E_P_OUT_OF_RANGE",
            biosense_simulator::BioSenseError::DpBudgetExceeded { .. } => "E_DP_BUDGET",
            biosense_simulator::BioSenseError::KAnonymityViolated { .. } => "E_K_ANON",
        };
        Self::bad_request(code, e.to_string())
    }
}
