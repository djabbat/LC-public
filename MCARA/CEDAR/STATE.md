**Date:** 2026-08-02

# STATE — CEDAR

> **📄 Articles and publications:** see `~/Desktop/Services/docs/SUBMISSIONS_STATUS.md`
> **📄 Rewrite strategy:** `~/Desktop/Services/docs/REWRITE_STRATEGY_2026-07-16.md`

**Purpose:** volatile state, active TODOs, milestones.

## 🔴 2026-08-13 — Meyer protocol integrated (Research Article)

> **Files:** `docs/Testing_the_Hypothesis_of_Centriolar_Entropy_as_a_Source_of_Transcriptomic_Aging_Clock_Signal.md` (+ .docx) — in CEDAR/docs/ and ERR/wp1_cedar/docs/

<!-- lang:ru -->
**Protocol (research article, APA7, 35 verified PMID) added to CEDAR/docs/ and ERR/wp1_cedar/docs/.** Core: testing the hypothesis "centriole — counter → source of stochastic variation in the aging clock." Key components: Phase 0 (U-ExM + SILAC + GEO reanalysis + digital twin), A.0 (p53/DREAM, STOP-rule), A-iv (rescue + chimeric centrioles), B (n=42, MitoQ, hTERT, CRISPR), C (validation), D (iPSC, SILAC isotope). Included Guichard's remark (cartwheel → persistent scaffold). Sent to Meyer (13.08). Candidate for submission as a design article (Nature Protocols / Cell Reports Methods / eLife).
<!-- /lang:ru -->
**Convention:** new entries at the top with date.
---

## 🟢 2026-08-09 — bioRxiv submission: BIORXIV/2026/743702 (Version 1)

<!-- lang:ru -->
- 📄 **Article:** "The Centriole as a Candidate Division Counter in Stem-Cell Aging: A Falsifiable Hypothesis with a Pre-Registered Protocol"
- 🚀 **Submission to bioRxiv:** BIORXIV/2026/743702 — Version 1 SUBMITTED (confirmation received). Awaiting screening (1–3 days) → preprint + DOI 10.1101/...
- 📁 **Files:** `CEDAR/articles/centriole-division-counter-biorxiv/` (md, docx, pdf, bioRxiv pdf, Abstract versions)
- 📄 **Format:** Research Article — hypothesis + systematic synthesis + pre-registered protocol P1–P9 (Stage 1); References in APA 7 (128 entries, all authors disclosed)
- 🔄 **Full cycle:** 18 peer-review reports processed (42–68/100), all revisions incorporated; emojis/CEDAR removed; AI traces eliminated
- ✅ **md2docx converter:** added `--apa` flag (author-year citations, References without numbering), `##`→H2 (no Subtitle), Title 22pt — committed (ff2cb7b5, 4eee16b4)
- 📨 **Confirmation received (Aug 9, 07:00):** bioRxiv confirmed submission of MS `BIORXIV/2026/743702`; screening 24–72 h
- 🔀 **Transfer option:** bioRxiv → "Submit Preprint to a Journal or Peer Review" (`submit.biorxiv.org/submission/queue?queueName=send_paper_away_author`) — direct route to journals/Review Commons without re-upload
- 📌 **Next step:** await screening (DOI 10.1101/...) → consider Review Commons (EMBO, free) or journal via transfer queue → journal-fit + inquiry
<!-- /lang:ru -->

---


---

<!-- lang:ru -->
- De-risking ladder L1–L5 (slow down the counter; segregate damage; hemi-elimination of the maternal centriole; conditioning; selection) precedes any elimination
- Full elimination — only when L1–L5 are exhausted; prognosis: ≥80% survival (hemi) vs <50% (full), Meitinger 2016
- Incorporated into the article MCARA_Four_Counters_Research.md (Rev 82, §4)
<!-- /lang:ru -->

---

## 🟢 2026-08-02 — Julia Mahamid response + Gönczy/Guichard letters SENT

<!-- lang:ru -->
- 📨 **Julia Mahamid** — replied (thanked; Tollervey — yes, the elimination/de novo idea; heading to Gönczy/Guichard)
- 📨 **Pierre Gönczy** — email sent (CEDAR hypothesis + elimination + cryo-ET, recommended by Mahamid)
- 📨 **Paul Guichard** — email sent (`paul.guichard@unige.ch`, expansion microscopy, recommended by Mahamid)
- 🔬 **Analysis of Tollervey et al. (2025) Dev Cell** — completed. Mother vs daughter are ALREADY structurally distinct. NO comparison of young vs aged.
- 🔬 **Analysis of Guichard lab:** Laporte 2024 Cell (U-ExM map, 24 proteins), Bournonville 2025 Nat Commun (A-C linker), Brunet 2025 EMBO J (Alms1→cartwheel). The field is methodologically ready.
- 🛡️ **Defence document:** `docs/REPAIR_OBJECTION_DEFENSE.md` — 5-level defense against the objection "repair mechanisms must exist"
- 📊 **Updated:** EVIDENCE.md (+§11, §12), THEORY.md (¬R v6.0), MEMORY.md (+2 entries), PARAMETERS.md (+13 kinetic parameters)
<!-- /lang:ru -->

<!-- lang:ru -->
**Next step:** wait for response from Gönczy/Guichard. Follow up in 2 weeks if no response.
<!-- /lang:ru -->

---

## 🟢 2026-07-26 — Autofix CEDAR: 98/100

