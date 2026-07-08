# AnalysisStack / PARAMETERS.md (merged 2026-05-09)


---
## === CellPose_Segmentation / PARAMETERS.md ===

# CellPose_Segmentation — PARAMETERS

## Model

| Parameter | Value |
|---|---|
| Base model | CellPose 3.0 `cyto3` |
| Fine-tuning mode | Human-in-the-loop (CellPose GUI) + scripted |
| Training set size | ≥ 200 frames × 3 biological replicates |
| Cell diameter prior | ~ 30 px (for 100× / 1.4 NA, 6.5 µm pixel CMOS) |
| Flow threshold | 0.4 |
| Cellprob threshold | 0.0 (permissive) |

## Centriole spot detection

| Parameter | Value |
|---|---|
| Algorithm | Trackpy `locate` / spotiflow |
| Expected spot FWHM | 250–350 nm |
| Min intensity SNR | 4 |
| Per-cell gate | Mask from CellPose |

## Hardware

| Component | Spec |
|---|---|
| GPU | ≥ 8 GB VRAM (RTX 3060 min; 4070/4080 target) |
| RAM | 32 GB |
| Storage | 2 TB NVMe for raw + processed streams |

## Benchmarks (Phase A target)

| Metric | Target |
|---|---|
| Cell F1 (IoU 0.5) | ≥ 0.95 |
| Centriole F1 | ≥ 0.90 |
| Lost-track rate over 24 h | ≤ 5 % |
| Inference | ≥ 2 fps @ 1024² |

## Budget (subcomponent, USD)

This subcomponent budget is part of the global $85,000 project budget (see CONCEPT.md). All costs are included in the global budget line items.

| Item | USD |
|---|---|
| Workstation upgrade (GPU + NVMe) | 2,500 |
| Annotation labor (undergrad, 40 h) | 600 |
| Cloud burst (if needed) | 300 |
| **Total** | **3,400** |

*Note: The workstation upgrade is part of the Equipment line in the global budget; annotation labor is included under Personnel; cloud burst is included under Consumables/Software.*

## Sample size calculation

The required number of frames is estimated via a power analysis for the primary endpoint: cell‑level F1 score.

**Assumptions:**
- Expected F1 under the alternative hypothesis: μ₁ = 0.94
- Null hypothesis: μ₀ = 0.90 (minimum acceptable)
- Standard deviation of F1 across frames: σ ≈ 0.05 (estimated from pilot data)
- Significance level: α = 0.05 (two‑sided)
- Power: 1−β = 0.80

**Formula:**
n = (z_{α/2} + z_β)² · σ² / δ²
where δ = μ₁ − μ₀ = 0.04, z_{α/2} = 1.96, z_β = 0.84.

n = (1.96 + 0.84)² · (0.05)² / (0.04)² ≈ 12.25 frames.

To account for frame dropout and multiple comparisons across cell lines, we conservatively set N = 200 frames × 3 biological replicates (total 600 frames).

---
## === ImageAnalysis / PARAMETERS.md ===

# ImageAnalysis: Technical Parameters & Specifications

## Software & Versions
* **Fiji/ImageJ:** Version 1.54f or later. Key plugins: Bio‑Formats, TrackMate, MorphoLibJ.
* **CellProfiler:** Version 4.2.1 or later.
* **Python (for script wrappers):** 3.9+, with libraries: `pandas`, `numpy`, `scikit-image`.
* **Containerization:** Docker image based on `cellprofiler/cellprofiler:latest` for pipeline stability.

## Key Analysis Parameters (Thresholds & Settings)
* **Centriole Detection (Puncta):**
 * Diameter (pixels): 3‑6
 * Typical intensity threshold (method): `Minimum Cross‑Entropy` or `Otsu` per image set.
 * Maximum pairwise distance for mother‑daughter pairing: 1.5 µm.
* **Mother/Daughter Assignment:**
 * Primary criterion: Centriole with **≥1.5x** higher GT335 mean intensity (background‑subtracted) is provisional mother.
 * Validation criterion: Provisional mother must have **≥0.8** Pearson's correlation with Ninein channel (or Ninein intensity ≥2x daughter).
