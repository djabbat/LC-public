# PARAMETERS.md — ARGUS

**Версия:** 2.0
**Дата:** 2026-05-15
**Назначение:** Численные параметры — optics, laser, camera, control, timing, acceptance criteria. Авторитетный источник для BOM + control software.

## Микроскоп

| Параметр | Значение |
|---|---|
| Model | Custom inverted microscope (Tsomaia design) |
| Год выпуска | ~1978-1989 (Opton era) |
| Объектив primary (Phase 0) | 40× / 0.65 (Plan) |
| Photo port projection | 3.2× (встроенный, OPTON 47 17 73-9901) |
| Effective magnification на sensor | 128× (объектив × 3.2) |
| Filter cube slider | OPTON 46 52 24-01 (2 позиции: DAPI + FITC/GFP) |

## Sampling / resolution (при 40× objective)

| Параметр | Значение |
|---|---|
| Sensor pixel size (ToupCam IMX264) | 3.45 µm |
| Image pixel в sample plane | 27 nm (oversampled 15×) |
| FOV at 40× on 5MP sensor | ~42 × 27 µm |
| Diffraction limit (Abbe, 550 nm, NA=0.65) | 423 nm |
| Рекомендация: 0.5× reducer | → 54 nm/pixel, 8× oversample, FOV удваивается |

## Освещение — transmitted light

| Параметр | Значение |
|---|---|
| **Custom microscope original spec** | 12V 60W halogen, Tsomaia design - custom LED illumination |
| **Spare box у PI** | OSRAM 64607 EFM HLX BELLAPHOT, 8V 50W, GZ6.35 MR16 (другое напряжение — нужно сверить с трансформатором) |
| Base (оба типа) | **GZ6.35** (bi-pin, 6.35 мм spacing, MR16-класс) |
| ⚠️ TODO: measure LED driver voltage
| Ресурс halogen | 50-200 ч (недостаточно для 6 мес = 4,320 ч непрерывно) |
| **Upgrade: LED retrofit** | Cree XHP50.2 J4 6500K underrun @ 5W — **с pulse-on-capture режимом** (см. ниже) |
| LED driver | Meanwell LDD-700H (CC 700mA, PWM gate input) |
| LED mount | Custom 3D-printed под GZ6.35 socket, height = высота оригинала (сохранить Köhler plane) |

### LED operation mode — **pulse-on-capture** (НЕ continuous 6 мес!)

| Параметр | Значение |
|---|---|
| **Mode** | Pulse/cycle — включается ТОЛЬКО на время capture |
| Warmup before trigger | 50 ms (settle LED driver + stable output) |
| Exposure window | 100 ms (brightfield Elodea) |
| Per-capture ON time | **150 ms** (50 warmup + 100 exposure) |
| Between captures | LED **OFF** (Arduino PWM pin = LOW) |
| Time-lapse interval | 30 min |
| Captures per timepoint | 2 (pre + post ablation) |
| **Duty cycle** | 300 ms / 30 min = **0.017%** (~1 part in 6,000) |
| **Total ON за 6 мес** | ~30 минут (vs 4,320 часов если 24/7) |
| Photodose на sample | **минимален** — сохраняет хлоропласты, избегает phototoxicity |
| Thermal drift | <0.5°C (vs +3-5°C при 24/7) |
| LED degradation за 6 мес | **<0.01%** (vs 3% при 24/7) |
| UPS runtime в standby (5W→2W idle) | **20+ часов** (vs 2-3 часа при 24/7) |

**Why pulse mode:**
1. 6 месяцев непрерывного 5W света @ 6500K = **phototoxicity disaster** для Elodea chloroplasts (выгорят за 1-2 дня, клетки умрут за неделю)
2. Background fluorescence поднимается 10-100× при continuous → SNR рушится
3. Thermal stability образца
4. Default-safe state (pin LOW = OFF) при Claude crash

**Arduino firmware pattern:**
```cpp
void capture_with_led(uint16_t exposure_ms) {
    analogWrite(LED_PWM_PIN, led_target_pwm);  // ON, start warmup
    delay(50);                                  // LED settles
    camera.trigger_start();                     // begin exposure
    delay(exposure_ms);                         // exposure window
    camera.trigger_stop();
    analogWrite(LED_PWM_PIN, 0);                // OFF immediately
}
```

