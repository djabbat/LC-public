<!-- AUTO-GENERATED from CONCEPT.md by TBPR orchestrator 2026-05-10 ensure_core (DeepSeek-reasoner). Review and edit as needed. -->

# DESIGN.md — ARGUS: Architecture & Implementation

**Версия:** 1.0  
**Статус:** Pre‑commissioning design

## 1. Architecture Overview
```
┌──────────────────────┐
│   AI Agent Layer     │
│  Claude Code (LLM)   │
│  DeepSeek Router     │
└────────┬─────────────┘
         │ JSON RPC
┌────────▼─────────────┐
│  Python API Layer    │
│  Tool Functions      │
│  (move_stage, ...)   │
└────────┬─────────────┘
         │ Serial USB
┌────────▼─────────────┐
│  Arduino Nano Layer  │
│  (Realtime FSM)      │
│  PWM, Stepper, Int.  │
└────────┬─────────────┘
         │ Physical
┌────────▼─────────────┐
│  Zeiss IM 35 +       │
│  LGY40-C XY Stage    │
│  Laser 450nm CW      │
│  Camera (USB)        │
└──────────────────────┘
```

## 2. Components

### 2.1 Hardware
- **Microscope:** Zeiss IM 35 / ICM 405 (inverted) — **no modification** of original mechanics.  
- **Stage:** LGY40‑C motorized XY stage stacked on top of original manual stage.  
- **Laser:** 450 nm CW diode laser, collimated, with TTL modulation.  
- **Camera:** USB CMOS/CCD sensor (generic, to be specified in BOM).  
- **Arduino Nano:** Firmware implementing real‑time control:  
  - Stepper driver (A4988) for LGY40‑C  
  - PWM for laser power  
  - Interlock circuit (door, temperature, emergency stop)  
- **Enclosure:** Light‑tight box, OD 4+ filtered windows.  
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
- Watchdog timer: 500 ms – if no serial command, enter SAFE.

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
2. Python validates input (bounds).  
3. Sends serial command `LASER 100\n` to Arduino.  
4. Arduino checks interlock: if SAFE, energises laser via PWM for 100 ms.  
5. Arduino returns `OK` or `ERROR` with code.  
6. Python logs and returns response to Agent.

## 4. API Specification

### 4.1 Tool Functions

| Function | Arguments | Returns | Description |
|----------|-----------|---------|-------------|
| `move_stage(x, y)` | `x`: μm (–5000..5000), `y`: μm (–5000..5000) | `{"status", "position"}` | Relative move (μm) |
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
- **Firmware Watchdog:** If no valid command for 500 ms, stage stops, laser off.  
- **Software Watchdog:** Python monitors Arduino response; if missing, kills agent process.  
- **Agent Check:** Claude Code sends periodic heartbeat; if fails, Python shuts down safely.

## 6. Deployment
- Rig assembled on standard desk (no optical table – risk accepted).  
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
- Phase A: 450 nm CW, $500, ~50 µm spot — simulator commissioning
- Phase B: 355 nm Q-switched, $15-20K, ~200 nm diffraction-limited — biology
- Phase B UV objective: Nikon CFI Plan Fluor или Olympus XLUMPLFL ($8-12K)
