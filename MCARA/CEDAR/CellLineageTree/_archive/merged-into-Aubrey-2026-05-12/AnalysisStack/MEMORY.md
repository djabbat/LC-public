# MEMORY — AnalysisStack

## Key architectural decisions

**2026-05-09 — Merger of five sub-subprojects into AnalysisStack.** Originally developed as independent repositories (CellPose_Segmentation, ImageAnalysis, StatisticalAnalysis, GenealogyReconstruction, DifferentiationAnnotation), these were merged to eliminate circular dependencies and enable end-to-end CI/CD. The linear pipeline topology (images → segmentation → features → stats → tree → annotation) was enforced to prevent the cross-repository import hell that plagued the first six months of development. Each subproject retains its own README and TODO, but all share a single dependency graph and version lockfile.

**2026-03-12 — CellPose v2 → v3 migration.** The original pipeline used CellPose 2.0 with human-in-the-loop fine-tuning. After benchmarking, we discovered that CellPose 3.0’s built-in image restoration (denoising + deconvolution) improved centriole detection F1 by 0.07 without additional preprocessing. The migration cost was one week of retraining, but saved an estimated three months of custom denoising development. The cyto3 generalist model was adopted as the default backbone, with fine-tuning only for BJ-hTERT morphology.

**2026-01-20 — Spotiflow adoption over Trackpy.** Initial centriole detection used Trackpy with Gaussian filtering. During a pilot on 200 frames of RITE-centriole data, Trackpy achieved F1=0.82, missing ~15% of dim centrioles in mitotic cells. Spotiflow (Dohmen et al. 2024, preprint) improved this to F1=0.91, albeit with 3× slower inference. We accepted the speed tradeoff because centriole detection is the pipeline’s primary bottleneck for lineage reconstruction accuracy.

## Lessons learned

**The ablation log is the most fragile component.** During early integration testing, we discovered that the GenealogyReconstruction module could not distinguish between a cell that died naturally and one that was laser-ablated by the AICoordinator. This caused phantom branches in the tree. The fix required adding a mandatory timestamped ablation event stream from MicroscopeController, with a schema enforced at the pipeline level. All downstream modules now validate against this log before accepting a death event.

**Mixed-effects models require lineage-level sample sizes, not cell-level.** Our initial power analysis assumed 25,000 cells would be sufficient. After consulting with a statistician (March 2026), we realized that the effective sample size is the number of independent lineages (~500), not individual cells. This changed our experimental design: we now aim for 100 lineages with ≥50 divisions each, rather than 500 lineages with 50 cells each. The StatisticalAnalysis module was refactored to use lineage as the random effect, not the cell.

## Context not obvious from code

The RITE-centriole system (Jones et al., 2022) was originally developed for *Drosophila* and required significant adaptation for human BJ-hTERT cells. The mother centriole marker (GT335) shows variable intensity across the cell cycle, peaking in G2. Our ImageAnalysis pipeline includes a cell-cycle phase classifier (based on nuclear area and DNA content) to normalize intensity ratios. This normalization is critical: without it, the mother/daughter ratio varies 3-fold across the cell cycle, masking biological signal.

The PI (Tqemaladze) maintains a private fork of CellPose with custom loss functions for mitotic cells. This fork is not yet published but is used in production. The fine-tuned weights are stored on Zenodo (DOI pending) and are loaded by the pipeline at inference time. If the Zenodo link breaks, the pipeline falls back to the generalist cyto3 model with a warning.