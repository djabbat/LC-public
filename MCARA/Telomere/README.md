# Telomere Shortening Counter (MCARA #2)  
**Status:** Active subproject within the Multi-Counter Organismal Aging (MCARA) architecture. Defines telomere shortening as a quantitative, modelable counter of cellular aging with clear kinetic parameters.  

## Summary  

This subproject formalizes the process of telomere shortening as **Counter #2** in the overall MCARA system. Unlike the simplified view of telomeres as simple "division clocks," our model describes their state using an equation that accounts for both division-dependent loss (end-replication problem) and accelerated shortening due to oxidative stress. The model integrates current data on mechanisms of telomeric DNA damage, the role of the chaperonin complex (TRiC) and the RIOK2 protein in telomerase assembly, as well as erroneous repair of 8-oxoguanine.  

The key result is a parametric kinetic equation for telomere length deficit `D₂(n, t)`. All its parameters (`α₂`, `β₂`, `n₂*`, `τ₂`) have empirical support in peer-reviewed literature (21 PMIDs). The state of this counter contributes a weighted input to the overall tissue aging burden `L_tissue` within the MCARA master equation.  

## Links to Other Files  

* **[THEORY.md](./THEORY.md)** — Complete formal specification: axioms, derivation of the master equation, mathematical predictions, and parameter interpretation within MCARA.  
* **[EVIDENCE.md](./EVIDENCE.md)** — Tables of verified sources (PMID/DOI) supporting each parameter and mechanism, as well as data not explained by the model (honest disclosure).  
* **[OPEN_PROBLEMS.md]()** — Critical unresolved questions, such as quantification of the time constant `τ₂` and separation of `α₂` and `β₂` contributions in vivo. For each problem, falsifiability tests with clear criteria are provided.  
* **[PARAMETERS.md](./PARAMETERS.md)** — Summary table of all model parameters (`α₂`, `β₂`, `n₂*`, `τ₂`, `D₂,₀`, weights `w₂`), their values, units, sources, and status (measured/assumption/requires calibration).  
* **[DESIGN.md](./DESIGN.md)** — Code architecture for simulations of this counter: file structure, API for updating `D₂` state and calculating contribution to `L_tissue`, usage examples.  
* **[AGENTS.md]()** — Instructions for AI agents (e.g., for literature analysis or experiment planning) with strict safety rules and references to canonical definitions.  
* **[JOURNAL.md]()** — Chronological log of changes, decisions, and their justifications within this subproject.  
* **[ROADMAP.md]()** — Plan for future work: priority tasks, dependencies on other subprojects (e.g., CEDAR for validation), integration milestones.  

## Context and Limitations  

* **Within MCARA:** The telomere counter is one of several (alongside centriolar, mitochondrial ROS, epigenetic drift). Its contribution `w₂(tissue)` varies between tissues and must be determined by calibration on data.  
* **According to CORRECTIONS_2026-04-22:** The model does not use retracted concepts such as the Health Score formula or `χ_Ze` as a biomarker. All statements comply with the updated canon.  
* **Three axioms of CEDAR:** If the project is considered in the context of CEDAR (Cellular Damage & Telomere Attrition), then its three axioms (1. Damage accumulates, 2. Telomeres are a counter of divisions and stress, 3. Signaling pathways integrate damage) are considered inviolable for the purposes of this subproject.  
* **Language:** The main technical description is in English. Explanatory
