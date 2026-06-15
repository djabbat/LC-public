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
