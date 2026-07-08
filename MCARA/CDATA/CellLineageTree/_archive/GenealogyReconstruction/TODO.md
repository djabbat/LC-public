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
