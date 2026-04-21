# MicroscopeController — PARAMETERS

## Software stack

| Component | Version |
|---|---|
| Python | 3.11 |
| pymmcore-plus | ≥ 0.11 |
| useq-schema | latest |
| napari | ≥ 0.5 (optional UI) |
| micromanager | 2.0.3 |
| OS | Ubuntu 24.04 LTS |

## Hardware interfaces supported

| Device | Bus / driver |
|---|---|
| Piezo Z (PI) | USB / PI GCS |
| Motorized XY | RS-232 / TTL |
| AOTF | USB / analog DAQ |
| Lasers 488 / 561 | USB SCPI |
| Cameras (FLIR / Hikrobot) | USB3 Vision / Genicam |
| Galvo (ablation) | DAQ analog out |
| Environmental chamber | RS-485 / Modbus TCP |
| Safety interlock | GPIO |

## Acquisition budget (per tile)

| Parameter | Value |
|---|---|
| Channels | 2 (488 + 561) |
| Z-stack | 9 planes × 500 nm |
| Exposure | 200 ms / channel |
| Per-tile duration | ~ 4 s |
| Tiles per field | 1 (single cell colony tracking) |
| Frame interval (lineage) | 10 min |
| Run duration | up to 72 h |

## Reliability targets

| Metric | Target |
|---|---|
| Uninterrupted run | ≥ 72 h |
| Auto-recovery rate from transient faults | ≥ 95 % |
| Data integrity (OME-NGFF checksums) | 100 % |
| Decision-to-action latency (callback) | ≤ 200 ms |

## Budget

| Item | EUR |
|---|---|
| DAQ boards (NI USB-6009 + breakout) | 500 |
| Workstation (primary) | 2,500 |
| UPS (2 kVA, for 72 h runs) | 800 |
| Software licenses | 0 (all FOSS) |
| **Total** | **~3,800** |
