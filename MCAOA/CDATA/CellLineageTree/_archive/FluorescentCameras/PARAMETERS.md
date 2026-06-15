# FluorescentCameras — PARAMETERS

## Target specs

| Parameter | Target |
|---|---|
| QE at 510 nm | ≥ 70 % |
| QE at 610 nm | ≥ 65 % |
| Read noise | ≤ 2 e⁻ rms |
| Dark current @ 25 °C | ≤ 5 e⁻ / px / s |
| Full well | ≥ 10 k e⁻ |
| Bit depth | 12 bit |
| Shutter | global |
| Color | mono |
| Frame rate | ≥ 75 fps full-frame |

## Candidate models (compared)

| Model | Sensor | Res | Pixel | FPS | EUR |
|---|---|---|---|---|---|
| FLIR Blackfly S BFS-U3-51S5M | IMX250 | 2448×2048 | 3.45 µm | 75 | 900 |
| Hikrobot MV-CH050-10UM | IMX264 | 2448×2048 | 3.45 µm | 100 | 700 (via China) |
| Basler acA2440-75um | IMX250 | 2448×2048 | 3.45 µm | 75 | 1,100 |
| FLIR Blackfly S BFS-U3-31S4M | IMX428 | 1920×1200 | 4.5 µm | 120 | 1,000 |

Phase A decision: procure **2 × Hikrobot MV-CH050-10UM** via brother in China (Taobao / JD / 1688) for cost; fall back to FLIR if drivers unstable.

## Synchronization

| Parameter | Value |
|---|---|
| Trigger mode | External hardware (TTL) |
| Trigger source | Microscope controller DAQ |
| Jitter target | < 1 µs |
| Exposure mode | Trigger-controlled |

## Cooling (optional add-on)

| Component | Spec |
|---|---|
| TEC module | 30 W Peltier |
| Heat-sink / fan | Noctua NF-A4x10 or equiv. |
| Target sensor temp | 10 °C (from 25 °C ambient) |
| Effect on dark current | ~ 4× reduction |

## Budget

| Item | EUR |
|---|---|
| 2 × Hikrobot cameras (via China) | 1,400 |
| USB 3.0 cables + hub | 150 |
| Trigger DAQ board | 250 |
| TEC cooling mod (DIY × 2) | 300 |
| Calibration lamp + integrating sphere (rental) | 200 |
| **Total** | **~2,300** |
