# AnalysisStack / TODO.md (merged 2026-05-09)


---
## === CellPose_Segmentation / TODO.md ===

# CellPose_Segmentation — TODO (Phase A)

## A1. Dataset collection
- [ ] Acquire ≥ 3 × 24 h live-cell datasets on BJ-hTERT-RITE
- [ ] Export representative frames (every 30 min) for annotation
- [ ] Recruit annotator; build annotation protocol

## A2. Fine-tune CellPose
- [ ] Baseline cyto3 performance (no fine-tuning)
- [ ] Human-in-the-loop correction pass 1 (50 frames)
- [ ] Human-in-the-loop correction pass 2 (full 200 frames)
- [ ] Cross-validation split

## A3. Integrate centriole spot detector
- [ ] Wire spotiflow / Trackpy downstream of CellPose masks
- [ ] Per-cell SNR calibration
- [ ] Benchmark vs hand counts

## A4. Pipeline packaging
- [ ] `run_segmentation.py` CLI: input = OME-TIFF, output = HDF5 cells + centrioles
- [ ] Dockerfile for reproducibility
- [ ] Zenodo release (weights + sample dataset + DOI)
- [ ] Preprint draft: methods note

## Gate to Phase B
- Cell F1 ≥ 0.95, centriole F1 ≥ 0.90
- Pipeline runs end-to-end on 24 h dataset < 4 h wall-clock

---
## === ImageAnalysis / TODO.md ===

# ImageAnalysis: Phase A Milestones (Prioritized)

## High Priority (Weeks 1-4)
* **H1:** Establish version-controlled repository with `CONCEPT.md`, `README.md`, `PARAMETERS.md`.
* **H2:** **Fixed-Sample MVP:** Build and validate the core CellProfiler pipeline for a single field.
 * H2.1: Successfully segment nuclei and identify centriole puncta.
 * H2.2: Implement mother/daughter pairing logic based on GT335 intensity.
 * H2.3: Output a correct GT335 ratio for 10+ manually validated centriole pairs (success criterion: >95% accuracy).
* **H3:** Integrate Ninein channel analysis to validate mother assignment.

## Medium Priority (Weeks 5-8)
* **M1:** **Fixed-Sample Scaling:** Extend pipeline to process multi-field, multi-well plates automatically.
* **M2:** Add ARL13B ciliation analysis module to the fixed-sample pipeline.
* **M3:** Create a standardized CSV output schema and a Python script to aggregate results from multiple plates.
* **M4:** Document pipeline thoroughly and create a test dataset.

## Low Priority (Weeks 9-12)
* **L1:** **Live-Cell Prototype:** Begin Fiji/ImageJ macro for time-series analysis of GT355 signal in live cells (requires collaboration with `../WetLabProtocols/` on live-cell staining).
* **L2:** Implement basic tracking of centriole pairs through one cell division in the live-cell macro.
* **L3:** Cross-validate intensity measurements between CellProfiler (fixed) and Fiji (live) pipelines using calibration samples.

## Collaboration Points
* Coordinate with **`../WetLabProtocols/`** to obtain pilot image sets for development and validation.
* Align output schema with **`../DataModel/`** team for direct database ingestion.
* Provide initial test data outputs to **`../LineageReconstruction/`** team by end of Phase A.

---
*Future ambitions are documented in [UPGRADE.md](UPGRADE.md).*

---
## === StatisticalAnalysis / TODO.md ===

# StatisticalAnalysis: Phase A Milestones (Prioritized)

## P0: Core Pipeline Infrastructure
* [ ] **S0:** Set up Python environment (`pymc`, `lifelines`, `SALib`, `arviz`) and version control.
* [ ] **S1:** Create data ingestion function (`load_fate_table`) for standardized CSV input.
* [ ] **S2:** Implement basic Kaplan-Meier plotting and log-rank test function (`run_survival_analysis`).
* [ ] **S3:** Build and test a basic Weibull survival model in PyMC (`model_weibull.py`).

## P1: Bayesian Workflow & Validation
* [ ] **S4:** Create MCMC sampling wrapper with default config and basic diagnostics (R-hat, ESS).
* [ ] **S5:** Generate synthetic data from known parameters and validate model recovery (simulate -> infer).
* [ ] **S6:** Develop function to visualize posterior distributions and survival function posterior predictive checks.

