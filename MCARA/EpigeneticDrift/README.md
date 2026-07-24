# Epigenetic Drift: Counter #4 in MCARA

**Epigenetic Drift** is a formalization of the erosion of epigenetic information as a discrete, measurable aging process within the Multi-Counter Architecture of Organismal Aging (MCARA). The project defines epigenetic drift not merely as a biomarker, but as a dynamic counter with its own kinetics, drivers, and interactions with other aging processes.

## Core Concept

Epigenetic drift is the cumulative deviation of the epigenetic landscape (DNA methylation, histone modifications, chromatin accessibility) from the juvenile, tissue-specific state. In MCARA, it is formalized as **Counter #4** with the state equation:
`D₄(n, t) = D₄,₀ + β₄·(t / τ₄) + α₄·(n / n₄*) + γ₄ · I(others)`

Where:
* `D₄` — drift state.
* `β₄` — linear coefficient dependent on chronological time.
* `α₄` — coefficient associated with cell divisions.
* `γ₄` — coupling parameter with other MCARA counters.

## Key Features

* **Quantitative formalization:** Equation parameters are justified by meta-analysis data from epigenetic clocks (Horvath, GrimAge, DunedinPACE) and stem cell aging studies.
* **Measurability:** The primary measurement method is DNA methylation arrays (Illumina EPIC) and chromatin accessibility analysis (ATAC-seq). The counter state is proxied via epigenetic clock algorithms.
* **Interactions:** The counter is linked to other aging processes (telomere shortening, mitochondrial ROS, proteostasis), reflected in the MCARA coupling matrix Γ.
* **Falsifiability:** The project clearly defines [critical unresolved questions]() and tests to verify them, including the ABL-2 paradox and causal relationships.

## Links to Other Project Files

* **[THEORY.md](THEORY.md):** Complete formal theory, axioms, equation derivation, and predictions.
* **[EVIDENCE.md](EVIDENCE.md):** Verified literature references (PMID/DOI), internal data, and refuting evidence.
* **[OPEN_PROBLEMS.md]():** Prioritized list of open scientific problems with falsification tests.
* **[PARAMETERS.md](PARAMETERS.md):** Table of all quantitative parameters, their origin, units, and status.
* **[DESIGN.md](DESIGN.md):** Code architecture, file tree, and API contracts for simulations and analysis.
* **[AGENTS.md]():** Instructions for LLMs (such as Claude) on working with the project, including strict rules and safety constraints.
* **[JOURNAL.md]():** Chronological log of changes, decisions, and their justifications.
* **[ROADMAP.md]():** Plan for future improvements, priorities, and dependencies.

## Project Goal

To create a rigorous, data-driven computational model of epigenetic drift as a core of aging that can:
1. Integrate data from various epigenetic platforms.
2. Quantitatively assess the contribution of time and cell divisions.
3. Model interactions with other damages.
4. Formulate testable predictions for experiments and interventions.

The project is part of the broader LC ecosystem and follows the canons established in the document **CORRECTIONS_2026-04-22**. Any claims retracted in that document (e.g., about the Health Score formula or χ_Ze as a validated biomarker) are not used here.

---

## v3 Update (2026-05-13)

CONCEPT.md updated with TBPR peer-review responses:
- Verified PMIDs through PubMed esummary (per `feedback_pmid_verify_always`)
- Removed fabricated references
- Addressed top blocking/critical reviewer concerns
- Statistical protocol additions where applicable
- Honest TODO sections for unmet requirements

See `CONCEPT.md` Section marked "v3" / "Address peer-review concerns"
for project-specific changes.
