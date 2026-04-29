# BioSense · PARAMETERS

**Status:** Canonical numerical parameters · regenerated 2026-04-28

---

## §1. Theoretical fixed point

| Symbol | Default | Range | Meaning | Source |
|--------|--------:|-------|---------|--------|
| `v*` | `0.45631` | (fixed by theory) | Optimal Ze velocity (variational extremum) | THEORY §3.3, source §2.1.6 |
| `k_λ` | `1.0` | `(0.5, 2.0)` | Dimensionless constant in `λ = T·k_λ` | THEORY §3.2 |
| `T` | `1.0` | `(0, ∞)` | Effective temperature (units: thermal). Convention only. | source §2.1.4 |

---

## §2. χ_Ze composition weights

| Modality | Weight (default) | Source |
|----------|----:|--------|
| `w_EEG`   | `0.30` | source §3.3 |
| `w_HRV`   | `0.30` | source §3.3 |
| `w_resp`  | `0.20` | source §3.3 |
| `w_sleep` | `0.20` | source §3.3 |

Sum = 1.0. Modifying weights requires re-fitting the bridge constants `(g_0, g_1)`.

---

## §3. Bridge constants (Lemma D — `A(t) = a + b·D + c·D² + ε`, `χ_Ze = g_0 − g_1·A + η`)

| Symbol | Default | Range | Meaning |
|--------|--------:|-------|---------|
| `a`   | `0.05` | `[0, 0.5]` | Asymptotic baseline disease activity at `D = 0` |
| `b`   | `1.20` | `(0, 5.0]` | Linear sensitivity of A to centriolar damage |
| `c`   | `0.40` | `[0, 2.0]` | Quadratic sensitivity (super-linear regime) |
| `g_0` | `0.95` | `(0.5, 1.0]` | χ_Ze ceiling near `D = 0` |
| `g_1` | `1.10` | `(0, 3.0]` | Sensitivity of χ_Ze to A |

**These are NOT theory-fixed** — they are exposed for cohort fits. Defaults come from the article's pilot fit (N=150, α=0.00025 underpowered; treat as illustrative).

---

## §4. Exacerbation classifier (computation 5)

| Symbol | Default | Meaning |
|--------|--------:|---------|
| `β_0`     | `−0.4` | Intercept |
| `β_age`   | `0.025` | Slope per year |
| `β_sex`   | `0.10` | Female = 0, Male = 1 (illustrative) |
| `β_chi`   | `−2.5` | Slope on χ_Ze (negative — lower χ_Ze ↑ risk) |
| `β_dchi`  | `−1.8` | Slope on 7-day Δχ_Ze |
| `window_days` | `7` | Δχ_Ze window |
| `horizon_days` | `30` | Forecast horizon |

---

## §5. Privacy stack

| Symbol | Default | Meaning | Source |
|--------|--------:|---------|--------|
| `eps` (ε) | `2.0` | DP budget per daily release | source §3.4 |
| `delta` (δ) | `1e-5` | DP slack | source §3.4 |
| `Δf` | `0.3` | Sensitivity of χ_Ze (2-decimal rounded over a day) | source §3.4 |
| `k`        | `7` | k-anonymity threshold | source §3.4 |
| `secagg_min` | `3` | Minimum participants per secure-aggregation round | source §3.4 |
| `daily_release_cap` | `1` | Max DP releases per device per day | derived |
| `composition_horizon_days` | `100` | RDP composition horizon for budget audit | source §3.4 |

---

## §6. Numerical safety

| Constant | Value | Rationale |
|----------|------:|-----------|
| `LOG_EPS` | `1e-30` | Floor for `log(p)` |
| `MARKOV_P_MIN` | `0.02` | Reject Markov inputs with `p < MARKOV_P_MIN` (small-correlation expansion breaks) |
| `MARKOV_P_MAX` | `0.98` | Reject `p > MARKOV_P_MAX` (same reason) |
| `RNG_SEED` | `20260428` | Master seed for stochastic routines |
| `BIN_WIDTH_HRV_S` | `0.001` | RR-interval binarisation width |

---

## §7. Hardware reference (firmware out of scope; informational only)

| Module | Spec | Source |
|--------|------|--------|
| MCU | Nordic nRF52840, ARM Cortex-M4, 64 MHz, Rust firmware | source §3.3 |
| EEG | ADS1299 front-end; dry Ag/AgCl Fp1/Fp2/Fpz; 128 Hz; 25–35 Hz Ze-band | source §3.3 |
| HRV | PPG MAX30105; 400 Hz; RR-interval extraction; LF/HF spectral; hysteresis δ=0.10 | source §3.3 |
| Respiration | Impedance pneumography; tidal volume derivative binarisation | source §3.3 |
| Sleep | Overnight EEG C3-C4; spindle detection | source §3.3 |
| Update cadence | χ_Ze every 10 min (EEG), every 5 min (HRV/resp), nightly (sleep); prognoses daily 06:00 local | source §3.3 |

These specs do not appear in the simulator API — they are the article's hardware reference. The simulator accepts already-symbolised binary streams.

---

## §8. API defaults

| Parameter | Default | Meaning |
|-----------|--------:|---------|
| `backend.port` | `4101` | axum HTTP port (offset +100 from Ze's 4001 to avoid collision) |
| `backend.host` | `127.0.0.1` | Loopback only |
| `backend.cors_origin` | `http://127.0.0.1:4100` | Phoenix dev origin |
| `backend.timeout_ms` | `5000` | Per-request timeout |
| `backend.max_body_kb` | `256` | Max payload |

---

## §9. Phoenix LiveView defaults

| Parameter | Default | Meaning |
|-----------|--------:|---------|
| `phoenix.port` | `4100` | LiveView UI |
| `phoenix.simulation_debounce_ms` | `200` | Slider debounce |
| `phoenix.plot_resolution` | `200` | Points per plotted curve |
| `phoenix.markov_default_p` | `0.45631` | Default `p` (= v*, illustrates the fixed point) |
