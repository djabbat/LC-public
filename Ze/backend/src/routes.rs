use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::{
    error::AppError,
    models::{
        ZeCounter, ZeCounterCreate, ZeCounterUpdate,
        ZeParameter, ZeParameterCreate, ZeParameterUpdate,
        ZeMeasurement, ZeMeasurementCreate, ZeMeasurementUpdate,
        ZeComputationRequest, ZeComputationResult,
    },
};

// State wrapper for pool
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

// Query parameters for listing
#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

// ZeCounter routes
async fn list_ze_counters(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<ZeCounter>>, AppError> {
    let page = pagination.page.unwrap_or(1).max(1) as i64;
    let per_page = pagination.per_page.unwrap_or(50).min(100) as i64;
    let offset = (page - 1) * per_page;

    let counters = sqlx::query_as::<_, ZeCounter>(
        "SELECT * FROM ze_counters ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&pool)
        .await?;

    Ok(Json(counters))
}

async fn get_ze_counter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ZeCounter>, AppError> {
    let counter = sqlx::query_as::<_, ZeCounter>(
        "SELECT * FROM ze_counters WHERE id = $1"
    )
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("ZeCounter not found".into()))?;

    Ok(Json(counter))
}

async fn create_ze_counter(
    State(pool): State<PgPool>,
    Json(payload): Json<ZeCounterCreate>,
) -> Result<(StatusCode, Json<ZeCounter>), AppError> {
    let counter = sqlx::query_as::<_, ZeCounter>(
        r#"
        INSERT INTO ze_counters (name, description, initial_tau_z, theta_z, hilbert_dimension)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
        .bind(payload.name)
        .bind(payload.description)
        .bind(payload.initial_tau_z)
        .bind(payload.theta_z)
        .bind(payload.hilbert_dimension)
        .fetch_one(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(counter)))
}

async fn update_ze_counter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ZeCounterUpdate>,
) -> Result<Json<ZeCounter>, AppError> {
    let counter = sqlx::query_as::<_, ZeCounter>(
        r#"
        UPDATE ze_counters
        SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            initial_tau_z = COALESCE($3, initial_tau_z),
            theta_z = COALESCE($4, theta_z),
            hilbert_dimension = COALESCE($5, hilbert_dimension),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $6
        RETURNING *
        "#
    )
        .bind(payload.name)
        .bind(payload.description)
        .bind(payload.initial_tau_z)
        .bind(payload.theta_z)
        .bind(payload.hilbert_dimension)
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("ZeCounter not found".into()))?;

    Ok(Json(counter))
}

async fn delete_ze_counter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query(
        "DELETE FROM ze_counters WHERE id = $1"
    )
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("ZeCounter not found".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}

// ZeParameter routes
async fn list_ze_parameters(
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<ZeParameter>>, AppError> {
    let ze_counter_id = params.get("ze_counter_id").and_then(|s| Uuid::parse_str(s).ok());
    
    let query = if let Some(counter_id) = ze_counter_id {
        sqlx::query_as::<_, ZeParameter>(
            "SELECT * FROM ze_parameters WHERE ze_counter_id = $1 ORDER BY parameter_name"
        )
        .bind(counter_id)
    } else {
        sqlx::query_as::<_, ZeParameter>(
            "SELECT * FROM ze_parameters WHERE ze_counter_id IS NULL ORDER BY parameter_name"
        )
    };
    
    let parameters = query.fetch_all(&pool).await?;
    Ok(Json(parameters))
}

async fn create_ze_parameter(
    State(pool): State<PgPool>,
    Json(payload): Json<ZeParameterCreate>,
) -> Result<(StatusCode, Json<ZeParameter>), AppError> {
    let param = sqlx::query_as::<_, ZeParameter>(
        r#"
        INSERT INTO ze_parameters (ze_counter_id, parameter_name, parameter_value, parameter_unit, description)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
        .bind(payload.ze_counter_id)
        .bind(payload.parameter_name)
        .bind(payload.parameter_value)
        .bind(payload.parameter_unit)
        .bind(payload.description)
        .fetch_one(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(param)))
}

async fn update_ze_parameter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ZeParameterUpdate>,
) -> Result<Json<ZeParameter>, AppError> {
    let param = sqlx::query_as::<_, ZeParameter>(
        r#"
        UPDATE ze_parameters
        SET
            parameter_value = COALESCE($1, parameter_value),
            parameter_unit = COALESCE($2, parameter_unit),
            description = COALESCE($3, description),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $4
        RETURNING *
        "#
    )
        .bind(payload.parameter_value)
        .bind(payload.parameter_unit)
        .bind(payload.description)
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("ZeParameter not found".into()))?;

    Ok(Json(param))
}

