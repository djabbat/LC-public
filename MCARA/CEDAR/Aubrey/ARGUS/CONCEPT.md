# ARGUS-LP — Hardware Architecture & Specification

> **⚠️ v2.0 (АКТУАЛЬНАЯ, 2026-06-27):** аппаратная архитектура полностью пересмотрена после переписки с Алексеем.
>
> **v1.0 (ниже, 2026-05-15, СОХРАНЕНА ДЛЯ ИСТОРИИ):** 100× Oil, sCMOS ORCA-Flash4, RTX 4090, Claude API, BSL-2, fluidics.

---

## v2.0 — Актуальная архитектура (2026-06-27)

### Ключевые изменения (v1.0 → v2.0)

| Параметр | v1.0 (15 мая) | **v2.0 (27 июня)** | Причина |
|----------|:---:|:---:|------|
| **Объектив** | 100×/1.4 Oil | **40×/0.95 сухой (V1) / 60×/1.2 Water (V2)** | Масло сохнет при 37°C за 4–6 ч |
| **Иммерсия** | Масляная | **Водяная (collar + шприцевой насос 0.1 мл/ч)** | Вода не сохнет, 24/7 |
| **Камера** | sCMOS ORCA-Flash4 | **RasPi HQ (V1) / sCMOS (V2)** | V1: бюджетный PoC |
| **Освещение** | Лазеры 488/561/405 | **LED 488/561 (V1) / Лазеры (V2)** | V1: без лазерной безопасности |
| **Управление** | RTX 4090 (внутри/рядом) | **RasPi 5 + SSD — снаружи ящика** | Экономия €3,500 → $80, без перегрева |
| **AI-ускоритель** | RTX 4090 | **Hailo-8L (13 TOPS, $70)** | CellPose <300 мс, 20W вместо 450W |
| **AI-мозг** | Claude/Gemini API (платный) | **Mac M4 Pro 48GB + Qwen VL 72B (локально, $0)** | Без API, без интернета |
| **LLM** | DeepSeek-V3 (RTX 4090) | **Mixtral 8×7B (V4) → Qwen 2.5 VL 72B (V6)** | Vision + код + рассуждение |
| **Агент** | Свой Python-цикл | **OpenHands (SEE→THINK→ACT→OBSERVE)** | Уровень Claude Code |
| **Самообучение** | — | **LoRA еженедельно + RAG (Qdrant) + аномалии** | V5 |
| **BSL** | 2 (лентивирус) | **1 (аденовирус)** | Не требует бокса и −80°C |
| **Fluidics** | Syringe pump + valves (RITE) | **Водяная муфта + насос (только иммерсия)** | Для PoC fluidics не нужна |
| **Сборка** | Цомая (195 ч × €20) | **Инженеры Алексея** | Кооперация |
| **Бюджет** | €13,850/ед. × 2 = €27,700 | **V1: $2,045 / V2: $5,945 / V6: $8,170** | Поэтапный подход |

### Шесть версий (поэтапная сборка)

| Версия | Бюджет | Объектив | Освещение | Камера | AI | Что делает |
|--------|:---:|------|------|------|-----|------|
| **V1** | **$2,045** | 40× сухой | LED 488/561 | RasPi HQ | CPU | Поймать деление |
| **V2** | **$5,945** | 60×/1.2 Water | Лазеры 405/488/561 | sCMOS | CPU | + элиминация |
| **V3** | **$6,020** | 60×/1.2 Water | Лазеры | sCMOS | Hailo-8L | + real-time зрение |
| **V4** | **$8,120** | 60×/1.2 Water | Лазеры | sCMOS | Mac M4 Pro + Mixtral | + AI-агент |
| **V5** | **$8,170** | 60×/1.2 Water | Лазеры | sCMOS | M4 Pro + LoRA + RAG | + самообучение 24/7 |
| **V6** | **$8,170** | 60×/1.2 Water | Лазеры | sCMOS | M4 Pro + Qwen VL 72B + OpenHands | Claude Code-уровень с vision |

