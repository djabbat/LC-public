# Proteostasis: Collapse of Proteostasis as Counter #5 in the MCARA Architecture

**Proteostasis** is a subproject within the overall LC architecture, formalizing the collapse of protein homeostasis (proteostasis) as a measurable and quantitative aging process. Within the meta-theoretical Multi-Counter Architecture of Replicative Aging (MCARA), this collapse is defined as **Counter #5**.

## Summary

Aging is accompanied by a progressive loss of the cell's ability to maintain proteostasis — a complex network responsible for protein synthesis, folding, transport, and degradation. This leads to the accumulation of misfolded, damaged, and aggregation-prone proteins, which is a key hallmark of aging and the basis of neurodegenerative diseases (Alzheimer's, Parkinson's) and sarcopenia.

This project does not merely state this fact but proposes a **formal quantitative model**. We define a damage metric *D₅(n, t)* that increases depending on the number of cell divisions (*n*) and chronological time (*t*). Each model parameter (e.g., critical number of divisions *n₅** or aggregation time constant *τ₅*) has a clear biological rationale and is linked to data from peer-reviewed studies.

The goal is to integrate this counter into the overall MCARA system, where it interacts with other counters (mitochondrial dysfunction, epigenetic drift, etc.) through a coupling matrix **Γ**. This transforms the study of proteostasis from a qualitative observation into a calculable, testable, and falsifiable component of a unified theory of organismal aging.

## Key Aspects of the Project

* **Formal Theory:** [THEORY.md](THEORY.md) presents the axiomatics, the counter's kinetic equation, and its connection to MCARA.
* **Evidence Base:** The file [EVIDENCE.md](EVIDENCE.md) contains tables of verified references (PMID/DOI) to studies that confirm or refute each element of the model.
* **Open Problems:** [OPEN_PROBLEMS.md]() describes key unresolved questions, priorities, and specific falsification tests for the model.
* **Quantitative Parameters:** [PARAMETERS.md](PARAMETERS.md) is a summary table of all model parameters, their values, units, and sources.
* **Architecture and Design:** [DESIGN.md](DESIGN.md) describes the structural principles, API, and code organization for implementing the model.
* **Instructions for AI Agents:** [AGENTS.md]() contains strict rules and constraints for LLMs working with the project materials.
* **Change Log:** [JOURNAL.md]() is a chronological record of all significant decisions and updates.
* **Roadmap:** [ROADMAP.md]() defines future development stages, priorities, and dependencies.

## Relationship with Other LC Components

Proteostasis is one of the **nine main counters** within MCARA. Its state affects the overall tissue damage metric *L_tissue(n, t)*. The model is directly linked to the projects:
* **CEDAR (Cellular Damage Theory of Aging):** Proteostasis collapse is one of the main sources of cellular damage (*D_CELL*) in the CEDAR theory.
* **FCLC (Functional Capacity & LifeCourse):** The decline in proteostatic reserve is a driver of functional capacity loss in post-mitotic tissues (brain, muscle).

## Status and Next Steps

The model is at the stage of theoretical development and parameterization based on published data. Key immediate tasks are validation of parameters on independent datasets and development of protocols for experimental measurement of the coupling strength *γ₅* with other counters (see [ROADMAP.md]().

For in-depth study, start with [THEORY.md](THEORY.md) and [EVIDENCE.md](EVIDENCE.md).

---
*This document was created in accordance with the CORRECTIONS_2026-04-22 canon. All statements have been verified against the current evidence base; retracted theses have been excluded.*

## v3 Update (2026-05-13)

CONCEPT.md updated with TBPR peer-review responses:
- Verified PMIDs through PubMed esummary (per `feedback_pmid_verify_always`)
- Removed fabricated references
- Address ed top bl