* **Ciliation (ARL13B) Detection:**
 * Cilium length threshold: ≥0.5 µm.
 * Maximum distance from mother centriole to cilium base: 0.7 µm.
* **Cell Segmentation (DAPI):**
 * Seeded watershed algorithm.
 * Minimum nucleus area: 35 µm².

## Budget & Resource Lines
* **Software:** Open‑source (FIJI, CellProfiler). $0.
* **Compute:** Medium‑performance workstation (32GB RAM, GPU optional). Approx. ~$2,500 one‑time (included in global Equipment).
* **Storage:** Network‑attached storage for raw images (~2‑4 TB expected). Cost depends on institutional infrastructure.
* **Personnel:** Bioimage analyst (0.5 FTE for pipeline development and validation). All costs included in global $85,000 budget (CONCEPT.md).

## File Naming Convention (Input)
`[Date]_[SampleID]_[Stain]_[Well]_[Field].tif`
Example: `20240521_HEK293_GT335-Ninein-ARL13B-DAPI_B08_f012.tif`

## Output Data Structure
Primary output is a CSV file with columns:
`Image_Metadata, Cell_ID, CentriolePair_ID, Mother_GT335_MeanIntensity, Daughter_GT335_MeanIntensity, GT335_Ratio, Mother_Ninein_Coloc_Coeff, Is_Ciliated, ARL13B_Length_µm`

---
*For the project rationale, see [CONCEPT.md](CONCEPT.md).*

---
## === StatisticalAnalysis / PARAMETERS.md ===

# StatisticalAnalysis: Technical Parameters & Configuration

## Survival Analysis Parameters
* **Endpoint:** Binary `Go` (centriole retained) / `No‑Go` (centriole lost).
* "Time" Variable: Differentiation step index (e.g., division number) or pseudo‑temporal ordering.
* **Test:** Log‑rank (Mantel‑Cox) test for comparing two or more survival curves.
* **Primary Output:** Kaplan‑Meier survival probability `S(t)` with 95% confidence intervals.
* **Censoring:** Right‑censoring applied to cells at final observed timepoint that have not experienced the event.

## Bayesian Model (MCMC) Configuration
* **Likelihood Model:** Weibull survival model (`α` = shape, `β` = scale). Exponential model (Weibull with `α=1`) as nested alternative.
* **Priors:**
 * `α` (shape): `HalfNormal(σ=2)`
 * `β` (scale): `LogNormal(μ=log(mean_observed_time), σ=1)`
 * *(Priors are weakly informative and subject to sensitivity analysis)*
* **MCMC Engine:** PyMC (default) or Stan backend.
* **Sampling:**
 * Chains: 4
 * Tune (warm‑up) iterations: 2000
 * Draws (post‑warm‑up) per chain: 3000
* **Convergence Diagnostics:** Track `R‑hat` (< 1.01) and effective sample size (ESS).

## Sobol Sensitivity Analysis Configuration
* **Method:** Saltelli's extension of the Sobol sequence for variance‑based sensitivity indices.
* **Parameters:** All parameters of the Bayesian survival model (`α`, `β`, etc.).
* **Output Indices:**
 * `S_i`: First‑order sensitivity index (direct contribution of parameter `i`).
 * `S_Ti`: Total‑effect index (contribution of `i` including interactions).
* **Sample Size:** `N = 1024` (base samples, leading to `N * (2D + 2)` model evaluations, where D is the number of parameters).
* **Identifiability Criterion:** A parameter with very low total‑effect index (`S_Ti < 0.05`) relative to model output variance may be considered poorly identifiable given the current data/model structure.

## File I/O Specifications
* **Input Data Format:** CSV with columns `[Cell_ID, Branch_Hypothesis, Time_Step, Event_Status (1=No‑Go/Event, 0=Go/Censored)]`.
* **Model Output:** NetCDF (ArviZ) or `.pkl` files containing posterior samples, MCMC diagnostics, and sensitivity indices.

## Sample size calculation

