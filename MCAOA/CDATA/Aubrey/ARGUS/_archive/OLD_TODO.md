# TODO.md — Experiment 0

## P0 — Pre-commissioning (до покупки compoments)

- [ ] **Измерить prefocus base** halogen socket (штангенциркуль — диаметр, pin spacing)
- [ ] **Сфотографировать photo port открытый** (после снятия M35W film camera) — для подбора C-mount adapter
- [ ] **Визуальная инвентаризация объективов** на турели (гравировка на каждом)
- [ ] **Замерить габариты** микроскопа (для ENCLOSURE.md — если >600×500×700, увеличить бокс)

## P1 — Закупки (Weeks 1-3)

- [ ] AliExpress Week 1 ($145): LED retrofit (Cree XHP50 + Meanwell LDD-700H), laser 450nm, safety goggles OD4+, Arduino starter kit, sensors (DS18B20, BPW34, IRLZ44N, reed switch, relay)
- [ ] AliExpress Week 2 ($130): RPi Camera Module 3 Wide NoIR + Pi Zero 2W, USB endoscope, Noctua 120mm fan, cable glands, EPDM seal
- [ ] AliExpress Week 3 ($395-610): ToupCam E3CMOS05000KMA MONO + Zeiss → C-mount adapter (+0.5× reducer опционально)
- [ ] AliExpress LGY40-C motorization ($50): 2× NEMA-8/11 steppers, A4988 drivers, flex couplers, endstops, mounting bracket
- [ ] Локально Тбилиси ($150): ACP 3mm, Al profile, матовая чёрная краска, Elodea canadensis, предметные/покровные стёкла, External HDD 4TB, виброподкладки
- [ ] eBay / Amazon: APC Smart-UPS SMT1500 (Renewed ~$299 ИЛИ Tbilisi tapio.ge $150)

## P2 — Сборка (Weeks 4-6)

- [ ] **Enclosure**: раскрой ACP + каркас Al profile + монтаж ACP + силикон + покраска внутри матовой чёрной (2 слоя)
- [ ] **Дверь + interlock**: петли, reed switch, магнитный замок, EPDM уплотнитель
- [ ] **Вентиляция**: Z-baffle light-trap × 2 + Noctua fan в потолке + cable glands
- [ ] **LED retrofit**: сборка Cree XHP50 + heatsink + driver + collimator + custom mount adapter
- [ ] **Монтаж микроскопа** в бокс + виброподкладки под ножки
- [ ] **LGY40-C motorization**: 3D-print bracket + установка steppers + flex couplers + endstops
- [ ] **Köhler alignment** после LED retrofit
- [ ] **Arduino sketch**: firmware (stepper control, PWM, interlock, sensors, serial protocol JSON-lines)

## P3 — Камера + software

- [ ] Снять M35W film camera → установить Zeiss→C-mount adapter → ToupCam
- [ ] Установить Micro-Manager 2.0 + ToupCam adapter (github.com/toupcam/toupcam-mm-plugin)
- [ ] Первое сфокусированное изображение Elodea chloroplast через 40× (test TIFF 16-bit)

## P4 — Claude agent integration

- [ ] Python driver layer: tool functions `move_stage`, `fire_laser`, `capture_image`, `detect_targets`, `check_interlock`, `get_sensor_reading`
- [ ] Arduino serial protocol JSON-lines через USB 115200 baud
- [ ] Claude Code /overnight agent скрипт: цикл detect → decide → act → log, с error recovery
- [ ] Systemd unit для auto-restart Claude Code при crash
- [ ] Test безопасности: interlock trip → physical laser kill → recovery

## P5 — Commissioning 6-мес сессии

- [ ] Dose matrix calibration (PWM 10-70%, duration 100-500ms, 7 сессий × N=10 chloroplasts)
- [ ] Sham controls (untreated / empty-location / mechanical / laser test — 4 arms)
- [ ] Single ablation run testing (1 target / session)
- [ ] 6-мес time-lapse в MDA config (30-min interval, 10 positions)
- [ ] Weekly rsync backup на external HDD

## P6 — Documentation / transition to Experiment A

- [ ] Preprint на bioRxiv о commissioning methodology (если rig + agent работают стабильно)
- [ ] После успешного commissioning — спланировать Experiment A с iPSC-organoids + Centriolin-RITE (Royall 2023) ИЛИ Drosophila GSC (Yamashita 2003)
- [ ] LGY40-C upgrade до Варианта B (linear stepper actuator) если Вариант A precision недостаточна

## Pending peer-review issues (см. PEER_REVIEW_DRAFT.md)

- [ ] Явно переформулировать что это commissioning, НЕ биологический пилот (для Impetus)
- [ ] Рассмотреть переход на 355 nm Q-switched ns laser для Experiment A
- [ ] Pre-registration protocol (OSF) перед запуском 6-мес сессии
- [ ] Pre-calculate required N для statistical power

