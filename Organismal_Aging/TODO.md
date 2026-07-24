# Organismal Aging — TODO

> **📄 Articles and publications:** see `~/Desktop/Services/publications/PUBLICATIONS_TRACKER.md`

**Date:** 2026-06-21

---

## 🔴 Phase 0: Core files + Architecture (Q3 2026)

- [x] CONCEPT.md v2.0 ✅
- [x] THEORY.md v1.0 ✅
- [x] PARAMETERS.md v1.0 ✅
- [x] EVIDENCE.md v1.0 ✅
- [x] STATE.md v1.0 ✅
- [x] MEMORY.md v1.0 ✅
- [ ] MAP.md
- [ ] README.md
- [ ] _pi.md
- [ ] OPEN_PROBLEMS.md
- [ ] DESIGN.md
- [ ] Archiving old CONCEPT.md subprojects → `_archive/subprojects_concepts/`
- [ ] Architectural decision: migration cell_dt + mcoa → sim_core

## 🟡 Phase 1: sim_core core (Q4 2026)

- [ ] `sim_core/Cargo.toml` + crate structure
- [ ] `centriole/` — Level #1: entropy, division, polyGlu
- [ ] `counters/` — Level #2: 5 counters + L_tissue aggregator
- [ ] Integration with existing cell_dt + mcoa code
- [ ] Tests: centriole, counters, integration
- [ ] Benchmark: 120 years in < 10 minutes

## 🟡 Phase 2: Tissues + Ze + Ontogenesis (Q1 2027)

- [ ] `tissue/` — 8 tissues with τ_renewal, w_i, L_crit
- [ ] `tissue/ze_conflict.rs` — Z_conflict(i,j,t)
- [ ] `organism/development.rs` — zygote → embryo → adult
- [ ] `microbiome/` — gut, skin, mouth
- [ ] `macrobiome/` — INFOGEST-compatible model
- [ ] `spatial/` — 3D anatomy (basic)

## 🟡 Phase 3: Self-learning + ARGUS (Q2 2027)

- [ ] `learning/bayesian.rs` — MCMC parameter update
- [ ] `argus_bridge/` — protocol, commands, parsing
- [ ] `infogest_bridge/` — standard INFOGEST protocol
- [ ] Cycle: hypothesis → ARGUS → result → update

## 🟢 Phase 4: Species + Web (Q3 2027)

- [ ] `species/human.rs` — basic parameterization
- [ ] `species/mouse.rs` — mouse
- [ ] `species/celegans.rs` — C. elegans
- [ ] `species/unicellular.rs` — unicellular (without centrioles)
- [ ] `sim_gui/` — Phoenix LiveView, 4D visualization
- [ ] `sim_py/` — Python bindings

## 🔵 Phase 5: Validation + Publication (Q4 2027)

- [ ] Calibration on GTEx
- [ ] Validation of Z_conflict
- [ ] Article → *Nature Computational Science*
- [ ] Open release v1.0

---

*Organismal Aging TODO v1.0 — 2026-06-21.*
