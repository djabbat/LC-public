# Organismal Aging — MAP

**Date:** 2026-06-21

## Structure of Organismal_Aging/ (within LC/)


Organismal_Aging/
├── CONCEPT.md                       ← Concept (v2.0)
├── THEORY.md                        ← Mathematical formalism
├── PARAMETERS.md                    ← Numerical parameters
├── MAP.md                           ← This file
├── STATE.md                         ← Current status
├── MEMORY.md                        ← Decision history
├── TODO.md                          ← Tasks
├── EVIDENCE.md                      ← Evidence base
├── README.md                        ← Description
├── _pi.md                           ← Rules for pi
├── OPEN_PROBLEMS.md                 ← Open questions
├── DESIGN.md                        ← Software architecture
│
├── docs/                            ← Documentation
│
├── scripts/                         ← Scripts
│
└── _archive/                        ← Archive


## Integration into LC

Organismal_Aging — **detailed documentation** for the integrator component within LC.

LC (root):
- `sim_core/` — core code (Rust)
- `argus_bridge/` — bridge to ARGUS-LP
- `infogest_bridge/` — bridge to INFOGEST
- `biosense/` — sensors
- `fclc/` — safety
- `hap/` — Health-Age Profiling
- `sim_gui/` — web interface

---

*Organismal Aging MAP v1.0 — 2026-06-21.*
