use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ZeCounter - represents a Ze synchronization counter
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ZeCounter {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub initial_tau_z: i32, // τ_Z initial value
    pub theta_z: f64,        // θ_Z threshold
    pub hilbert_dimension: i32, // dim(H)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeCounterCreate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub initial_tau_z: i32,
    pub theta_z: f64,
    pub hilbert_dimension: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeCounterUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub initial_tau_z: Option<i32>,
    pub theta_z: Option<f64>,
    pub hilbert_dimension: Option<i32>,
}

// ZeParameter - stores theory parameters (global or per-counter)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ZeParameter {
    pub id: Uuid,
    pub ze_counter_id: Option<Uuid>, // NULL for global parameters
    pub parameter_name: String,
    pub parameter_value: f64,
    pub parameter_unit: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeParameterCreate {
    pub ze_counter_id: Option<Uuid>,
    pub parameter_name: String,
    pub parameter_value: f64,
    pub parameter_unit: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeParameterUpdate {
    pub parameter_value: Option<f64>,
    pub parameter_unit: Option<String>,
    pub description: Option<String>,
}

// ZeMeasurement - stores measurements of Ze counters
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ZeMeasurement {
    pub id: Uuid,
    pub ze_counter_id: Uuid,
    pub measurement_time: DateTime<Utc>,
    pub v: f64, // N_S/(N-1) synchronization rate
    pub n_s: i32, // Number of S events
    pub n: i32,   // Total events
    pub v_star_passive: Option<f64>, // v*_passive used in computation
    pub v_star_active: Option<f64>,  // v*_active used in computation
    pub chi_ze: f64, // Computed χ_Ze
    pub tau_z: i32,  // Current τ_Z value
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeMeasurementCreate {
    pub ze_counter_id: Uuid,
    pub measurement_time: DateTime<Utc>,
    pub v: f64,
    pub n_s: i32,
    pub n: i32,
    pub v_star_passive: Option<f64>,
    pub v_star_active: Option<f64>,
    pub tau_z: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeMeasurementUpdate {
    pub v: Option<f64>,
    pub n_s: Option<i32>,
    pub n: Option<i32>,
    pub v_star_passive: Option<f64>,
    pub v_star_active: Option<f64>,
    pub tau_z: Option<i32>,
}

// Computation request/response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeComputationRequest {
    pub v: f64,
    pub v_star_passive: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeComputationResult {
    pub v: f64,
    pub v_star_passive: f64,
    pub chi_ze: f64,
    pub formula: String,
}