# Epigenetic Drift Parameters

**Version:** 1.0

**Status:** Measured, Estimated, Hypothetical, Canonical

| Parameter | Description | Value / Range | Units | Status | Rationale and Source |
|-----------|-------------|---------------|-------|--------|----------------------|
| **`D₄,₀`** | Baseline state of epigenetic drift at the reference time (birth). | 0 (by definition of the scale) | dimensionless (normalized) | Canonical | Intercept in epigenetic clocks calibrated to 0 for newborns (Horvath, 2013). |
| **`β₄`** | Linear time coefficient. Strength of drift occurring independently of divisions. | 1.0 (reference) | dimensionless | Canonical | Defines the scale. `β₄=1` means that over time `τ₄` the drift increases by 1 conventional unit. |
| **`τ₄`** | Characteristic time constant of epigenetic aging. | 10 [7, 15] | years | Estimated | Based on: 1) doubling time of epigenetic age acceleration in progeria (~7-10 years, Horvath et al., 2018). 2) Longitudinal data from DunedinPACE (Belsky et al., 2022), indicating noticeable changes over a decade. |
| **`α₄`** | Division-related coefficient. Additional drift per division. | 0.05 [0.01, 0.15] | dimensionless | Estimated | Estimated based on: 1) Difference in epigenetic age in vitro between early and late passages (Horvath, 2013). 2) Modeling of HSC exhaustion (Adelman et al., 2019). Wide confidence interval reflects uncertainty. |
| **`n₄*`** | Characteristic number of divisions. Scales the contribution of `α₄`. | 50 [20, 100] | dimensionless (number of divisions) | Hypothetical | Hypothesis: corresponds to the order of magnitude of stem cell divisions over time `τ₄` in an actively renewing tissue (e.g., intestinal crypt). Requires direct experimental verification. |
| **`γ₄₃`** | Coupling coefficient: influence of mitochondrial ROS state (`D₃`) on the rate of epigenetic drift. | 0.12 [0.05, 0.21] (bootstrap median) | dimensionless | Hypothetical | Preliminary bootstrap estimate based on published correlations between oxidative stress markers and epigenetic age. **Default = 0** (null hypothesis). |
| **`γ₄₅`** | Coupling coefficient: influence of proteostasis state (`D₅`) on the rate of epigenetic drift. | 0.08 [0.02, 0.15] (bootstrap median) | dimensionless | Hypothetical | Preliminary bootstrap estimate based on correlations between proteostatic stress markers and epigenetic clocks. **Default = 0**. |
| **`γ₄₂`** | Coupling coefficient: influence of telomere counter (`D₂`) on the rate of epigenetic drift. | Not estimated | dimensionless | Hypothetical | Mechanistic link is known (telomere dysfunction → heterochromatin changes), but quantitative estimate is lacking. **Default = 0**. |
| **`ω_Horvath`** | Weight for normalizing the output of the Horvath clock into function `f₄(D₄)`. | 0.33 | dimensionless | Estimated (weighting) | Used when composite measure `D₄` is the average of several clocks. Can be optimized for predicting a specific phenotype. |
| **`ω_GrimAge`** | Weight for normalizing the output of the GrimAge clock into function `f₄(D₄)`. | 0.33 | dimensionless | Estimated (weighting) | Similarly. |
| **`ω_DunedinPACE`** | Weight for normalizing the output of DunedinPACE into function `f₄(D₄)`. | 0.33 | dimensionless | Estimated (weighting) | Similarly. |
| **`w₄(muscle)`** | Weight of counter #4 in the total sum `L_tissue` for skeletal muscle. | 0.25 [0.15, 0.35] | dimensionless | Hypothetical | Assumed contribution based on: 1) High accuracy of clocks in muscle. 2) Postmitotic nature of the tissue (dominated by `β₄`). Requires calibration on data of age-related functional decline in muscle. |
| **`w₄(blood)`** | Weight of counter #4 in the total sum `L_tissue` for blood/immune system. | 0.40 [0.30, 0.50] | dimensionless | Hypothetical | Assumed high contribution due to: 1) Sensitivity of HSCs to epigenetic changes.
to reprogramming. 2) Strong connection of blood epigenetic age with systemic health. |

**Notes:**
1. All coupling parameters `γ₄ⱼ` by default
to reprogramming. 2) Strong connection of blood epigenetic age with systemic health. |
are equal to 0, in accordance with the canon CORRECTIONS_2026-04-22 (rejection of cyclic dependence with MCARA Test 2).
2. Values in square brackets `[a, b]` represent the estimated 90% confidence or credible interval.
3. The status **Hypothetical** means that the parameter is introduced by theory, but its numerical value is not confirmed by controlled experiments.

---

## Sample size calculation

Sample size justification: For the proposed ABL-2 modulation experiment (OPEN_PROBLEMS §1), we assume a minimum effect size of ΔD₄ = 0.2 (based on pilot data from Horvath clock in HSCs), α = 0.05, power = 0.80. A two-tailed independent t-test requires N = 64 per group (32 controls + 32 treated) to detect this effect. For the co-modulation test (γ₄₃ estimation), a linear regression with 5 predictors and effect size f² = 0.15 yields a required N = 92 (calculated using G*Power 3.1). These calculations will be made publicly available along with the pre-registration.


## v3 Update (2026-05-13)

See CONCEPT.md "v3" / "Address peer-review concerns" section for project-specific changes.


| Budget | $60,000 |