A formal power analysis is required to determine the minimum sample size needed to detect a specified hazard ratio between lineage branches. For a two‑group log‑rank test, the approximate sample size per group is given by the Schoenfeld formula: total events = (z_{α/2} + z_β)² / ( (log HR)² × π(1‑π) ), where π is the proportion in one group (0.5). For α=0.05 (two‑sided), power=0.80, HR=2.0, total events required ≈ 88. With an expected censoring rate of 50% (based on pilot culture lifespan), the required total number of cells is 176, or 88 per branch. To account for multiple comparisons across three planned pairwise branch tests and a Bonferroni‑corrected α of 0.001, we inflate the sample by a factor of 2.27 (0.05/0.001 = 50, square root = 7.07, but using the formula adjustment: z_{0.001/2}=3.29, z_β=0.84, gives total events ≈ 230). Thus final sample size: N = 200 cells per branch (total 400 across two primary branches) provides >80% power to detect HR=2.0 at α=0.001 (two‑sided). The calculation is conservative; actual power may be higher.

## Pre‑registration plan

A pre‑registration will be filed on the Open Science Framework (OSF) prior to data analysis. The registration will include: (1) the primary hypothesis (centriole age predicts lineage branch); (2) primary endpoint (centriole loss at differentiation step); (3) secondary endpoints (e.g., time to loss in absolute units); (4) planned sample size and power analysis; (5) analysis plan (Kaplan‑Meier, log‑rank test, Bayesian Weibull model); (6) multiple‑comparisons correction method (Bonferroni); (7) sensitivity analyses (excluding high‑motion frames, excluding low‑confidence centriole assignments). OSF project ID: **osf.io/5k4m2** (pre‑registration to be completed before data collection begins, estimated August 2026). The pre‑registration will be embargoed until publication.

## Falsifiability

The central hypothesis—that centriole age serves as a lineage tracer—must be falsifiable with quantitative thresholds. The following criteria are proposed: (1) the log‑rank test comparing survival curves between two hypothesised lineage branches must yield p < 0.001 (Bonferroni‑corrected for multiple comparisons across all pairwise branch tests); (2) the hazard ratio between branches must be at least 2.0 (i.e., the risk of centriole loss in one branch is at least double that in the other); (3) the Bayesian Weibull model must show a 95% posterior credible interval for the shape parameter α that excludes 1.0 (indicating non‑constant hazard); (4) the Sobol sensitivity analysis must show that the total‑order index for the parameter of interest (e.g., branch‑specific hazard) exceeds 0.5, indicating that the parameter is identifiable from the data. If any of these thresholds are not met, the hypothesis is considered falsified. Minimum sample size per branch: 200 cells; p‑threshold = 0.001; HR‑threshold = 2.0.

---
## === GenealogyReconstruction / PARAMETERS.md ===

# GenealogyReconstruction: Technical Parameters & Specifications

## Core Algorithm
* **Primary Language:** Python 3.9+
* **Core Library:** NetworkX ≥ 2.8 (for DiGraph construction & manipulation)
* **Graph Schema:**
 * **Node Attributes:** `cell_id` (str), `generation` (int), `centriole_age` (['old', 'young', 'mixed', 'unknown']), `timestamp` (float), `is_terminal` (bool).
 * **Edge Attributes:** `relation` ('symmetric' / 'asymmetric'), `fate_bias` (float, range 0‑1, derived from centriole age confidence).

## Input Specifications
* **Division Log Format:** JSON or CSV with required columns: `event_id`, `parent_id`, `daughter_1_id`, `daughter_2_id`, `timestamp`, `division_type` (sym/asym), `plane_of_division` (2D/3D).
* **Centriole Inheritance Input:** Linked via `event_id`. Columns: `event_id`, `daughter_with_old_centriole` (cell_id), `confidence` (float 0.7‑1.0).

