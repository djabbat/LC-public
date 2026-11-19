# ARGUS — Validation target: extracellular γ-tubulin in the preimplantation window (mouse)

**Date:** 2026-11-11
**Status:** watch-item / candidate validation target for the ARGUS imaging platform. Source = single-lab preprint (unconfirmed). NOT evidence for grants/manuscripts.
**Source:** Zhao R, Liu J, Bakhtari A, Tian X. *Novel extracellular, extra-embryonic and intercellular γ-tubulin assemblies in mouse oocytes and preimplantation embryos.* bioRxiv preprint 2026-10-25, DOI `10.64898/2026.10.22.761027` (Crossref ✅, Europe PMC PPR1349676 ✅).
**Master analysis:** `~/Desktop/Services/docs/ANALYSIS_2026-11-11_Extracellular_gammaTubulin_Zhao_Tian.md`.

## Why this is a target for ARGUS
The claim is IF-based (single lab). ARGUS's orthogonal methods — **label-free (phase/DIC/polarized/QPI), expansion microscopy, and live time-lapse** — can independently confirm or refute the four most falsifiable predictions. This matches the platform's stated capability and its "old ≠ degraded / structure ≠ artifact" discipline (see `ARGUS_OS2_PTM_readouts.md`, `ARGUS_label_free_centriole_assignment.md`).

## The four falsifiable predictions (from full text)
| # | Prediction | Why it discriminates artifact from biology | Method on ARGUS |
|---|---|---|---|
| P1 | γ-tubulin signal persists on oocyte/embryo surface **without membrane permeabilization** | IF without Triton/methanol labels only extracellular proteins → distinguishes surface tethering from leak-in | Non-permeabilized staining control (replicate) + confocal |
| P2 | γ-TES/γ-TNC **disappear after hatching**; intracellular MTOCs remain | Stage-specific disappearance is incompatible with nonspecific background | Longitudinal live imaging zygote→hatched blastocyst |
| P3 | **γ-hemisphere asymmetry**: denser γ-TES on one side; meiotic spindle in the trailing hemisphere; γ-TES count doubles (p<0.05) only on γ-hemisphere after fertilization | Quantitative, side-specific change → not uniform artifact | Live imaging + quantitation (≥15/stage, 3 replicates, ANOVA+Tukey as in source) |
| P4 | Peri-embryonic MT web absent at blastomere–blastomere contact zones | Localized absence argues against uniform stain deposit | High-res z-stacks + 3D reconstruction |

## What is NOT closed by the preprint (orthogonal methods still missing)
- Electron microscopy of the extracellular assemblies.
- Mass spectrometry of the extracellular/perivitelline space.
- Functional γ-tubulin knockout with nucleation readout (MT-regrowth from γ-TNC) — the name "nucleation center" is functional without a functional test.
- Export mechanism: γ-tubulin has no signal peptide → "backdoor" export unknown (authors admit).
- Independent replication by a second lab.

## Practical notes
- Species/model: source = mouse oocytes/embryos (CD-1, IVF, KSOM, WOW for zona-free). ARGUS main model is C. elegans — a mouse-embryo validation track would be a separate (collaborative) effort; C. elegans embryos can be used as an orthogonal cross-species check of "extracellular γ-tubulin" if antibodies/tags allow.
- Priority: **low-priority watch-item** — do not block ARGUS-OS1/OS2/OS3 roadmap; revisit when the peer-reviewed version + EM/MS appear.
- Next action: track DOI 10.64898/2026.10.22.761027 for the peer-reviewed version; add to weekly literature watch.
