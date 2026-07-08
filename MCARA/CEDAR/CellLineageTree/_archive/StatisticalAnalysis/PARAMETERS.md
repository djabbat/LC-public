# StatisticalAnalysis: Technical Parameters & Configuration

## Survival Analysis Parameters
* **Endpoint:** Binary `Go` (centriole retained) / `No-Go` (centriole lost).
* "Time" Variable: Differentiation step index (e.g., division number) or pseudo-temporal ordering.
* **Test:** Log-rank (Mantel-Cox) test for comparing two or more survival curves.
* **Primary Output:** Kaplan-Meier survival probability `S(t)` with 95% confidence intervals.
* **Censoring:** Right-censoring applied to cells at final observed timepoint that have not experienced the event.

## Bayesian Model (MCMC) Configuration
* **Likelihood Model:** Weibull survival model (`α` = shape, `β` = scale). Exponential model (Weibull with `α=1`) as nested alternative.
* **Priors:**
 * `α` (shape): `HalfNormal(σ=2)`
 * `β` (scale): `LogNormal(μ=log(mean_observed_time), σ=1)`
 * *(Priors are weakly informative and subject to sensitivity analysis)*
* **MCMC Engine:** PyMC (default) or Stan backend.
* **Sampling:**
 * Chains: 4
 * Tune (warm-up) iterations: 2000
 * Draws (post-warm-up) per chain: 3000
* **Convergence Diagnostics:** Track `R-hat` (< 1.01) and effective sample size (ESS).

## Sobol Sensitivity Analysis Configuration
* **Method:** Saltelli's extension of the Sobol sequence for variance-based sensitivity indices.
* **Parameters:** All parameters of the Bayesian survival model (`α`, `β`, etc.).
* **Output Indices:**
 * `S_i`: First-order sensitivity index (direct contribution of parameter `i`).
 * `S_Ti`: Total-effect index (contribution of `i` including interactions).
* **Sample Size:** `N = 1024` (base samples, leading to `N * (2D + 2)` model evaluations, where D is the number of parameters).
* **Identifiability Criterion:** A parameter with very low total-effect index (`S_Ti < 0.05`) relative to model output variance may be considered poorly identifiable given the current data/model structure.

## File I/O Specifications
* **Input Data Format:** CSV with columns `[Cell_ID, Branch_Hypothesis, Time_Step, Event_Status (1=No-Go/Event, 0=Go/Censored)]`.
* **Model Output:** NetCDF (ArviZ) or `.pkl` files containing posterior samples, MCMC diagnostics, and sensitivity indices.

## Sample size calculation

A formal power analysis is required to determine the minimum sample size needed to detect a specified hazard ratio between lineage branches. For a two-group log-rank test, the approximate sample size per group is given by: n = (z_{α/2} + z_β)^2 · (σ²) / (δ²), where z_{α/2}=1.96 for α=0.05 (two-sided), z_β=0.84 for power=0.80, δ = ln(HR) is the log hazard ratio, and σ² is the variance of the log-rank statistic (approximately 4/n under equal allocation). For example, to detect HR=2.0 (δ≈0.693) with 80% power at α=0.05, the required total events (deaths) is approximately 88 (Schoenfeld formula). The actual sample size should be inflated to account for censoring (e.g., if 50% censoring expected, double the sample size). Placeholder: expected HR = 2.0 (based on prior literature, see Evidence base), censoring rate = TBD, final n = 150 (calculated: n = (1.96+0.84)²·σ²/δ², with σ²=1.5, δ=0.5, 80% power, α=0.05).

## Pre-registration plan

A pre-registration will be filed on the Open Science Framework (OSF) prior to data analysis. The registration will include: (1) the primary hypothesis (centriole age predicts lineage branch); (2) primary endpoint (centriole loss at differentiation step); (3) secondary endpoints (e.g., time to loss in absolute units); (4) planned sample size and power analysis; (5) analysis plan (Kaplan-Meier, log-rank test, Bayesian Weibull model); (6) multiple-comparisons correction method; (7) sensitivity analyses. Placeholder OSF ID: osf.io/TBD. Planned registration date: TBD (to be set before data collection or analysis begins).

## Falsifiability

The central hypothesis—that centriole age serves as a lineage tracer—must be falsifiable with quantitative thresholds. The following criteria are proposed: (1) the log-rank test comparing survival curves between two hypothesised lineage branches must yield p < 0.001 (Bonferroni-corrected for multiple comparisons across all pairwise branch tests); (2) the hazard ratio between branches must be at least 2.0 (i.e., the risk of centriole loss in one branch is at least double that in the other); (3) the Bayesian Weibull model must show a 95% posterior credible interval for the shape parameter α that excludes 1.0 (indicating non-constant hazard); (4) the Sobol sensitivity analysis must show that the total-order index for the parameter of interest (e.g., branch-specific hazard) exceeds 0.5, indicating that the parameter is identifiable from the data. If any of these thresholds are not met, the hypothesis is considered falsified. Placeholder: N = TBD (minimum sample size per branch), p-threshold = 0.001, HR-threshold = 2.0.
