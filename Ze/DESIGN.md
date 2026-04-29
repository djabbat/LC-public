# Ze · DESIGN

**Status:** Canonical architecture · regenerated 2026-04-28
**Authority:** `CONCEPT.md` describes *what*; this file describes *how*. `THEORY.md` is the math; this is the wiring.

---

## §1. Workspace layout

```
~/Desktop/LongevityCommon/Ze/
├── CLAUDE.md                # subproject project-level rules (existing)
├── CONCEPT.md               # canonical concept
├── THEORY.md                # canonical math
├── DESIGN.md                # this file
├── PARAMETERS.md            # numerical defaults + safety bounds
├── MAP.md                   # cross-file consistency map
├── STATE.md                 # current state of subproject
├── EVIDENCE.md              # empirical-support table
├── OPEN_PROBLEMS.md         # known gaps and uncertainties
├── TODO.md                  # phased work plan
├── README.md                # public-facing intro
├── LICENSE                  # MIT (preserved)
├── Cargo.toml               # workspace manifest
├── ze-simulator/            # Rust crate: lib + CLI binary `ze_sim`
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── impedance.rs
│   │   ├── proper_time.rs
│   │   ├── chsh.rs
│   │   ├── correlation.rs
│   │   ├── qfi.rs
│   │   └── bin/ze_sim.rs
│   └── tests/
│       ├── f1_impedance.rs
│       ├── f2_proper_time.rs
│       ├── f3_chsh_constant.rs
│       ├── f4_correlation_decay.rs
│       ├── f5_qfi_consistency.rs
│       └── f6_qfi_proportionality.rs
├── ze-backend/              # Rust crate: axum HTTP API on :4001
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── routes.rs
│       └── error.rs
└── ze-web/                  # Phoenix LiveView app on :4000
    ├── mix.exs
    ├── config/
    ├── lib/ze_web/
    │   ├── application.ex
    │   ├── endpoint.ex
    │   ├── router.ex
    │   ├── live/simulator_live.ex
    │   └── components/plots.ex
    └── assets/
```

The previous Phoenix app under `frontend/` and the previous Rust workspace are archived under `_archive/v1_pre_2026-04-28/`. Do not import from there.

---

## §2. Rust simulator API

### §2.1 Public types

```rust
// ze-simulator/src/lib.rs
pub mod impedance;
pub mod proper_time;
pub mod chsh;
pub mod correlation;
pub mod qfi;

pub use impedance::Distribution;
pub use proper_time::{ProperTimeIntegrator, IntegratorMethod};
pub use chsh::{ChshDeformation, ChshOptimizer};
pub use correlation::CorrelationDecay;
pub use qfi::{QfiBound, QfiResult};
```

### §2.2 Impedance

```rust
pub struct Distribution {
    probs: Vec<f64>,
}
impl Distribution {
    pub fn new(probs: Vec<f64>) -> Result<Self, ZeError>;       // normalizes, validates
    pub fn kl_to(&self, model: &Distribution) -> f64;            // I(self ‖ model)
}
```

Quantum case (density matrices) is `Distribution`-agnostic: `pub fn relative_entropy(rho: &nd::Array2<f64>, sigma: &nd::Array2<f64>) -> f64`. (Real symmetric matrices only; complex Hermitian deferred to Phase 2.)

### §2.3 Proper time

```rust
pub trait ImpedanceFn: Fn(f64) -> f64 + Send + Sync {}
pub struct ProperTimeIntegrator { alpha: f64, method: IntegratorMethod, dt: f64 }
impl ProperTimeIntegrator {
    pub fn integrate(&self, i_of_t: impl ImpedanceFn, t_max: f64, tau_0: f64) -> Vec<(f64, f64)>;
}
```

Returns trajectory `[(t, τ_Ze(t))]`. `IntegratorMethod = Rk4 | Euler`.

### §2.4 CHSH

```rust
pub struct ChshDeformation { delta: f64 }
impl ChshDeformation {
    pub fn correlation(&self, a: [f64; 3], b: [f64; 3]) -> f64;        // E_Ze(a, b)
    pub fn s_value(&self, angles: ChshAngles) -> f64;                   // CHSH at given settings
    pub fn s_optimal(&self, optimizer: ChshOptimizer) -> f64;           // S_Ze max
}
```

`ChshOptimizer` is an enum: `NelderMead { tol: f64 }` | `Grid { n: usize }`. F3 uses `Grid`.

### §2.5 Correlation decay

```rust
pub struct CorrelationDecay { c0: f64, beta: f64, impedance: f64 }
impl CorrelationDecay {
    pub fn at(&self, tau: f64) -> f64;
    pub fn lgi_k(&self, tau: f64) -> f64;             // 2C(τ) − C(2τ)
}
```

Refuses (returns `Err`) when `β·I·τ > BTAU_LIMIT` (PARAMETERS §5).

### §2.6 QFI

```rust
pub struct QfiBound { c0: f64, beta: f64, impedance: f64, q_squared_expectation: f64 }
pub struct QfiResult {
    pub f_q_lower_bound: f64,
    pub regime: &'static str,                 // "stationary_optimal_tau" | "extrapolation_refused"
    pub tau_used: f64,
}
impl QfiBound {
    pub fn at(&self, tau: f64) -> QfiResult;
    pub fn at_optimal_tau(&self) -> QfiResult;
}
```

---

## §3. CLI binary `ze_sim`

