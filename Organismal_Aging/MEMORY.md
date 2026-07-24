# Organismal Aging — MEMORY

**Date:** 2026-06-21

---

## Decision #1: Creating the Organismal Aging Project

**Date:** 2026-06-21
**Initiator:** Dzhaba
**Context:** Working session with pi

**Decisions:**
1. Name: **Organismal Aging** (not Organizmal)
2. Location: within **LC** as a single integrator
3. Aging — **the basis of all diseases** (aging-driven, not age-associated)
4. **Self-learning 4D simulator** (3D + time) with physical robot ARGUS-LP
5. **Three levels:** Centriole → 5 MCARA counters → Tissues + Ze conflicts
6. **8 basic tissues** with expansion capability
7. **Species universality:** human, mouse, C. elegans, unicellular organisms
8. **piRNA (#6) excluded from v1.0** — left as an expansion slot
9. **License:** Apache 2.0
10. **Open project** — all code, data, hardware (ARGUS-LP)

**Dzhaba verbatim:** "A simulator with a robot that tests hypotheses on physical simulators like INFOGEST, self-learns, creates a model of the organism in time and space, its development from zygote and age-related changes, possible injuries, other diseases, microbiome and macrobioime."

---

## Decision #2: Merging LC into a Single Project

**Date:** 2026-06-21
**Initiator:** Dzhaba

**Decision:** LC ceases to be an "ecosystem of subprojects." All former subprojects (CEDAR, MCARA, Ze, BioSense, FCLC, HAP) — components of the unified Organismal Aging system. Old CONCEPT.md → archive.

**Structure:** `sim_core/` (Rust) — core. `argus_bridge/`, `infogest_bridge/` — physical bridges. `biosense/`, `fclc/`, `hap/` — supporting components.

---

*Organismal Aging MEMORY v1.0 — 2026-06-21.*
