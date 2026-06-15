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
