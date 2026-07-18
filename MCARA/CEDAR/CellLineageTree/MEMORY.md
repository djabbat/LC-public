# MEMORY — CytogeneticTree

## 🔬 Literature Review 2026-07-18 — Asymmetric Inheritance Evidence

> Полный обзор: `../docs/LITERATURE_REVIEW_2026-07-18.md`
> Брифинг для MCARA: `../docs/MCARA_BRIEFING_2026-07-18.md`

### Ключевое для CellLineageTree:
- **CENP-A/CENP-C как маркеры epigenetic age** (Carty 2021 PLoS Genet, PMID 34014920): CENP-C required for CID assembly; CID lost in aged GSC. Измерять alongside GT335.
- **Ciliary membrane inheritance** (Paridaen 2013 Cell, PMID 24120134): ARL13B/SMO alongside Ninein — второй канал asymmetric readout.
- **De novo centriole synthesis frequency:** см. Bobinnec 1998 (PMID 9730976), Khodjakov & Rieder 2001 (PMID 11285289). Предсказание D1 → прямое измерение.
- **Mitotic drive model** (Ranjan & Chen 2022, PMID 35437581) — теоретическая рамка для biased segregation.

### Слабые места, которые платформа должна адресовать:
- Частота de novo центриолей (<5%? >20%?) — Prediction D1-D3
- Ninein-needed vs not-needed в разных видах — тест на mammalian cells (BJ-hTERT)
- Asymmetric histone segregation под вопросом (Li 2025 PNAS) — не полагаться на гистоны как аргумент; фокус на центросомах

## Permanent rules

- **Literature**: every PMID must be verified via PubMed esummary API before entering any file in this subproject. No DeepSeek for citation search.
- **RITE-Centriolin**: treat as *not yet published* / *must be de-novo cloned* until proven otherwise. Fallback: Dendra2-Centrin photoconvertible.
- **Connection to CEDAR**: CytogeneticTree is the **empirical test-bed** for CEDAR theoretical predictions, NOT a parallel theory. Keep this clear in outreach.
- **Connection to Impetus LOI**: Phase 1 MVCT is a *minimum-viable demo*. The full Cytogenetic Tree is a multi-year programme beyond the grant.
- **Scope**: this subproject handles *lineage-level* reconstruction; single-cell biochemistry lives in CEDAR; tissue-level biomarkers live in Ze / BioSense.

## Dated entries

### 2026-04-21

- Subproject created (CytogeneticTree) as new LC subproject per Jaba's request.
- Umbrella CONCEPT + 10 core files scaffolded (this set).
- 12 technology sub-subprojects scaffolded via parallel agent (CONCEPT + 5-file core each).
- Literature landscape agent (parallel) gathering KNOWLEDGE.md — PubMed + bioRxiv + arXiv + Google Scholar. **Search starts AFTER scaffolding completes** (per Jaba's explicit instruction).
- Connection to Impetus LOI 2026-04-25 documented — Phase A experiment = MVCT demonstration.
- Connection to Ilia Zheleznov HSC simulator noted — his computational model could pre-validate expected tree topology before wet lab.
- Open question logged: does RITE-Centriolin construct already exist publicly? Impetus audit 2026-04-21 said NO.

## Technology gaps (known today)

- **RITE-Centriolin construct** — likely first in the world; de-novo synth.
- **AI-directed real-time ablation orchestration** — novel; Claude Code `/overnight` will be first operational framework.
- **Full genealogy-reconstruction algorithm** for centriole lineage — no off-the-shelf tool; will be built (GenealogyReconstruction subproject).
- **Lineage-to-differentiation annotation** at centriole granularity — not previously done.

## Things to remember across sessions

- Always check `CONCEPT.md` is in sync with `CEDAR/CONCEPT.md` + `MCARA/` claims. If CEDAR updates its theory, CytogeneticTree predictions might need update.
- Keep clear separation: Impetus = funded Phase A (MVCT fibroblasts, binary Go/No-Go). Cytogenetic Tree = 3-phase multi-year programme.
- `~/Desktop/LC/AutomatedMicroscopy/` is the shared hardware platform dir; CytogeneticTree is the methodology layer on top.

## Related memory files

- `project_longevity_georgia_ngo` — NGO that hosts this research
- `feedback_deepseek_no_citations` — enforce for all KNOWLEDGE.md entries
- `feedback_verify_references` — verify every PMID before commit
- `feedback_cedar_docs_sync` — when editing CEDAR, check if CytogeneticTree docs need update (and vice versa)
- `project_mcoa_nature_correspondence` — MCARA manuscript at Nature Aging; CytogeneticTree is the empirical counterpart


## v3 Update (2026-05-13)

CONCEPT.md updated with TBPR peer-review responses:
- Verified PMIDs through PubMed esummary (per `feedback_pmid_verify_always`)
- Removed fabricated references
- Addressed top blocking/critical reviewer concerns
- Statistical protocol additions where applicable
- Honest TODO sections для unmet requirements

See `CONCEPT.md` Section с пометкой "v3" / "Адрес peer-review concerns"
для project-specific changes.

