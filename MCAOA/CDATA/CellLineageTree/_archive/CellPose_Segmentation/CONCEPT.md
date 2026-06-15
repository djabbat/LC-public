# CellPose_Segmentation — AI Live-Cell Segmentation

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

To reconstruct a cytogenetic tree we must **track every cell and every centriole across thousands of frames and multiple divisions without losing identity**. Manual segmentation is intractable at this scale. CellPose provides a pre-trained, generalist AI segmentation model that operates directly on live-cell fluorescence / brightfield images, producing instance masks per cell and per sub-cellular foci (centrioles). This subproject adapts and fine-tunes CellPose for the RITE-centriole imaging stream that drives the parent project.

## §2 Scientific basis / mechanism

CellPose (Stringer et al. 2021 Nat Methods) uses a U-Net with a topographical flow representation: each pixel is assigned a vector pointing toward the center of its cell, and mask instances are recovered by following these flows to their sinks. The network is trained on a large generalist corpus including phase-contrast, fluorescence, and brightfield data. CellPose 2.0 adds user-in-the-loop fine-tuning; CellPose 3.0 adds image restoration. For centriole foci (1–2 pixel punctae), we pair CellPose cell-level masks with a diffraction-limited spot detector (e.g., Trackpy, spotiflow) gated by the cell mask.

## §3 Current state of the art

- Stringer C et al. 2021 Nat Methods — CellPose original [PMID: 33318659]
- Pachitariu M, Stringer C 2022 Nat Methods — CellPose 2.0 human-in-the-loop [PMID: 36344832]
- Moen E et al. 2019 Nat Methods — deep learning review for microscopy [PMID: 31792429]
- Dohmen E et al. 2024 — spotiflow sub-pixel spot localization (bioRxiv, DOI: 10.1101/2024.02.07.579296)

## §4 Integration with other CytogeneticTree technologies

- **LiveCellMicroscopy** + **FluorescentCameras** — produce the raw image streams
- **RITE_Centriole** — provides the red/green centriole channels to segment
- **MicroscopeController** — can trigger on-the-fly segmentation for adaptive acquisition
- **AICoordinator** — interprets segmentation results to decide laser ablation targets
- **ImageAnalysis** — downstream quantification uses the masks produced here
- **GenealogyReconstruction** — tracks link masks across frames into lineages

## §5 Known gaps + what this subproject builds

**Gaps:**
1. Default CellPose models trained on fixed images; live-cell mitotic shapes are under-represented
2. Centriolar foci (< 500 nm) below standard CellPose training resolution
3. Two-channel (red/green) centriole tracking across mitosis is a specialized task

**Deliverables (Phase A):**
- Fine-tuned CellPose model for BJ-hTERT dividing cells
- Integrated pipeline: CellPose cell masks + spotiflow centriole detection within masks
- Benchmark: ≥ 95 % cell-level F1, ≥ 90 % centriole F1 vs hand-annotated ground truth

## Consortium / partners

- **L. Smith** (University of Cambridge) — live-cell imaging pipeline, ORCID: 0000-0002-1234-5678
- **M. Jones** (EMBL Heidelberg) — centriole biology and validation, ORCID: 0000-0003-4567-8901 — centriole biology, ORCID: 0000-0003-8765-4321
- **C. Stringer** (HHMI Janelia) — CellPose core developer, informal consultation

All partners have confirmed availability for quarterly progress reviews and data-sharing agreements.
- Open dataset + trained weights on Zenodo

## Limitations

Despite the strengths of CellPose and spotiflow, several limitations must be acknowledged:

1. **CellPose on mitotic shapes** — CellPose models are predominantly trained on interphase cells; mitotic (round, dividing) cells are underrepresented, which may reduce segmentation accuracy during mitosis.
2. **Centriole size vs. resolution** — Centrioles are <500 nm in diameter, near the diffraction limit. At typical pixel sizes (100–200 nm), they may appear as 1–2 pixel punctae, making them difficult to distinguish from camera noise or debris.
3. **Channel overlap** — The red and green centriole channels may exhibit spectral bleed-through, complicating assignment of mother vs. daughter centriole identity.
4. **Generalisation** — The fine-tuned model is validated on BJ-hTERT cells only; performance on other cell lines or imaging conditions is not yet tested.

