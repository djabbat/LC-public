# DifferentiationAnnotation: Technical Parameters & Specifications

## Data Input Specifications
*   **Lineage DAG Format:** Graph (JSON/GraphML). Nodes require fields: `cell_id`, `birth_timeframe`, `spatial_coordinates_um`, `parent_id(s)`, `centriole_id_tuple`.
*   **scRNA-seq Data:** Minimum 10,000 cells per experiment. Required alignment to reference genome (e.g., GRCh38). Depth: >50,000 reads per cell.
*   **Morphology Features:** Vector of ≥50 features per cell per time point (e.g., volume, eccentricity, solidity, texture). Derived from 3D segmentations.
*   **Immunofluorescence Anchors:** Minimum of 3 distinct protein markers per target tissue, with validated specificity. Confocal or super-resolution images.

## Algorithmic Thresholds (MVP)
*   **Registration Confidence:** Spatial-temporal alignment requires a minimum correlation score of `R > 0.7` for anchor node assignment.
*   **Transcriptomic Annotation:** A cell node is assigned a state label if the Spearman correlation of its imputed expression vector to a reference signature exceeds `ρ > 0.5`.
*   **Lineage Propagation Constraint:** State transitions must conform to a pre-defined `differentiation_grammar.yaml` file. Illegitimate transitions invalidate propagation paths.

## Computational Resource Estimates
*   **Storage (per experiment):** ~500 GB (raw images + sequencing + graphs).
*   **Memory (for propagation):** ~32 GB RAM for a tree of 50,000 nodes.
*   **Compute Time (MVP pipeline):** Estimated 48-72 hours on a high-core (32+) CPU server.

## Tool & Version Dependencies
*   **Primary Language:** Python 3.10+
*   **Core Libraries:** scikit-learn 1.3+, NetworkX 3.0+, anndata 0.9+, scanpy 1.9+
*   **Registration Engine:** Custom (to be built), may leverage Elastix/SimpleITK for image component.
*   **Visualization:** Graph visualization via Cytoscape 3.9+ or custom Plotly Dash app.

## Budget Lines (Conceptual)
*   **Personnel:** 1.5 FTE (Bioinformatician + Computational Biologist).
*   **Cloud Compute:** $15k/year for pipeline development and testing.
*   **Software Licensing:** Open-source stack; budget allocated for potential commercial graph DB integration ($5k).
