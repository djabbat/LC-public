# PARAMETERS — Aubrey (ARGUS-LP)

**Version:** 1.0

> **⚠️ UPDATED 2026-06-27:** version 2.0. Immersion — water (not oil). Control — RasPi 5 outside. BSL-1.
> **Previous version (v1.0, 2026-05-15):** 100× Oil, sCMOS ORCA-Flash4, RTX 4090, BSL-2. Archive: `ARGUS/_archive/`.
> **Current BOM:** `~/Desktop/Marketing/ARGUS-LP/docs/correspondence/ARGUS_LP_components_attachment_2026-06-27.md`

## Optical Path (v2.0, V1/V2)

| Parameter | V1 (without lasers) | V2 (with lasers) |
|----------|:---:|:---:|
| Objective | 40×/0.95 NA Plan Apo dry | 60×/1.2 NA Plan Apo Water Immersion |
| Immersion | Air | Water (collar + syringe pump 0.1 ml/h) |
| Camera | RasPi HQ (Sony IMX477) | sCMOS (PCO.panda 4.2 / ZWO ASI183MM Pro) |
| Fluorophores | Centrin1-GFP (488/525), Cep164-RFP (561/605) | Same + mEos3.2 (405 photoconversion) |
| Dichroic | GFP/RFP dual-band (Semrock Di01-R488/561) | 405/488/561 combiner |
| GFP emission | 525/50 bandpass | Same |
| RFP emission | 605/70 bandpass | Same |
| Excitation | LED 488 + LED 561 | Laser 488 CW 50 mW + 561 DPSS 50 mW |
| Photoconversion | — | 405 nm pulsed 100 mW + galvo-scanner |

## Stage

| Parameter | Value |
|----------|----------|
| Type | XY motorized, NEMA 11 ×2 + TMC2208 |
| Travel | 50×50 mm |
| Z-focus | NEMA 11 stepper + lead screw, autofocus (Laplacian variance) |
| Repeatability | ≤1 μm RMS (target) |

## Environment

| Parameter | Value |
|----------|----------|
| O₂ | 2% (LuminOx sensor + N₂ purge) |
| CO₂ | 5% (Sensirion SCD30 + 16g cartridge) |
| Temperature | 37±0.3°C (Peltier TEC1-12706 ×2 + PID) |
| Humidity | >95% RH (evaporative tray) |
| Enclosure | Aluminum 40×30×30 cm, sealed, laser-protected (V2: OD6+ Class 3B) |

## Control and Storage (outside enclosure)

| Parameter | Value |
|----------|----------|
| Computer | Raspberry Pi 5 (8 GB) |
| Storage | SSD 1 TB USB3 (Samsung T7) |
| Software | Python + picamera2, CellPose v3, spotiflow |
| AI | Rule-based classifier (red/green ratio). No LLM, no API. |

## Protection (V2)

| Parameter | Value |
|----------|----------|
| Scientific camera | Emission filter OD6+ + dichroic OD4+ + mechanical shutter (Uniblitz LS2) |
| Surveillance cameras | Notch filters 405/488/561 OD4+ ×2 |
| Laser safety | Class 3B: lid interlock, key switch, safety goggles OD6+ |

## Biology

| Parameter | Value |
|----------|----------|
| Cells | BJ-hTERT (ATCC CRL-4001), P25→P50 |
| O₂ during cultivation | 2% (physiological hypoxia) |
| Markers | Centrin1-GFP + Cep164-RFP (adenovirus, BSL-1) |

## Acceptance Criteria

| Criterion | Threshold |
|----------|:----:|
| SNR on beads | ≥5× background |
| Z-drift | ≤100 nm over 24 h |
| Temperature | 37±0.3°C over 48 h |
| CO₂ | 5±0.5% over 48 h |
| Viability (48 h) | ≥90% of unilluminated control |
| Divisions per 48 h | ≥2 per cell |
| Asymmetry Ratio (target) | A ≥ 0.6 → Phase 2 / A < 0.6 → null result publication |

## Budget

| Version | Amount |
|--------|:----:|
| V1 (without lasers, 35 items) | ~$2,045 |
| V2 (with lasers, 54 items) | ~$5,945–6,540 |
