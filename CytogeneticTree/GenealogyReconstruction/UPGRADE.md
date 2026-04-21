# GenealogyReconstruction: Future Extensions

## Phase B: Enhanced Biological Fidelity
*   **Probabilistic Tree Models:** Move from deterministic to probabilistic graph models (e.g., using Bayesian networks) to better represent uncertainty in centriole assignment and low-confidence divisions.
*   **Integration of Secondary Markers:** Incorporate other cytoskeletal or cortical inheritance markers (e.g., midbody remnants) as additional, weak signals to resolve ambiguous branching points.
*   **Temporal Dynamics:** Model cell cycle duration as a node attribute and use temporal inconsistencies to flag potential reconstruction errors.

## Phase C: Scalability & Advanced Computation
*   **GPU-Accelerated Graph Processing:** For very large trees (>100k cells), explore porting core adjacency matrix operations to use GPU-accelerated libraries like CuGraph.
*   **Incremental/Online Reconstruction:** Enable tree updating upon streaming new division events, suitable for real-time or ongoing experimental analysis.
*   **Alternative Backend Support:** Add support for graph databases (e.g., Neo4j) for persistent storage and complex, query-based lineage tracing.

## Phase D: Cross-Modality Integration
*   **Direct Image Analysis Hook:** Develop an adapter that can take segmented cell tracking data (e.g., from TrackMate) and automatically generate the division-event log, reducing manual input.
*   **Phylogenetic Consistency Check:** Build a module to compare the cytogenetic tree with a single-cell mutational phylogeny from the same sample, identifying and investigating points of discordance.
*   **Spatial Tree Embedding:** Integrate with spatial transcriptomics coordinates to create a combined lineage-spatial graph, enabling analysis of clonal spatial dispersion.

== END ==
```
