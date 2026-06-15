# CellPose_Segmentation — UPGRADE

## U1. 3D segmentation
- Move from 2D to 3D stacks using CellPose 3D mode
- Requires Piezo-Z live-cell protocol (see `LiveCellMicroscopy`)

## U2. Real-time on-microscope inference
- Run CellPose inference inline with acquisition via PyMMCore-Plus callbacks
- Enables adaptive acquisition (e.g., auto-trigger high-speed mode at mitosis)

## U3. SAM / SAM2 hybrid
- Integrate Segment-Anything (SAM2) for zero-shot fallback on unfamiliar cell types
- Ensemble: CellPose + SAM2 → boosted recall

## U4. Mitotic stage classifier
- Fine-tune secondary model to classify prophase / metaphase / anaphase / telophase per cell
- Feeds lineage tracker's event detector

## U5. Active learning loop
- Uncertainty sampling to query annotator only on low-confidence frames
- Target: halve annotation cost for new cell types

## U6. Centriole triple-channel
- Extend to mCherry / GFP / BFP after RITE triple-tag upgrade (see `RITE_Centriole/UPGRADE.md` U1)

## U7. Cross-microscope generalization
- Test trained weights on Yokogawa CSU, Zeiss Airyscan, DMi8
- Publish "cytogenetic-cyto" model on CellPose model zoo
