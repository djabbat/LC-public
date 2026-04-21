# StatisticalAnalysis: Phase A Milestones (Prioritized)

## P0: Core Pipeline Infrastructure
*   [ ] **S0:** Set up Python environment (`pymc`, `lifelines`, `SALib`, `arviz`) and version control.
*   [ ] **S1:** Create data ingestion function (`load_fate_table()`) for standardized CSV input.
*   [ ] **S2:** Implement basic Kaplan-Meier plotting and log-rank test function (`run_survival_analysis()`).
*   [ ] **S3:** Build and test a basic Weibull survival model in PyMC (`model_weibull.py`).

## P1: Bayesian Workflow & Validation
*   [ ] **S4:** Create MCMC sampling wrapper with default config and basic diagnostics (R-hat, ESS).
*   [ ] **S5:** Generate synthetic data from known parameters and validate model recovery (simulate -> infer).
*   [ ] **S6:** Develop function to visualize posterior distributions and survival function posterior predictive checks.

## P2: Sensitivity & Identifiability Analysis
*   [ ] **S7:** Implement Sobol analysis wrapper (`run_sobol.py`) around the Bayesian model.
*   [ ] **S8:** Test identifiability on synthetic data with known parameter trade-offs (e.g., make `α` and `β` non-identifiable).
*   [ ] **S9:** Produce a standardized identifiability report for a given dataset/model.

## P3: Integration & Documentation
*   [ ] **S10:** Create master pipeline script (`statistical_pipeline.py`) that chains S1->S9.
*   [ ] **S11:** Apply MVP pipeline to first pilot dataset (from `ImagingQuantification`).
*   [ ] **S12:** Document all functions and create a basic tutorial notebook (`demo_analysis.ipynb`).
