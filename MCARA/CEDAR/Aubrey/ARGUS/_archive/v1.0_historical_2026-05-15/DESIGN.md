<!-- AUTO-GENERATED from CONCEPT.md by TBPR orchestrator 2026-05-10 ensure_core (DeepSeek-reasoner). Review and edit as needed. -->

# DESIGN.md — ARGUS: Architecture & Implementation

**Версия:** 2.0  
**Статус:** Post TBPR cycle-7 redesign (v4.0 CONCEPT)

## 1. Architecture Overview
```
┌──────────────────────────────────┐
│   AI Agent Layer (v4.0)         │
│  Claude Code (LLM)              │
│  DeepSeek Router                │
│  AI Constitution (§12)          │
│  (5 prohibited actions)         │
└────────┬─────────────────────────┘
         │ JSON RPC
┌────────▼─────────────────────────┐
│  Python API Layer                │
│  Tool Functions                  │
│  CUSUM Quality Control (§5)      │
│  (sliding window 50 cycles)      │
└────────┬─────────────────────────┘
         │ Serial USB
┌────────▼─────────────────────────┐
│  ESP32-S3 / STM32 Layer          │
│  (Realtime FSM)                  │
│  TMC2209 Stepper                 │
│  Physical Beacon Decoder         │
└────────┬─────────────────────────┘
         │ Physical
┌────────▼─────────────────────────┐
│  Custom inverted microscope (Tsomaia design) +                   │
│  LGY40-C XY Stage                │
│  Laser 450nm CW                  │
│  Camera (USB)                    │
│  Physical Beacon LED (10 Hz)     │
│  Hardware laser kill (reed)      │
└──────────────────────────────────┘
```

## 2. Components

### 2.1 Hardware
- **Microscope:** Custom inverted microscope (Tsomaia design) - **no modification** of original mechanics.
- **Stage:** LGY40-C motorized XY stage stacked on top of original manual stage.
- **Laser:** 450 nm CW diode laser, collimated, with TTL modulation.
- **Camera:** USB CMOS/CCD sensor (generic, to be specified in BOM).
- **ESP32-S3 / STM32:** Firmware implementing real‑time control:
  - Stepper driver (TMC2209) for LGY40‑C — OTA updates, silent step
  - PWM for laser power
  - Interlock circuit (door, temperature, emergency stop)
  - **Physical Beacon decoder** — reads 10 Hz LED flash, blocks laser if absent
- **Enclosure:** Light-tight box, OD 4+ filtered windows.
- **Safety:** UPS, hardware kill switch, thermal cutoff.

### 2.2 Software
- **Host OS:** Ubuntu 22.04 LTS (headless)
- **Agent:** Claude Code (Anthropic) via API; DeepSeek router as fallback router (not primary).
- **API Language:** Python 3.10+, using `pyserial`, `numpy`, `opencv`, `json`.
- **Tool Functions** (see 3.1).
- **Data Pipeline:** Local SSD → encrypted backup → optional cloud (not in scope).

### 2.3 Firmware (Arduino)
- Language: C++ (Arduino IDE / PlatformIO)
- State Machine: `IDLE → MOVING → LASERING → IMAGING → LOGGING → IDLE`
- Watchdog timer: 500 ms - if no serial command, enter SAFE.

## 3. Data Flow

```
Agent (L2) ──JSON RPC──> Python API (L1) ──Serial──> Arduino (L0) ──PWM──> Laser
                                        ──GPIO──> Stepper
                                        ──ADC──> Sensors
                                        <── Serial ── Status
Camera ──USB──> Python API ──Base64──> Agent (Image stored locally)
```

### Control Flow (Example: fire laser)
1. Agent sends `{"action": "fire_laser", "duration_ms": 100}` via REST/stdio.
2. Python validates input (bounds) AND checks **CUSUM control chart** — if AI accuracy <93% sliding window → **auto-stop test**.
3. Python checks **Physical Beacon** status — if beacon not detected → BLOCK laser.
4. Python checks **AI Constitution** — is this action prohibited? (§12)
5. Sends serial command `LASER 100\n` to ESP32-S3.
6. ESP32-S3 checks hardware interlock AND Physical Beacon decoder: if both OK, energises laser via PWM for 100 ms.
7. ESP32-S3 returns `OK` or `ERROR` with code.
8. Python logs and returns response to Agent.

