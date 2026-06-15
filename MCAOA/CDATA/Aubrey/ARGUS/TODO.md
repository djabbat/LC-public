# ARGUS-LP — Build Checklist

**Owner:** G. Tsomaia (engineering subcontractor)
**Status:** Pending funding decision
**Reference:** `CONCEPT.md` (this dir), `~/Desktop/ARGUS-LP_hardware_spec.md`

## Phase 0 — Procurement (Weeks 1-4)

- [ ] Order 100×/1.4 NA Plan Apo objective (Nikon CFI, used — eBay / surplus)
- [ ] Order sCMOS camera (PCO.edge 5.5 or Hamamatsu ORCA-Flash4, used — eBay / lab surplus)
- [ ] Order 488 nm + 561 nm CW laser modules (Taobao 50 mW)
- [ ] Order 405 nm pulsed diode + galvo steering mirror + driver
- [ ] Order NEMA17 steppers + TMC2209 drivers + endstops + GT2 belts + pulleys
- [ ] Order optical breadboard (400 × 500 mm, metric M6 grid)
- [ ] Order ACP sheet (3 mm, 1200 × 600 mm) + Al profile (2020 series)
- [ ] Order O₂ sensor (SprintIR-W or LuminOx), CO₂ sensor (K30), N₂ purge valve (solenoid)
- [ ] Order heater cartridge + PID controller + thermocouple
- [ ] Order syringe pump + 3-port pinch valves + Tygon tubing + reservoir vials
- [ ] Order dichroic 488 LP (Edmund #87-242 or Chroma ZT488rdc)
- [ ] Order bandpass 510/20 (Edmund #87-789), dichroic 561 LP, bandpass 590/20
- [ ] Order cage system components (Thorlabs / Edmund: cage plates, lens tubes, adapters)
- [ ] Order brightfield LED (white, CW) + collimator
- [ ] Order RTX 4090 GPU + mini-PC or workstation
- [ ] Order UPS ×2 (APC Smart-UPS SMT1500 or equivalent)
- [ ] Order 4G modem + IoT relay for remote monitoring
- [ ] Order misc: wiring, connectors, heat shrink, solder, M6 bolts, L-brackets, silicone sealant, matte black paint

## Phase 1 — CAD & Design (Weeks 5-6)

- [ ] CAD: Enclosure layout (ACP + Al profile, 600 × 500 × 400 mm)
- [ ] CAD: Optical rail / breadboard layout (objective → sCMOS → laser combiner)
- [ ] CAD: XY stage mount + motorization (NEMA17 brackets, belt tensioner, endstop mounts)
- [ ] CAD: Fluidics manifold + syringe pump mount + reservoir positions
- [ ] CAD: Gas controller board layout (O₂, CO₂, N₂, heater, PID)
- [ ] CAD: Cable gland positions, Z-baffle light trap design
- [ ] CAD: Brightfield path (LED → condenser → beam combiner into objective path)
- [ ] Review CAD with P. Tqemaladze

## Phase 2 — Unit #1 Build (Weeks 7-14)

### Enclosure
- [ ] Cut ACP panels, assemble frame with Al profile
- [ ] Seal all seams with silicone + matte black interior paint (2 coats)
- [ ] Install door + hinges + magnetic interlock (reed switch + solenoid lock)
- [ ] Install Z-baffle light traps (intake + exhaust)
- [ ] Install cable glands, fan (Noctua 120mm, if needed for electronics cooling)

### Optics
- [ ] Mount breadboard in enclosure on vibration pads
- [ ] Assemble laser combiner (488 → dichroic 488 LP → objective; 561 → dichroic 561 LP)
- [ ] Mount objective (100× Plan Apo) in cage plate, align on breadboard
- [ ] Mount sCMOS camera on translation stage for fine focus adjust
- [ ] Mount brightfield LED + condenser + beam combiner
- [ ] Mount 405 nm galvo + steering optics (per geometry in schematic)
- [ ] Initial Köhler alignment with TetraSpeck beads

### Stage
- [ ] Assemble XY stage (NEMA17 + TMC2209 + GT2 belt)
- [ ] Mount on breadboard, align to optical axis
- [ ] Calibrate step/mm, measure hysteresis, backlash compensation
- [ ] Endstops → limit switch wiring

### Fluidics
- [ ] Assemble syringe pump + 3-port valves on breadboard
- [ ] Route tubing to dish position (capillary tip, adjustable height)
- [ ] Test flow rates, dead volume, bubble prevention

### Environmental
- [ ] Install O₂ sensor, CO₂ sensor in enclosure
- [ ] Install N₂ purge solenoid valve + CO₂ solenoid valve
- [ ] Install heater cartridge + thermocouple
- [ ] Wire gas controller (Arduino / ESP32 → PID → solenoid valves)
- [ ] Calibrate: target 2-3% O₂, 5% CO₂, 37°C, 95% RH

### Electronics
- [ ] Wire all actuators to Arduino / ESP32-S3 (steppers, lasers, pump, valves, fan, relay)
- [ ] Wire all sensors to Arduino / ESP32-S3 (O₂, CO₂, temp, endstops, interlock)
- [ ] Serial protocol: JSON-lines commands, polling sensors, logging to SD
- [ ] PC connection: USB serial to AI station (RTX 4090 PC)

## Phase 3 — Software Integration (Weeks 15-16)

- [ ] Install PyMMCore / Micro-Manager server on AI station
- [ ] Install CellPose v3 + spotiflow (conda env)
- [ ] Install DeepSeek-V3 (local, Ollama or llama.cpp)
- [ ] Write agent loop: PyMMCore commands → focus → acquire → segment → track → log
- [ ] Write fluidics scheduler: timepoint → open valve → pump RITE stain → wait → flush → close
- [ ] Write gas controller communication script (serial → PID setpoint)
- [ ] Write remote monitoring dashboard (fastAPI + Web + 4G modem)
- [ ] Test on TetraSpeck beads (Phase 0 acceptance: SNR ≥ 5×)

## Phase 4 — Calibration & Acceptance (Week 17)

- [ ] Full system run: 48 h continuous on TetraSpeck beads
- [ ] Measure: stage drift, focus stability, illumination uniformity, laser power stability
- [ ] Measure: O₂/CO₂/T stability over 48 h
- [ ] Acceptance criteria:
  - SNR ≥ 5× background on 100 nm TetraSpeck beads
  - XY stage repeatability ≤ 1 µm RMS
  - Focus drift ≤ 100 nm over 24 h (auto-focus / refocus every 30 min)
  - Gas: O₂ 2-3%, CO₂ 5%, temp 37°C ± 0.5°C over 48 h
  - Fluidics: stain delivery ± 10% volume, no bubbles

## Phase 5 — Unit #2 Build (Weeks 18-22)

- [ ] Repeat Phase 2-4 for Unit #2 (reduced time: ~40 h vs 80 h due to learnings)
- [ ] Cross-calibrate: same bead sample measured on both units, compare SNR, stage accuracy

## Documentation

- [ ] Build guide (with photos)
- [ ] Calibration protocol
- [ ] Troubleshooting guide
- [ ] Safety checklist (laser interlock, OD4+ goggles, UV warning signs)
- [ ] Shipping protocol (packing, transport, re-calibration at replication site)
