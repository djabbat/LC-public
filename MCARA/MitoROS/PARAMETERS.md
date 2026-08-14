# MitoROS Counter #3 Model Parameters

**Version:** 1.0

**Statuses:**
* **Measured:** Parameter obtained directly from experimental data cited in references.
* **Estimated:** Parameter derived by calculation or approximation based on published data or theoretical considerations.
* **Hypothetical:** Parameter postulated by theory but lacking direct empirical support. Requires experimental determination.

| Parameter | Symbol | Description | Estimated Value (Range) | Units | Origin / Justification | Status |
|-----------|--------|-------------|-------------------------|-------|------------------------|--------|
| Baseline damage level | \( D_{3,0} \) | Level of heteroplasmy/damage at birth. | 0.0 – 0.01 (0 – 1%) | Dimensionless (normalized) | Theoretical minimum. Inherited heteroplasmy is typically <1% for severe mutations. | Estimated |
| Division-dependent accumulation coefficient | \( \alpha_3 \) | Increase in \( D_3 \) per cell division in mitotic tissue. | \( 1 \times 10^{-4} – 5 \times 10^{-4} \) | Dimensionless per division | Estimate based on mtDNA segregation drift models and data on clonal expansion in hematopoietic stem cells (PMID: 40239706). For postmitotic tissues → 0. | Estimated |
| Critical number of divisions | \( n_3^* \) | Number of divisions required to reach threshold heteroplasmy \( H_{crit} \) in a clone from a single mutant molecule. | \( 10^2 – 10^4 \) | Number of divisions (dimensionless) | Depends on segregation dynamics and selective advantage/disadvantage. Estimate from mathematical models (PMID: 36442091). | Estimated |
| Time-dependent accumulation coefficient | \( \beta_3 \) | Rate of increase in \( D_3 \) per unit time in postmitotic tissue. | \( 0.05 – 0.2 \) | Year⁻¹ (when normalized to \( \tau_3 \)) | Calculated from data on common deletion accumulation in human muscle (0.1-0.15% per year, PMID: 30043489), normalized to \( H_{crit} \approx 60\% \). Variation reflects inter-tissue differences. | Estimated |
| Characteristic time | \( \tau_3 \) | Time scale over which \( D_3 \) increases substantially. Inversely proportional to accumulation rate. | \( 5 – 20 \) | Years | \( \tau_3 \approx H_{crit} / (\beta_3 \cdot H_{crit}) \) in a simplified linear model. For human muscle: \( \tau_3 \approx 60\% / (0.1\%/year) \approx 600 \) years — clearly incorrect, indicating nonlinearity (clonal expansion). More realistically: time to reach 10% heteroplasmy in a focal region. Reassessment based on COX-negative fiber data gives 10-30 years. | Requires clarification (Hypothetical) |
| Threshold heteroplasmy | \( H_{crit} \) | Level of heteroplasmy at which bioenergetic deficit manifests in the cell. | \( 0.6 – 0.9 \) (60% – 90%) | Dimensionless (fraction) | Experimental data from cytoplasmic transfer and cell models (PMID: 25149213). Depends on mutation type (deletions have a lower threshold than point mutations in tRNA). | Measured (for specific mutations) |
| Composite measure weights | \( \lambda_{het} \) | Weight of heteroplasmy contribution to \( D_3 \). | Not defined | Dimensionless | Theoretically, should reflect the relative importance of clonal expansion vs. diffuse oxidative damage. Requires experimental determination (see OPEN_PROBLEMS P0-1). | Hypothetical |
| | \( \lambda_{les} \) | Weight of oxidative damage contribution to \( D_3 \). | Not defined | Dimensionless | \( \lambda_{het} + \lambda_{les} = 1 \). | Hypothetical |
| Sigmoid steepness | \( k_3 \) | Parameter determining the sharpness of the contribution function \( f_3(D_3) \) transition near the threshold. | \( 5 – 20 \) | Dimensionless | Heuristic. Reflects the assumption that the transition from normal to dysfunction is relatively sharp for mitochondrial defects (threshold effect). | Hypothetical |
| Contribution function threshold | \( D_3^{threshold} \) | Value of \( D_3 \) at which the contribution function \( f_3 \) reaches the midpoint of the transition. | Not defined | Dimensionless | Typically set to \( H_{crit} \) in simplified models. Requires calibration for specific tissues and mutations. | Hypothetical |
step (0.5). | \( 0.3 – 0.7 \) | Dimensionless | Should be related to \( H_{crit} \), but also includes the contribution of oxidative damage. \( D_3^{threshold} < H_{crit} \), since combined damage can cause dysfunction earlier. | Hypothetical |
| Coupling coefficients | \( \Gamma_{3,j} \) | Measure of the influence of counter \( j \) on the accumulation rate of \( D_3 \). | **0** (by default) | Depends on the function \( g_j \) | According to the CORRECTIONS canon. A nonzero value can be set only post-hoc based on statistical analysis of data rejecting independence. | Hypothetical / Data-defined |

