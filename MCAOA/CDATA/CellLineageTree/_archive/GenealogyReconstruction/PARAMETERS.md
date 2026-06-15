# GenealogyReconstruction: Technical Parameters & Specifications

## Core Algorithm
* **Primary Language:** Python 3.9+
* **Core Library:** NetworkX ≥ 2.8 (for DiGraph construction & manipulation)
* **Graph Schema:**
 * **Node Attributes:** `cell_id` (str), `generation` (int), `centriole_age` (['old', 'young', 'mixed', 'unknown']), `timestamp` (float), `is_terminal` (bool).
 * **Edge Attributes:** `relation` ('symmetric' / 'asymmetric'), `fate_bias` (float, range 0-1, derived from centriole age confidence).

## Input Specifications
* **Division Log Format:** JSON or CSV with required columns: `event_id`, `parent_id`, `daughter_1_id`, `daughter_2_id`, `timestamp`, `division_type` (sym/asym), `plane_of_division` (2D/3D).
* **Centriole Inheritance Input:** Linked via `event_id`. Columns: `event_id`, `daughter_with_old_centriole` (cell_id), `confidence` (float 0.7-1.0).

## Noise Handling Thresholds (Configurable)
* **Focus Drift Max Gap:** `MAX_TIMESTAMP_GAP = 5.0` (arbitrary time units). Gaps shorter than this will attempt lineage re-linking.
* **Mixed Centriole Threshold:** `MIXED_SIGNAL_CONFIDENCE_CUTOFF = 0.8`. Inheritance confidence below this triggers a probabilistic branch.
* **Out-of-Plane Validation Angle:** `PLANE_DEVIATION_TOLERANCE = 15.0` (degrees). Diverting beyond this requires 3D coordinate validation if available.

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
| R1 | **Focus Drift:** Temporary loss of cell tracking during imaging leads to gaps in division log | 4 | 4 | 16 | Implement temporal gap-closing algorithm (interpolation of missing frames); use redundant markers (nuclear + centriole) to cross-validate; acquire z-stacks to recover out-of-plane divisions |
| R2 | **Mixed Centriole Signal:** Ambiguous inheritance pattern (e.g., both daughters receive fragments of old centriole) prevents definitive assignment | 3 | 5 | 15 | Develop probabilistic branch-point model (Bayesian assignment with confidence score); validate on synthetic data with known ground truth; exclude ambiguous events from primary analysis (sensitivity analysis) |
| R3 | **Out-of-Plane Division:** Cell divides perpendicular to imaging plane, obscuring centriole inheritance | 3 | 4 | 12 | Use 3D imaging (confocal stacks) to reconstruct division plane; apply spatial validation step to correct inferred 2D lineage connections; flag unresolved events for manual review |
| R4 | **Low Centriole Labeling Efficiency:** Insufficient fluorescent signal to detect centrioles in all cells | 2 | 5 | 10 | Optimize labeling protocol (e.g., Centrin-GFP expression level); use machine learning to detect low-signal centrioles; exclude cells with undetectable centrioles (report proportion) |
| R5 | **False Positive Edges:** Incorrect mother-daughter assignment due to noise or coincidental proximity | 3 | 3 | 9 | Validate reconstructed tree against independent markers (e.g., DNA barcoding in subset of cells); compute bootstrap confidence for each edge; report false discovery rate on simulated data |