```
ze_sim impedance --real <p1,p2,...> --model <q1,q2,...>
ze_sim proper_time --alpha 1.0 --i 0.5 --t_max 10 --method rk4
ze_sim chsh --delta 0.1 --optimizer grid --n 64
ze_sim correlation --c0 1.0 --beta 1.0 --i 0.5 --tau 0.5
ze_sim qfi --c0 1.0 --beta 1.0 --i 0.5 --tau 0.5
ze_sim qfi-sweep --i_min 0.01 --i_max 1.0 --n 50           # F6 helper
```

Output is JSON to stdout. Errors return non-zero exit code and a JSON error payload to stderr.

---

## §4. HTTP API (`ze-backend`)

Base URL: `http://127.0.0.1:4001`

| Method | Path | Body shape | Returns |
|--------|------|------------|---------|
| `GET`  | `/healthz` | — | `{"status":"ok","version":"..."}` |
| `POST` | `/api/impedance` | `{"real":[...],"model":[...]}` | `{"impedance": f64}` |
| `POST` | `/api/proper_time` | `{"alpha":f64,"i":f64,"t_max":f64,"dt":f64,"method":"rk4"}` | `{"trajectory":[[t,tau],...]}` |
| `POST` | `/api/chsh` | `{"delta":f64,"optimizer":{"type":"grid","n":64}}` | `{"s_qm":f64,"s_ze":f64,"angles":{...}}` |
| `POST` | `/api/correlation` | `{"c0":f64,"beta":f64,"i":f64,"tau_grid":[...]}` | `{"c":[f64,...]}` |
| `POST` | `/api/qfi` | `{"c0":f64,"beta":f64,"i":f64,"tau":f64}` | `{"f_q_lower_bound":f64,"regime":"...","tau_used":f64}` |
| `POST` | `/api/qfi_sweep` | `{"c0":f64,"beta":f64,"i_grid":[...]}` | `{"f_q":[f64,...],"dtau_dt":[f64,...]}` |

### §4.1 Error model

All errors return JSON of the form

```json
{"error":{"code":"E_NORMALIZATION","message":"distribution does not normalize within 1e-12","field":"real"}}
```

with HTTP status `400` for client errors and `500` (with code `E_INTERNAL`) for internal panics. Never `.unwrap()` in handlers; use `?` and convert via `From<ZeError> for ApiError`.

### §4.2 CORS

Dev: `Access-Control-Allow-Origin: http://127.0.0.1:4000`.
Prod: empty list until explicitly configured (CLAUDE.md rule).

---

## §5. Phoenix LiveView (`ze-web`)

### §5.1 Pages

- `/` — landing with three plots: `C(τ)` decay, `S_Ze` vs `δ`, `F_Q` vs `|dτ_Ze/dt|`. All three are computed live from sliders.
- `/about` — short HTML rendering of `CONCEPT.md` summary §1.

### §5.2 LiveView module

`Ze.Web.SimulatorLive` (file: `lib/ze_web/live/simulator_live.ex`):

State:
```elixir
%{
  alpha: 1.0, beta: 1.0, delta: 0.0, c0: 1.0,
  i: 0.5, tau_max: 5.0,
  results: %{decay: [...], s_ze: ..., qfi_curve: [...]},
  loading: false
}
```

Events:
- `phx-change="update_param"` on each slider → updates state, debounced 200 ms (PARAMETERS §7).
- After debounce → `Task.async` → POST to `http://127.0.0.1:4001/api/...` → `handle_info({ref, result}, ...)` → assign results.

### §5.3 Plot rendering

Charts are SVG, rendered server-side in a `Plots` component (no client JS chart library in Phase 1; Plotly etc. is Phase 2 if needed). Three plots:

1. **Correlation decay**: `C(τ) = C₀·exp(−β·I·τ)` over `[0, τ_max]`.
2. **CHSH parameter**: `S_Ze` vs `δ` for `δ ∈ [0, 0.5]`.
3. **QFI sweep**: `F_Q,max` vs `|dτ_Ze/dt|` for `I ∈ [0.01, 1]`.

---

## §6. Build, run, test

### §6.1 Rust

```bash
cd ze-simulator && cargo build --release && cargo test
cd ../ze-backend && cargo build --release
cd .. && cargo run -p ze-backend            # serves on :4001
```

### §6.2 Phoenix

```bash
cd ze-web && mix deps.get && mix compile
mix phx.server                             # serves on :4000
```

### §6.3 Integration smoke test

With both servers up:

```bash
curl -fsS http://127.0.0.1:4001/healthz
curl -fsS -X POST http://127.0.0.1:4001/api/impedance \
  -H "content-type: application/json" \
  -d '{"real":[0.5,0.5],"model":[0.4,0.6]}'
curl -fsS http://127.0.0.1:4000 | grep -i "ze theory"   # LiveView landing renders
```

---

## §7. Concurrency / threading

The simulator is pure-CPU; the backend uses `tokio` multi-threaded runtime; CHSH grid mode at `n=256` is parallelized via `rayon` (used inside `ze-simulator`, not from the backend layer). The `qfi_sweep` endpoint uses `rayon::par_iter` over the impedance grid.

No global mutable state. No `lazy_static!` / `OnceLock` for anything that affects results.

---

## §8. Observability

- Structured logs via `tracing` in both Rust crates (JSON format in prod).
- Log levels: handlers `INFO`; numerical primitives `DEBUG`; F-tests `TRACE`.
- Phoenix uses default Logger; LiveView events log at `:debug`.

No metrics/Prometheus in Phase 1.

---

## §9. Security

- Backend is loopback-only by default. If exposed publicly: behind a reverse proxy with basic auth and rate limiting (out of Phase 1 scope; flagged in `OPEN_PROBLEMS.md`).
- Input length limits enforced server-side (PARAMETERS §6).
- No persistence; no DB; no secrets needed for Phase 1.