## P2: Sensitivity & Identifiability Analysis
* [ ] **S7:** Implement Sobol analysis wrapper (`run_sobol.py`) around the Bayesian model.
* [ ] **S8:** Test identifiability on synthetic data with known parameter trade-offs (e.g., make `α` and `β` non-identifiable).
* [ ] **S9:** Produce a standardized identifiability report for a given dataset/model.

## P3: Integration & Documentation
* [ ] **S10:** Create master pipeline script (`statistical_pipeline.py`) that chains S1->S9.
* [ ] **S11:** Apply MVP pipeline to first pilot dataset (from `ImagingQuantification`).
* [ ] **S12:** Document all functions and create a basic tutorial notebook (`demo_analysis.ipynb`).

---
## === GenealogyReconstruction / TODO.md ===

# GenealogyReconstruction: Phase A Milestones

**Phase A Goal:** Produce a functional MVP that can reconstruct an accurate tree from clean simulated data and handle one type of noise.

## Priority P1 (Week 1-2)
* [ ] **A.1 Core Graph Builder:** Implement `TreeBuilder` class that ingests a clean division/centriole log and outputs a valid NetworkX DiGraph with basic attributes.
* [ ] **A.2 Basic I/O:** Create functions to read/write standard log formats (CSV/JSON) and export the graph to GraphML/JSON.
* [ ] **A.3 Unit Test on Clean Data:** Develop a small, hand-crafted ground-truth dataset (≤50 cells) and verify 100% reconstruction accuracy.

## Priority P2 (Week 3-4)
* [ ] **A.4 Asymmetric Division Logic:** Extend `TreeBuilder` to correctly tag edges with `fate_bias` property based on centriole inheritance input.
* [ ] **A.5 Focus Drift Module:** Implement `gap_closer` submodule that can reconnect lineages across short, timed gaps in the event log.
* [ ] **A.6 Validation Metric I:** Implement `calculate_edge_accuracy(graph_pred, graph_true)` to compare reconstructed vs. ground-truth trees.

## Priority P3 (Week 5-6)
* [ ] **A.7 Integration Test:** Run the full pipeline on a larger, clean simulated tree (≈500 cells) from the `../LineageImaging/Simulator`.
* [ ] **A.8 Noise Test I:** Validate the focus drift module on simulated data with introduced tracking gaps.
* [ ] **A.9 Documentation & Packaging:** Create basic `README.md`, `requirements.txt`, and a simple example Jupyter notebook demonstrating the MVP workflow.

== END

---
## === DifferentiationAnnotation / TODO.md ===

# DifferentiationAnnotation: Phase A Milestones

**Priority Order: P0 (Critical Path) > P1 (Essential) > P2 (Enhancement)**

## P0: Foundation & Data Flow
1. **Define Unified Data Schema:** Establish and document the common JSON schema for the lineage DAG that all upstream modules (`CentrioleLineageTracer`, `LiveImagingMorphometry`) must output.
2. **Build Minimum Viable Registration (MVR) Engine:** Develop the first-pass algorithm to map a subset of lineage nodes to cells in a dissociated scRNA-seq dataset using shared IF anchor markers. Deliverable: A script that outputs a rough alignment map.
3. **Create 'Differentiation Grammar' Draft:** Collaborate with domain biologists to draft the initial YAML file defining permissible state transitions for the first target tissue (e.g., embryonic mouse liver).

## P1: Core Annotation Pipeline
4. **Implement Constrained Label Propagator (v0.1):** Develop the core algorithm that takes anchor node labels and propagates them through the DAG, respecting the differentiation grammar. Use simple majority voting from neighbors initially.
5. **Integrate scRNA-seq Annotation (v0.1):** Implement basic correlation-based cell state labeling using a pre-defined marker gene dictionary. Integrate this as an input layer to the Propagator.
6. **End-to-End Test on Synthetic Data:** Generate a simulated lineage DAG with known state labels and test the full MVP pipeline (Registration -> scRNA-seq labeling -> Propagation). Target accuracy: >85% node recall.

## P2: Robustness & Validation
7. **Incorporate Morphology Classifier:** Train a simple Random Forest classifier on morphology features from anchor nodes. Integrate its predictions as a third evidence layer into the propagator.
8. **Develop Confidence Scoring System:** Implement a module that outputs a confidence score (0-1) for each node's annotation based on agreement between evidence layers and distance from anchors.
9. **Validate on First Pilot Dataset:** Run the complete v0.5 pipeline on the first real, small-scale (≈1000 cells) CytogeneticTree dataset. Compare annotations to expert manual curation.
