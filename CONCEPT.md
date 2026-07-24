# CONCEPT — LC (LongevityCommon)  
**Date:** 2026-06-21 (complete rethinking — merging all subprojects)  
**Version:** 7.0 — Organismal Aging as a single integrator  
**Author:** Jaba Tqemaladze, MD — Georgia Longevity Alliance  
**License:** Apache 2.0  
**Codebase:** 360+ files (Rust + Python + Phoenix/Elixir)  

---

## 0. NEW ARCHITECTURE (v7.2 — FINAL)

**LC is no longer an ecosystem of subprojects. LC = Organismal Aging — a platform for testing aging hypotheses.**

**Fundamental principle:** The project’s strength lies not in a single hypothesis (centriole), but in the ability to test **any** aging hypotheses within a unified architecture. The centriole is one of many hypotheses, a replaceable module. The simulator core is MCARA (level #2) + tissues/Ze-conflicts (level #3). Level #1 is **plug-and-play**: any candidate for the “root cause” can be connected, tested, and replaced.

### Old vs new structure

```
OLD (v6.0): fragmented ecosystem                    NEW (v7.0): single integrator
                                                       │
LC/                                                     LC/ = Organismal Aging
├── MCARA/     (its own CONCEPT, its own crates)            │
├── CEDAR/     (its own CONCEPT, its own crates)            ├── sim_core/         ← core (Rust)
├── Ze/        (its own CONCEPT, its own hierarchy)         │   ├── centriole/    ← Level #1
├── BioSense/  (its own CONCEPT, its own sensors)           │   ├── counters/     ← Level #2
├── HAP/       (its own CONCEPT)                            │   ├── tissue/       ← Level #3
├── FCLC/      (its own CONCEPT)                            │   ├── organism/     ← integration
├── Organismal_Aging/  (new, separate)                      │   ├── species/      ← species
│                                                            │   ├── microbiome/   ← microbiome
│                                                            │   ├── macrobiome/   ← macrobiome
│                                                            │   ├── learning/     ← self-learning
│                                                            │   └── spatial/      ← 3D model
│                                                            │
│                                                            ├── argus_bridge/    ← ARGUS-LP robot
│                                                            ├── infogest_bridge/ ← INFOGEST
│                                                            ├── biosense/        ← sensors
│                                                            ├── fclc/            ← safety
│                                                            ├── hap/             ← Health-Age
│                                                            │
│                                                            ├── sim_cli/         ← CLI
│                                                            ├── sim_api/         ← REST API
│                                                            ├── sim_gui/         ← Phoenix LiveView
│                                                            ├── sim_py/          ← Python
```


## Hypothesis

*To be specified — see project documentation.*


## Budget

*See PARAMETERS.md for detailed budget.*


## References

| # | Reference | PMID |
|---|-----------|------|
| 1 | See parent project | — |