## Освещение — epi-fluorescence (Phase 0 optional, Phase A required)

| Параметр | Значение |
|---|---|
| Housing | OPTON HBO 50W mercury (физически установлен) |
| Current state | Лампа скорее всего дохла (30+ лет) |
| Filter cube 1 (DAPI) | BP 436 / FT 460 / LP 470 |
| Filter cube 2 (FITC/GFP) | 450-490 / FT 510 / LP 520 |
| **Upgrade Phase A** | HBO 50W bulb $50-150 ИЛИ LED epi-source $400-500 |

## Лазер (Phase 0)

| Параметр | Значение |
|---|---|
| Wavelength | 450 nm (blue diode) |
| Type | CW (continuous wave) |
| Max optical power | 500 mW |
| Operating range | 10-100 mW (underrun via PWM) |
| Focal spot через 40× | 2-5 µm |
| Phototoxicity radius | >10 µm (CW → limited to plant cells) |
| TTL input | 0-5V PWM, rise time ~100µs |
| Safety class | 3B / 4 on peak |
| **Upgrade Phase A target** | Cobolt Tor 355 nm Q-switched ns ($15-25K) |

## Камера (scientific)

| Параметр | Значение |
|---|---|
| Model | ToupCam E3CMOS05000KMA MONO |
| Sensor | Sony IMX264 CMOS |
| Resolution | 5 MP (2448 × 2048) |
| Pixel size | 3.45 µm |
| Shutter | Global |
| QE peak | 73% @ 550 nm |
| Read noise | 2.4 e⁻ RMS |
| Dark current @ 25°C | 0.3 e⁻/s (no TEC) |
| Bit depth | 12 bit |
| Frame rate | 23 fps at full res |
| Interface | USB 3.0 |

## XY stage (motorized)

| Параметр | Значение |
|---|---|
| Base mechanical | XIMU LGY40-C manual cross-roller XY (куплен) |
| Table size | 40 × 40 мм |
| **Travel** | **±6.5 мм = 13 мм total per axis** (verified per vendor spec 2026-04-24) |
| Load capacity | 19.6 N (2 kg) |
| Parallelism | 0.06 мм |
| Micrometer heads | 2× класса 0-13мм (шаг 0.5 мм/rev, верньер 10 µm/div) |
| **Thimble Ø** | **14.5 мм** (замерено штангенциркулем 2026-04-24) |
| Weight | ~0.26 kg |
| Stepper (Variant A) | NEMA-11 0.9°/step, 400 full steps/оборот, 5mm shaft |
| Driver | A4988 или DRV8825, microstepping ×16 |
| Coupler | **Flex 5mm → 14mm + heatshrink 0.5мм** (эффективно 14.5мм) |
| **Precision open-loop** | 0.5mm / (400×16) = **78 nm/microstep** theoretical |
| **Practical repeatability** | **1-5 µm** (после homing + backlash comp) |
| **Abs accuracy open-loop** | **±10 µm** over 13mm travel |
| Homing | 4× endstop microswitches (X+, X-, Y+, Y-) |

## Control electronics

| Компонент | Model | Role |
|---|---|---|
| MCU main | Arduino Nano R3 | Realtime (PWM, stepper, interlock, sensors) |
| WiFi | NodeMCU ESP8266 | MQTT, alerts |
| Temp sensors | DS18B20 × 3 | LED heatsink, box air, stage |
| Photodiode | BPW34 | Laser dose feedback |
| Door switch | Magnetic reed NC | Interlock (hardware-first) |
| Laser kill | SPDT 12V relay | Hardware kill (физически разрывает +12V) |
| LED/Laser PWM | IRLZ44N MOSFET × 2 | Power switching |

## Power consumption

| Устройство | Мощность |
|---|---|
| LED (underrun 5W) | 5 W |
| Laser (50% PWM avg) | 5-10 W |
| Fan Noctua 120mm | 2 W |
| Arduino + sensors | 0.5 W |
| IR LED strip (overview cam) | 1 W |
| PC idle | 50 W |
| ToupCam USB | 2 W |
| **Total inside box** | ~15 W |
| **Total system** | ~80-100 W |
| UPS SMT1500 capacity | 900 W (derated VA) |
| Runtime @ 100W | 2-3 hours |

