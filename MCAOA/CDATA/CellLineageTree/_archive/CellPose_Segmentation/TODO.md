# CellPose_Segmentation — TODO (Phase A)

## A1. Dataset collection
- [ ] Acquire ≥ 3 × 24 h live-cell datasets on BJ-hTERT-RITE
- [ ] Export representative frames (every 30 min) for annotation
- [ ] Recruit annotator; build annotation protocol

## A2. Fine-tune CellPose
- [ ] Baseline cyto3 performance (no fine-tuning)
- [ ] Human-in-the-loop correction pass 1 (50 frames)
- [ ] Human-in-the-loop correction pass 2 (full 200 frames)
- [ ] Cross-validation split

## A3. Integrate centriole spot detector
- [ ] Wire spotiflow / Trackpy downstream of CellPose masks
- [ ] Per-cell SNR calibration
- [ ] Benchmark vs hand counts

## A4. Pipeline packaging
- [ ] `run_segmentation.py` CLI: input = OME-TIFF, output = HDF5 cells + centrioles
- [ ] Dockerfile for reproducibility
- [ ] Zenodo release (weights + sample dataset + DOI)
- [ ] Preprint draft: methods note

## Gate to Phase B
- Cell F1 ≥ 0.95, centriole F1 ≥ 0.90
- Pipeline runs end-to-end on 24 h dataset < 4 h wall-clock