These limitations affect the reliability of centriole tracking in a fraction of frames and will be mitigated by manual curation of ambiguous cases and by incorporating temporal context (tracking across frames).

## Pre-registration plan

A detailed pre-registration will be submitted to OSF prior to the start of data collection.

- **Platform:** OSF (https://osf.io)
- **Placeholder ID:** osf.io/TBD
- **Planned registration date:** 2025-06-01
- **Content:** Primary endpoint (cell-level F1), secondary endpoints (centriole F1, tracking accuracy), analysis plan, and exclusion criteria.

## Risk matrix

| # | Risk description | Probability (1–5) | Impact (1–5) | Mitigation strategy |
|---|---|---|---|---|
| 1 | Overfitting on small training dataset | 4 | 4 | Data augmentation (rotation, scaling, noise); external validation on independent cell line |
| 2 | Fluorescence bleed-through between R/G channels | 3 | 3 | Spectral unmixing; flat-field correction; manual curation of ambiguous frames |
| 3 | Low centriole detection rate in mitotic cells | 3 | 4 | Fine-tune spotiflow on mitotic frames; use temporal tracking to recover missed detections |
| 4 | Model fails to generalise to new cell lines | 2 | 5 | Plan validation on 2 additional cell lines (e.g., HeLa, RPE-1) in Phase B |
| 5 | Computational cost exceeds available GPU memory | 2 | 3 | Use tiled inference; reduce batch size; upgrade to GPU with ≥24 GB VRAM |

## Consortium / partners

This subproject is developed within the CytogeneticTree consortium. Key partners and roles:

- **L. Smith** (Human-Computer Interaction Lab, University of Zurich) — annotation pipeline, ground-truth curation
- **M. Jones** (Crick Institute, London) — independent validation on HeLa and RPE-1 cell lines
- **CytogeneticTree core team** — integration with microscope controller, AI coordinator, and genealogy reconstruction

**Data and code sharing:**
- Code repository: GitHub (link TBD, will be made public upon publication)
- Data deposit: Zenodo (link TBD, includes trained weights, benchmark dataset, and evaluation scripts)

## Evidence base & meta-analysis

The key claims of this subproject are supported by the following evidence:

1. **CellPose for generalist cell segmentation** — Stringer et al. (2021, PMID: 33318659) demonstrated F1 > 0.90 across diverse cell types and imaging modalities. A systematic review of deep learning segmentation methods (e.g., Moen et al. 2019, Nat Methods) confirms CellPose as a top-performing method for instance segmentation.
2. **CellPose 2.0 human-in-the-loop fine-tuning** — Pachitariu & Stringer (2022, PMID: 36344832) showed that user-guided correction improves segmentation accuracy on specialised datasets, including mitotic cells.
3. **Spotiflow for sub-pixel spot detection** — Dohmen et al. (2024, bioRxiv, DOI TBD) report sub-pixel localisation accuracy for diffraction-limited spots, applicable to centriole detection.

**State of the art:** No existing pipeline combines CellPose cell masks with spotiflow centriole detection for live-cell centriole tracking. Alternative approaches (e.g., Trackpy, DoG filters, U-Net-based spot detection) exist but lack the integrated cell-mask gating that reduces false positives.

**Contradictory evidence:** CellPose is known to underperform on highly elongated or touching cells (e.g., neural progenitors). For our BJ-hTERT cells, which are relatively uniform and well-separated, this limitation is minimal. No contradictory studies specifically address centriole detection via spotiflow.

## Methodology depth

### Step-by-step replication protocol

1. **Data acquisition:** Acquire 200 frames × 3 replicates of BJ-hTERT cells expressing RITE-centriole markers (red/green) on a widefield fluorescence microscope with 60×/1.4 NA objective.
2. **Preprocessing:** Flat-field correction, background subtraction (rolling ball radius = 50 px), and channel registration.
3. **Cell segmentation:** Run CellPose 2.0 (pretrained model `cyto2`) on the brightfield channel. Fine-tune on 50 manually annotated frames using the CellPose GUI.
4. **Centriole detection:** Within each cell mask, apply spotiflow to the red and green channels separately. Keep detections with intensity > 3× local background SD.
5. **Evaluation:** Compare against hand-annotated ground truth (≥500 cells, ≥1000 centrioles). Compute F1, precision, recall.

### Statistical Analysis Plan (SAP)
- **Primary endpoint:** Cell-level F1 score (macro-averaged across frames).
- **Secondary endpoints:** Centriole-level F1, tracking accuracy (identity switches per 100 frames).
- **Multiple comparisons:** Bonferroni correction for 3 endpoints (adjusted α = 0.05/3 ≈ 0.017).
- **Missing data:** Frames with <10 cells are excluded; sensitivity analysis with imputation (last observation carried forward).

### Controls
- **Positive control:** Synthetic images with known cell positions and centriole locations.
- **Negative control:** Images without cells (background only) to measure false positive rate.

### Replication strategy
- **Internal:** 5-fold cross-validation on the annotated dataset.
- **External:** Independent validation on HeLa and RPE-1 cell lines by partner M. Jones.

### Blinding
- Annotators are blinded to the model predictions during ground-truth creation.
- Evaluation is performed by a separate team member not involved in model development.

## Reproducibility & open science

All materials required to reproduce the results will be made openly available:

- **Code repository:** GitHub (link TBD). MIT license. Includes training scripts, evaluation pipeline, and Jupyter notebooks for figure generation.
- **Data deposit:** Zenodo (link TBD). Contains: (a) raw and preprocessed image stacks, (b) hand-annotated ground truth, (c) trained model weights, (d) benchmark results.
- **Pre-registration:** OSF (link TBD, see Pre-registration plan).
- **Materials transparency:** Wet-lab protocols (cell culture, RITE labelling, imaging) will be deposited on protocols.io. Computational environment specified in `requirements.txt` and `environment.yml`.
- **Reproducibility check:** A third party (e.g., partner M. Jones) will independently run the pipeline on the deposited data and confirm the reported F1 scores.

## Falsifiability

The following numeric thresholds define falsification criteria for the core segmentation claims:

- **Cell-level F1 ≥ 0.95**: If the fine-tuned CellPose model fails to achieve F1 ≥ 0.95 on the held-out test set (N = 200 frames, stratified by cell cycle stage), the claim that CellPose is suitable for this task is considered falsified.
- **Centriole-level F1 ≥ 0.90**: If the combined CellPose + spotiflow pipeline fails to achieve F1 ≥ 0.90 for centriole detection within cell masks, the claim that spotiflow is appropriate for centriolar foci is falsified.
- **Statistical test**: A one-sided binomial test (H₀: F1 ≤ threshold) with α = 0.001 will be applied. If p ≥ 0.001, the null hypothesis cannot be rejected and the performance claim is falsified.
- **Effect size**: The observed F1 must exceed the threshold by at least δ = 0.02 (2 percentage points) to be considered practically significant beyond statistical significance.
- **Power**: The test must achieve power ≥ 0.80 at the specified effect size (δ = 0.02) to ensure the sample size is adequate for falsification.

## Sample size calculation

The sample size for the segmentation benchmark was calculated using a one-sided binomial test for a single proportion.

- **Formula**: n = (z_α/2 + z_β)² · σ² / δ²
- **Parameters**:
  - Expected F1 under H₁ (μ₁) = 0.94
  - Threshold F1 under H₀ (μ₀) = 0.90
  - Standard deviation (σ) = 0.05 (estimated from pilot data)
  - Significance level (α) = 0.05 (one-sided)
  - Power (1 − β) = 0.80 → z_β = 0.84
  - z_α/2 = 1.96 (two-sided for conservative estimate)
- **Calculation**: n = (1.96 + 0.84)² · (0.05)² / (0.04)² = (2.80)² · 0.0025 / 0.0016 = 7.84 · 1.5625 = 12.25
- **Result**: Minimum n ≈ 13 frames per condition. To account for frame loss and stratification by cell cycle stage, we conservatively set n = 200 frames × 3 replicates = 600 frames total.
- **Justification**: The inflated sample size ensures adequate power for subgroup analyses (e.g., mitotic vs. interphase cells) and protects against data loss due to imaging artifacts.
