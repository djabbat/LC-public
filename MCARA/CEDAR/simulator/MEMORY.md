# MEMORY.md — CEDAR-v2 Simulator

## Decision History

### 2026-06-27 — Audit and creation of core files
- **Decision:** A deep audit of all projects was conducted. CEDAR_simulator was found without core files.
- **Action:** All 10 core files were created (_pi.md, CONCEPT.md, TODO.md, PARAMETERS.md, MAP.md, STATE.md, MEMORY.md, DESIGN.md, THEORY.md, EVIDENCE.md).
- **Context:** README.md and pyproject.toml already existed.

### Choice of Python (instead of Rust)
- **Decision:** The simulator is written in Python 3.10+, despite the general trend of Rust in the ecosystem.
- **Reason:** NumPy/SciPy ecosystem for scientific computing, development speed, accessibility for the scientific community.
- **Consequences:** Porting to Rust is in the backlog (low priority).

### Private repository
- **Decision:** GitHub repo is private.
- **Reason:** The article has not been published in a peer-reviewed journal.
- **Plan:** Open after publication of the article.

### GPL v3 license
- **Decision:** GPL v3 instead of Apache 2.0 (as in LC).
- **Reason:** Scientific code — requirement of reproducibility.

### 2026-08-08 — rDNA clock (TRCS) as third senescence counter (v4.7)
- **Decision:** Integrated 45S rDNA copy-loss as the THIRD molecular clock (SenescenceTrigger::RdnDnaShortening) in the Rust AgingEngine (`crates/cell_dt_modules/aging_engine`).
- **Basis:** TRCS model (Huang 2026, Ageing Longev. Res. 2(1):2, DOI 10.53941/alr.2026.100002): telomere + rDNA co-regulation of senescence via p53. Supporting: Kobayashi 2014 (Proc Jpn Acad B 90:119); Defossez 1999 (MCB 19:3848).
- **Key biology:** unlike telomeres (maintained by constitutive telomerase), rDNA arrays ARE lost in dividing stem cells; knockdown of 45S rDNA → p53/p21/p16/SA-β-GAL up (Huang 2026).
- **Parameters:** RDNA_LOSS_PER_DIVISION=0.0006 (calibrated: HSC 12 div/yr → 0.5 threshold ~70 yr); RDNA_CRIT=0.5; RDNA_MIN=0.2; RDNA_RESTORATION_RATE=0.02 (intervention `rdna_restoration`).
- **Priority in SenescenceTrigger::evaluate:** centriolar > telomere > rDNA; `Both` preserved (centriolar+telomere).
- **Tests:** 547 pass (workspace). New: rDNA decreases with age, floor, restoration slows loss, snapshot field, trigger boundary.
- **Docs:** analysis `~/Desktop/Services/docs/ANALYSIS_CDATA_v2_vs_Huang_TRCS_2026-08-08.md`
