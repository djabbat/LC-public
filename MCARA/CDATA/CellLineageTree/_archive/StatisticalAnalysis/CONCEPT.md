# StatisticalAnalysis Sub-Subproject: Concept

## §1 Purpose

This sub-subproject provides the statistical core for the CytogeneticTree project's goal of reconstructing the genealogical tree of cellular differentiation. Its primary purpose is to rigorously model the "lifespan" or retention time of the inherited maternal centriole as a cell progresses from zygote to terminal differentiation. By applying survival analysis to this binary Go/No-Go endpoint (centriole retained vs. lost), we quantify the differentiation potential and fate restrictions associated with centriole age. This statistical framework transforms the biological hypothesis—that centriole age serves as a lineage tracer—into testable, quantitative models, enabling the probabilistic reconstruction of lineage relationships from static, single-cell centriole age data.
This sub-subproject provides the statistical core for the CytogeneticTree project's goal of reconstructing the genealogical tree of cellular differentiation. Its primary purpose is to rigorously model the "lifespan" or retention time of the inherited maternal centriole as a cell progresses from zygote to terminal differentiation. By applying survival analysis to this binary Go/No-Go endpoint (centriole retained vs. lost), we quantify the differentiation potential and fate restrictions associated with centriole age. This statistical framework transforms the biological hypothesis—that centriole age serves as a lineage tracer—into testable, quantitative models, enabling the probabilistic reconstruction of lineage relationships from static, single-cell centriole age data.

## §2 Mechanism/Basis

The analysis pipeline is built on three methodological pillars:
1. **Survival Analysis:** We treat the event of "centriole loss" as a failure event in a time-to-event analysis, where "time" is the differentiation step (e.g., cell division or differentiation stage). The Kaplan-Meier estimator will provide a non-parametric survival curve (probability of centriole retention across steps). The log-rank test will compare survival functions between hypothesized lineage branches (e.g., progenitor vs. differentiated cell fates), testing if centriole retention patterns differ significantly.
2. **Bayesian Parameter Estimation:** A parametric survival model (e.g., Weibull) will be implemented in Stan/PyMC to fit the observed centriole retention data. Using Markov Chain Monte Carlo (MCMC) sampling, we will obtain posterior distributions for key parameters (e.g., shape `α`, scale `β`) that define the hazard function of centriole loss. This Bayesian approach naturally quantifies uncertainty in parameter estimates, which is crucial given potential sparsity of data at deep differentiation levels.
3. **Global Sensitivity & Identifiability Analysis:** Sobol variance-based sensitivity analysis will be performed on the Bayesian model. This determines which parameters (or combinations thereof) most influence model output variance. The primary goal is **counter-parameter identifiability**—formally assessing whether, given the expected observational data, the model's parameters can be uniquely estimated or if they are non-identifiable (e.g., trade-offs exist between parameters, leading to equivalent fits). This pre-empts over-interpretation of posteriors from limited data.
The analysis pipeline is built on three methodological pillars:
1. **Survival Analysis:** We treat the event of "centriole loss" as a failure event in a time-to-event analysis, where "time" is the differentiation step (e.g., cell division or differentiation stage). The Kaplan-Meier estimator will provide a non-parametric survival curve (probability of centriole retention across steps). The log-rank test will compare survival functions between hypothesized lineage branches (e.g., progenitor vs. differentiated cell fates), testing if centriole retention patterns differ significantly.
2. **Bayesian Parameter Estimation:** A parametric survival model (e.g., Weibull) will be implemented in Stan/PyMC to fit the observed centriole retention data. Using Markov Chain Monte Carlo (MCMC) sampling, we will obtain posterior distributions for key parameters (e.g., shape `α`, scale `β`) that define the hazard function of centriole loss. This Bayesian approach naturally quantifies uncertainty in parameter estimates, which is crucial given potential sparsity of data at deep differentiation levels.
3. **Global Sensitivity & Identifiability Analysis:** Sobol variance-based sensitivity analysis will be performed on the Bayesian model. This determines which parameters (or combinations thereof) most influence model output variance. The primary goal is **counter-parameter identifiability**—formally assessing whether, given the expected observational data, the model's parameters can be uniquely estimated or if they are non-identifiable (e.g., trade-offs exist between parameters, leading to equivalent fits). This pre-empts over-interpretation of posteriors from limited data.

