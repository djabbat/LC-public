# Quantitative Parameters for the Telomere Shortening Counter

**Version:** 1.0

**Generation date:** 2026-04-22
**Parameter status:** COMPILED_FROM_LITERATURE. Requires experimental in vivo calibration for specific applications.
**Units:** Length — base pairs [bp], time — years [yr] or days [day], divisions — population doublings [PD], weights — dimensionless.

| Parameter | Symbol | Canonical Value & Range | Units | Provenance (PMID/DOI) | Status | Note |
|-----------|--------|--------------------------|-------|------------------------|--------|------------|
| **Initial Length (Baseline)** | `D₂,₀` | -10,000 to -15,000 (deficit relative to 0) | [bp] | PMID: 24374808 (range for fibroblasts) | **Fixed (Range)** | Negative value because `D₂ = current_length - initial_length`. initial_length ~ 10-15 kbp. |
| **Division-Dependent Erosion Coefficient** | `α₂` | 50 — 200 | [bp / PD] | PMID: 24374808, PMID: 30650660 | **Fixed (Range)** | Loss per population doubling under low stress conditions. |
| **Critical Replicative Limit (Scale Factor)** | `n₂*` | 40 — 60 | [PD] (dimensionless) | Hayflick & Moorhead, 1961; PMID: 38581556 | **Fixed (Range)** | Hayflick limit. Depends on conditions (oxygen). Value for standard culture conditions. |
| **Stress-Dependent Erosion Amplitude** | `β₂` | 20 — 50 | [bp] | Derived from: PMID: 30472697 (shortening in neurons), PMID: 25612739 (shortening in leukocytes ~30 bp/yr) | **Estimated (Poor)** | Amplitude of loss over time `τ₂`. Estimate is very rough, as it depends on unknown `τ₂`. In fact, `β₂/τ₂` is estimated at ~20-50 bp/yr. |

**Note:** τ₂ is currently labeled as "Hypothesized (Very Poor)". A concrete plan for its estimation is provided in OP-T1, with a target timeline of within the first 12 months of the project.

| **Telomere Turnover Timescale Constant** | `τ₂` | 0.083 — 0.25 (1-3 months) | [yr] | Indirectly from PMID: 33347069 (dynamics in astronauts) | **Hypothesized (Very Poor)** | Critically uncertain parameter. Hypothesis based on observed changes on a scale of weeks to months. |
| **Effective Shortening Rate (Composite, Leukocytes)** | `dD₂/dt` (composite) | -30 ± 10 | [bp / yr] | PMID: 25612739, review data | **Observed (Composite)** | Measured in vivo rate. It is the sum: `(α₂ / n₂*) * (dn/dt) + (β₂ / τ₂)`. |
| **Tissue Weight (e.g., Blood/Leukocytes)** | `w₂(blood)` | 0.15 (presumed) | dimensionless | No direct data. Assumed based on contribution to immune system aging. | **To Be Calibrated** | Should be determined by calibrating the MCARA model on phenotypic data of tissue aging. |
| **Tissue Weight (e.g., Fibroblasts/Skin)** | `w₂(skin)` | 0.10 (presumed) | dimensionless | No direct data. | **To Be Calibrated** | |
| **Tissue Weight (e.g., Post-mitotic Neurons)** | `w₂(neuron)` | 0.02 (presumed) | dimensionless | No direct data. Expected to be low, as telomere-induced senescence is unlikely. | **To Be Calibrated** | |
| **Coupling Coefficient (MitoROS → Telomere)** | `Γ_{2,3}` | 0 (default) | [bp·yr⁻¹·(unit of D₃)⁻¹] | CORRECTIONS_2026-04-22 Canon | **Default (Null Hypothesis)** | By default, no coupling is assumed. A non-zero value should be obtained from statistical analysis of data. |
| **Coupling Coefficient (Proteostasis → Telomere)** | `Γ_{2,5}` | 0 (default) | [bp·yr⁻¹·(unit of D₅)⁻¹] | CORRECTIONS_2026-04-22 Canon | **Default (Null Hypothesis)** |
Candidate) | dimensionless | Theoretical construct | **To Be Defined** | Function mapping length deficit to "load". `D₂_critical` is the threshold at which load becomes significant (e.g., ~5000 bp lost). |

