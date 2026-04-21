# Ze Frontend

Phoenix 1.7 LiveView frontend for the CommonHealth subproject "Ze" (Counter S - Synchronization in MCOA Framework).

## Features

- **Dashboard LiveView**: Displays Ze parameters, MCOA counter registry, CDATA Sobol sensitivity visualization, and BioSense raw streams
- **Detail LiveView**: Entity-level drill-down with HSC lineage tracking and event history
- **Production-ready**: Telemetry, error handling, graceful degradation
- **CORRECTIONS_2026-04-22 compliant**: No unvalidated biomarker claims, proper v = N_S/(N-1) display

## Requirements

- Elixir 1.14+
- Phoenix 1.7+
- Backend service running at BACKEND_URL (default: http://localhost:3009)

## Installation

1. Clone the repository
2. Install dependencies: `mix deps.get`
3. Set environment variables (optional):
   ```bash
   export BACKEND_URL=http://localhost:3009
   export PORT=4009
   export SECRET_KEY_BASE=$(mix phx.gen.secret)
   ```
4. Start the server: `mix phx.server`

## Architecture

- `ZeFrontendWeb.DashboardLive`: Main dashboard with Ze parameters and MCOA counters
- `ZeFrontendWeb.DetailLive`: Single entity detail view
- `ZeFrontendWeb.BackendClient`: HTTP client for Rust backend API
- Telemetry instrumentation for monitoring

## API Endpoints

The frontend expects the following backend endpoints:

- `GET /api/ze/dashboard` - Dashboard summary data
- `GET /api/mcoa/counters` - MCOA counter registry
- `GET /api/cdata/sensitivity` - CDATA Sobol sensitivity analysis
- `GET /api/ze/entities/:id` - Single entity detail
- `GET /api/cdata/lineage/:id` - HSC lineage data

## Deployment

Build with Docker:

```bash
docker build -t ze-frontend .
docker run -e BACKEND_URL=http://backend:3009 -p 4009:4009 ze-frontend
```

Or deploy to production using the generated release:

```bash
MIX_ENV=prod mix release
_build/prod/rel/ze_frontend/bin/ze_frontend start
```

## License

Part of the CommonHealth research framework.