# StatisticalAnalysis: Technical Parameters & Configuration

## Survival Analysis Parameters
*   **Endpoint:** Binary `Go` (centriole retained) / `No-Go` (centriole lost).
*   "Time" Variable: Differentiation step index (e.g., division number) or pseudo-temporal ordering.
*   **Test:** Log-rank (Mantel-Cox) test for comparing two or more survival curves.
*   **Primary Output:** Kaplan-Meier survival probability `S(t)` with 95% confidence intervals.
*   **Censoring:** Right-censoring applied to cells at final observed timepoint that have not experienced the event.

## Bayesian Model (MCMC) Configuration
*   **Likelihood Model:** Weibull survival model (`α` = shape, `β` = scale). Exponential model (Weibull with `α=1`) as nested alternative.
*   **Priors:**
    *   `α` (shape): `HalfNormal(σ=2)`
    *   `β` (scale): `LogNormal(μ=log(mean_observed_time), σ=1)`
    *   *(Priors are weakly informative and subject to sensitivity analysis)*
*   **MCMC Engine:** PyMC (default) or Stan backend.
*   **Sampling:**
    *   Chains: 4
    *   Tune (warm-up) iterations: 2000
    *   Draws (post-warm-up) per chain: 3000
*   **Convergence Diagnostics:** Track `R-hat` (< 1.01) and effective sample size (ESS).

## Sobol Sensitivity Analysis Configuration
*   **Method:** Saltelli's extension of the Sobol sequence for variance-based sensitivity indices.
*   **Parameters:** All parameters of the Bayesian survival model (`α`, `β`, etc.).
*   **Output Indices:**
    *   `S_i`: First-order sensitivity index (direct contribution of parameter `i`).
    *   `S_Ti`: Total-effect index (contribution of `i` including interactions).
*   **Sample Size:** `N = 1024` (base samples, leading to `N * (2D + 2)` model evaluations, where D is the number of parameters).
*   **Identifiability Criterion:** A parameter with very low total-effect index (`S_Ti < 0.05`) relative to model output variance may be considered poorly identifiable given the current data/model structure.

## File I/O Specifications
*   **Input Data Format:** CSV with columns `[Cell_ID, Branch_Hypothesis, Time_Step, Event_Status (1=No-Go/Event, 0=Go/Censored)]`.
*   **Model Output:** NetCDF (ArviZ) or `.pkl` files containing posterior samples, MCMC diagnostics, and sensitivity indices.