### Актуальные документы

| Документ | Путь |
|----------|------|
| **Полная версия v3 (все 6 версий)** | `~/Desktop/ARGUS-LP_v3_final_2026-06-27.md` |
| **Инженерный BOM (35 + 19 позиций)** | `~/Desktop/Marketing/ARGUS-LP/docs/correspondence/ARGUS_LP_components_attachment_2026-06-27.md` |
| **Сопроводительное письмо Алексею** | `~/Desktop/ARGUS_LP_followup_v3_2026-06-27.md` |
| **Сравнительный анализ всех версий** | `~/Desktop/Marketing/ARGUS-LP/docs/correspondence/ARGUS_LP_comparison_all_versions_2026-06-27.md` |
| **Параметры** | `../PARAMETERS.md` |

### Компоновка (v2.0)

Герметичный алюминиевый ящик 40×30×30 см. **Инвертированная схема:** объектив 60×/1.2 Water Immersion снизу, чашка Петри 35mm со стеклянным дном — сверху, неподвижно. X-Y-Z каретка перемещает объектив + оптический тракт + лазеры. **RasPi 5 + Hailo-8L + SSD 1 TB — снаружи ящика.** Mac M4 Pro 48GB — отдельно, управляет агентом.

**Водяная иммерсия снизу вверх:** водяная муфта (collar, 3D-печать PETG) + капиллярная подача от шприцевого насоса 0.1 мл/ч. Поверхностное натяжение (~500 Па) >> гидростатического (~10 Па). Вода не падает.

### Защита от лазерного излучения (V2+)

- Научная камера: эмиссионный фильтр OD6+ + дихроик OD4+ + механический затвор Uniblitz LS2 (закрыт синхронно с 405 nm)
- Камеры наблюдения: notch-фильтры 405/488/561 OD4+
- Водяной объектив: абляция под стеклом, дебрис в среде, вода чистая
- Лазерная безопасность Class 3B: стенки OD6+, магнитный интерлок, ключ-выключатель, защитные очки

### AI-конвейер (V3–V6)

**Быстрый мозг (RasPi 5 + Hailo-8L):** CellPose v3 + spotiflow <300 мс/кадр.  
**Умный мозг (Mac M4 Pro 48GB + Qwen VL 72B + OpenHands):** видит кадры + маски, рассуждает, принимает решения, пишет код. Локально, без API.  
**Самообучение (V5):** LoRA еженедельно на исправлениях человека + RAG-память (Qdrant) + детекция аномалий.

---

## v1.0 — Историческая версия (2026-05-15)

---

# ARGUS-LP — Hardware Architecture & Specification (v1.0)

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

## 5. Two-Unit Strategy (v1.0, историческая)

| Unit | Location | Operator | Status |
|------|----------|----------|--------|
| #1 | GLA Abastumani (PI site) | Tqemaladze + GLA technician | Funded (Phase A) |
| #2 | Independent replication site (Zheleznov) | Ilia Zheleznov | LoS pending |

Both units are identical. A single GLA technician (€6,000, 50% FTE × 6 mo) maintains cell culture for both units; the postdoctoral fellow (€9,000, 50% FTE × 6 mo) leads image analysis and manuscript preparation.

## 6. References (v1.0)

- Full hardware specification: `~/Desktop/ARGUS-LP_hardware_spec.md`
- Schematic (diagrams.net): `~/Desktop/ARGUS-LP_schematic.drawio`
- Budget: `~/Desktop/ARGUS-LP_budget_v5.md`
- Biology CONCEPT (parent): `../CONCEPT.md`
- Meta-review (55/55): `~/Desktop/Marketing/Aubrey/Aubrey_META-REVIEW_v5.md`
- Engineering literature: `refs/` directory


---

## PR Recommendations Applied (v1.0)

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
