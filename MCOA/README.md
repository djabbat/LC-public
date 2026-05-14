<!-- AUTO-TRANSLATED via DeepSeek 2026-05-13. Source language: russian. Original preserved at README.ru.md. -->

# MCOA — Multi-Counter Architecture of Organismal Aging

**MCOA** is a theoretical meta-framework that formalizes organismal aging as a weighted sum of several parallel damage accumulation processes ("counters"). Each counter possesses its own kinetics, dependent on cell divisions and chronological time, and fixed *a priori* weight coefficients for each tissue, ensuring falsifiability. MCOA does not replace specific subprojects (CDATA, Ze, BioSense) but provides a common formal language and architecture for their integration.

## Core Principles

* **Parallel Counters:** Aging is defined by at least two independent damage accumulation processes operating in parallel. No single counter is sufficient to explain the universality of replicative limits (Axiom M1).
* **Dimensional Consistency:** All components of the damage equation must be rendered dimensionless using *a priori* specified reference scales (divisions `n_i*`, time `τ_i`) derived from independent cell-biological knowledge (Axiom M2).
* **A Priori Tissue Weighting:** The contribution of each counter `w_i(tissue)` to the total tissue burden must be predicted prior to model fitting, based on biological parameters (division rate, metabolic activity, etc.). Post-hoc fitting of weights is prohibited (Axiom M3).
* **Falsifiability as a First-Class Goal:** Every claim derived from MCOA must be accompanied by a clear experimental test that could refute it (Axiom M4).

## Key Components

1.  **Canonical Counters:** MCOA defines five primary counters: (1) centriolar polyglutamylation (CDATA), (2) telomeres, (3) mitochondrial ROS/mtDNA, (4) epigenetic drift, (5) proteostasis collapse.
2.  **Formalism:** Damage for counter `i` is described as `D_i(n, t) = D_i₀ + α_i·(n/n_i*) + β_i·(t/τ_i) + γ_i·I(others)`. Total tissue burden: `L_tissue = Σ_i [ w_i(tissue) · f_i(D_i(n, t)) ]`.
3.  **Coupling Matrix (Γ):** Defines how one counter accelerates another (e.g., oxidative stress accelerates telomere shortening). Elements of Γ must be measured, not fitted. By default `γ_i = 0` (independence hypothesis).
4.  **Functional Transition:** A cell enters a state of senescence, apoptosis, or dysfunction upon exceeding `L_tissue > L_critical(tissue)` or `D_i > D_critical(i, tissue)`.

## Relationship with Other LongevityCommon Projects

* **[THEORY.md](THEORY.md):** Complete formal exposition of MCOA axioms, equations, and predictions.
* **[EVIDENCE.md](EVIDENCE.md):** Verified literature sources (PMID/DOI), internal data, and refuting evidence supporting or challenging MCOA.
* **[OPEN_PROBLEMS.md](OPEN_PROBLEMS.md):** Unresolved scientific problems, priorities, and specific falsifiable tests for MCOA.
* **[PARAMETERS.md](PARAMETERS.md):** Table of all quantitative parameters, their sources, units of measurement, and calibration status.
* **[DESIGN.md](DESIGN.md):** Code architecture, file structure, and API contracts for the Rust reference implementation.
* **[AGENTS.md](AGENTS.md):** Instructions and strict rules for LLM agents working with MCOA code and documentation.
* **[JOURNAL.md](JOURNAL.md):** Chronological log of changes, decisions, and their justifications.
* **[ROADMAP.md](ROADMAP.md):** Plan for future development, priorities, and dependencies.

## Important Canonical Corrections (2026-04-22)

* **Health Score formula removed.** The weights 0.40·organism + 0.25·psyche... lacked mathematical justification from MCOA. Direct tissue burden `L_tissue` is used instead.
* **χ_Ze — theoretical construct, not a validated biomarker.** The claim "χ_Ze predicts biological age with R²=0.84" has been retracted, as it was based on synthetic data.
* **Coupling γ_i and MCOA Test 2.** The coupling parameter `γ_i` defaults to 0. MCOA Test 2 is a future protocol for *measuring* couplings between already operational counters, not a source of *a priori* `γ_i` values.
* **EIC Pathfinder Structure.** In the current application (Variant B), MCOA constitutes WP1 (€0.3M, M1-M12). Subprojects Ze, BioSense, and Aqtivirebuli are not included as separate work packages.

**Status:** Concept approved. Manuscript preparation for *Nature Aging* (deadline 2026-04-25) and development of the Rust reference implementation are underway.

## v3 Update (2026-05-13)

CONCEPT.md updated with TBPR peer-review responses:
- Verified PMIDs through PubMed esummary (per `feedback_pmid_verify_always`)
- Removed fabricated references
- Addressed top blocking/critical reviewer concerns
- Statistical protocol additions where applicable
- Honest TODO sections for unmet requirements

See `CONCEPT.md` Section marked "v3" / "Address peer-review concerns"
for project-specific changes.