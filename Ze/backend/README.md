# Ze Backend

Ze Theory backend implementation for CommonHealth project. Provides REST API for managing Ze synchronization counters, parameters, and measurements.

## Features

- **ZeCounter Management**: CRUD operations for Ze counters
- **Parameter Storage**: Global and counter-specific theory parameters
- **Measurement Storage**: Store Ze measurements with automatic χ_Ze computation
- **Computation Endpoint**: Compute χ_Ze given v and v* values
- **PostgreSQL Database**: Full relational schema with migrations
- **Production-ready**: Error handling, tracing, graceful shutdown

## API Endpoints

### ZeCounters
- `GET /api/ze_counters` - List counters with pagination
- `POST /api/ze_counters` - Create new counter
- `GET /api/ze_counters/:id` - Get counter by ID
- `PUT /api/ze_counters/:id` - Update counter
- `DELETE /api/ze_counters/:id` - Delete counter

### ZeParameters
- `GET /api/ze_parameters` - List parameters (filter by ze_counter_id)
- `POST /api/ze_parameters` - Create parameter
- `PUT /api/ze_parameters/:id` - Update parameter
- `DELETE /api/ze_parameters/:id` - Delete parameter

### ZeMeasurements
- `GET /api/ze_measurements` - List measurements (filter by ze_counter_id)
- `POST /api/ze_measurements` - Create measurement (χ_Ze auto-computed)
- `DELETE /api/ze_measurements/:id` - Delete measurement

### Utilities
- `POST /api/compute/chi_ze` - Compute χ_Ze from v and v*
- `GET /api/health` - Health check endpoint

## Database Schema

### ze_counters
- `id` UUID (primary key)
- `name`, `description` - Optional identifiers
- `initial_tau_z` - Initial τ_Z value (default: 200)
- `theta_z` - θ_Z prediction threshold (default: 0.30)
- `hilbert_dimension` - dim(H) (default: 2)
- `created_at`, `updated_at` - Timestamps

### ze_parameters
- `id` UUID (primary key)
- `ze_counter_id` - Optional foreign key (NULL for global params)
- `parameter_name`, `parameter_value`, `parameter_unit` - Parameter data
- `description` - Optional description
- `created_at`, `updated_at` - Timestamps

### ze_measurements
- `id` UUID (primary key)
- `ze_counter_id` - Required foreign key
- `measurement_time` - When measurement was taken
- `v` - Synchronization rate N_S/(N-1)
- `n_s`, `n` - Event counts
- `v_star_passive`, `v_star_active` - Optimal values used
- `chi_ze` - Computed χ_Ze = 1 - |v - v*| / max(v*, 1-v*)
- `tau_z` - Current τ_Z value
- `created_at` - Timestamp

## Getting Started

1. **Clone and setup environment**:
```bash
cp .env.example .env
# Edit .env with your database credentials
```

2. **Run database**:
```bash
docker run --name ze-db -e POSTGRES_USER=cn -e POSTGRES_PASSWORD=cn -e POSTGRES_DB=ze_db -p 5432:5432 -d postgres:15
```

3. **Run migrations and server**:
```bash
cargo run
```

## Configuration

Environment variables:
- `PORT` - Server port (default: 3009)
- `DATABASE_URL` - PostgreSQL connection string (default: postgres://cn:cn@localhost/ze_db)

## Technology Stack

- **Rust 2021** - Systems programming language
- **Axum 0.7** - Web framework
- **SQLx 0.7** - Async SQL toolkit
- **PostgreSQL 15** - Database
- **Tokio** - Async runtime
- **Tracing** - Structured logging

## Notes

- Implements canonical χ_Ze formula: χ_Ze = 1 - |v - v*| / max(v*, 1-v*)
- Follows CommonHealth CORRECTIONS_2026-04-22: γ_i = 0 by default, no health score aggregation
- All timestamps in UTC
- UUIDs used for all primary keys