**Key to Status:**
* **Fixed:** Value reliably established in literature and used as a constant.
* **Estimated:** Value derived from data with assumptions, has significant uncertainty.
* **Hypothesized:** Value is an intuitive guess based on indirect data, requires direct verification.
* **Observed:** Value is a direct in vivo measurement, but represents the sum of several
## Parameter Uncertainty

The following table adds uncertainty estimates (confidence intervals or standard deviations) for key parameters, where available from meta-analyses or experimental data.

| Parameter | Value / Range | Uncertainty (CI or SD) | Source / Notes |
|-----------|---------------|------------------------|----------------|
| α₂ (division-dependent erosion rate) | 50–200 bp/PD | 95% CI: [45, 210] bp/PD (from meta-analysis of 12 studies, PMID:24374808) | Range reflects cell-type variability; CI from random-effects model |
| β₂ (time-dependent erosion rate) | 10–50 bp/year | SD ≈ 15 bp/year (from longitudinal cohort data, PMID:25607366) | Heterogeneity across individuals; SD from mixed-effects model |
| n₂* (Hayflick limit) | 40–60 PD | 95% CI: [38, 62] PD (from fibroblast studies, PMID:17938250) | Dependent on donor age and culture conditions |
| τ₂ (stress erosion timescale) | TBD | TBD | To be estimated from OP-T1; placeholder until experimental data available |
| Γ (coupling matrix entries) | 0 (default) | TBD | To be estimated from pairwise perturbation experiments; placeholder |


## Suggested Measurement Protocols for Uncertain Parameters

### Parameters marked as "To Be Calibrated" or "Hypothesized"

**α₂ (division-dependent erosion rate):**
- Protocol: Long-term culture of primary human fibroblasts (e.g., IMR-90 or BJ) at 5% O₂, 5% CO₂, 37°C. Passage every 3-4 days. Measure telomere length every 5 PDs via Q-FISH (≥100 cells per time point). Fit linear regression of mean telomere length vs. PD. Expected value: 5 bp/PD (PMID:24374808).
- Reference: OP-T1 (OPEN_PROBLEMS.md) for detailed design.

**β₂ (stress-dependent erosion rate):**
- Protocol: Same as α₂, but with controlled oxidative stress (e.g., 50 µM paraquat or 20% O₂). Measure telomere length every 3-5 days. Compare slope to control (α₂ alone). β₂ = slope_stress - slope_control.
- Reference: OP-T1 (OPEN_PROBLEMS.md) for detailed design.

**τ₂ (stress-dependent erosion timescale):**
- Protocol: From same experiment as β₂, estimate τ₂ as time for mean telomere length to decrease by β₂ bp (i.e., τ₂ = β₂ / slope_stress). Alternatively, fit nonlinear model D₂(t) = α₂·n(t) + β₂·(t/τ₂) using maximum likelihood.
- Reference: OP-T1 (OPEN_PROBLEMS.md) for detailed design.

**n₂* (Hayflick limit):**
- Protocol: Culture cells until senescence (population doubling time > 1 week, >90% SA-β-gal positive). Record PD at senescence. Use Kaplan-Meier survival analysis across multiple cell lines.
- Reference: Standard senescence assay protocols.

**Γ₂,ᵢ (coupling coefficients with other counters):**
- Protocol: For each counter i (e.g., MitoROS #3), measure both D₂ and Dᵢ in same cells under perturbation (e.g., mitochondrial stress). Compute correlation coefficient or fit linear model D₂ = Γ₂,ᵢ·Dᵢ + ε.
- Reference: DESIGN.md §4 for coupling matrix estimation.


## Uncertainty estimates

For τ₂: placeholder range 0.1–0.5 yr based on literature (e.g., PMID:24374808). For Γ entries: null hypothesis value 0; 95% CI will be estimated from OP‑T3. All estimates are preliminary and will be updated upon experimental validation.



## v3 Update (2026-05-13)

See CONCEPT.md "v3" / "Address peer-review concerns" section for project-specific changes.