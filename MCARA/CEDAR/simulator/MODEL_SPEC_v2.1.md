# MODEL SPEC — CEDAR-v2.1 reference implementation

**Version:** 2.1.0-ref · **Date:** 2026-11-16 · **Code:** `cdata_sim/` · **Results:** `results/v21_results.md`, `results/v21_results.json`

## Why this file exists

The published article (Tqemaladze 2026, *Longevity Horizon* 2(5), doi:10.65649/wjtcf387) reports results but does **not** publish the update equations, and the original simulation code is lost:

- `pyproject.toml` and `README.md` point to `https://github.com/djabbat/CEDAR-sim` — **this repository does not exist** (`gh repo list djabbat` returns no such name; an unauthenticated clone is refused).
- The editable install on this machine pointed to `/home/oem/Desktop/LC/MCAOA/CDATA/simulator/cdata_sim` — **the directory no longer exists**.
- Only `simulator/cedar_sim.egg-info/` survived, which reveals the true module layout (`cdata_sim/{model,calibration,regularization,sensitivity}.py`, `tests/test_model.py`).

Consequently `cdata_sim/` here is an **independent re-implementation**, not the author's original code. Every choice the article leaves unspecified is listed below so that a reader can see exactly which freedom was used and how much of the result depends on it.

## What is taken verbatim from the publication

State variables (D, Ms, Mf, A, Nmat, PLK1) and the 23 parameters of §2.2, including µ_P = 0.020, r₀ = 0.040, r_age = 0.002, λ_age = 0.025, k_cat = 0.100, µ_s = µ_f = 0.015, η_s = 0.040, η_f = 0.050, ω = 0.030, α_CEP = 0.050, β₀ = 0.040, α_AurA_215 = 4.0, α_AurA_315 = 3.0, γ_D = 0.005, γ_S = γ_F = 0.015, γ_N = 0.005, D_healthy = 0.3, M_healthy = 0.7, R_recovery = 0.035, K_s = K_f = 1.5, κ = 0.350, κ_D = 0.100, D_senescence = 1.0, σ_N = 0.200, τ = 2.5, A_exit = 0.55.

Also verbatim: additive asymmetry decay with recovery, Michaelis–Menten saturating synthesis ∝ 1/(1 + M/K), dual exit (p53-mediated arrest / replicative exhaustion when A < 0.55), 10,000 cells × 10 seeds for validation.

## Interpretive choices (NOT in the publication)

| # | Choice | Value used | Why it is needed | How much it matters |
|---|---|---|---|---|
| I1 | `decay_scale` — scalar on the asymmetry-decay sum | 2.45 | The table gives γ's but no normalisation of D, so the absolute decay rate is underdetermined | Sets the Hayflick median almost linearly (S1 ≈ 0.2 for mean exit) |
| I2 | `s_D` — damage scale inside the ATM term: `κ_D·(D − D_senescence)/s_D` | 0.0060 | κ_D = 0.1 is too small to cross the p53 threshold with the published D dynamics, so the published numbers imply an unstated normalisation | Controls the ATM-arrest fraction |
| I3 | `q_amp` — per-generation probability of a centrosome-amplification event (modulated by Aurora A, Mf, D) | 0.0079 | The article reports a 52% amplification frequency but publishes no production rate | Sets the amplification frequency |
| I4 | Amplification events **persist** (Nmat is not halved at division) | — | Required to reach the published 52%; with halving, the equilibrium frequency is ≪10% | All amplification- and ATM-related output |
| I5 | `aurA_cv` — log-normal spread of Aurora A across the population | 0.62 | Explains the coexistence of arrested and silenced cells in one population | Sets the split between arrest and silencing |
| I6 | `r_mult_cv` — per-cell heterogeneity of damage production | 0.20 | Cells in the published simulation are not identical; this is the only source of that variance | Sets the IQR |
| I7 | Silencing = `G_215 < 0.5` blocks arrest; blocked cells are labelled `exhaustion_silenced` and are counted inside the exhaustion route | — | v2.0 reports a two-way split (28.5 / 71.5) with no silenced category, so silencing must fold into exhaustion | Route labels only |
| I8 | v2.1 PIDDosome brake: `p_p21 = 1 − exp(−λ_p21 · P · σ(k_P·(Nmat − N_crit,P)) · G_215)` per generation, **no τ delay** | P=1, N_crit,P=2.0, k_P=4.0, λ_p21=0.20 | The published article has no PIDDosome module at all; parameters are new | Dominant lever of the v2.1 result (S1 = 0.94 for mean exit) |
| I9 | v2.1 autophagy edge: LC3B+ cells return to Nmat = 1 with probability `k_auto·p_lc3b`; `p_lc3b = 0.12·(1 + 1.5·max(0, 1 − Mf))` | k_auto = 0.15 | The correction rate is unknown; Coelho & Glover 2026 report the pathway, not a rate | Sets the corrected fraction and the drop in instantaneous amplification |
| I10 | Alisertib arm modelled as Aurora A × 0.35; Tet-On arm as Aurora A × 3 | — | Pharmacological magnitudes are not simulated, only activity scaling | Direction of Prediction C only |

## Reproduction quality (v2.0 baseline, 10 seeds × 10,000 cells)

| Metric | Reference | Published v2.0 | Deviation |
|---|---|---|---|
| Hayflick median, generations | 50.5 ± 0.5 | 48 ± 0.4 | +5.2% |
| Hayflick IQR | 9.5 | 8 | +18.8% |
| p53-mediated route, % | 26.7 | 28.5 | −6.2% |
| Amplification frequency, % | 49.9 | 52 | −4.1% |

**Read this honestly:** the agreement is the result of fitting four free normalisations (I1–I4) to four published targets. It demonstrates that the published numbers are *reachable* by a model with this architecture, and that the implementation is not obviously wrong. It does **not** confirm the original code, and it cannot: the original equations were never published and the code no longer exists.

## What follows from that for v2.1

The v2.1 numbers in `results/v21_results.md` are therefore *indicative*, with the following epistemic status:

- **Robust (direction):** adding an ATM-independent p21 brake increases the fraction of p53-mediated exits and decreases instantaneous centrosome amplification; Aurora A silencing suppresses both brakes; PIDD1/CASP2 knockout returns the partition to the v2.0 baseline.
- **Not robust (magnitude):** every number moves with N_crit,P (dominant), λ_p21, k_auto. Report brackets, not point values, in any manuscript text.
- **Not reproduced:** Prediction B's clause "centrosome number keeps rising in the knockout" holds only marginally (+2 pp in cumulative amplification, 47% → 49%); in this implementation the autophagy edge dominates that comparison. This is a genuine open issue, not a presentation problem.

## Commands

```bash
cd simulator
python3 scripts/run_v21_results.py            # full table, ~50 s
python3 -m pytest tests -q                    # smoke tests
```

## Reproducibility of the software itself

`pyproject.toml` still names `https://github.com/djabbat/CEDAR-sim` as Homepage. That URL must either be created and populated with this directory (recommended — it also repairs the broken data-availability statement of the published article) or removed from the metadata. This is a publication decision and has not been taken.