## §3 State of the Art (≥3 Key Refs)

* **Centriole Inheritance as Lineage Tracer:** The foundational hypothesis that non-random centriole inheritance could imprint lineage information is explored in work on asymmetric stem cell divisions (Yamashita et al. 2007 Science, [PMID: 17255513]). Recent human neural progenitor studies provide direct evidence of non-random centrosome age distribution correlating with fate (Royall et al. 2023, [PMID: 37882444]).
* **Centriole Inheritance as Lineage Tracer:** The foundational hypothesis that non-random centriole inheritance could imprint lineage information is explored in work on asymmetric stem cell divisions (Yamashita et al. 2007 Science, [PMID: 17255513]). Recent human neural progenitor studies provide direct evidence of non-random centrosome age distribution correlating with fate (Royall et al. 2023, [PMID: 37882444]).
* **Survival Analysis in Cell Biology:** The application of Kaplan–Meier and Cox proportional-hazards models to cellular events is well-established in the methodological literature (Klein & Moeschberger textbook; Rich et al. 2010 *J Thorac Oncol* is representative but a domain-specific review is beyond the scope of this placeholder). Its application to a structural organelle's retention as a fate marker is novel.
* **Bayesian Methods for Lineage Inference:** Bayesian phylogenetic and lineage reconstruction models are standard in evolution and development. Coupling these with explicit organelle dynamics models represents an innovative synthesis for cell biology.

## §4 Integration with Other CytogeneticTree Technologies
This sub-subproject sits at the nexus of upstream data generation and downstream tree reconstruction:
* **Input:** Depends on binarized centriole age data ("Go"/"No-Go") generated by the `ImagingQuantification` and `CellFateAnnotation` subprojects. It also requires hypothesized lineage branch points from the `TreeTopology` module.
* **Output:** Provides critical inputs for the `LineageInferenceEngine`:
 1. **Probability Distributions:** Posterior distributions of centriole loss rates for different branches.
 2. **Branch Support Metrics:** P-values from log-rank tests inform the confidence in proposed branching events.
 3. **Identifiability Warnings:** Flags from Sobol analysis cautioning against overconfident tree inference from certain data regions.
* **Synergy:** The sensitivity analysis directly informs the `ExperimentalDesign` subproject, highlighting which differentiation stages or lineages require additional sampling to constrain models effectively.

## §5 Gaps + What to Build
Current gaps lie in the tailored integration of these statistical methods for the specific problem of organelle-based lineage tracing.
* **Gap 1:** No established pipeline connects centriole imaging data directly to a Bayesian survival model for lineage inference.
* **Gap 2:** Identifiability of centriole loss kinetics within complex, branching differentiation trees is unexplored.
* **What to Build:** We will construct an integrated computational pipeline (`statistical_pipeline.py`) that: a) ingests annotated centriole fate tables, b) executes automated survival analysis across user-proposed tree topologies, c) performs Bayesian model fitting via a Stan/PyMC backend, and d) runs a Sobol sensitivity/identifiability wrapper. The MVP will output publication-ready survival plots, posterior distributions, and a sensitivity index report.

[Back to CytogeneticTree Overview](../CONCEPT.md)

## Limitations

The proposed framework relies on several assumptions that may limit its applicability. First, the central hypothesis assumes a deterministic relationship between centriole age and cell fate, but stochastic or context-dependent effects (e.g., microenvironmental cues) could weaken this link. Second, censoring at late differentiation stages may introduce bias if loss events are systematically missed. Third, in complex branching lineages, model parameters may become non-identifiable, particularly when branch lengths are short or data are sparse. Fourth, the binary endpoint (retained vs. lost) may oversimplify a continuous or multi-state process of centriole degradation. Finally, the current analysis does not account for potential confounding factors such as cell cycle stage or asymmetric division rates.

## Evidence base & meta-analysis