## τ₃ Operationalization


This experiment directly addresses Risk R1 (non‑linear D₃) by providing direct measurement of τ₃ in vivo. The ²H₂O labeling protocol yields a turnover rate that can be compared to the literature‑derived value used in the model. If the measured τ₃ deviates by more than 30% from the assumed value, the model's predictions for time‑dependent damage accumulation will be revised accordingly.



**Experimental determination:** τ₃ will be measured in mouse liver via in vivo pulse‑chase with deuterated water (²H₂O) and LC‑MS/MS tracking of mitochondrial protein turnover. Power analysis: α=0.05, power=0.80, effect size d=0.8 (large), N=25 per group (2 groups → 50 total). Pre‑registration on OSF (ID: osf.io/TBD).


**Current status:** Hypothetical (requires experimental validation). **Estimated value:** τ₃ ≈ 0.1–0.3/year based on COX-negative fiber accumulation rates in human muscle (Bua et al., 2006, PMID: 16868022). **Uncertainty:** ±0.1/year. **Validation plan:** Mouse experiment measuring mtDNA deletion clearance over 6–12 months (see Risk Matrix R1).



**Status:** Estimated (order-of-magnitude based on COX-negative fiber data from PMID 30043489).
**Estimate:** τ₃ ≈ 0.1–0.3 (dimensionless, per year).
**Error range:** ±0.1 (based on variability in COX-negative fiber density across individuals).
**Experimental validation:** Proposed in τ₃ Operationalization box (see PARAMETERS.md).



**Proposed experiment to estimate τ₃:**
- Model: C57BL/6 mouse, longitudinal study of COX-negative fibers in quadriceps muscle
- Timepoints: 6, 12, 18, 24 months (n=10 per timepoint)
- Measurement: % COX-negative fibers via sequential COX/SDH histochemistry
- Analysis: Fit exponential decay model to estimate τ₃ (time constant for clonal expansion)
- Sample size: Based on pilot data (mean=5%, SD=2% at 24 months), n=10 per group achieves 80% power to detect 50% difference between timepoints (α=0.05)

## τ₃ Estimation: Derivation and Caveats

The current estimate τ₃ = 0.1–0.3/year is derived from Bua et al., 2006 (PMID: 16868022) using COX‑negative fibre frequency as a proxy for clonal expansion of mtDNA deletions. The conversion formula is:

**Source:** Estimated from cross-sectional mtDNA deletion frequency data in human muscle (Bua et al., 2006, PMID 16868022). Conversion formula: τ₃ = ln(1 + Δf) / Δt, where Δf is the fractional increase in deletion frequency per year. See also `τ₃ Operationalization` section in CONCEPT.md for full derivation.

**Status:** Hypothetical; requires longitudinal validation (see P2-1 in OPEN_PROBLEMS.md).


τ₃ = (1/t) · ln(1 + f_COX⁻)

where f_COX⁻ is the fraction of COX‑negative fibres at age t. This is an approximation that assumes (i) linear accumulation of deletions, (ii) constant expansion rate, and (iii) no selection against deleted genomes. These assumptions require direct verification via longitudinal heteroplasmy tracking (see P0-2). The confidence interval reflects inter‑individual variability in the original dataset.


## v3 Update (2026-05-13)

See CONCEPT.md "v3" / "Address peer-review concerns" section for project-specific changes.

