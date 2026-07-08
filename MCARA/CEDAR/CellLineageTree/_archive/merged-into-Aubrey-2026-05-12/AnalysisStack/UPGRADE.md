# AnalysisStack / UPGRADE.md (merged 2026-05-09)


---
## === CellPose_Segmentation / UPGRADE.md ===

# CellPose_Segmentation — UPGRADE

## U1. 3D segmentation
- Move from 2D to 3D stacks using CellPose 3D mode
- Requires Piezo-Z live-cell protocol (see `LiveCellMicroscopy`)

## U2. Real-time on-microscope inference
- Run CellPose inference inline with acquisition via PyMMCore-Plus callbacks
- Enables adaptive acquisition (e.g., auto-trigger high-speed mode at mitosis)

## U3. SAM / SAM2 hybrid
- Integrate Segment-Anything (SAM2) for zero-shot fallback on unfamiliar cell types
- Ensemble: CellPose + SAM2 → boosted recall

## U4. Mitotic stage classifier
- Fine-tune secondary model to classify prophase / metaphase / anaphase / telophase per cell
- Feeds lineage tracker's event detector

## U5. Active learning loop
- Uncertainty sampling to query annotator only on low-confidence frames
- Target: halve annotation cost for new cell types

## U6. Centriole triple-channel
- Extend to mCherry / GFP / BFP after RITE triple-tag upgrade (see `RITE_Centriole/UPGRADE.md` U1)

## U7. Cross-microscope generalization
- Test trained weights on Yokogawa CSU, Zeiss Airyscan, DMi8
- Publish "cytogenetic-cyto" model on CellPose model zoo

---
## === ImageAnalysis / UPGRADE.md ===

# ImageAnalysis: Future Extensions Beyond MVP

## 1. Advanced Live-Cell & 4D Analysis
* **Integrated Tracking:** Combine centriole age signature tracking with whole-cell lineage tracking (e.g., from nuclear label) in a single Fiji/ImageJ or Python (Napari) platform.
* **Dynamic Metrics:** Move beyond endpoint ratios to calculate **rate of polyglutamylation accumulation** on the mother centriole over the cell cycle as a more precise aging clock.
* **Event-Triggered Analysis:** Automatically detect cell division events in a timelapse and analyze the inheritance of centriole age signatures by daughter cells.

## 2. Enhanced Phenotypic Profiling
* **Shape & Texture Metrics:** Incorporate advanced morphology features (e.g., centriole pair orientation, signal texture using Haralick features) that may correlate with cell state or lineage history.
* **Multi-Plex Expansion:** Design analysis modules for additional relevant stains (e.g., cell cycle markers like EdU, differentiation markers) to enrich the contextual data around each centriole measurement.
* **3D Analysis:** Extend pipelines to work with confocal z-stacks, enabling volumetric quantification of signals and more accurate cilia detection.

## 3. Machine Learning Augmentation
* **Detection Model:** Train a U-Net or similar model for more robust centriole and cilium detection across diverse cell types and imaging conditions, reducing reliance on manual threshold tuning.
* **State Classification:** Use the quantitative image-derived features (GT335 ratio, ciliation, morphology) to train a classifier that predicts a cell's position in the differentiation tree.

## 4. Pipeline Engineering & Deployment
* **Web Interface:** Develop a lightweight local web UI (e.g., using Gradio) to allow biologists to upload images, select pipelines, and view results without command-line interaction.
* **Cloud-Native Version:** Containerize the entire pipeline (e.g., with Nextflow) for scalable execution on cloud or cluster environments, enabling population-scale image analysis.
* **Public Repository:** Curate and publish a set of validated, versioned pipelines on a platform like GitHub or BioImage Archive, along with benchmark datasets, to serve the wider cell biology community.

---
*These extensions depend on the successful completion of the Phase A milestones outlined in [TODO.md](TODO.md).*

---
## === StatisticalAnalysis / UPGRADE.md ===

# StatisticalAnalysis: Future Extensions Beyond MVP

## 1. Advanced Survival & Hazard Models
* **Time-Varying Covariates:** Extend to Cox proportional hazards models incorporating dynamic molecular markers (e.g., expression of fate determinants) as covariates.
* **Competing Risks:** Model where a cell can exit the lineage via alternative routes (e.g., apoptosis, quiescence) as competing risks to differentiation-driven centriole loss.
* **Spatial Survival Models:** Incorporate neighborhood/cell-contact information from imaging data as frailty terms in the hazard function.

## 2. Enhanced Bayesian Framework
* **Hierarchical Models:** Implement multi-level models to share information across related lineages or biological replicates, improving parameter estimation.
* **Model Averaging/Selection:** Use Bayesian stacking or PSIS-LOO to formally compare and average across alternative parametric families (Weibull, Log-Normal, Gompertz).
* **Gaussian Process (GP) Survival Models:** Replace parametric hazard functions with a GP prior for flexible, data-driven hazard shape inference.

