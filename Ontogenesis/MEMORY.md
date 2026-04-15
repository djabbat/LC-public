# Ontogenesis — SESSION MEMORY

Running log of decisions, context, and key facts across sessions.

---

## Project State (2026-03-29)

- **Version:** v4.1
- **Concept date:** 2026-03-27
- **Status:** Stage 0 (data collection & harmonization) — in progress
- **MVP timeline:** 24–36 months total
- **Tech stack:** Rust (core algorithms, Data Harmonization Layer) + Three.js/WebGL 2.0 (3D rendering) + WebAssembly bridge
- **Scope:** Human ontogenesis 0–25 years, 24 parameters across 5 domains
- **Part of:** AIM ecosystem (~/Desktop/AIM/)
- **Ecosystem role:** Provides age norms (hormones 0–25 yr) to AIM/lab_reference.py; regeneration coefficients to AIM/treatment_recommender.py; upper boundary (25 yr) = lower boundary of CDATA simulation

---

## Completed (Stage 0 — partial)

- CONCEPT.md v4.1 finalized (2026-03-27)
- `src/data/ingestion.rs` — DataIngestion: CSV + JSON import (2026-03-28)
- `src/data/normalization.rs` — Normalization: unified 1-month age grid (0–300 months) (2026-03-28)
- `src/analysis/cv_analysis.rs` — CVAnalysis + RangeAnalysis (2026-03-28)
- `src/analysis/transition_detection.rs` — TransitionDetection algorithm §2.2–2.3 (2026-03-28)

---

## Key Decisions

- Transitions are detected empirically only — never postulated by age or count
- Longitudinal threshold: 2 SD from individual trajectory (anatomical); 3 SD (endocrine)
- Cross-sectional threshold: CV(A,t) > mean_CV + 2×SD_CV (anatomical); Range_90_10 > mean + 2×SD (endocrine)
- Clustering: 6-month radius; min stable period 3 months; min 30 transitions per cluster
- LOD system: >5m organism, 2–5m organ, 0.5–2m tissue, <0.5m microscopic
- Target performance: ≥30 FPS on desktop
- Platform does NOT store personal patient data — all simulations on aggregated/synthetic data
- Embryological metaphor mode: germ layer color coding (ectoderm #FF3333, mesoderm #33FF33, endoderm #FFFF33, germline #FFFFFF)
- Team: all key roles (Rust developer, 3D engineer, data scientist, endocrinology consultant) still needed

---

## Recent Work

| Date | Task | Result |
|------|------|--------|
| 2026-03-28 | Data Harmonization Layer (Rust) | 4 modules complete: ingestion, normalization, CVAnalysis, TransitionDetection |
| 2026-03-27 | CONCEPT.md v4.1 | Full concept with methodology, 24 parameters, 4 implementation stages |
