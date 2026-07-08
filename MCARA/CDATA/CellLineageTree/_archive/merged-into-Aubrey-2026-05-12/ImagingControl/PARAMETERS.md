# ImagingControl / PARAMETERS.md (merged 2026-05-09)


---
## === MicroscopeController / PARAMETERS.md ===

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

## Full Project Budget

The total requested budget for the ImagingControl subproject is **$92,500** (≈€85,000), within the Impetus $75–100K range. Breakdown:

| Category | Description | EUR | USD (approx) |
|---|---|---|---|
| **Personnel** | 1 Postdoctoral Fellow (12 months, 100% effort) | 60,000 | 65,000 |
| **Indirect costs** | Institutional overhead (20% of direct costs) | 12,000 | 13,000 |
| **Equipment** | DAQ boards, workstation, UPS (see detailed list below) | 3,800 | 4,100 |
| **Cameras** | 2 × Hikrobot MV-CH050-10UM + cables, TEC mod, calibration | 2,300 | 2,500 |
| **AICoordinator** | Cloud API usage (DeepSeek, Claude) for 18 months | 540 | 590 |
| **Consumables** | Media, plates, optical components, fuses, shipping | 3,000 | 3,300 |
| **Contingency** | 5% of total direct costs | 3,600 | 3,900 |
| **TOTAL** | | **85,240** | **92,500** |

*Note: Personnel costs are based on NIH postdoc minimum for Orange County, CA; indirect cost rate is 20% as per institution policy. Equipment costs remain as previously detailed below.*

### Equipment Detail (from earlier budget)

| Item | EUR |
|---|---|
| DAQ boards (NI USB-6009 + breakout) | 500 |
| Workstation (primary) | 2,500 |
| UPS (2 kVA, for 72 h runs) | 800 |
| Software licenses | 0 (all FOSS) |
| **Total** | **~3,800** |

## Timeline and Milestones

| Month | Milestone | Deliverable |
|---|---|---|
| 1–2 | Hardware procurement & assembly | Microscope fully assembled; cameras calibrated |
| 3 | Software integration (PyMMCore-Plus, drivers) | All device interfaces operational in test harness |
| 4 | Autofocus & drift correction functional | PD controller tuned; >90% in-focus in 24h test |
| 5 | Segmentation pipeline (CellPose) deployed | Dice coefficient ≥0.85 on 20 test frames |
| 6 | AICoordinator dry-run integration | End-to-end pipeline runs on recorded data without hardware |
| 7 | Pilot ablation + viability test | ≥10 ablation events; cell survival >95% at 2h |
| 8–9 | 48h endurance test | Full pipeline running for 48h; tracking accuracy assessed |
| 10 | Data analysis & validation | Lineage trees from pilot; risk mitigation implemented |
| 11–12 | Final adjustments & documentation | Complete protocol, code repository, data archive on Zenodo |
| 13–18 | (Optional extension) Scale to multiple fields | ROI burst handling; extended runs (72h) if budget permits |

## Risk Matrix

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Camera driver instability (Hikrobot) | Medium | High – loss of dual-channel imaging | Fallback to FLIR cameras; extended warranty and spare unit stocked |
| Autofocus drift >200 nm after 24h | Low | Medium – focus quality degrades | Implement software-based refocus check every 10 cycles; use differential Z-scan |
| Cell segmentation accuracy <0.85 | Medium | High – tracking errors | Retrain CellPose on more manual masks (target 50 frames); use ensemble model |
| AICoordinator LLM latency >200 ms | High | Low – decision speed slower than ideal | Cache previous decisions; prioritize local rule-based fallback for timing-critical actions |
| Laser ablation insufficient power | Low | Medium – incomplete centriole removal | Pre-characterize power at sample; use 2 ms pulse at 100% AOTF; verify with test beads |
| Power outage / USB disconnect | Medium | High – run abort | UPS (2 kVA) + auto-recovery script; data saved per frame to Zarr |
| Personnel turnover | Low | High – project delay | Cross-train two team members; document all code; hire postdoc with 1-year commitment |

---
## === FluorescentCameras / PARAMETERS.md ===

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
**Note:** Warranty support for Hikrobot cameras is limited; we will purchase extended warranty (~$100) through the Chinese vendor and keep a spare unit in stock.

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

---
## === AICoordinator / PARAMETERS.md ===

# AICoordinator — PARAMETERS

## Orchestration stack

| Component | Purpose |
|---|---|
| Claude Code `/overnight` | Session persistence, retry, systemd-inhibit sleep |
| DeepSeek API (`deepseek-reasoner`) | Heavy reasoning on ambiguous decisions |
| Local `PROMPT.md` | Experiment policy, invariants, safety rules |
| Zarr store reader | Ingest segmentation + graph state |
| JSON command emitter | Dispatch to MicroscopeController |

## Decision loop

| Parameter | Value |
|---|---|
| Loop period | 30–60 s |
| Per-decision token budget | ≤ 2 k tokens (default) |
| Hard safety timeout | 10 s per command |
| Human override latency | < 5 s via dashboard |

## Policy categories (in PROMPT.md)

1. **Tree shape** — keep ≤ 8 active leaves in field of view
2. **Centriole age bias** — prune daughters inheriting the younger (green) centriole when policy demands old-lineage tracking; control ablations target daughters inheriting the older centriole
3. **Focus / drift** — trigger `adaptive_refocus` when drift > 200 nm
4. **Mitotic burst** — switch to 30 s interval when prometaphase detected
5. **Phototoxicity budget** — throttle exposure if division rate drops > 20 %
6. **Abort conditions** — death of > 50 % tracked cells → abort run

## Safety rules

- Every ablation call dry-runs first (logs intent; pauses 1 s; executes unless veto flag set)
- Daily summary auto-posted to user (Slack / Telegram)
- Hard stop on 3 consecutive device errors
- All commands logged immutably (append-only JSON-Lines)

## Budget

| Item | EUR / month |
|---|---|
| DeepSeek API | 10 (typical load) |
| Claude Code subscription | 20 (if Pro) |
| Monitoring (Slack / Grafana cloud) | 0–10 |
| **Total** | **~30 / month** |

---