## 4. API Specification

### 4.1 Tool Functions

| Function | Arguments | Returns | Description |
|----------|-----------|---------|-------------|
| `move_stage(x, y)` | `x`: μm (-5000..5000), `y`: μm (-5000..5000) | `{"status", "position"}` | Relative move (μm) |
| `fire_laser(duration_ms)` | `duration_ms`: int (1..10000) | `{"status", "energy_mJ"}` | Continuous wave pulse |
| `capture_image(exposure_ms)` | `exposure_ms`: int (10..5000) | `{"image_base64", "metadata"}` | Returns image |
| `detect_targets(image)` | `image`: base64 | `{"targets": [x,y,size,...]}` | Chloroplast detection |
| `get_status()` | none | `{"stage", "laser", "temp", "interlock"}` | Full system state |
| `set_laser_power(percent)` | `percent`: 0..100 | `{"status"}` | Calibrated power |

### 4.2 Serial Protocol (L1→L0)
- Baud: 115200
- Format: `CMD [arg]\n`
- Responses: `ACK` or `ERR <code>\n`
- Commands: `MOVE X Y\n`, `LASER DURATION\n`, `STATUS\n`, `CALIBRATE\n`, `STOP\n`

## 5. Safety Infrastructure
- **Hardware Interlock:** Door switch → cuts laser power supply directly.
- **Physical Beacon (§11 CONCEPT):** LED with 10 Hz encoded flash. Laser fires ONLY if beacon detected + frequency matches calibration. Hardware-level block, independent of AI.
- **Firmware Watchdog:** If no valid command for 500 ms, stage stops, laser off.
- **Software Watchdog:** Python monitors ESP32-S3 response; if missing, kills agent process.
- **Agent Check:** Claude Code sends periodic heartbeat; if fails, Python shuts down safely.
- **AI Constitution (§12 CONCEPT):** 5 prohibited actions: no laser power change >±10%, no disable beacon check, no safety param changes, no code without static analysis, no operation under any failure mode.
- **CUSUM Control Chart (§5 CONCEPT):** Monitors AI decision accuracy; if <93% on sliding window of 50 cycles → automatic stop test.

### FMEA (Failure Mode & Effects Analysis)

| Failure Mode | Effect | RPN | Mitigation |
|-------------|--------|:---:|------------|
| AI hallucinates target on empty slide | Laser fires at blank with no target | 60 | Physical Beacon (hardware block); second-AI cross-check 10% cycles |
| ESP32-S3 firmware crash | Stage stops, laser off | 40 | Watchdog 500 ms + systemd auto-restart |
| DeepSeek API outage | No strategic reasoning | 36 | Fallback: Llama 3.2 90B via Groq (~3s switch) |
| Stage repeatability >±5 µm | Position miss | 32 | n=50 pre-test; MKS SERVO42 closed-loop |
| 405 nm laser damages dichroic | Optical damage | 50 | Beam block before dichroic; sacrificial filter test |
| Camera failure | No visual feedback | 25 | Redundant camera path (Pi HQ as backup) |
| Full stack down | Total system halt | 80 | Telegram alert + laser disabled + stage parked (<30s detect)

## 6. Deployment
- Rig assembled on standard desk (no optical table - risk accepted).
- Remote access via SSH + reverse tunnel (for monitoring).
- Logging: all actions, errors, images saved with UTC timestamp.

---
## v3.1 Hardware Design Updates (2026-05-13)

### Modern motor stack (replaces Arduino Nano)
- MCU: ESP32-S3 / RP2040 / STM32 (RTOS-capable, OTA updates)
- Stepper driver: TMC2209 / TMC5160 silent step + StallGuard
- Closed-loop integrated: MKS SERVO42 (NEMA с magnetic encoder)
- Z focus: piezo actuator или voice coil servo
- Firmware: Klipper / Reach (lookahead motion planning)

### Laser Phase A vs Phase B
- Phase A: 450 nm CW, $500, ~50 μm spot - simulator commissioning
- Phase B: 355 nm Q-switched, $15-20K, ~200 nm diffraction-limited - biology
- Phase B UV objective: Nikon CFI Plan Fluor или Olympus XLUMPLFL ($8-12K)
