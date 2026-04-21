# CellPose_Segmentation — PARAMETERS

## Model

| Parameter | Value |
|---|---|
| Base model | CellPose 3.0 `cyto3` |
| Fine-tuning mode | Human-in-the-loop (CellPose GUI) + scripted |
| Training set size | ≥ 200 frames × 3 biological replicates |
| Cell diameter prior | ~ 30 px (for 100× / 1.4 NA, 6.5 µm pixel CMOS) |
| Flow threshold | 0.4 |
| Cellprob threshold | 0.0 (permissive) |

## Centriole spot detection

| Parameter | Value |
|---|---|
| Algorithm | Trackpy `locate` / spotiflow |
| Expected spot FWHM | 250–350 nm |
| Min intensity SNR | 4 |
| Per-cell gate | Mask from CellPose |

## Hardware

| Component | Spec |
|---|---|
| GPU | ≥ 8 GB VRAM (RTX 3060 min; 4070/4080 target) |
| RAM | 32 GB |
| Storage | 2 TB NVMe for raw + processed streams |

## Benchmarks (Phase A target)

| Metric | Target |
|---|---|
| Cell F1 (IoU 0.5) | ≥ 0.95 |
| Centriole F1 | ≥ 0.90 |
| Lost-track rate over 24 h | ≤ 5 % |
| Inference | ≥ 2 fps @ 1024² |

## Budget

| Item | EUR |
|---|---|
| Workstation upgrade (GPU + NVMe) | 2,500 |
| Annotation labor (undergrad, 40 h) | 600 |
| Cloud burst (if needed) | 300 |
| **Total** | **~3,400** |