- 🔴 **PARAMETERS.md** — fixed: was template text "data validation framework", replaced with real centriolar parameters (Cell‑DT, LLPS, evolution, grant)
- 🔴 **MAP.md** — rewritten: old version contained non-existent folders (cedar_sim/, tests/, gui/), did not contain real ones (Aubrey-Platform/, CellLineageTree/, articles/).
- 🟡 **DESIGN.md** — expanded from 353 to 1807 bytes (architecture, components, data pipeline, key decisions)
- 🟡 **TODO.md** — added active tasks (CIRCBIO-07, autofix, articles, LLPS section in THEORY.md)
- ✔ **VERIFICATION_CENTRIOLE_LAND_WATER_2026-07-26.md** — created: full verification of hypothesis "centriole/LLPS/land vs water" with PMID
- ✔ **scripts/cedar_autofix.sh** — created: CEDAR-specific autofix script (adapted from DEEP_AUDIT_ALGORITHM.md)
- 📊 **Final autofix score: 98/100.** Criterion 95+ passed.

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

---

## 🟢 2026-08-08 — rDNA clock (TRCS) integrated — CEDAR v4.7

<!-- lang:ru -->
- ⏱️ **Third counter — rDNA clock (TRCS, Huang 2026):** `SenescenceTrigger::RdnDnaShortening` + `rdna_copy_number` in AgingEngine. 45S rDNA decreases with stem cell divisions (unlike telomeres), falling below 0.5 → p53 senescence.
- 🧮 **Parameters:** RDNA_LOSS_PER_DIVISION=0.0006 (HSC 12 divisions/year → threshold ~70 years), RDNA_CRIT=0.5, RDNA_MIN=0.2, intervention `rdna_restoration` (+2%/year, TRCS rejuvenation strategy).
- ✅ **Tests:** 547 pass (workspace); added rDNA tests (decline, threshold, restoration, snapshot).
- 🔗 **Bridge:** centrioles (CEDAR) + telomeres + rDNA (TRCS) = multi-counter architecture (MCARA). Analysis: `~/Desktop/Services/docs/ANALYSIS_CEDAR_v2_vs_Huang_TRCS_2026-08-08.md`
<!-- /lang:ru -->

**Next step:** mirror the rDNA clock in Python cedar-sim; publish the code (GitHub + Zenodo).

---

## 🟢 2026-08-08 — Zenodo DOI obtained for CEDAR v4.7

<!-- lang:ru -->
- 📦 **DOI: 10.5281/zenodo.21852388** (https://doi.org/10.5281/zenodo.21852388) — release v0.4.8-rdna-clock, djabbat/LC-public
- 🏷️ DOI badge added to the repository README (commit 99c8263)
- 🔁 Zenodo-GitHub integration is active — subsequent releases receive DOIs automatically
- 📌 Code citation: Tqemaladze J. (2026). CEDAR v4.7. Zenodo. https://doi.org/10.5281/zenodo.21852388
<!-- /lang:ru -->

**Next step:** mirror the rDNA clock in Python cedar-sim (in the simulator TODO); regenerate the CEDAR-v2 PDF from the corrected md (OJS article 188).

**LERR — Ladder, Eliminate, Reprogram, Rebuild.**

**Step 1 (Ladder).** Cut the damage load first: slow the counter, push old centrioles into differentiating daughters, remove only the mother centriole, keep spare young ones.

**Step 2 (Eliminate).** Take out the old centriole. Restore telomeres. Wipe the epigenome. Rescue mitochondria.

**Step 3 (Reprogram).** Push to totipotency with DUX4 + KDM4D + DPPA3.

**Step 4 (Rebuild).** Grow fresh centrioles de novo. Derive clean, young adult stem cells.
**Step 1 (Ladder).** De-risk before elimination based on current data: slow down the counter (NAC antioxidant; reversible PTMs: TTL re-tyrosination, CCP5/6 deglutamylation); segregate damage via asymmetric inheritance of the mother centriole into differentiating progeny (Yamashita, 2007; Royall, 2023 — human NPCs); hemi-eliminate only the mother centriole (laser/PROTAC), preserving duplication control and avoiding p53-dependent G1 arrest (Meitinger, 2016); condition the cell (reserve PLK4 centrioles, G1/S synchronization, proteostasis); select the least damaged pool (FACS by low Δ2/polyGlu).
**Step 2 (Eliminate).** Remove the old damaged centriole; restore telomeres (telomerase/ZSCAN4 via H3K14ac/H3K18ac; Meltzer, 2024); erase epigenetic marks (OSK/TET1-TET2-TDG; Lu, 2020 — partially, linear memory remains); select healthy mitochondria (PINK1-dependent mitophagy; Vázquez-Martín, 2016).
**Step 3 (Reprogram).** Induce totipotency: DUX4 + KDM4D + DPPA3 — DUX4 opens cleavage-stage genes (Hendrickson, 2017), KDM4D removes the H3K9me3 reprogramming barrier, DPPA3 (Stella) stabilizes the totipotent (2C-like) state.
**Step 4 (Rebuild).** Reassemble young centrioles de novo (PLK4 → SAS-6 → STIL → CPAP; Nigg & Holland, 2018; Gönczy, 2012) after complete elimination (Khodjakov, 2002; Uetake, 2007); control geometry (9-fold symmetry, triplets, length); obtain safe young adult stem cells (karyotype check, p53 restoration).
**Step 1 (Ladder).** De-risk before elimination: slow down the counter, segregate damage, hemi-eliminate the mother centriole, condition the cell, select the least damaged pool.
**Step 2 (Eliminate).** Remove the old centriole; restore telomeres; erase epigenetic marks; select healthy mitochondria.
**Step 3 (Reprogram).** Induce totipotency: DUX4 + KDM4D + DPPA3.
**Step 4 (Rebuild).** Reassemble young centrioles de novo; obtain safe young adult stem cells.
