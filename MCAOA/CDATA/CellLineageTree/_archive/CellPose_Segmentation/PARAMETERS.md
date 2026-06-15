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

## Sample size calculation

The required number of frames is estimated via a power analysis for the primary endpoint: cell-level F1 score.

**Assumptions:**
- Expected F1 under the alternative hypothesis: μ₁ = 0.94
- Null hypothesis: μ₀ = 0.90 (minimum acceptable)
- Standard deviation of F1 across frames: σ ≈ 0.05 (estimated from pilot data)
- Significance level: α = 0.05 (two-sided)
- Power: 1−β = 0.80

**Formula:**
n = (z_{α/2} + z_β)² · σ² / δ²
where δ = μ₁ − μ₀ = 0.04, z_{α/2} = 1.96, z_β = 0.84.

n = (1.96 + 0.84)² · (0.05)² / (0.04)² ≈ 12.25 frames.

To account for frame dropout and multiple comparisons across cell lines, we conservatively set N = 200 frames × 3 biological replicates (total 600 frames).
