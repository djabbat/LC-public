# STATE — CEDAR

> **📄 Статьи и публикации:** см. `~/Desktop/Services/docs/SUBMISSIONS_STATUS.md`
> **📄 Стратегия переписывания:** `~/Desktop/Services/docs/REWRITE_STRATEGY_2026-07-16.md`

**Purpose:** volatile state, active TODOs, milestones.
**Convention:** new entries at the top with date.

---

## 📚 2026-07-16 — BioEssays desk reject + стратегия переписывания

- 🔴 **BioEssays `4799098` — desk reject 15 июл (< 24 ч).** Причина: pre-submission inquiry не отправлен, статья слишком большая для Problems & Paradigms.
- ✅ **Post-mortem записан** в MEMORY.md
- 📄 **Стратегия переписывания:** Centriole Elimination (792 строки) → разделить на 3 статьи:
  - **A.** CEDAR hypothesis (3000 слов) → Differentiation
  - **B.** 13-group experimental design → Cell Cycle
  - **C.** CAMC molecular model → BioSystems
- 🟢 **npj Aging `2e8466c7` — Peer Review** (с 12 июн, активность 6 июл) — **ЖДАТЬ.**
- 🟡 **MCARA Biogerontology `7cc6de62`** — апелляция подана 15 июл.
- 📄 **Препринты:** v1.0 `rs-10309814`, v2.0 `rs-10320333` — оба на Research Square.

---

## 📚 2026-07-10 — Сабмит в BioEssays (← устарело, см. выше)

- Препринт Research Square `rs-10309814` — ✅ DOI получен
- Сабмит BioEssays → 🔴 desk reject 15 июл
- Centrioles в npj Aging `2e8466c7` — Peer Review

---

## 📚 2026-07-09 — Итог сессии

- 🔑 **Фундаментальный принцип (Dzhaba):** polyGlu = компенсаторный маркер накопленной энтропии (не её механизм). Асимметричное наследование — элемент механизма необратимой дифференцировки. Накопление энтропии в СК — плата за возможность дифференцировки.
- 🔴 Peer Review v2: 55 PMID, 6 off-topic исправлены
- 🧬 M1-M9, M3/CASID (5 evidence), SPEM гипотеза
- 📊 Оценка: 7.5/10

---

## Current Status (2026‑04‑25)

- **Version:** v5.3 (Counter #1 framing, team/budget added, Sobol paradox resolved via coupling)
- **Status:** C2 confirmed in mammals (2 cell types). Blocking barrier — C1+C2 in HSC.
- **Metrics:** in‑sample R²(MCAI)=0.745; LOO‑CV mean=‑0.093 (requires ROS equation fix and calibration with coupling).
- **Submission readiness:** Longevity Impetus LOI (deadline 2026‑04‑25) — COMPLETE. TEAM_AND_BUDGET.md created, counter‑argument added, Sobol coupling resolution presented, ¬R argument strengthened, sample size and risk matrix added.
- **Next milestone:** Experimental start (Aim 1) – purchase animals and antibodies (Month 1‑2).

---

## Active TODOs (CONCEPT↔CODE mismatches, audit 2026‑04‑25)

### L1 — ✅ MOSTLY RESOLVED (v5.3)
- α_HSC = 0.0082 ✅ (Round‑7 MCMC posterior, fitted)
- ν_HSC = 1.2/yr ✅
- β_HSC = 0.005 (additive cell_dt_cli) ✅
- τ_protection = 24.3 ✅
- π_0 = 0.87 ✅
- π_baseline = 0.10 ✅
- **Updated parameter:** `r_ep` (0.045) replaced by `ep_rate_base` (0.01, from MCMC pilot) and `k_ep` (0.8, from analytical coupling). This will be finalized after Cell‑DT v4.0 calibration.

### L2 — Rename `pi_baseline` → `pi_base` (still pending)
Cross‑crep rename, ~30 refs including tests. Scheduled for v4.0 refactor.

### L3 — Document two damage equations (resolved)
Cell‑DT v4.0 will unify the additive and multiplicative forms using the damage‑integral formulation (see THEORY.md §3.3).

### L4 — P1..P10 prediction test harness
Created `predictions_P1_to_P10.rs` with stubs (v3.0). Tests will be implemented as experimental data become available.

### L6 — `cedar_coupling` Sobol range
Updated coupling parameters: `γ_epi` range [0, 0.05] (still zero default). Coupling k_ep range [0.5 – 2.0] (to be calibrated).

### L7 — Python ↔ Rust name map
Will be generated after v4.0 refactor.

### L8 — ABL‑2 disclosure
Added to CONCEPT.md §ABL‑2 with resolution statement.

### L9 — Counter numbering
Unified “Counter #1 (Centriolar)” across all files. ✅

---

## Milestones

### v5.3 — Counter #1 framing + Grant Submission ✅ 2026‑04‑25
- [x] TEAM_AND_BUDGET.md created with full budget and PI track record
- [x] Counter‑argument to “consequence only” alternative added in CONCEPT.md
- [x] Sobol paradox resolved via coupling model (theoretical)
- [x] ¬R argument strengthened with deglutamylase decline evidence
- [x] Sample size calculation and risk matrix added to EVIDENCE.md
- [x] Pre‑registration plan with formal power analysis
- [x] Confirmation bias section added (no contradictory studies found)
- [x] All files updated for consistency

### v6.0 — Cell‑DT v4.0 with coupling (planned 2026‑08)
- [ ] Implement ep_age(t) = ep_rate_base × t + k_ep × ∫D dτ
- [ ] Repeat Sobol analysis on full ODE
- [ ] Calibrate ep_rate_base and k_ep on literature data

---

## Decision Log

### 2026‑04‑25 — Grant submission package updated
Added sample size, risk matrix, strengthened ¬R, and confirmation bias section. Ready for Longevity Impetus LOI.

### 2026‑04‑22 — CORRECTIONS canon (unchanged)

---

## What NOT to do

[Same as v5.2 – plus: do not claim preliminary data that does not exist; the proposal is explicit about lacking own data.]

## Startup Checklist

1. Read CONCEPT v5.3 + latest Decision Log
2. Ensure TEAM_AND_BUDGET.md is attached to submission
3. Prepare administrative documents for Ilia State University IACUC approval