## Бокс (enclosure)

| Параметр | Значение |
|---|---|
| Material | ACP 3mm black/black |
| External dims | 600 × 500 × 700 мм |
| Internal volume | ~210 л |
| Frame | Al profile 20×20×2 мм, 12 рёбер |
| Vents | 2× Z-baffle light-trap 80×80×40 мм + Noctua 120 |
| Light tightness target | <0.01 lux leakage снаружи при 100% LED |
| Interlock | Reed switch + SPDT relay + Arduino interrupt |

## Timeline / Data

| Параметр | Значение |
|---|---|
| Assembly | 2-3 недели (Weeks 1-3 покупки, Weeks 4-6 сборка) |
| Commissioning session | 6 месяцев непрерывно |
| Time-lapse interval | 30 мин |
| N positions | до 10 fields |
| Per-image size | 10 MB (5MP mono 16-bit TIFF) |
| Per-day data | ~5 GB |
| Total 6мес | ~900 GB |
| Storage primary | Internal PC SSD 1TB |
| Storage backup | External HDD 4TB (weekly rsync) |

## IF validation reagents (E2 block — после live-cell съёмки)

Детальный BOM на 12 экспериментов — см. **`E2_IF_Validation_Block.md`** в этой же папке (E2-Validation, 2026-05-12).

| Категория | Сумма $ | Примечание |
|---|---|---|
| Первичные антитела (Centrin-1 20H5, CEP164, Centrobin, CP110, γ-Tubulin) | 1,860 | 5 аликвот, ≥100 IF total |
| Вторичные антитела (Alexa 488 / 555 / 647) | 770 | 3 флуорофора |
| Фиксация + пермеабилизация (PFA, MeOH, Triton, BSA, MgCl₂, DAPI) | 240 | — |
| Расходники (cover slips, slides, mounting medium) | 114 | — |
| Контроли (RPE1 опционально) | 0–400 | если требуется positive control |
| Транспортировка/таможня 10 % | 300 | — |
| **ИТОГО IF блок** | **$3,284–3,684** | отдельная статья сверх живой съёмки |

Этот блок **дополняет** бюджет E0 commissioning ($881–1,687 hardware): IF validation выполняется на фиксированных клетках после съёмки и проверяет mother/daughter centriole asymmetry антителами CEP164 (M) + Centrobin (D). Используется в Phase A ARGUS-LP grant как verification step per imaging block.

## Бюджет (Phase A, полная спецификация)

| Category | Item | Cost |
|----------|------|:----:|
| **Optics** | TetraSpeck beads (T7279) | $280 |
| | FluoSpheres 488/520 | $250 |
| | Semrock dichroic FF497-Di01 | $395 |
| | Emission bandpass FF01-525/45 | $295 |
| **Laser** | 450 nm 500 mW CW TTL module | $60 |
| | Laser safety goggles OD4+ | $25 |
| **Camera** | ToupCam E3CMOS05000KMA mono (IMX264) | $394 |
| | Custom C-mount adapter | $56 |
| **Stage** | LGY40-C (already owned) | $50 |
| | NEMA-11 stepper + TMC2209 driver | $35 |
| | MKS SERVO42 closed-loop | $85 |
| | Flex couplers + endstops | $15 |
| **Control** | ESP32-S3 DevKit | $12 |
| | Sensors (DS18B20, BPW34, IRLZ44N, reed) | $18 |
| **Infrastructure** | ACP 3mm black + Al profile | $100 |
| | Noctua 120mm fan | $25 |
| | UPS SMT1500 (renewed) | $299 |
| | External HDD 4TB | $80 |
| **AI compute** | DeepSeek API (6mo ~500 calls/day × $0.0005) | $450 |
| | Gemini 2.5 Flash API (vision ~100 calls/day) | $180 |
| **Contingency** | 15% | $420 |
| **Total** | | **$3,524** |

Note: Previous estimate ($881-1,687) reflected only core hardware without optics (dichroic, bandpass), stage closed-loop upgrade, and AI compute. The $3,524 total is the full Phase A cost.

## v4.0 Statistical Protocol (2026-05-15, updated per TBPR)

