# LaserAblation_405 — PARAMETERS

## 405 nm path

| Parameter | Value |
|---|---|
| Source | Coherent OBIS / Oxxius 405 LX, ≥ 100 mW |
| Mode | TEM00, beam quality M² < 1.2 |
| Delivery | Single-mode fiber to galvo head |
| Spot size | ~ 400 nm at focus (diffraction-limited) |
| Ablation dose (nuclear kill, BJ-hTERT) | 5–20 mW × 200–500 ms (empirical) |
| "Mark" (non-lethal) dose | < 2 mW × 100 ms |

## fs-IR path (optional, Phase B)

| Parameter | Value |
|---|---|
| Source | Toptica FemtoFiber / Menlo fs laser, 800 nm |
| Pulse width | < 200 fs |
| Rep rate | 40–80 MHz |
| Avg power at sample | 10–50 mW (attenuated) |
| Target organelle cut | ~ 1 ms dwell |

## Steering / automation

| Parameter | Value |
|---|---|
| Galvo | Thorlabs GVS012 (2-axis) or Cambridge 6215H |
| Shutter | AOTF / mechanical, < 1 ms switching |
| Control bus | USB / analog via DAQ (NI-6009 or equiv.) |
| Python API | PyMMCore-Plus + custom wrapper |

## Risk matrix

| # | Risk | Probability (1–5) | Impact (1–5) | Mitigation |
|---|------|-------------------|--------------|------------|
| 1 | Laser misalignment causes collateral damage to non-target cells | 3 | 4 | Daily power calibration; ROI verification before each pulse |
| 2 | Overheating of objective due to prolonged ablation | 2 | 3 | Inter-pulse delay ≥100 ms; temperature monitoring |
| 3 | Photobleaching of fluorescent markers in target region | 4 | 2 | Use low-power pre-pulse check; limit to ≤3 pulses per cell |
| 4 | Software crash during ablation sequence | 2 | 5 | Hardware kill switch; log all commands for recovery |
| 5 | Cell death from cumulative ROS beyond intended ablation | 3 | 4 | Dose-response calibration for each cell line; use ROS scavenger controls |
| 6 | Galvo drift over long experiments (>12 h) | 2 | 3 | Auto-calibrate every 1000 pulses using reference bead

- Enclosure interlock on microscope door
- Laser goggles ANSI Z136 (405 nm OD ≥ 4)
- Lab SOP + sign-off required before first use

## Budget (Phase A, 405 only)

| Item | EUR |
|---|---|
| 405 nm 100 mW diode + driver | 3,500 |
| 2-axis galvo + driver + mounts | 2,200 |
| Optics (dichroic, lenses, fiber) | 1,800 |
| DAQ + cabling | 600 |
| Interlock + enclosure | 800 |
| Goggles × 3 | 300 |
| **Total (Phase A)** | **~9,200** |

## Budget (Phase B, fs-IR add-on)

| Item | EUR |
|---|---|
| fs-IR laser (used / refurb) | 25,000–60,000 |
| Additional optics + beam combiner | 4,000 |
| **Total (Phase B)** | **~30,000–65,000** |
