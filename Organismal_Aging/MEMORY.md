# Organismal Aging — MEMORY

**Date:** 2026-07-29

---

## 📚 2026-07-29: Review — Hydra: Ultrastructure of Stemness (Seybold, Salvenmoser et al., Innsbruck)

**Article:** Seybold A, Salvenmoser W, Pfaller K, Redl S, Hess MW, Hobmayer B. "Ultrastructure of stemness and differentiated state in *Hydra* epithelial cells." bioRxiv 2026-07-23. DOI: `10.64898/2026.07.20.739505`

**Essence:** Hydra epithelial cells simultaneously divide and perform differentiated functions. Nuclear markers of stemness vs polarity/secretion. Cryofixation.

**Significance for Organismal_Aging:** Hydra — negligible senescence; multifunctional epithelium — ancestral condition.

**✅ 2026-07-29: Letter to Bert Hobmayer sent** — question about centrioles in Hydra. Awaiting reply.
---

## 🔴 2026-07-27: Post-Mortem — Research Square rejection (rs-10483434)

**Type:** Preprint platform rejection.
**Days to decision:** 3 (24 Jul → 27 Jul).
**Reason:** «the manuscript type or its content is not suitable for posting as a preprint on Research Square.»

### What they said
> «Our screeners have determined that the manuscript type or its content is not suitable for posting as a preprint on Research Square. This decision does not reflect the quality or importance of the work and is made on the basis of our editorial policies with respect to content type and screening.»

### What We Missed
1. **Research Square — not for hypothesis papers.** They accept research articles with data. A pure hypothesis without experimental results is not their format.
2. **Length.** ~11,000 words is too long for a preprint. Many platforms have limits.
3. **Mentioning the previous rejection.** In Acknowledgments: “the four reviewers of the original manuscript (npj Aging, ID 2e8466c7)” — this is a red flag for screeners.
4. **Research Square is not the best choice for this paper.** bioRxiv, arXiv (q-bio), Zenodo, or OSF Preprints accept hypothesis/theory papers.

### What to Change
- [ ] For preprint: **Zenodo** (MCARA already there — 10.5281/zenodo.21299683) or **bioRxiv**
- [ ] Remove mention of npj Aging rejection from Acknowledgments — not for a public preprint
- [ ] Medical Hypotheses — wait for decision (submitted Jul 24)

### Next Step
1. **Medical Hypotheses** — a journal specifically for hypothesis papers. This is the right target. Wait.
2. **Preprint:** upload to Zenodo (like MCARA) or bioRxiv. Not Research Square.
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
