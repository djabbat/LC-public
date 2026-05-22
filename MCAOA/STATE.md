<!-- AUTO-TRANSLATED via DeepSeek 2026-05-13. Source language: russian. Original preserved at STATE.ru.md. -->

# STATE — MCAOA

**Purpose:** volatile state.

---

## Current status (2026-05-15)

- **CONCEPT.md replaced:** Now contains MCAOA Phase III (Corrected v2.0) — Validation of Counters #2–#6 in HSCs using ARGUS
- **PMCID corrections applied:** PMID 40021217 removed (non-existent), PMID 40072817 → 40738832
- **Power calculation added:** n=18 clones per arm (Holm-Bonferroni α=0.0042)
- **Arm E expanded:** Cyclophilin A (PPIA) data from Maneix et al. *Nat Cell Biol* 2024
- **Arm F supported:** piRNA counter (#6) now has literature support (PMID: 38142432, 21942366)
- **Submission #1:** Nature Aging NATAGING-P13741 (MCAOA v5 Perspective), submitted 2026-04-19, status review
- **Manuscript #2 (NOT YET PUBLISHED, draft):** "A Stem-Cell-Centric Multi-Counter Theory of Organismal Aging" — extension paper, 3 core principles + counter #6 (piRNA, now with literature support: PMID 38142432, 21942366), VEXAS as evidence of independence of #5 from #2. Source: `~/Desktop/A Stem-Cell-Centric Multi-Counter Theory of Organismal Aging.md` + review chain `docs/manuscripts/HAYFLICK_HIERARCHY/01–15`. Ready for final TBPR reconciliation.
- **Manuscript #3 (NOT YET PUBLISHED, draft):** "Epigenomic Rejuvenation Without Functional Restoration" — systematic review + meta-analysis, PROSPERO **CRD42026218473**, n=14 studies, 274 animals; **damage shadow hypothesis**. Target: *Nature Aging* / *Cell Metabolism* / *Lancet Healthy Longevity* (IF>18). Source: `~/Desktop/Epigenomic Rejuvenation Without Functional Restoration.md`.
- **Counters:** 5 canonical + χ_Ze (S-counter) + **#6: piRNA-counter** (supported: PMID 38142432, 21942366; included in Phase III as Arm F)
- **Tissue weights:** w_HSC, w_skin, w_neural, w_muscle (see PARAMETERS.md)

### Phase III Design

```
Arm A: Control (TERT + 3% O₂)     → >50 divisions expected
Arm B: Counter #2 (BIBR1532)       → Telomeric
Arm C: Counter #3 (antimycin A)    → Mitochondrial
Arm D: Counter #4 (CASIN)          → Epigenetic (Cdc42)
Arm E: Counter #5 (chloroquine)    → Proteostatic (PPIA expanded)
Arm F: Counter #6 (Dicer/Piwi KD)  → piRNA-mediated
```

**Budget:** €351,600 ($390,000). **Conditional:** Phase A + Phase B completion first.

---

## Active TODOs

- [ ] Await Nature Aging editorial decision (manuscript #1)
- [ ] Prepare response to reviewer comments (if any)
- [x] eLife reconsideration — отправлено 2026-05-20
- [x] eLife — отказ 2026-05-21 (Dr Peter Rodgers: «better suited to a specialist journal»)
- [x] F1000Research submission — подано 2026-05-22, статья #183257
- [ ] Suggest reviewers for F1000Research (через My Account → Submissions)
- [ ] Await editorial checks (2-3 рабочих дня)
- [ ] Sobol ABL-2 paradox for Counter #1 — close in coordination with CDATA L1
- [ ] Tissue-specific weights calibration against real data HSC/skin/neural
- [ ] **Stem-Cell-Centric extension:** final TBPR reconciliation, verify VEXAS PMID, JAK/NLRP3 therapeutic refs, formalize D_pi (piRNA counter) kinetics for §4.1 THEORY.md
- [ ] **Damage Shadow review:** transfer draft to `docs/manuscripts/DAMAGE_SHADOW/`, verify PROSPERO record, add EpigeneticDrift subproject EVIDENCE.md cross-link
- [ ] piRNA-counter (#6): search for mammalian-specific data (validation outside germline) — blocks inclusion in canonical set

---

## Milestones

### v5 — Nature Aging submission ✅ 2026-04-19
- [x] MCOA_v5_NatureAging_2026-04-21.pdf ready
- [x] Cover letter
- [x] Submission via editorial system
- [x] 2 follow-up correspondence (2026-04-21)

### v9-file core ✅ 2026-04-25
- [x] CLAUDE.md created
- [x] STATE.md created

### Code baseline ✅ 2026-04-25 (overnight #5 fixed)
- [x] cargo build --release: success
- [x] mcoa_core: 6/6 unit tests pass (was 3 → +3 new)
- [x] **NEW:** `aging_rate_is_weighted_sum` — formula `Σ w_i · C_i = 0.42` on test values
- [x] **NEW:** `null_gamma_yields_zero_influence` — γ=0 default per CORRECTIONS-2026-04-22
- [x] **NEW:** `identity_gamma_yields_self_value` — γ identity = self-value
- [x] mcoa_tests crate (workspace integration tests) — empty, reserved for future
- [x] mcoa_cli, mcoa_api — compile

### Python scripts → Rust port ✅ 2026-04-25 (overnight)

Created `crates/mcoa_compare/`:
- [x] `mcoa-compare-cdata` binary — replaces `scripts/compare_mcoa_cdata.py` (markdown report without plot)
- [x] `mcoa-compare-all` binary — replaces `scripts/compare_all.py` (pairwise Δ matrix)
- [x] `mcoa_compare` lib — `read_csv`, `delta_stats`, `compare_mcoa_cdata`. **3/3 tests pass.**
- [x] cargo build --release: success
- [x] Plot generation excluded from Rust port scope (can be added via `plotters` crate later)
- [x] Old Python scripts remain in `scripts/` for cross-validation

---

## Decision Log

### 2026-05-10 — Two new draft manuscripts integrated into MCAOA roadmap
1. **Stem-Cell-Centric extension (HAYFLICK_HIERARCHY v12+).** Three new theses are proposed (context-dependent counter priority, tissue-specific winner-counter atlas, priority of the Hayflick limit in stem cells). VEXAS syndrome (UBA1 mutation) introduced as clinical evidence: counter #5 (proteostasis) can be rate-limiting **independently of telomeres**, which strengthens M1 (parallel counters). Candidate **counter #6 — piRNA** introduced.

   *Updated 2026-05-15: piRNA counter #6 is no longer just a candidate — now supported by literature (PMID: 38142432, 21942366) and included in Phase III as Arm F (Dicer/Piwi KD). See CONCEPT.md §3.7.*
2. **Damage Shadow systematic review (PROSPERO CRD42026218473).** Pooled correlation between ΔDNAmAge and Δfunction r=0.09 (NS); threshold ΔDNAmAge ≈ −2.4 yrs-equiv before modest tissue-specific gain appears; Lu 2020 (RGC) and Berdugo-Vega 2026 (engram neurons) refine, not refute, the general thesis. Hierarchical model: transcriptomics > epigenomics > structural damage shadow. **Direct implication for MCAOA:** justifies M1 (single-counter epigenetic reset is insufficient) and formalizes the concept of **structural counters** (collagen cross-links, mtDNA, ECM stiffening) as epigenetically-independent. Mesenchymal drift (Li & Tay 2026) — candidate for operationalisation of counter #4 (epigenetic drift) as reversible vs irreversible component.

### 2026-04-25 — 9-file core scheme
CLAUDE + STATE added. Existing 7 files (CONCEPT/DESIGN/EVIDENCE/OPEN_PROBLEMS/PARAMETERS/README/THEORY) already conform to the new scheme.

### 2026-05-20 — eLife reconsideration request + journal cascade
- Review Commons declined (genre mismatch, theory vs experiment).
- Sent letter to Yamini Dalal (eLife Senior Editor) requesting direct reconsideration.
- Established **free journal cascade**: eLife → F1000Research (LMIC waiver) → Annals of Rejuvenation Science.
- Written to MEMORY.md.

### 2026-05-19 — Review Commons decline (#RC-2026-03569)
Rejected on genre grounds: theoretical framework does not fit RC's experimental focus. Decision does not affect affiliate journals.

### 2026-05-13 — eLife soft-decline + RC transfer
Senior Editor Yamini Dalal: no Reviewing Editor available, invited transfer to Review Commons.

### 2026-04-28 — Nature Aging desk reject
NATAGING-P13741 rejected without peer review.

### 2026-04-19 — Nature Aging submission
MCAOA v5 submitted to Nature Aging as the flagship meta-theory of LC. Includes Counter #1 (CDATA), and formalizes the general multi-counter architecture.

---

## Journal Cascade (приоритет — бесплатные журналы)

Бесплатные (diamond OA / no APC) выделены **жирным**. Платные — только с LMIC waiver.

### Основной путь (сейчас)
1. ~~**eLife**~~ — ❌ отказ 2026-05-21 («better suited to a specialist journal»)
2. ~~**F1000Research**~~ — ✅ **подано 2026-05-22, статья #183257**. Статус: редакционная проверка.
3. Если F1000Research откажет — **Annals of Rejuvenation Science** (GLA journal, бесплатно) ✅

### Запасные (платные, только с грантом)
4. npj Aging — APC €2,190 ❌ (только если будет бюджет)
5. Nature Aging — если примут (APC €9,500+ или подписка)

### Правило
> **Сначала все бесплатные маршруты (eLife → F1000Research → Annals). Только потом платные.**

---

## What NOT to do

- Do not publish MCAOA preprint before Nature Aging decision
- Do not add new counters without explicit integration with CONCEPT.md
- Do not confuse "5 counters" with "5 hallmarks" (Counter ≠ hallmark)

## Startup checklist

1. Read CONCEPT + STATE Decision Log
2. Check Nature Aging response
3. Ask the user