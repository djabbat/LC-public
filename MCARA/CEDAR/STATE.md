**Date:** 2026-08-02

# STATE — CEDAR

> **📄 Articles and publications:** see `~/Desktop/Services/docs/SUBMISSIONS_STATUS.md`
> **📄 Rewrite strategy:** `~/Desktop/Services/docs/REWRITE_STRATEGY_2026-07-16.md`

**Purpose:** volatile state, active TODOs, milestones.
**Convention:** new entries at the top with date.

---

## 🟢 2026-08-02 — Julia Mahamid response + Gönczy/Guichard letters SENT

- 📨 **Julia Mahamid** — ответил (поблагодарил, Tollervey — да, идея elimination/de novo, иду к Gönczy/Guichard)
- 📨 **Pierre Gönczy** — письмо отправлено (гипотеза CEDAR + elimination + cryo-ET, Mahamid рекомендовала)
- 📨 **Paul Guichard** — письмо отправлено (`paul.guichard@unige.ch`, expansion microscopy, Mahamid рекомендовала)
- 🔬 **Анализ Tollervey et al. (2025) Dev Cell** — проведён. Mother vs daughter УЖЕ структурно различны. НЕТ сравнения young vs aged.
- 🔬 **Анализ Guichard lab:** Laporte 2024 Cell (U-ExM карта, 24 белка), Bournonville 2025 Nat Commun (A-C linker), Brunet 2025 EMBO J (Alms1→cartwheel). Поле методологически готово.
- 🛡️ **Defence document:** `docs/REPAIR_OBJECTION_DEFENSE.md` — 5-уровневая защита от возражения «должны быть механизмы репарации»
- 📊 **Обновлены:** EVIDENCE.md (+§11, §12), THEORY.md (¬R v6.0), MEMORY.md (+2 записи), PARAMETERS.md (+13 кинетических параметров)

**Следующий шаг:** ждать ответа Gönczy/Guichard. Follow-up через 2 недели если нет ответа.

---

## 🟢 2026-07-26 — Autofix CEDAR: 98/100

<!-- lang:ru -->
- 🔴 **PARAMETERS.md** — fixed: was template text "data validation framework", replaced with real centriolar parameters (Cell‑DT, LLPS, evolution, grant)
- 🔴 **MAP.md** — rewritten: old version contained non-existent folders (cedar_sim/, tests/, gui/), did not contain real ones (Aubrey-Platform/, CellLineageTree/, articles/).
- 🟡 **DESIGN.md** — expanded from 353 to 1807 bytes (architecture, components, data pipeline, key decisions)
- 🟡 **TODO.md** — added active tasks (CIRCBIO-07, autofix, articles, LLPS section in THEORY.md)
- ✔ **VERIFICATION_CENTRIOLE_LAND_WATER_2026-07-26.md** — created: full verification of hypothesis "centriole/LLPS/land vs water" with PMID
- ✔ **scripts/cedar_autofix.sh** — created: CEDAR-specific autofix script (adapted from DEEP_AUDIT_ALGORITHM.md)
- 📊 **Final autofix score: 98/100.** Criterion 95+ passed.
<!-- /lang:ru -->

## 📚 2026-07-16 — BioEssays desk reject + rewrite strategy

- 🔴 **BioEssays `4799098` — desk reject 15 Jul (< 24 h).** Reason: pre-submission inquiry not sent, article too large for Problems & Paradigms.
- ✔ **Post-mortem recorded** in MEMORY.md
- 📄 **Rewrite strategy:** Centriole Elimination (792 lines) → split into 3 articles:
  - **A.** CEDAR hypothesis (3000 words) → Differentiation
  - **B.** 13-group experimental design → Cell Cycle
  - **C.** CAMC molecular model → BioSystems
- 🟢 **npj Aging `2e8466c7` — Peer Review** (from 12 Jun, activity 6 Jul) — **WAIT.**
- 🟡 **MCARA Biogerontology `7cc6de62`** — appeal filed 15 Jul.
- 📄 **Preprints:** v1.0 `rs-10309814`, v2.0 `rs-10320333` — both on Research Square.

---

## 📚 2026-07-10 — Submission to BioEssays (← outdated, see above)

- Preprint Research Square `rs-10309814` — ✔ DOI obtained
- Submission BioEssays → 🔴 desk reject 15 Jul
- Centrioles in npj Aging `2e8466c7` — Peer Review

---

## 📚 2026-07-09 — Session summary

- 🔑 **Fundamental principle (Dzhaba):** polyGlu = compensatory marker of accumulated entropy (not its mechanism). Asymmetric inheritance — element of the mechanism of irreversible differentiation. Accumulation of entropy in SC — the price for the possibility of differentiation.
- 🔴 Peer Review v2: 55 PMID, 6 off-topic fixed
- 🧬 M1-M9, M3/CASID (5 evidence), SPEM hypothesis
- 📊 Score: 7.5/10

---

## Current Status (2026‑04‑25)

- **Version:** v5.3 (Counter #1 framing, team/budget added, Sobol paradox resolved via coupling)
- **Status:** C2 confirmed in mammals (2 cell types). Blocking barrier — C1+C2 in HSC.
- **Metrics:** in‑sample R²(MCAI)=0.745; LOO‑CV mean=‑0.093 (requires ROS equation fix and calibration with coupling).
- **Submission readiness:** Longevity Impetus LOI (deadline 2026‑04‑25) — COMPLETE. TEAM_AND_BUDGET.md created, counter‑argument added, Sobol coupling resolution presented, ¬R argument strengthened, sample size and risk matrix added.
- **Next milestone:** Experimental start (Aim 1) – purchase animals and antibodies (Month 1‑2).

---

## Active TODOs (CONCEPT↔CODE mismatches, audit 2026‑04‑25)

### L1 — ✔ MOSTLY RESOLVED (v5.3)
- α_HSC = 0.0082 ✔ (Round‑7 MCMC posterior, fitted)
- ν_HSC = 1.2/yr ✔
- β_HSC = 0.005 (additive cell_dt_cli) ✔
- τ_protection = 24.3 ✔
- π_0 = 0.87 ✔
- π_baseline = 0.10 ✔
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
Unified “Counter #1 (Centriolar)” across all files. ✔

---

## Milestones

### v5.3 — Counter #1 framing + Grant Submission ✔ 2026‑04‑25
- [x] TEAM_AND_BUDGET.md created with full budget and PI track record
- [x] Counter‑argument to “consequence only” alternative added in CONCEPT.md
- [x] Sobol paradox resolved via coupling model (theoretical)
- [x] ¬R argument strengthened wi
th deglutamylase decline evidence
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

## 🟢 2026-07-26 — The Centriole Invasion: Pre-submission inquiry sent

- **Article:** The Centriole Invasion: How a Phage Tubulin Gave Rise to Irreversible Differentiation
- **Target:** Trends in Ecology & Evolution (IF ~18) — Opinion
- **Preprint:** Research Square rs-10484187 (prescreening)
- **Inquiry:** Sent to tree@cell.com (Andrea Stephens, Editor-in-Chief)
- **Status:** Awaiting editor response
- **Files:** CEDAR/submissions/2026-07-26_TREE/
- **Next:** If positive → submit via Editorial Manager