## ✅ v3.1→v4.0 выполнено (2026-05-15)

- [x] **CONCEPT.md v4.0** — полный рерайт: MCAOA disclaimer, Physical Beacon, AI Constitution, CUSUM, FMEA
- [x] **§0 MCAOA Disclaimer** — добавлен Zenodo DOI: 10.5281/zenodo.20055806
- [x] **§2 PMIDs расширены** — с 6 до 14, добавлены La Terra 2005 [15738265], Uetake 2007 [17227892] и др.
- [x] **§4 PI publications** — удалены (кроме PMID 36583780), per reviewer requirement
- [x] **§5 CUSUM control chart** — tertiary endpoint
- [x] **§5 Old/new bead discrimination** — secondary endpoint (обоснование Royall 2023)
- [x] **§9 Honest comparison** — нейтральная нотация, удалён straw man
- [x] **§11 Physical Beacon** — hardware AI hallucination protection
- [x] **§12 AI Constitution** — 5 prohibited actions
- [x] **§13 Reviewer block resolutions** — компактный список
- [x] **§15 Acceptance criteria** — расширены с 5 до 7
- [x] **§16 Final Response to Peer Review** — трекинг всех комментариев
- [x] **DESIGN.md v2.0** — Physical Beacon, AI Constitution, CUSUM, FMEA, обновлённая архитектура
- [x] **PARAMETERS.md v2.0** — обновлён стат. протокол, 7 acceptance criteria

## ✅ v4.1 фиксы (2026-05-15, post TBPR cycle-9)

- [x] **Budget table** — добавлена в CONCEPT §15 (line-item $3,524)
- [x] **Work Package structure** — 8 WP с месяцами, дедлайнами, зависимостями (CONCEPT §16)
- [x] **Risk Matrix** — 8 failure modes с RPN (CONCEPT §17)
- [x] **§0 MCAOA Disclaimer** — сокращён до одного параграфа
- [x] **§4 PI publications** — убран accusatory / DELETED AS REQUIRED тон
- [x] **§8 Team** — Liz Parrish role → Advisor & Co-PI
- [x] **Embedded rebuttal** — удалён

## ✅ v4.2 фиксы (2026-05-15, post TBPR cycle-10)

- [x] **§0 MCAOA Disclaimer** — ещё короче, убрана defensive blockquote
- [x] **§4 PI publications** — нейтральный тон "scope of self-citation"
- [x] **§8 Team** — Liz Parrish как Advisor & Co-PI
- [x] **§10 Execution credibility** — новый раздел: почему theorist может построить hardware
- [x] **README.md** — синхронизирован с v4.2
- [x] **Нумерация секций** — исправлена

## ✅ v4.3 фиксы (2026-05-15, post TBPR cycle-11)

- [x] **§8 Team** — PI-only (Jaba Tqemaladze). Убрана Parrish из команды ARGUS
- [x] **§9 Grant context** — новая секция: Parrish + Geiger как external commitments для Phase B (Aubrey), не для ARGUS
- [x] **TBPR concern resolved** — «Co-PI mismatch»: в ARGUS больше нет внешних Co-PI
- [x] **README.md** — синхронизирован с v4.3
- [x] **Нумерация секций** — 0→19

## Pending (v4.1 phase)

### 🔬 Minimal preliminary data (для PARANOID reviewer)

- [ ] **1. XY stage repeatability** — измерить на любом доступном микроскопе (или линейкой/микрометром). 10-50 измерений. Показать ±5 µm. $0, ~1 час.
- [ ] **2. AI agent simulation** — запустить Claude Code /overnight на 50-100 циклах детекции цели на синтетических изображениях (фото beads). Залогировать accuracy.
- [ ] **3. SNR measurement** — сфотографировать fluorescent beads на любом доступном микроскопе. Измерить signal/noise. Показать ≥5× достижимо.

### Infrastructure

- [ ] **OSF pre-registration submission** (before Phase A start)
- [ ] Photodiode SNR ≥ 5× baseline acceptance test
- [ ] XY stage repeatability n=50 measurements (formal)
- [ ] AI agent ≥95% accuracy on ≥500 simulated ablation cycles
- [ ] 6-month continuous operation log + uptime metric
- [ ] CUSUM control chart implementation (Python monitoring loop)
- [ ] Physical Beacon circuit assembly + firmware integration
- [ ] AI Constitution enforcement (static analysis hooks)
- [ ] FMEA review + RPN tracking (update after Phase A data)
- [ ] Phase B planning: Q-switched 355 nm laser + UV objective procurement spec
- [ ] Aubrey biology phase transition criteria document
