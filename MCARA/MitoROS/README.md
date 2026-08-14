# MitoROS: Mitochondrial ROS and mtDNA Damage as Counter #3 in the Multi-Counter Architecture of Replicative Aging (MCARA)

**MitoROS** is a subproject within the LC ecosystem, formalizing the accumulation of mitochondrial DNA (mtDNA) damage and reactive oxygen species (ROS) production as a discrete, measurable “counter” of aging within the formal theory MCARA (Multi-Counter Architecture of Replicative Aging).

## Abstract

Aging is a process of accumulation of various types of molecular damage. Although the role of mitochondrial dysfunction is well known, its precise quantitative contribution to the aging trajectory remains a subject of debate. This project presents **MitoROS Counter #3** — a mathematical formalization that describes the kinetics of mtDNA damage accumulation (heteroplasmy, deletions, oxidative damage) as a function of the number of cell divisions (n) and chronological time (t). This counter is integrated into the master equation of MCARA, allowing assessment of its tissue-specific contribution to the overall aging phenotype.

The main hypothesis: accumulation of somatic mtDNA mutations and disruption of redox signaling are among the fundamental, measurable drivers of aging, whose contribution varies between tissues (e.g., high contribution in postmitotic neurons and myocytes, low in rapidly renewing epithelium).

## Key Project Components

* **Formal Theory (`THEORY.md`):** Axioms, definitions, kinetic equation of Counter #3 \( D_3(n, t) \) and its integration into the MCARA master equation.
* **Empirical Basis (`EVIDENCE.md`):** Tables of verified references (PMID/DOI) supporting and refuting the main propositions. Includes meta-analysis data from 24 studies.
* **Open Problems (`OPEN_PROBLEMS.md`):** Clearly formulated scientific questions and design of falsifying experiments with priorities (P0-P2).
* **Quantitative Parameters (`PARAMETERS.md`):** Table of model parameters (\( \alpha_3, \beta_3, \tau_3, n_3^* \)) with origin, units, and status (measured/estimated/hypothetical).
* **Architecture (`DESIGN.md`):** Code structure, API for simulations and data analysis, file tree.
* **Instructions for AI Agents (`AGENTS.md`):** Rules and constraints for LLMs when working with project code and documentation.
* **Change Log (`JOURNAL.md`):** Chronological record of all significant decisions, updates, and their justifications.
* **Roadmap (`ROADMAP.md`):** Future work stages, priorities, and dependencies.

## Relationship to the General MCARA Theory

MitoROS Counter #3 is an integral part of MCARA. Its equation:
\[
D_3(n, t) = D_{3,0} + \alpha_3 \cdot \left( \frac{n}{n_3^*} \right) + \beta_3 \cdot \left( \frac{t}{\tau_3} \right) + \sum_{j \neq 3} \Gamma_{3,j} \cdot g(D_j)
\]
enters the master aging equation for a tissue:
\[
L_{tissue}(n,t) = \sum_{i} w_i(tissue) \cdot f_i(D_i(n,t))
\]
where \( w_3(tissue) \) is an a priori, tissue-specific weight determined by tissue biology (e.g., metabolic load, mitophagy level), not fitted to data.

## Important Limitations and Canon

This project strictly follows the **CORRECTIONS_2026-04-22 canon**:
1. **Does not use the retracted Health Score formula.**
2. **Does not reference χ_Ze as a validated clinical biomarker.**
3. **Does not claim that coupling parameters γ_i are measured in MCARA Test 2.** By default γ_i = 0 (independence hypothesis).
4. **Avoids self-citation (Tqemaladze, Chichinadze, Longevity Horizon).**

The primary description language is Russian; technical terms are given in English.

## Purpose

This project serves as:
1. **A theoretical foundation** for formulating precise, testable hypotheses about the role of mitochondria in aging.
2. **An experimental planning tool** for quantitative assessment of mtDNA damage accumulation in different tissues and conditions.
3. **A module** for integration into larger computational aging models within the LC ecosystem.

To dive into details, start with `THEORY.md` and `EVIDENCE.md`.

## v3 Update (2026-05-13)


CONCEPT.md updated with TBPR peer-review responses:
- Verified PMIDs through PubMed esummary (per `feedback_pmid_verify_always`)
- Removed fabricated references
- Addressed top blocking/critical reviewer concerns
- Statistical protocol additions where applicable
- Honest TODO sections for unmet requirements

See `CONCEPT.md` Section marked "v3" / "Address peer-review concerns"
for project-specific changes.

