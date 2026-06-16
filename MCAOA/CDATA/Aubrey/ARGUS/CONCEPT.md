# ARGUS-LP — Hardware Architecture & Specification

**Version:** 1.0 (2026-05-15)
**Status:** Design complete. Engineering build (Tsomaia G.) pending funding.
**Biological context:** See parent `../CONCEPT.md` (Aubrey — centriole age tracking, Phase A)

## 1. Design Philosophy

ARGUS-LP is a **purpose-built autonomous live-cell imaging station** designed from first principles for a single task: tracking individual centrioles in BJ-hTERT fibroblasts over 6 months with periodic staining, fully autonomous AI-driven analysis, and zero operator intervention.

Key design constraints:
1. **COTS only** — no custom optics; all components off-the-shelf or used-grade
2. **Isolated enclosure** — maintains 2-3% O₂, 5% CO₂, 37°C, 24/7
3. **Autonomous AI** — local RTX 4090 runs CellPose v3 + spotiflow + DeepSeek-V3 (local); zero API cost
4. **Periodic staining** — fluidics system (syringe pump + valves) delivers RITE reagents on schedule
5. **Independent replication** — 2 identical units: #1 at GLA Abastumani, #2 at independent site (Zheleznov, LoS pending)
6. **No Zeiss retrofit** — this is a new build, not a Zeiss IM 35 modification

## 2. Block Diagram

```
┌─────────────────────────────────────────────────────┐
│              ISOLATED ENCLOSURE                      │
│  ┌───────────────────────────────────────────┐      │
│  │  Optical Table (breadboard, 400×500 mm)    │      │
│  │                                            │      │
│  │  ┌──────┐    ┌───────────┐  ┌──────────┐  │      │
│  │  │ 100× │    │  sCMOS    │  │ 488/561  │  │      │
│  │  │ Plan │◄───│  ORCA-    │  │  laser   │  │      │
│  │  │ Apo  │    │  Flash4   │  │  module  │  │      │
│  │  └──┬───┘    └───────────┘  └──────────┘  │      │
│  │     │                                      │      │
│  │  ┌──▼───┐    ┌───────────┐  ┌──────────┐  │      │
│  │  │  XY  │    │  405 nm   │  │  Bright  │  │      │
│  │  │Stage │    │  galvo    │  │  field   │  │      │
│  │  │(NEMA)│    │  ablation │  │  LED     │  │      │
│  │  └──────┘    └───────────┘  └──────────┘  │      │
│  │                                            │      │
│  └───────────────────────────────────────────┘      │
│                                                      │
│  ┌───────────────────────────────────────────┐      │
│  │  Fluidics System                          │      │
│  │  Syringe pump + 3-port valves → dish      │      │
│  │  (RITE stain delivery on schedule)         │      │
│  └───────────────────────────────────────────┘      │
│                                                      │
│  ┌───────────────────────────────────────────┐      │
│  │  Environmental Control                    │      │
│  │  O₂ sensor → N₂ purge valve → 2-3% O₂    │      │
│  │  CO₂ sensor → CO₂ valve → 5%             │      │
│  │  Heater + PID → 37°C                     │      │
│  └───────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────┐
│              AI STATION (external to enclosure)       │
│  ┌────────────┐  ┌──────────┐  ┌────────────────┐   │
│  │ RTX 4090   │  │ PyMMCore │  │ DeepSeek-V3    │   │
│  │ GPU        │  │ ─Micro-  │  │ (local, no API)│   │
│  │            │  │  Manager │  │ ─agent loop    │   │
│  └────────────┘  └──────────┘  └────────────────┘   │
│                                                      │
│  ┌────────────┐  ┌──────────┐  ┌────────────────┐   │
│  │ CellPose   │  │ spotiflow│  │ Stitching      │   │
│  │ v3 (segm) │  │ (track)  │  │ (MIST / Fiji)  │   │
│  └────────────┘  └──────────┘  └────────────────┘   │
└─────────────────────────────────────────────────────┘
```

## 3. Bill of Materials Summary

Full BOM: `~/Desktop/ARGUS-LP_hardware_spec.md` (€13,850 per unit)

| Subsystem | Key Components | Cost |
|-----------|---------------|------|
| **Optics** | 100×/1.4 NA Plan Apo (Nikon CFI, used), sCMOS (PCO.edge / ORCA-Flash4, used) | €4,000 |
| **Lasers** | 488 nm + 561 nm CW modules (Taobao 50 mW), 405 nm pulsed diode + galvo | €1,400 |
| **Stage** | XY motorized (NEMA17 + Arduino + TMC2209 + endstops) | €500 |
| **Enclosure** | ACP + Al profile, O₂ sensor, N₂ purge, CO₂ control, heater, PID, optical breadboard | €2,200 |
| **Fluidics** | Syringe pump + 3-port valves + tubing + reservoirs | €400 |
| **Brightfield** | LED + condenser + beam combiner | €300 |
| **PC + GPU** | Mini PC / workstation + RTX 4090 | €3,500 |
| **Accessories** | Dichroics, bandpass filters, cage plates, adapters, wiring, connectors | €1,550 |
| **Total per unit** | | **€13,850** |

## 4. Build & Integration

Engineering subcontractor **Giorgi Tsomaia** (Dipl. Ing. Tbilisi Polytechnic, constructor and producer of automated systems) is responsible for:

1. **CAD** (40 h) — mechanical design of enclosure, stage mount, optics rail, fluidics integration
2. **Unit #1 assembly** (80 h) — first build at GLA Abastumani workshop
3. **Unit #2 assembly** (40 h) — second unit (reduced effort due to learnings)
4. **Calibration** (20 h) — Köhler alignment, stage calibration, laser power curve, fluidics timing
5. **Documentation** (15 h) — build guide, calibration protocol, troubleshooting
6. **Total: 195 h × €20/h = €3,900**

## 5. Two-Unit Strategy

| Unit | Location | Operator | Status |
|------|----------|----------|--------|
| #1 | GLA Abastumani (PI site) | Tqemaladze + GLA technician | Funded (Phase A) |
| #2 | Independent replication site (Zheleznov) | Ilia Zheleznov | LoS pending |

Both units are identical. A single GLA technician (€6,000, 50% FTE × 6 mo) maintains cell culture for both units; the postdoctoral fellow (€9,000, 50% FTE × 6 mo) leads image analysis and manuscript preparation.

## 6. References

- Full hardware specification: `~/Desktop/ARGUS-LP_hardware_spec.md`
- Schematic (diagrams.net): `~/Desktop/ARGUS-LP_schematic.drawio`
- Budget: `~/Desktop/ARGUS-LP_budget_v5.md`
- Biology CONCEPT (parent): `../CONCEPT.md`
- Meta-review (55/55): `~/Desktop/Marketing/Aubrey/Aubrey_META-REVIEW_v5.md`
- Engineering literature: `refs/` directory


---

## PR Recommendations Applied

**Error analysis added:**
- Optical resolution test: USAF target, ≤1 μm @ 40×
- Z-drift measurement: <1 μm over 10 min
- Temperature stability: 37±0.5°C over 48h
- Ablation precision: <0.5 μm targeting error
- AI pipeline latency: <500 ms/frame on RTX 4090

**Controls added:**
- Negative: fixed cells (no division) — verify no false positives
- Positive: cells with known centriole age asymmetry (mEos3.2)
- Blank: no-cell wells — detect background
- Replicate: 3 independent BJ-hTERT clones
