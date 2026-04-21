```markdown
# UPGRADE.md — LentiviralTools

This document outlines the planned future extensions and roadmap for the **LentiviralTools** sub-project, part of Dr. Jaba Tkemaladze's CytogeneticTree project for centriolar lineage tracking.

---
## v1.0 (2026-04-21, current) — 5-file scaffolding
*   **Status:** MVP (Minimum Viable Product) established.
*   **Scope:** Core plasmid file structure, basic protocols for Twist/Addgene design, HEK293T packaging, and BJ-hTERT transduction.

---
## v1.1 (Phase 0 refinement, target: ~2026-05)
- [ ] Order and receive RITE-Centriolin de novo synthesis (Twist Bio).
- [ ] Validate pLenti-Cre-ERT2 backbone cloning protocol.
- [ ] Document complete HEK293T lentiviral packaging workflow.

---
## v2.0 (Phase 1 experimental, contingent on Impetus Go, target: ~2026-06)
- [ ] Validate functional RITE-Centriolin construct expression in BJ-hTERT cells.
- [ ] Establish and document clonal selection workflow (FACS-based).
- [ ] Achieve Tamoxifen-induced Cre-ERT2 recombination efficiency ≥ 90% within 48 hours.
- [ ] Complete CCP1-OE and PACT-CCP1-OE construct assembly for Phase A rescue experiments.

---
## v3.0 (Phase 2 mouse HSC, target: ~2027)
- [ ] Validate RITE-Centriolin system functionality in mouse primary LSK hematopoietic stem cells (HSCs).
- [ ] Develop protocol for high-titer retroviral packaging (preferred over lentiviral for murine HSC transduction).

---
## v4.0 (Phase 3 embryo, target: ~2028-2029)
- [ ] Develop mRNA-delivered RITE system for zygote microinjection (integration-free, safer for embryogenesis).
- [ ] **Alternative Path:** Engineer site-specific integration system using AAV + HDR for generating stable transgenic vertebrate embryo lines.

---
## v5.0 (Long-term platform, target: 2030+)
- [ ] Platform release: deposit all finalized plasmid sequences and maps on Addgene.
- [ ] Engineer inducible expression variants (e.g., Tet-On, estrogen-responsive promoters) for conditional system control.
- [ ] Develop multi-color RITE system (e.g., red→green→blue sequential switches) for tracking ≥3 consecutive generational lineages.

---
## Known Blockers / Decision Points

| Blocker | Decision Date | If Resolved | If Blocked |
| :--- | :--- | :--- | :--- |
| Twist Bio synthesis turnaround time & success. | Month 2 of Phase 1 | Proceed directly to packaging & validation. | Switch vendor to Genscript or IDT; reassess timeline. |
| Cre-ERT2 recombination efficiency in BJ-hTERT cells. | Month 2-3 of Phase 1 | Continue with established RITE workflow. | Fall back to plan using Dendra2-Centrin photoconversion for lineage tracing. |
```
