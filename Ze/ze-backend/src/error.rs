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
    pub fn internal(message: impl Into<String>) -> Self {
        Self { status: StatusCode::INTERNAL_SERVER_ERROR, code: "E_INTERNAL", message: message.into() }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = json!({
            "error": { "code": self.code, "message": self.message }
        });
        (self.status, Json(body)).into_response()
    }
}

impl From<ze_simulator::ZeError> for ApiError {
    fn from(e: ze_simulator::ZeError) -> Self {
        let code = match &e {
            ze_simulator::ZeError::Normalization { .. } => "E_NORMALIZATION",
            ze_simulator::ZeError::LengthMismatch { .. } => "E_LENGTH_MISMATCH",
            ze_simulator::ZeError::InfiniteKl { .. } => "E_INFINITE_KL",
            ze_simulator::ZeError::ExtrapolationRefused { .. } => "E_EXTRAPOLATION",
            ze_simulator::ZeError::InvalidParameter { .. } => "E_INVALID_PARAM",
            ze_simulator::ZeError::OptimizerFailed { .. } => "E_OPTIMIZER",
        };
        Self::bad_request(code, e.to_string())
    }
}