### Primary endpoint
- **XY stage repeatability:** ±5 µm (n=50, α=0.05, β=0.2, effect size 1.5σ)
- **Pre-registration:** OSF (DOI assigned prior Phase A start)

### Secondary endpoint (new — old/new discrimination)
- **Discrimination between 'old' (photobleached) and 'new' (fresh) centriole mimetic beads**
- Target: ≥95% correct classification on ≥500 simulated ablation cycles
- Power calculation: n=500 cycles, α=0.05 (Holm-Bonferroni corrected), power 0.9 for detecting 5% deviation from 95%
- Justification: Royall 2023 [PMID: 37882444] — asymmetric inheritance of older mother centriole

### Tertiary endpoint (new — CUSUM)
- **CUSUM (Cumulative Sum) control chart** for monitoring AI performance drift
- If accuracy falls below 93% on sliding window of 50 cycles → automatic stop test
- **Acceptance:** 0 crossings of lower control limit over 6 months

### Multiple comparisons
- Number of comparisons: 8 (2 endpoints × 4 timepoints)
- Correction: Holm-Bonferroni (family-wise error rate ≤0.05)

### Blinding
- Operator blinded to experimental condition
- AI agent receives no metadata about condition
- 10% of cycles reviewed by second AI model (cross-check)

## Acceptance criteria (Phase A → graduation)

| # | Criterion | Metric | Method |
|:-:|-----------|--------|--------|
| 1 | SNR | ≥ 5× background on Kaede-beads | Photodiode measurement |
| 2 | XY repeatability | ±5 µm (n=50) | Holm-Bonferroni corrected |
| 3 | Discrimination accuracy | ≥95% old vs new beads (≥500 cycles) | Power 0.9 |
| 4 | CUSUM | 0 crossings of 93% boundary over 6 months | Sliding window 50 cycles |
| 5 | Physical Beacon | 10+ stress tests (beacon hidden → laser blocked) | Hardware test |
| 6 | Safety interlock FMEA | RPN < 100 for all failure modes | FMEA table (§5 DESIGN) |
| 7 | Uptime | ≥99% over 6 months | Auto-logged |

## Risk Matrix (Phase A)

| Risk | P | I | RPN | Mitigation |
|------|:-:|:-:|:---:|------------|
| ESP32-S3 firmware crash during 6-month run | 3 | 4 | 12 | Watchdog 500 ms + systemd auto-restart |
| AI hallucinates target on empty slide | 2 | 5 | 10 | Physical Beacon hardware block (CONCEPT §11) |
| DeepSeek API outage | 3 | 3 | 9 | Fallback: Llama 3.2 90B via Groq (~3s switch) |
| Stage drift >±5 µm over 6 months | 2 | 4 | 8 | Weekly cal + MKS SERVO42 closed-loop feedback |
| CUSUM false alarm (drift below 93%) | 2 | 3 | 6 | Adjustable LCL; human operator review |
| Camera failure | 1 | 4 | 4 | Redundant Pi HQ camera path |
| Physical Beacon LED burnout | 1 | 3 | 3 | Redundant LED; test at WP4 commissioning |
| Power outage > UPS runtime | 1 | 4 | 4 | Ordered shutdown; auto-resume on power return |

**P** = probability (1-5), **I** = impact (1-5), **RPN** = P × I. All RPNs < 15.

## v4.1 changes (2026-05-15)

| Change | Reason |
|--------|--------|
| Budget table: from $881-1,687 min/max → full line-item $3,524 | Top TBPR concern: no budget table |
| Risk matrix added (8 failure modes, RPN < 15) | Missing from v4.0 |
| Work Package structure added (CONCEPT §15) | Top TBPR concern: no project plan |

## v3.1→v4.0 changes

| Parameter | v3.1 | v4.0 | Reason |
|-----------|:----:|:----:|--------|
| Secondary endpoint | AI accuracy ≥95% | Old/new bead discrimination ≥95% | Biological justification (Royall 2023) |
| Tertiary endpoint | — | CUSUM control chart | Quality control against AI drift |
| Acceptance criteria | 5 | 7 | Physical Beacon + CUSUM + FMEA added |
| Safety | Software interlock | Physical Beacon hardware block | Hardware-level AI hallucination protection |
