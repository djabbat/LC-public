# DifferentiationAnnotation: Technical Parameters & Specifications

## Data Input Specifications
* **Lineage DAG Format:** Graph (JSON/GraphML). Nodes require fields: `cell_id`, `birth_timeframe`, `spatial_coordinates_um`, `parent_id(s)`, `centriole_id_tuple`.
* **scRNA-seq Data:** Minimum 10,000 cells per experiment. Required alignment to reference genome (e.g., GRCh38). Depth: >50,000 reads per cell.
* **Morphology Features:** Vector of ≥50 features per cell per time point (e.g., volume, eccentricity, solidity, texture). Derived from 3D segmentations.
* **Immunofluorescence Anchors:** Minimum of 3 distinct protein markers per target tissue, with validated specificity. Confocal or super-resolution images.

## Algorithmic Thresholds (MVP)
* **Registration Confidence:** Spatial-temporal alignment requires a minimum correlation score of `R > 0.7` for anchor node assignment.
* **Transcriptomic Annotation:** A cell node is assigned a state label if the Spearman correlation of its imputed expression vector to a reference signature exceeds `ρ > 0.5`.
* **Lineage Propagation Constraint:** State transitions must conform to a pre-defined `differentiation_grammar.yaml` file. Illegitimate transitions invalidate propagation paths.

## Computational Resource Estimates
* **Storage (per experiment):** ~500 GB (raw images + sequencing + graphs).
* **Memory (for propagation):** ~32 GB RAM for a tree of 50,000 nodes.
* **Compute Time (MVP pipeline):** Estimated 48-72 hours on a high-core (32+) CPU server.

## Tool & Version Dependencies
* **Primary Language:** Python 3.10+
* **Core Libraries:** scikit-learn 1.3+, NetworkX 3.0+, anndata 0.9+, scanpy 1.9+
* **Registration Engine:** Custom (to be built), may leverage Elastix/SimpleITK for image component.
* **Visualization:** Graph visualization via Cytoscape 3.9+ or custom Plotly Dash app.

## Budget Lines (Conceptual)
* **Personnel:** 1.5 FTE (Bioinformatician + Computational Biologist).
* **Cloud Compute:** $15k/year for pipeline development and testing.
* **Software Licensing:** Open-source stack; budget allocated for potential commercial graph DB integration ($5k).

## Falsifiability

To ensure the scientific claims of this project are testable, the following falsifiable hypotheses with numeric thresholds are defined:

**H0 (Null):** The proportion of correctly annotated nodes in the lineage DAG (compared to ground-truth IF anchor points) is ≤ 0.30.

**H1 (Alternative):** The proportion of correctly annotated nodes is ≥ 0.50.

**Statistical parameters:**
- Significance level (α): 0.05
- Statistical power (1-β): 0.80
- Effect size (Cohen's h): 0.41 (arcsine difference between 0.30 and 0.50)
- Required sample size (N): TBD (to be calculated using power.prop.test in R)

**Additional testable predictions:**
- The correlation between centriole age asymmetry and transcriptomic differentiation state (measured by Spearman's ρ) must exceed 0.5 with p < 0.001 to support the lineage barcode hypothesis.
- If the observed ρ ≤ 0.3 with p > 0.05, the centriole age tracking method will be considered invalid for lineage reconstruction.
- The constrained propagation algorithm must improve annotation accuracy by at least 15 percentage points over unconstrained baseline (random forest without DAG constraints) on held-out test data.

## Sample size calculation

**Power analysis for primary endpoint (annotation accuracy):**

Using the formula for two-proportion z-test:
n = (Z_α/2 + Z_β)² × [p₁(1-p₁) + p₂(1-p₂)] / (p₁ - p₂)²

Where:
- Z_α/2 = 1.96 (for α = 0.05, two-tailed)
- Z_β = 0.84 (for power = 0.80)
- p₁ = 0.50 (expected accuracy under H1)
- p₂ = 0.30 (null threshold)

n = (1.96 + 0.84)² × [0.5(0.5) + 0.3(0.7)] / (0.5 - 0.3)²
n = 7.84 × [0.25 + 0.21] / 0.04
n = 7.84 × 0.46 / 0.04
n = 90.16

**Required minimum:** 91 ground-truth anchor nodes (IF-validated cells) per experiment.

**Note:** The previously stated "10,000 cells per experiment" is a technical throughput estimate, not a statistically justified sample size. The actual power analysis above supersedes that number for hypothesis testing purposes. Final N may be adjusted based on expected effect size from pilot data (TBD).
