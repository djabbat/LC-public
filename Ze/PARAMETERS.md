# Ze · PARAMETERS

**Status:** Canonical numerical parameters · regenerated 2026-04-28
**Authority:** Any change here without re-running F1–F6 (see `THEORY §7`) is invalid.

---

## §1. Theoretical constants (dimensionless)

| Symbol | Default | Range | Meaning | Source |
|--------|--------:|-------|---------|--------|
| `α` | `1.0` | `(0, ∞)` | Coupling between impedance and proper-time consumption rate. Sets the unit of `τ_Ze`. | Postulate (THEORY §2.2). Numerical value chosen by convention; no experiment fixes it. |
| `β` | `1.0` | `(0, ∞)` | Coefficient in `C(τ) = C₀ exp(−β·I·τ)`. Depends on state-space geometry. | THEORY §4. |
| `δ` | `0.0` | `[0, 0.5]` | Ze deformation parameter for CHSH. `δ = 0` = standard QM. | THEORY §3. Source §4.3, §8.2. |
| `Λ_Ze` | `1.0` | `(0, ∞)` | Impedance scale parameter. Enters `δ ∝ (∇I)² / Λ_Ze²`. | THEORY §3.2. |
| `C₀` | `1.0` | `(0, ∞)` | Initial correlation amplitude. | THEORY §4. |

Default values are chosen so that all five canonical quantities are `O(1)` for `I = 1`, `τ = 1`. This is purely a sanity-of-output convention; the *meaningful* outputs are scale-invariant ratios (e.g. `S_Ze − 2√2`).

---

## §2. CHSH evaluation

| Parameter | Default | Range | Meaning |
|-----------|--------:|-------|---------|
| `chsh.opt_method` | `"nelder_mead"` | `nelder_mead`, `lbfgs`, `grid` | Optimizer for measurement-angle search. `grid` is for F3 verification only. |
| `chsh.grid_n` | `64` | `[16, 256]` | Resolution of polar angle grid in `grid` mode (CHSH lives on `S²`, so total grid is `n² × n²`). |
| `chsh.tolerance` | `1e-8` | `[1e-12, 1e-4]` | Convergence tolerance for optimizer. |
| `chsh.expected_const` | `1.7478` | (fixed) | The constant in `S_Ze = 2√2 + δ·1.7478`. F3 must reproduce this from grid optimization to within `1e-4`. |

---

## §3. Proper-time integrator

| Parameter | Default | Range | Meaning |
|-----------|--------:|-------|---------|
| `pt.method` | `"rk4"` | `rk4`, `euler` | Integration method for `dτ/dt = −α·I(t)`. RK4 is canonical. Euler is for F2 baseline. |
| `pt.dt` | `1e-3` | `[1e-6, 1e-1]` | Time step. Smaller = more accurate, slower. |
| `pt.t_max` | `10.0` | `[0.1, 1e3]` | Default time horizon. |

---

## §4. Reaction–diffusion and other deferred primitives

Not part of canonical simulator. Their previous parameters from CLAUDE.md (`σ`, `λ`, `I_max`) are **archived**. If/when reintroduced as a demo of impedance gradients in a 1D spatial setting, parameters will live in `_archive/` or in a separate sub-directory and will be explicitly labeled "demo, not canonical."

---

## §5. Numerical safety

| Constant | Value | Rationale |
|----------|------:|-----------|
| `LOG_EPS` | `1e-30` | Floor for `log(p)` to avoid `−inf` when `p ≈ 0`. |
| `KL_NORMALIZE` | `true` | Re-normalize input distributions to sum to 1 (within `1e-12`) before computing KL. Reject if normalization fails. |
| `BTAU_LIMIT` | `1.0` | Maximum value of `β·I·τ` before the QFI formula returns "extrapolation regime — refused" instead of a number. |
| `RNG_SEED` | `20260428` | Master seed for any stochastic routine (probability-distribution sampling in tests). YYYYMMDD of regeneration. |

---

## §6. API defaults

| Parameter | Default | Meaning |
|-----------|--------:|---------|
| `backend.port` | `4001` | axum HTTP port. Locked by CLAUDE.md. |
| `backend.host` | `127.0.0.1` | Loopback only. |
| `backend.cors_origin` | `http://127.0.0.1:4000` (dev) | Phoenix dev origin. Production must whitelist explicitly (CLAUDE.md). |
| `backend.timeout_ms` | `5000` | Per-request timeout. CHSH grid mode at `n=256` may exceed this; use `nelder_mead` for production. |
| `backend.max_body_kb` | `64` | Reject larger payloads. Distributions of length > 1024 are out of scope. |

---

## §7. Phoenix LiveView defaults

| Parameter | Default | Meaning |
|-----------|--------:|---------|
| `phoenix.port` | `4000` | Locked by CLAUDE.md. |
| `phoenix.simulation_debounce_ms` | `200` | Debounce slider input. |
| `phoenix.plot_resolution` | `200` | Number of points per plotted curve. |
| `phoenix.tau_max_default` | `5.0` | Default x-axis upper bound for `C(τ)` and `F_Q(τ)` plots. |

---

## §8. Source-document anchors for these parameters

| Parameter | docx § |
|-----------|--------|
| `α` | §2.2 |
| `β` | §5.2 (introduced in derivation; numerical value not fixed by source) |
| `δ` | §4.3, §8.2 |
| `Λ_Ze` | §4.3 |
| `1.7478` | §4.4 (Lemma C) and §8.2 |
| `C₀` | §5.2, §5.3 (introduced in correlation-decay derivation) |
