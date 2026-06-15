# LC · PARAMETERS

**Status:** Canonical numerical/operational parameters · 2026-04-28 (CONCEPT v5.6)

---

## §1. Cross-subproject constants

**`v*` canonical convention (decided 2026-05-07):** Article form.
- Conversion: `Article = 2 · Python − 1` (linear bijection).
- Python form retained as **internal helper** in Ze theorem proofs and
 Ze/Python code; Article form is authoritative for cross-subproject
 data exchange, manuscripts, grants, and external APIs.
- The values below refer to **`v*_active`** (empirical / dynamic
 optimum, see `Ze/CONCEPT § 8`), NOT `v*_passive = 1 − ln 2 ≈ 0.3069`
 which is a separate theoretical quantity (Shannon-optimal point of
 the passive counter, see `Ze/THEORY § 3.3`).

| Symbol | Value (Article) | Equivalent (Python) | Source | Used by |
|--------|----------------:|--------------------:|--------|---------|
| `v*_active` canonical | `−0.08738` | `0.45631` | `Ze/THEORY §3.3` Theorem 1 (active agent regime) | BioSense, χ_Ze, Ze API |
| `v*_active` empirically tested | `−0.098` (95% CI `−0.114`…`−0.082`) | `0.451` (CI `0.443`-`0.459`) | All-of-Us N=500 swept-v* search (article §4.4) | falsification protocol |
| Sensitivity range `v*_active` | `[−0.36, 0.16]` for `k_λ ∈ [0.5, 2.0]` | `[0.32, 0.58]` | article §4.4 | falsification |
| `v*_passive` (theorem) | `−0.3862` | `0.3069 = 1 − ln 2` | `Ze/THEORY §3.3` Theorem 1 (passive counter) | Ze internal proofs |
| Tsirelson bound | `2√2 ≈ 2.828` | QM | Ze CHSH |
| CHSH deformation const | `1.7478` | `Ze/THEORY §3.4` Lemma C | Ze, F3 test |
| `α` (Ze coupling) | `1.0` (convention) | `Ze/PARAMETERS §1` | Ze proper-time |
| `β` (Ze decay) | `1.0` (convention) | `Ze/PARAMETERS §1` | Ze correlation |
| `δ` (CHSH deformation) | input parameter | `Ze/PARAMETERS §1` | Ze CHSH |

## §2. χ_Ze composition weights

| Modality | Weight (default, post-hoc pilot) | Status |
|----------|----:|--------|
| EEG | `0.30` | post-hoc; not theory-fixed |
| HRV | `0.30` | post-hoc |
| Respiration | `0.20` | post-hoc |
| Sleep | `0.20` | post-hoc |

⚠ **Weights are post-hoc fits on underpowered pilot (N=150)**. NOT theory-derived. Re-fit required on pre-registered cohort N≥2000.

## §3. CDATA bridge constants (status: inconclusive)

| Symbol | Default | Status |
|--------|--------:|--------|
| `a` | `0.05` | underpowered fit (5 params on N=196) |
| `b` | `1.20` | underpowered |
| `c` | `0.40` | underpowered |
| `g₀` | `0.95` | underpowered |
| `g₁` | `1.10` | underpowered |

⚠ Bridge **moved to Supplementary** in article v5; 5 params / N=196 = 39 obs/param < Harrell standard 10/param. Defaults illustrative only.

## §4. Privacy stack (FCLC + BioSense)

### §4.1 FCLC (federated)

| Symbol | Value | Source |
|--------|------:|--------|
| `σ` (noise multiplier) | `1.5` | `FCLC/PARAMETERS` v13.4 |
| `q` (Poisson sampling) | `0.013` | `FCLC/PARAMETERS` |
| `T` (rounds) | `5` (canonical pilot) | `FCLC/PARAMETERS` |
| `δ` | `1e-5` | standard |
| Per-round ε | `≤ 1.0` | RDP envelope |
| `ε_total` | `≈ 0.43` (or `≈ 2.4` after BioSense composition — see §4.3) | Mironov 2017 + Wang/Balle 2019 |
| Krum Byzantine threshold | `≤ 25%` malicious clients | `FCLC/aggregation/krum.rs` |

### §4.2 BioSense (on-device)