## 3. Integration with Tree Reconstruction
* **Direct Probabilistic Lineage Inference:** Embed the centriole survival likelihood directly into a tree-search algorithm (e.g., as a novel mutation model in a phylogenetic inference engine), moving from a two-step (analyze then infer) to a joint inference process.
* **Bayesian Nonparametrics:** Use Dirichlet Process mixtures to automatically infer the number of distinct centriole loss regimes (lineage branches) from the data.

## 4. Computational & Scalability Upgrades
* **GPU-Accelerated MCMC:** Transition key models to `numpyro` or `tensorflow-probability` for GPU-accelerated sampling on large single-cell datasets.
* **Approximate Bayesian Computation (ABC):** For models with intractable likelihoods, implement ABC for rapid screening of parameter space.

## 5. Experimental Design Optimization
* **Formal Optimal Design:** Use the sensitivity analysis framework and expected posterior entropy to calculate the most informative next differentiation stage or lineage to sample, creating a closed-loop between statistics and experiment.

---
## === GenealogyReconstruction / UPGRADE.md ===

# GenealogyReconstruction: Future Extensions

## Phase B: Enhanced Biological Fidelity
* **Probabilistic Tree Models:** Move from deterministic to probabilistic graph models (e.g., using Bayesian networks) to better represent uncertainty in centriole assignment and low-confidence divisions.
* **Integration of Secondary Markers:** Incorporate other cytoskeletal or cortical inheritance markers (e.g., midbody remnants) as additional, weak signals to resolve ambiguous branching points.
* **Temporal Dynamics:** Model cell cycle duration as a node attribute and use temporal inconsistencies to flag potential reconstruction errors.

## Phase C: Scalability & Advanced Computation
* **GPU-Accelerated Graph Processing:** For very large trees (>100k cells), explore porting core adjacency matrix operations to use GPU-accelerated libraries like CuGraph.
* **Incremental/Online Reconstruction:** Enable tree updating upon streaming new division events, suitable for real-time or ongoing experimental analysis.
* **Alternative Backend Support:** Add support for graph databases (e.g., Neo4j) for persistent storage and complex, query-based lineage tracing.

## Phase D: Cross-Modality Integration
* **Direct Image Analysis Hook:** Develop an adapter that can take segmented cell tracking data (e.g., from TrackMate) and automatically generate the division-event log, reducing manual input.
* **Phylogenetic Consistency Check:** Build a module to compare the cytogenetic tree with a single-cell mutational phylogeny from the same sample, identifying and investigating points of discordance.
* **Spatial Tree Embedding:** Integrate with spatial transcriptomics coordinates to create a combined lineage-spatial graph, enabling analysis of clonal spatial dispersion.

== END ==
```

---
## === DifferentiationAnnotation / UPGRADE.md ===

# DifferentiationAnnotation: Future Extensions Beyond MVP

## 1. Advanced Probabilistic Graphical Model
Replace the rule-based propagator with a full probabilistic model (e.g., a Conditional Random Field over the lineage DAG). This would naturally handle uncertainty, integrate all data modalities as probabilistic evidence, and learn the parameters of state transition probabilities directly from data.

## 2. Dynamic State & Transition Discovery
Move beyond pre-defined static states. Implement methods to *discover* discrete or continuous differentiation states directly from the integrated data along the lineage, identifying novel intermediate or transient populations that were not previously annotated.

## 3. Spatial Neighborhood Context Integration
Incorporate the spatial neighborhood of a cell (from imaging) as an explicit contextual factor in annotation. This would model the influence of cell-cell communication and positional information on state assignment (e.g., a cell might be annotated as a "border fibroblast" based on lineage *and* location).

## 4. Temporal Dynamics & State Velocity
Annotate not just the state, but the *kinetics* of state transition. By leveraging the precise birth timestamps in the lineage tree, calculate the residence time in a state and the "velocity" of transcriptional change leading to a fate decision, enriching the **FateDecisionMap**.

## 5. Cross-Species & Cross-Tissue Generalization Framework
Develop a meta-architecture that allows the annotation models trained on one model organism or tissue (e.g., mouse hematopoiesis) to be adapted or transferred to another (e.g., human cerebral organoids), significantly accelerating new applications.

## 6. Real-Time Annotation for Guided Experimentation
In a closed-loop system with live imaging and sequential single-cell biopsy/sequencing, upgrade the pipeline to perform near-real-time annotation. This would allow experimentalists to identify rare lineage branches of interest as they emerge and physically guide subsequent sampling.
