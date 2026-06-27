# STATE — MCAOA

**Аудит 2026-06-16:** состояние подтверждено (глубокий аудит pi).
**Date:** 2026-06-27

> **📄 Статьи и публикации:** см. `~/Desktop/Services/publications/PUBLICATIONS_TRACKER.md`

## 👤 Автор
- **Jaba Tqemaladze, MD** — начал PhD в **UNED Madrid** осенью 2026
- Заявка UNED: Nº 712356513 ✅

## Current status (v3.2, conditionally approved)

- **CONCEPT.md v3.2** — after 3 rounds of expert review
- **Final evaluation:** 86.5/100 — **recommended for PhD defense**
- **All corrections implemented:** TERT fix, sigmoidal f_i(x), VEXAS caveat, L_tissue calibration, piRNA→EXPLORATORY, Janke correction, FDR stat, chloroquine caveat, Nolde 2013 refuting evidence, OSF pre-registration
- **Simulator:** Rust-based MCAOA CLI + API with EDC module, open source on GitHub (`djabbat/LC`, branch `mcaoa-v3.2`)
- **Preprint:** Zenodo DOI 10.5281/zenodo.20055806

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

## Active TODOs

- [ ] Prepare response to reviewer comments (if any)
- [ ] Sobol ABL-2 paradox for Counter #1 — close in coordination with CDATA L1
- [ ] Tissue-specific weights calibration against real data HSC/skin/neural
- [ ] **Stem-Cell-Centric extension:** final TBPR reconciliation, verify VEXAS PMID, JAK/NLRP3 therapeutic refs, formalize D_pi (piRNA counter) kinetics for §4.1 THEORY.md
- [ ] **Damage Shadow review:** transfer draft to `docs/manuscripts/DAMAGE_SHADOW/`, verify PROSPERO record, add EpigeneticDrift subproject EVIDENCE.md cross-link
- [ ] piRNA-counter (#6): search for mammalian-specific data (validation outside germline) — blocks inclusion in canonical set

## Milestones

### v9-file core ✅ 2026-04-25
- [x] CLAUDE.md created
- [x] STATE.md created

### Code baseline ✅ 2026-04-25
- [x] cargo build --release: success
- [x] mcoa_core: 6/6 unit tests pass
- [x] mcoa_compare: 3/3 tests pass
- [x] Old Python scripts remain in `scripts/` for cross-validation

## Decision Log

### 2026-05-10 — Two new draft manuscripts integrated into MCAOA roadmap
1. **Stem-Cell-Centric extension (HAYFLICK_HIERARCHY v12+).** Three new theses proposed. VEXAS syndrome (UBA1 mutation) as clinical evidence. Candidate **counter #6 — piRNA** introduced.
2. **Damage Shadow systematic review (PROSPERO CRD42026218473).** Pooled correlation ΔDNAmAge vs Δfunction r=0.09 (NS). Hierarchical model: transcriptomics > epigenomics > structural damage shadow. Mesenchymal drift (Li & Tay 2026) — candidate for counter #4.

## What NOT to do
- Do not add new counters without explicit integration with CONCEPT.md
- Do not confuse "5 counters" with "5 hallmarks" (Counter ≠ hallmark)