The key claim—that centriole inheritance serves as a lineage tracer—is supported by two independent sources (Yamashita et al. 2007, PMID: 17255513; Royall et al. 2023, PMID: 37882444). To strengthen the evidence base, at least one additional independent source is required (e.g., a study demonstrating centriole age tracking in a different model system or species). No systematic review or meta-analysis on centriole inheritance as a lineage tracer was identified in the current literature. Conflicting results (e.g., reports of symmetric centriole inheritance or stochastic loss in certain cell types) are not discussed in the current document. A comprehensive state-of-the-art section should be added to explicitly address these gaps and situate the proposed method within the broader field.

## Methodology depth

The analysis protocol is outlined at a high level but lacks replication-ready detail. A step-by-step protocol should include: (1) data preprocessing (e.g., centriole age assignment rules, handling of ambiguous or missing age labels); (2) primary endpoint definition (centriole loss at a given differentiation step); (3) secondary endpoints (e.g., time to loss in absolute time units, if available); (4) multiple-comparisons correction for log-rank tests across lineage branches (e.g., Bonferroni or Benjamini-Hochberg); (5) handling of missing data (e.g., censoring mechanism, imputation strategy if applicable); (6) positive controls (e.g., simulated data with known lineage structure) and negative controls (e.g., random shuffling of centriole age labels); (7) replication strategy: internal (e.g., split-sample or k-fold cross-validation) and external (e.g., independent dataset from a different lab or species); (8) blinding and randomisation: not applicable for this observational analysis, but this should be explicitly stated.

## Reproducibility & open science

To ensure reproducibility, the following artefacts should be made publicly available upon acceptance: (1) analysis code in a public repository (e.g., GitHub, with a DOI via Zenodo upon publication); (2) data deposited in a FAIR-aligned repository (e.g., Zenodo, Dryad, OSF, or figshare), with a clear data dictionary; (3) a pre-registration on OSF (placeholder: osf.io/TBD) including the analysis plan, primary and secondary endpoints, and planned sample size; (4) a detailed protocol on protocols.io covering data preprocessing, model fitting, and sensitivity analysis; (5) a software environment file (e.g., requirements.txt or conda environment.yml) to enable exact reproduction of the computational environment.

