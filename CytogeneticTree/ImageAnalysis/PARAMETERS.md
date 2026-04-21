# ImageAnalysis: Technical Parameters & Specifications

## Software & Versions
*   **Fiji/ImageJ:** Version 1.54f or later. Key plugins: Bio-Formats, TrackMate, MorphoLibJ.
*   **CellProfiler:** Version 4.2.1 or later.
*   **Python (for script wrappers):** 3.9+, with libraries: `pandas`, `numpy`, `scikit-image`.
*   **Containerization:** Docker image based on `cellprofiler/cellprofiler:latest` for pipeline stability.

## Key Analysis Parameters (Thresholds & Settings)
*   **Centriole Detection (Puncta):**
    *   Diameter (pixels): 3-6
    *   Typical intensity threshold (method): `Minimum Cross‑Entropy` or `Otsu` per image set.
    *   Maximum pairwise distance for mother-daughter pairing: 1.5 µm.
*   **Mother/Daughter Assignment:**
    *   Primary criterion: Centriole with **≥1.5x** higher GT335 mean intensity (background-subtracted) is provisional mother.
    *   Validation criterion: Provisional mother must have **≥0.8** Pearson's correlation with Ninein channel (or Ninein intensity ≥2x daughter).
*   **Ciliation (ARL13B) Detection:**
    *   Cilium length threshold: ≥0.5 µm.
    *   Maximum distance from mother centriole to cilium base: 0.7 µm.
*   **Cell Segmentation (DAPI):**
    *   Seeded watershed algorithm.
    *   Minimum nucleus area: 35 µm².

## Budget & Resource Lines
*   **Software:** Open-source (FIJI, CellProfiler). $0.
*   **Compute:** Medium-performance workstation (32GB RAM, GPU optional). Approx. ~$2,500 one-time.
*   **Storage:** Network-attached storage for raw images (~2-4 TB expected). Cost depends on institutional infrastructure.
*   **Personnel:** Bioimage analyst (0.5 FTE for pipeline development and validation).

## File Naming Convention (Input)
`[Date]_[SampleID]_[Stain]_[Well]_[Field].tif`
Example: `20240521_HEK293_GT335-Ninein-ARL13B-DAPI_B08_f012.tif`

## Output Data Structure
Primary output is a CSV file with columns:
`Image_Metadata, Cell_ID, CentriolePair_ID, Mother_GT335_MeanIntensity, Daughter_GT335_MeanIntensity, GT335_Ratio, Mother_Ninein_Coloc_Coeff, Is_Ciliated, ARL13B_Length_µm`

---
*For the project rationale, see [CONCEPT.md](CONCEPT.md).*