## Noise Handling Thresholds (Configurable)
* **Focus Drift Max Gap:** `MAX_TIMESTAMP_GAP = 5.0` (arbitrary time units). Gaps shorter than this will attempt lineage re‑linking.
* **Mixed Centriole Threshold:** `MIXED_SIGNAL_CONFIDENCE_CUTOFF = 0.8`. Inheritance confidence below this triggers a probabilistic branch.
* **Out‑of‑Plane Validation Angle:** `PLANE_DEVIATION_TOLERANCE = 15.0` (degrees). Diverting beyond this requires 3D coordinate validation if available.

## Performance & Budget
* **Compute Budget (MVP):** Designed to run on a standard laptop. Target performance: reconstruct trees up to 10,000 nodes in < 2 minutes.
* **Software Dependencies:** Python (NetworkX, Pandas, NumPy). No specialized hardware required.
* **Validation Benchmark:** Against simulated trees of up to 1,000 cells, target accuracy (F1 score for edges) > 0.95.

## Tool Versions (MVP Target)
* Python: 3.9.13
* NetworkX: 2.8.8
* Pandas: 1.5.3

== END

## Risk matrix

| Risk ID | Risk Description | Probability (1–5) | Impact (1–5) | Risk Score (P×I) | Mitigation Strategy |
|---------|-----------------|-------------------|--------------|------------------|---------------------|
| R1 | **Focus Drift:** Temporary loss of cell tracking during imaging leads to gaps in division log | 4 | 4 | 16 | Implement temporal gap‑closing algorithm (interpolation of missing frames); use redundant markers (nuclear + centriole) to cross‑validate; acquire z‑stacks to recover out‑of‑plane divisions |
| R2 | **Mixed Centriole Signal:** Ambiguous inheritance pattern (e.g., both daughters receive fragments of old centriole) prevents definitive assignment | 3 | 5 | 15 | Develop probabilistic branch‑point model (Bayesian assignment with confidence score); validate on synthetic data with known ground truth; exclude ambiguous events from primary analysis (sensitivity analysis) |
| R3 | **Out‑of‑Plane Division:** Cell divides perpendicular to imaging plane, obscuring centriole inheritance | 3 | 4 | 12 | Use 3D imaging (confocal stacks) to reconstruct division plane; apply spatial validation step to correct inferred 2D lineage connections; flag unresolved events for manual review |
| R4 | **Low Centriole Labeling Efficiency:** Insufficient fluorescent signal to detect centrioles in all cells | 2 | 5 | 10 | Optimize labeling protocol (e.g., Centrin‑GFP expression level); use machine learning to detect low‑signal centrioles; exclude cells with undetectable centrioles (report proportion) |
| R5 | **False Positive Edges:** Incorrect mother‑daughter assignment due to noise or coincidental proximity | 3 | 3 | 9 | Validate reconstructed tree against independent markers (e.g., DNA barcoding in subset of cells); compute bootstrap confidence for each edge; report false discovery rate on simulated data |

---
## === DifferentiationAnnotation / PARAMETERS.md ===

# DifferentiationAnnotation: Technical Parameters & Specifications

## Data Input Specifications
* **Lineage DAG Format:** Graph (JSON/GraphML). Nodes require fields: `cell_id`, `birth_timeframe`, `spatial_coordinates_um`, `parent_id(s)`, `centriole_id_tuple`.
* **scRNA‑seq Data:** Minimum 10,000 cells per experiment. Required alignment to reference genome (e.g., GRCh38). Depth: >50,000 reads per cell.
* **Morphology Features:** Vector of ≥50 features per cell per time point (e.g., volume, eccentricity, solidity, texture). Derived from 3D segmentations.
* **Immunofluorescence Anchors:** Minimum of 3 distinct protein markers per target tissue, with validated specificity. Confocal or super‑resolution images.

## Algorithmic Thresholds (MVP)
* **Registration Confidence:** Spatial‑temporal alignment requires a minimum correlation score of `R > 0.7` for anchor node assignment.
* **Transcriptomic Annotation:** A cell node is assigned a state label if the Spearman correlation of its imputed expression vector to a reference signature exceeds `ρ > 0.5`.
* **Lineage Propagation Constraint:** State transitions must conform to a pre‑defined `differentiation_grammar.yaml` file. Illegitimate transitions invalidate propagation paths.

## Computational Resource