async fn delete_ze_parameter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query(
        "DELETE FROM ze_parameters WHERE id = $1"
    )
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("ZeParameter not found".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}

// ZeMeasurement routes
async fn list_ze_measurements(
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<ZeMeasurement>>, AppError> {
    let ze_counter_id = params.get("ze_counter_id").and_then(|s| Uuid::parse_str(s).ok());
    
    let query = if let Some(counter_id) = ze_counter_id {
        sqlx::query_as::<_, ZeMeasurement>(
            "SELECT * FROM ze_measurements WHERE ze_counter_id = $1 ORDER BY measurement_time DESC"
        )
        .bind(counter_id)
    } else {
        sqlx::query_as::<_, ZeMeasurement>(
            "SELECT * FROM ze_measurements ORDER BY measurement_time DESC"
        )
    };
    
    let measurements = query.fetch_all(&pool).await?;
    Ok(Json(measurements))
}

async fn create_ze_measurement(
    State(pool): State<PgPool>,
    Json(payload): Json<ZeMeasurementCreate>,
) -> Result<(StatusCode, Json<ZeMeasurement>), AppError> {
    // Compute χ_Ze according to canonical formula: χ_Ze = 1 - |v - v*| / max(v*, 1-v*)
    let v_star = payload.v_star_passive.unwrap_or(0.3069); // default 1 - ln(2)
    let max_denom = v_star.max(1.0 - v_star);
    let chi_ze = if max_denom > 0.0 {
        1.0 - (payload.v - v_star).abs() / max_denom
    } else {
        0.0
    };

    let measurement = sqlx::query_as::<_, ZeMeasurement>(
        r#"
        INSERT INTO ze_measurements 
        (ze_counter_id, measurement_time, v, n_s, n, v_star_passive, v_star_active, chi_ze, tau_z)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#
    )
        .bind(payload.ze_counter_id)
        .bind(payload.measurement_time)
        .bind(payload.v)
        .bind(payload.n_s)
        .bind(payload.n)
        .bind(payload.v_star_passive)
        .bind(payload.v_star_active)
        .bind(chi_ze)
        .bind(payload.tau_z)
        .fetch_one(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(measurement)))
}

async fn delete_ze_measurement(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query(
        "DELETE FROM ze_measurements WHERE id = $1"
    )
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("ZeMeasurement not found".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}

// Computation endpoint
async fn compute_chi_ze(
    Json(payload): Json<ZeComputationRequest>,
) -> Result<Json<ZeComputationResult>, AppError> {
    let v_star = payload.v_star_passive.unwrap_or(0.3069);
    let max_denom = v_star.max(1.0 - v_star);
    let chi_ze = if max_denom > 0.0 {
        1.0 - (payload.v - v_star).abs() / max_denom
    } else {
        0.0
    };

    let result = ZeComputationResult {
        v: payload.v,
        v_star_passive: v_star,
        chi_ze,
        formula: "χ_Ze = 1 - |v - v*| / max(v*, 1-v*)".to_string(),
    };

    Ok(Json(result))
}

// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

pub fn api_routes() -> Router<PgPool> {
    Router::new()
        // ZeCounter routes
        .route("/ze_counters", get(list_ze_counters))
        .route("/ze_counters", post(create_ze_counter))
        .route("/ze_counters/:id", get(get_ze_counter))
        .route("/ze_counters/:id", put(update_ze_counter))
        .route("/ze_counters/:id", delete(delete_ze_counter))
        
        // ZeParameter routes
        .route("/ze_parameters", get(list_ze_parameters))
        .route("/ze_parameters", post(create_ze_parameter))
        .route("/ze_parameters/:id", put(update_ze_parameter))
        .route("/ze_parameters/:id", delete(delete_ze_parameter))
        
        // ZeMeasurement routes
        .route("/ze_measurements", get(list_ze_measurements))
        .route("/ze_measurements", post(create_ze_measurement))
        .route("/ze_measurements/:id", delete(delete_ze_measurement))
        
        // Computation endpoint
        .route("/compute/chi_ze", post(compute_chi_ze))
        
        // Health check
        .route("/health", get(health_check))
}