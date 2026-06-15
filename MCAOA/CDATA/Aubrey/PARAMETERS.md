# PARAMETERS — Aubrey (ARGUS-LP)

*Full hardware BOM: `~/Desktop/ARGUS-LP_hardware_spec.md`*
*Optical schematic: `~/Desktop/ARGUS-LP_schematic.drawio`*

## Optical chain (per ARGUS-LP unit)

| Parameter | Value |
|-----------|-------|
| Objective | 100×/1.4 NA Plan Apo (Nikon CFI, used grade) |
| Detection | sCMOS (PCO.edge 5.5 or Hamamatsu ORCA-Flash4, used grade) |
| Fluorophores | Kaede green (488 nm ex, 510 nm em), Kaede red (561 nm ex, 590 nm em) |
| Dichroic 488 | 488 LP (Edmund #87-242 or Chroma ZT488rdc) |
| Dichroic 561 | 561 LP (Chroma ZT561rdc or equivalent) |
| Bandpass 488 | 510/20 (Edmund #87-789, OD ≥4) |
| Bandpass 561 | 590/20 (Chroma or Edmund) |
| Photo-conversion | 405 nm pulsed diode + galvo steering, 30-60 s ROI dose |

## Stage

| Parameter | Value |
|-----------|-------|
| Type | XY motorized, NEMA17 steppers + TMC2209 drivers |
| Controller | Arduino / ESP32-S3 with serial JSON-lines protocol |
| Repeatability | ≤1 µm RMS (target) |
| Travel | 20 × 20 mm (multi-FOV tile scanning) |

## Environmental

| Parameter | Value |
|-----------|-------|
| O₂ | 2-3% (SprintIR-W or LuminOx sensor + N₂ purge solenoid) |
| CO₂ | 5% (K30 sensor + CO₂ solenoid) |
| Temp | 37°C ± 0.5°C (PID controller + heater cartridge) |
| Humidity | >95% RH (evaporation tray in enclosure) |

## AI station

| Parameter | Value |
|-----------|-------|
| GPU | RTX 4090 (24 GB VRAM) |
| Local LLM | DeepSeek-V3 (Ollama or llama.cpp) |
| Segmentation | CellPose v3 |
| Tracking | spotiflow (Trackpy as fallback) |
| Control | Micro-Manager / PyMMCore |
| Autonomy | Fully autonomous — no operator confirmation per ablation shot |

## Acceptance criteria (commissioning)

| Criterion | Threshold |
|-----------|-----------|
| SNR on 100 nm TetraSpeck beads | ≥ 5× background |
| XY stage repeatability | ≤ 1 µm RMS |
| Focus drift (auto-focus every 30 min) | ≤ 100 nm over 24 h |
| O₂ stability | 2-3% over 48 h |
| CO₂ stability | 5% ± 0.5% over 48 h |
| Temperature stability | 37°C ± 0.5°C over 48 h |
| Laser power stability | ≤ 1% CV over 24 h |
| Fluidics volume accuracy | ± 10% of setpoint |
