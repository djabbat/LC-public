# ARGUS — AI-Resident Robotic Genealogical Ultra-surveillance for Lineage Purification

**Версия:** 4.3 (2026-05-15)
**Статус:** Post TBPR cycle-11 — **under revision**
**Фаза:** Construction — **simulator-only**, no live cells

## Цель

Валидация hardware + AI software stack для автономной laser ablation платформы.
Обоснование: валидация предсказаний центриолярного Counter #1 в рамках MCAOA framework.

## Что валидируется (v4.0)

1. **Claude Code `/overnight` agent + DeepSeek Router** — автономное управление 24/7
2. **Physical Beacon (§11)** — hardware block AI hallucinations (10 Hz LED encoder)
3. **AI Constitution (§12)** — 5 запрещённых действий (enforced через static analysis)
4. **CUSUM control chart (§5)** — мониторинг дрейфа точности AI (93% boundary)
5. **Laser TTL control** — ESP32-S3 PWM от Python команды → ablation target
6. **Motorized stage (ESP32-S3 + TMC2209)** — AI командует перемещение
7. **Feedback loop** — детекция → решение → действие → логирование
8. **Error handling + FMEA** — 7 failure modes, RPN < 100
9. **Data pipeline** — 6-мес TIFF stream без data loss

## Что НЕ валидируется

- ❌ Живые клетки (BJ-hTERT, iPSC, Elodea)
- ❌ Центриолярная биология
- ❌ Translational claims к mammalian CDATA
- ❌ Impetus / EIC Pathfinder грантовое позиционирование

## Архитектура (v4.0)

```
┌──────────────────────────────────────────────────┐
│  AI AGENT LAYER                                  │
│  Claude Code + DeepSeek-reasoner (strategic)     │
│  Gemini 2.5 Flash (vision)                       │
│  AI CONSTITUTION (5 prohibited actions)          │
└──────────────────────┬───────────────────────────┘
                       │ JSON RPC
┌──────────────────────▼───────────────────────────┐
│  PYTHON API LAYER                                │
│  Tool Functions (move_stage, fire_laser...)      │
│  CUSUM Control Chart (window 50 cycles)          │
│  Physical Beacon Verifier (10 Hz check)          │
└──────────────────────┬───────────────────────────┘
                       │ Serial USB (115200 baud)
┌──────────────────────▼───────────────────────────┐
│  ESP32-S3 FIRMWARE LAYER                         │
│  TMC2209 Stepper Driver (closed-loop)            │
│  Physical Beacon Decoder (hardware laser kill)   │
│  Watchdog (500 ms)                               │
│  FSM: IDLE→MOVING→LASERING→IMAGING→LOGGING      │
└──────────────────────┬───────────────────────────┘
                       │ Physical
┌──────────────────────▼───────────────────────────┐
│  HARDWARE LAYER                                  │
│  Custom inverted microscope (Tsomaia design)                          │
│  LGY40-C XY Stage + MKS SERVO42 (closed-loop)   │
│  Laser 450 nm CW (Phase A) → 355 nm Q-sw (Ph.B) │
│  ToupCam E3CMOS05000KMA (Sony IMX264)           │
│  Physical Beacon LED (10 Hz encoded flash)       │
│  Hardware interlock (reed switch → SPDT relay)   │
└──────────────────────────────────────────────────┘
```

## Acceptance criteria (Phase A → graduation)

| # | Criterion | Metric |
|:-:|-----------|--------|
| 1 | SNR | ≥ 5× background on Kaede-beads |
| 2 | XY repeatability | ±5 µm (n=50, Holm-Bonferroni) |
| 3 | Discrimination accuracy | ≥95% old vs new beads (≥500 cycles) |
| 4 | CUSUM | 0 crossings of 93% boundary / 6 months |
| 5 | Physical Beacon | 10+ stress tests (hidden → laser blocked) |
| 6 | Safety interlock FMEA | RPN < 100 |
| 7 | Uptime | ≥99% over 6 months |

## Фазы (6 месяцев construction)

| Phase | Sample | Cost | Goal |
|-------|--------|:----:|------|
| 0a | TetraSpeck beads | $280 | Optical alignment |
| 0b | FluoSpheres 488/520 | $250 | Fluorescence baseline |
| 1 | Kaede-beads simulator | $300-500 | AI/mechanical calibration |
| 2 | Phase A commissioning complete | included | Graduation to biology |
| 3 | Aubrey biology phase (separate) | $3K+ | BJ-hTERT + Centrin1-Kaede |

## Core files (v4.3)

| File | Description |
|------|-------------|
| `CONCEPT.md` | ⭐ Master document — full proposal (v4.3) |
| `DESIGN.md` | Architecture, Physical Beacon, FMEA, AI Constitution |
| `PARAMETERS.md` | Technical specs, acceptance criteria, statistical protocol |
| `THEORY.md` | Formal framework with CUSUM, falsifiable predictions |
| `KNOWLEDGE.md` | 14 verified PMIDs, MCAOA context |
| `EVIDENCE.md` | Literature evidence & gaps |
| `TODO.md` | v4.0 checklist |
| `UPGRADE.md` | Changelog |
| `MAP.md` | Project map |
| `MEMORY.md` | Context & decisions |
| `CLAUDE.md` | Instructions for Claude Code |
| `OPEN_PROBLEMS.md` | Open technical issues |
| `LINKS.md` | Vendor + reference links |
| `E1_Surrogates_for_Commissioning.md` | Cheap bead surrogates for debugging |
| `E2_IF_Validation_Block.md` | IF validation block (post-commissioning) |
| `PROJECT_AUDIT_2026-05-12.md` | Last audit |

## MCAOA framework context

ARGUS operates within the **Multi-Counter Architecture of Organismal Aging (MCAOA)**
framework (Tqemaladze J. 2026, DOI: 10.5281/zenodo.20055806). The centriolar
**Counter #1** is one of the key limiters of stem cell replicative potential;
ARGUS is the empirical validation tool for its predictions.

---

*README v4.3 — 2026-05-15. Synced with CONCEPT.md v4.3: Parrish moved to §9 (Grant context — Phase B downstream). Team §8 is PI-only.*