| Symbol | Value | Notes |
|--------|------:|-------|
| `ε_local` (per device, daily release) | `2.0` | `BioSense/PARAMETERS §5` |
| `Δf` (sensitivity of χ_Ze, 2-decimal rounded) | `0.3` | `BioSense/PARAMETERS §5` |
| `k` (k-anonymity) | `≥ 7` | `BioSense/PARAMETERS §5` |
| `secagg_min` | `3` | `BioSense/PARAMETERS §5` |
| `daily_release_cap` | `1` | derived |

### §4.3 Composed ε (via Rényi α=2 composition, Mironov 2017)

`ε_total ≈ ε_local ⊕ ε_federated ≈ 2.4` (article v5 §6 corrected; was `0.43` due to double-counting which has been fixed).

## §5. Falsifiability thresholds (M4 operational)

| Quantity | Threshold | Source | Status |
|----------|-----------|--------|--------|
| `N` (cohort size) | `≥ 2000` | community standard | not yet collected |
| `α` (significance) | `0.001` | per challenge | applies to all M-axiom tests |
| Partial r² for all-cause mortality (per counter, controlling age + sex) | `< 0.05` → counter falsified | community standard | not yet tested |
| Power | `80%` for R² = 0.3 → N=1875 | derived | informational |

## §6. Hardware reference (BioSense — informational)

| Module | Spec |
|--------|------|
| MCU | Nordic nRF52840, ARM Cortex-M4 |
| EEG | ADS1299 dry electrodes Fp1/Fp2/Fpz, 128 Hz |
| HRV | PPG MAX30105, 400 Hz, RR-interval, LF/HF, hysteresis δ=0.10 |
| Respiration | Impedance pneumography |
| Sleep | Overnight C3-C4 EEG, spindle detection |
| Update cadence | χ_Ze every 10 min EEG, 5 min HRV/resp, nightly sleep |

## §7. Social layer ports

| Service | Dev port | Prod port | Notes |
|---------|---------:|----------:|-------|
| `server/` (Rust axum) | `8080` | `8080` (loopback) | Loopback only by default |
| `web/` (React+Vite) | `5173` | static (built) | Vite dev server |
| `realtime/` (Phoenix Channels) | `4500` | `4500` | **moved from 4001 to avoid conflict with Ze backend** |
| Postgres (social) | `5432` | `5432` (docker network) | Separate from FCLC postgres |

⚠ Conflict resolution: subproject backends own `4000-4101`. Social realtime moves to `4500+` range.

## §8. Subproject port matrix (canonical, prod-aligned 2026-05-07)

Production reality на сервере:

| Subproject | Backend (API) | Frontend (Phoenix / static) | nginx host |
|------------|--------------:|----------------------------:|------------|
| Ze | 4401 | 4400 | ze.longevity.ge |
| BioSense | **4502** (decision 2026-05-07; was 4101) | 4501 | biosense.longevity.ge |
| FCLC (server-resident) | 4002 | 4003 | fclc.longevity.ge |
| AIM Phoenix umbrella | 4040 | 4040 | aim.longevity.ge |
| AIM Hive Queen | 8090 | — | hive.longevity.ge |
| AIM LLM router | 8770 | — | (loopback only) |
| MCAOA | (none — landing only) | static `/var/www/mcoa-landing/` | mcoa.longevity.ge |
| CDATA | (none — landing only) | static `/var/www/cdata-landing/` | cdata.longevity.ge |
| (umbrella social) realtime | — | 4500 | app.longevity.ge |
| (umbrella social) server | **4600** (decided 2026-05-08; 8080 held by docker-proxy on production host) | — | app.longevity.ge |
| (umbrella social) web | — | 5173 (dev) / static (prod) | app.longevity.ge |

**Dev port matrix** (laptop):
- AIM Phoenix dev: `4099` (per `scripts/desktop/aim_local_launch.sh`)
- AIM Hive Queen (laptop user-mode): `8090` (`~/.config/systemd/user/aim-hive-queen.service`)

## §9. Numerical safety (cross-subproject)

| Constant | Value | Where |
|----------|------:|-------|
| `LOG_EPS` | `1e-30` | Ze, BioSense |
| `BTAU_LIMIT` | `1.0` | Ze (extrapolation refusal) |
| `MARKOV_P_MIN` | `0.02` | BioSense |
| `MARKOV_P_MAX` | `0.98` | BioSense |
| `RNG_SEED` | `20260428` (YYYYMMDD of regen) | All deterministic routines |

## §10. Versioning

- CONCEPT v5.6 → matches this PARAMETERS file
- Bump to v5.7 if any §1-§5 value changes; downstream code must re-test.
- Subproject parameters override here on internal math; PARAMETERS.md aligns cross-subproject.
