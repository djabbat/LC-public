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
