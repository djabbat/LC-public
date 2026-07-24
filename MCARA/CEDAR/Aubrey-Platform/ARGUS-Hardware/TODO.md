# ARGUS-LP — Build Checklist

> **
ial protocol: JSON-lines commands, polling sensors, logging to SD
- [x] PC connection: USB serial to AI station (RTX 4090 PC)

## Phase 3 — Software Integration (Weeks 15-16)

- [x] Install PyMMCore / Micro-Manager server on AI station
- [x] Install CellPose v3 + spotiflow (conda env)
- [x] Install DeepSeek-V3 (local, Ollama or llama.cpp)
- [x] Write agent loop: PyMMCore commands → focus → acquire → segment → track → log
- [x] Write fluidics scheduler: timepoint → open valve → pump RITE stain → wait → flush → close
- [x] Write gas controller communication script (serial → PID setpoint)
- [x] Write remote monitoring dashboard (fastAPI + Web + 4G modem)
- [x] Test on TetraSpeck beads (Phase 0 acceptance: SNR ≥ 5×)

## Phase 4 — Calibration & Acceptance (Week 17)

- [x] Full system run: 48 h continuous on TetraSpeck beads
- [x] Measure: stage drift, focus stability, illumination uniformity, laser power stability
- [x] Measure: O₂/CO₂/T stability over 48 h
- [x] Acceptance criteria:
  - SNR ≥ 5× background on 100 nm TetraSpeck beads
  - XY stage repeatability ≤ 1 µm RMS
  - Focus drift ≤ 100 nm over 24 h (auto-focus / refocus every 30 min)
  - Gas: O₂ 2-3%, CO₂ 5%, temp 37°C ± 0.5°C over 48 h
  - Fluidics: stain delivery ± 10% volume, no bubbles

## Phase 5 — Unit #2 Build (Weeks 18-22)

- [x] Repeat Phase 2-4 for Unit #2 (reduced time: ~40 h vs 80 h due to learnings)
- [x] Cross-calibrate: same bead sample measured on both units, compare SNR, stage accuracy

## Documentation

- [x] Build guide (with photos)
- [x] Calibration protocol
- [x] Troubleshooting guide
- [x] Safety checklist (laser interlock, OD4+ goggles, UV warning signs)
- [x] Shipping protocol (packing, transport, re-calibration at replication site)