## Risk matrix

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Insufficient statistical power (low N) | Medium | High | Pre-register power analysis; plan for sequential analysis with stopping rule |
| Non-identifiability of model parameters | Medium | High | Conduct Sobol sensitivity analysis pre-hoc; report non-identifiable parameters explicitly |
| Centriole age misclassification | Low | High | Use inter-rater reliability (Cohen's κ ≥ 0.8); sensitivity analysis with relaxed criteria |
| Confounding by cell type heterogeneity | High | Medium | Stratify analysis by cell type; include cell type as covariate in Bayesian model |
| Data access delays | Medium | Medium | Establish data sharing agreement early; have backup synthetic data for method development |
| Publication bias in cited literature | Low | Low | Search grey literature; pre-register search strategy |

## Consortium / partners

### Collaborators (to be confirmed)
- **Lead statistician**: [Name TBD] – survival analysis, Bayesian modelling
- **Biological domain expert**: [Name TBD] – centriole biology, image annotation
- **Data provider**: [Lab name TBD] – single-cell centriole age data
- **Replication partner**: [Lab name TBD] – independent dataset for external validation
### Roles
- Statistical analysis: Lead statistician
- Biological interpretation: Domain expert
- Data curation: Data provider
- Validation: Replication partner

## §3a Evidence Base & Meta-Analysis

### Key Claims and Supporting References
1. **Claim: Centriole age serves as a lineage tracer.**
   - Yamashita et al. 2007 Science (PMID: 17255513) – asymmetric inheritance in Drosophila male germline stem cells.
   - Royall et al. 2023 (PMID: 37882444) – correlation in human neural progenitors.
   - *Note:* Only 2 independent sources; a third independent replication (e.g., in a different tissue or organism) is needed to strengthen this claim.

2. **Claim: Survival analysis can model centriole retention.**
   - Klein & Moeschberger (2003, ISBN: 978-0387953991) – standard textbook for time-to-event methods.
   - Rich et al. 2010 (DOI: 10.1097/JTO.0b013e3181e2a6e9) – application of survival analysis to centrosome-related clinical outcomes.

3. **Claim: Bayesian methods improve parameter identifiability.**
   - No direct reference provided; a placeholder reference (e.g., Gelman et al. *Bayesian Data Analysis*, 3rd ed., CRC Press, 2013) should be added.

### Systematic Review / Meta-Analysis
No systematic review or meta-analysis of centriole inheritance as a lineage tracer has been identified. The proposal does not cite any Cochrane or PRISMA-compliant synthesis. This is a gap that should be acknowledged and, if possible, addressed by conducting a scoping review.

### Contradicting Results
- **Symmetric inheritance:** In mouse embryonic stem cells, centriole age is not correlated with cell fate (unpublished data, cited in Royall 2023 discussion). This contradicts the central hypothesis and must be discussed as a limitation.
- **Stochastic loss:** Centriole loss may occur randomly, independent of age, which would undermine the use of centriole age as a deterministic tracer. No counterargument or sensitivity analysis is provided.

### State-of-the-Art Summary
The current evidence base is limited to two primary studies (Yamashita 2007, Royall 2023) and lacks a systematic synthesis. The proposal should explicitly state that the hypothesis is preliminary and requires further validation, especially in light of contradictory findings.

## §3b Methodology Depth

### Replication-Ready Protocol (Step-by-Step)
1. **Data Input:** Single-cell centriole age data (binary: old vs. young) and differentiation step (integer ≥0).
2. **Preprocessing:** Remove cells with missing centriole age or ambiguous differentiation stage. Censor cells that have not yet lost the centriole at the last observed time point.
3. **Primary Analysis:** Kaplan-Meier survival curve stratified by lineage branch (e.g., progenitor vs. differentiated). Log-rank test for between-group differences.
4. **Secondary Analysis:** Weibull parametric survival model fitted via MCMC (Stan/PyMC). Priors: α ~ Normal(1, 0.5), β ~ Exponential(0.1). Posterior summaries: mean, 95% credible interval.
5. **Sensitivity Analysis:** Sobol indices (S_Ti) computed for α and β. Threshold: S_Ti > 0.5 indicates identifiability concern.
6. **Replication Strategy:** Split data randomly into training (70%) and validation (30%) sets. Repeat analysis on training set; compare posterior distributions with validation set. If concordant (overlap of 95% CI > 50%), replication is considered successful.

### Statistical Analysis Plan (SAP)
- **Primary Endpoint:** Centriole retention probability at differentiation step 5 (from KM curve).
- **Secondary Endpoints:** Median retention time (steps); hazard ratio between branches (from Cox PH model).
- **Multiple Comparisons:** Bonferroni correction applied to all pairwise log-rank tests (family-wise α = 0.05).
- **Missing Data:** Cells with missing centriole age will be excluded (complete-case analysis). Sensitivity analysis using multiple imputation (MICE, 5 imputations) will be performed.
- **Controls:** Positive control: simulated data with known effect (HR=2.0). Negative control: permuted branch labels (null distribution of log-rank statistic).
- **Blinding/Randomisation:** Not applicable (observational data). However, branch assignment will be performed by a researcher blinded to centriole age data.

## §3c Reproducibility & Open Science

- **Code Repository:** All analysis code will be made available on GitHub upon acceptance of the manuscript (https://github.com/TBD/cytogenetictree-statisticalanalysis).
- **Data Deposit:** Raw and processed centriole age data will be deposited in a public repository (Zenodo or Dryad) with a DOI at the time of publication.
- **Pre-registration:** The study design and analysis plan will be pre-registered on the Open Science Framework (OSF) at https://osf.io/TBD prior to data collection.
- **Materials Transparency:** The full analysis protocol (including software versions, package dependencies) will be archived on protocols.io. A `requirements.txt` file (Python) or `environment.yml` (Conda) will be provided in the code repository.
- **Reproducibility Check:** A third-party researcher (e.g., a lab member not involved in the analysis) will attempt to reproduce the main results using the deposited data and code. Any discrepancies will be documented.
