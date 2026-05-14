<!-- AUTO-TRANSLATED via DeepSeek 2026-05-13. Source language: russian. Original preserved at STATE.ru.md. -->

# STATE — MCOA

**Purpose:** volatile state.

---

## Current status (2026-05-10)

- **Submission #1:** Nature Aging NATAGING-P13741 (MCOA v5 Perspective), submitted 2026-04-19, status review
- **Manuscript #2 (NOT YET PUBLISHED, draft):** "A Stem-Cell-Centric Multi-Counter Theory of Organismal Aging" — extension paper, 3 core principles + potential 6th counter (piRNA), VEXAS as evidence of independence of #5 from #2. Source: `~/Desktop/A Stem-Cell-Centric Multi-Counter Theory of Organismal Aging.md` + review chain `docs/manuscripts/HAYFLICK_HIERARCHY/01–15`. Ready for final TBPR reconciliation.
- **Manuscript #3 (NOT YET PUBLISHED, draft):** "Epigenomic Rejuvenation Without Functional Restoration" — systematic review + meta-analysis, PROSPERO **CRD42026218473**, n=14 studies, 274 animals; **damage shadow hypothesis**. Target: *Nature Aging* / *Cell Metabolism* / *Lancet Healthy Longevity* (IF>18). Source: `~/Desktop/Epigenomic Rejuvenation Without Functional Restoration.md`.
- **Counters:** 5 canonical + χ_Ze (S-counter) + **#6 candidate: piRNA-counter** (placeholder, not included in v5; see THEORY.md §4.1)
- **Tissue weights:** w_HSC, w_skin, w_neural, w_muscle (see PARAMETERS.md)

---

## Active TODOs

- [ ] Await Nature Aging editorial decision (manuscript #1)
- [ ] Prepare response to reviewer comments (if any)
- [ ] Backup: secondary target (npj Aging, eLife) if rejected
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

### 2026-05-10 — Two new draft manuscripts integrated into MCOA roadmap
1. **Stem-Cell-Centric extension (HAYFLICK_HIERARCHY v12+).** Three new theses are proposed (context-dependent counter priority, tissue-specific winner-counter atlas, priority of the Hayflick limit in stem cells). VEXAS syndrome (UBA1 mutation) introduced as clinical evidence: counter #5 (proteostasis) can be rate-limiting **independently of telomeres**, which strengthens M1 (parallel counters). Candidate **counter #6 — piRNA** introduced; not canonical until validation in mammalian non-germline tissue.
2. **Damage Shadow systematic review (PROSPERO CRD42026218473).** Pooled correlation between ΔDNAmAge and Δfunction r=0.09 (NS); threshold ΔDNAmAge ≈ −2.4 yrs-equiv before modest tissue-specific gain appears; Lu 2020 (RGC) and Berdugo-Vega 2026 (engram neurons) refine, not refute, the general thesis. Hierarchical model: transcriptomics > epigenomics > structural damage shadow. **Direct implication for MCOA:** justifies M1 (single-counter epigenetic reset is insufficient) and formalizes the concept of **structural counters** (collagen cross-links, mtDNA, ECM stiffening) as epigenetically-independent. Mesenchymal drift (Li & Tay 2026) — candidate for operationalisation of counter #4 (epigenetic drift) as reversible vs irreversible component.

### 2026-04-25 — 9-file core scheme
CLAUDE + STATE added. Existing 7 files (CONCEPT/DESIGN/EVIDENCE/OPEN_PROBLEMS/PARAMETERS/README/THEORY) already conform to the new scheme.

### 2026-04-19 — Nature Aging submission
MCOA v5 submitted to Nature Aging as the flagship meta-theory of LongevityCommon. Includes Counter #1 (CDATA), and formalizes the general multi-counter architecture.

---

## What NOT to do

- Do not publish MCOA preprint before Nature Aging decision
- Do not add new counters without explicit integration with CONCEPT.md
- Do not confuse "5 counters" with "5 hallmarks" (Counter ≠ hallmark)

## Startup checklist

1. Read CONCEPT + STATE Decision Log
2. Check Nature Aging response
3. Ask the user