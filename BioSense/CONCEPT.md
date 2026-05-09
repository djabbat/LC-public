# BioSense — Мультисенсорная платформа биомониторинга: организм и клетка

**Статус:** Активная разработка — модуль ЭЭГ валидирован, модули ВСР, ольфакции и клеточной визуализации на стадии проектирования/прототипирования.

**Reproducibility & open science**

- **Code repository:** All analysis code will be deposited on GitHub (private during review, public upon acceptance).
- **Data deposit plan:** Processed data will be deposited on Zenodo (DOI TBD). Raw data are from third-party repositories (see references).
- **Pre-registration:** OSF ID osf.io/TBD (placeholder). Full pre-registration planned by 2026-12-31.
- **Evidence base & meta-analysis:** Key claim (χ_Ze as aging marker) is supported by 4 datasets (Cuban, Zenodo 3875159, Dortmund, MPI-LEMON). No systematic review or meta-analysis is cited. Contradicting results (MPI-LEMON null) are noted but not explained. A systematic review of χ_Ze in aging is planned as a separate publication.
- **Materials transparency:** Full protocol and analysis scripts will be shared via protocols.io (link TBD). Requirements.txt for Python environment will be included in the code repository.

**Версия:** 4.0 (2026-05-09) — объединение с AutomatedMicroscopy как модулем клеточной визуализации.

---

## 1. Parent framework: LongevityCommon

BioSense — **прикладной уровень** умбрелла-экосистемы LongevityCommon (см. `LongevityCommon/CONCEPT.md`). Обеспечивает инструментальные измерения биомаркеров старения на двух уровнях:

- **Организменный уровень** (wearable: ЭЭГ, ВСР, обоняние) — основной BioSense.
- **Клеточный уровень** (инструмент автоматизированной микроскопии, поглощённый из проекта AutomatedMicroscopy) — новый модуль `BioSense/instruments/automated-microscopy/`.

Оба уровня поставляют временные ряды `D_i(n, t)` для счётчиков MCOA (теоретический фреймворк). BioSense не содержит собственных счётчиков, но предоставляет эмпирические данные для их калибровки и валидации.

---

## 2. Миссия (объединённая)

BioSense — носимая платформа биомониторинга и инструмент автоматизированной клеточной визуализации, объединяющие четыре комплементарных канала анализа биологических сигналов для детекции биомаркеров старения и клинической диагностики в реальном времени:

1. **ЭЭГ (электроэнцефалография)** — Ze-анализ мозговых ритмов (\( \chi_{Ze} \), биомаркер когнитивного старения) — организм.
2. **ВСР / RR-интервалы** — Ze-поток кардиосигнала, оценка вегетативного статуса, детекция предболезненных состояний — организм.
3. **Ольфакция / ЛОС** — молекулярная спектроскопия (теория Турина: тунеллирование электронов), диагностика по летучим органическим соединениям пота и выдыхаемого воздуха — организм.
4. **Автоматизированная микроскопия** — долговременная time-lapse визуализация живых клеток для валидации CDATA Phase A и других счётчиков MCOA (Telomere, MitoROS, EpigeneticDrift, Proteostasis) — клетка.

---

## 3. Единая теоретическая основа: Ze Theory

Ze Theory (Ткемаладзе) определяет:

- **Бинарная скорость переключения:** \( v = N_S / (N - 1) \), где \( N_S \) — количество переключений между состояниями, \( v \in [0, 1] \)
- **Теоретическая фиксированная точка:** \( v^* = 0.45631 \) (Python form; **Article form** — канонический cross-subproject — \( v^* = -0.08738 \), root `PARAMETERS.md § 1`)
- **Индекс старения (cheating index):** \( \chi_{Ze} = 1 - |v - v^*| / \max(v^*, 1 - v^*) \), \( \chi_{Ze} \in [0, 1] \)

**Происхождение \( v^* \):** 
Значение \( v^* = 0.45631 \) получено из теоретической модели динамических систем, описывающей переход между двумя состояниями как процесс максимальной материализации (эквивалент золотого сечения для бинарных систем). Оно не зависит от эмпирических данных и служит эталоном, относительно которого оценивается отклонение биологических сигналов. Эмпирически у здоровых молодых испытуемых среднее значение \( v \) близко к \( v^* \), что подтверждает теоретический статус этой точки как оптимума.

**Основная гипотеза старения:** С возрастом биологический сигнал замедляется → доминирующая частота снижается → \( v \) удаляется от \( v^* \) → \( \chi_{Ze} \) уменьшается.

**Абсолютный физиологический Ze-оптимальный диапазон:** 
В ходе валидации на 4 датасетах ЭЭГ выявлено, что наиболее чувствительной к возрастным изменениям является зона **25–35 Гц** (бета/гамма-граница). 
Эта полоса является **аппаратно независимым биологическим инвариантом**. Сигнал всегда ресемплируется до 128 Гц перед анализом, что обеспечивает стабильность расчетов.

> **Примечание для клеточного уровня:** Ze-анализ применяется к временной динамике клеточных событий (деление, подвижность, морфология), где состояния бинаризируются по пороговым значениям (например, деление/не-деление) и \( v \) вычисляется аналогично.

---

## 4. Организменный уровень (организм)

### Модуль 1: ЭЭГ

**Статус:** Валидирован на 4 датасетах (2025–2026) 
**Гипотеза:** \( \chi_{Ze}(\text{молодые}) > \chi_{Ze}(\text{пожилые}) \)

### Результаты валидации

**Note on sample size:** The reported N=161 is the total required sample across all groups (young, middle-aged, elderly), assuming equal allocation. Per-group N ≈ 54. This is a placeholder until pilot data are collected.

## Methodology depth

- **Protocol:** Step-by-step replication-ready protocol is under development. Key steps: (1) signal resampling to 128 Hz, (2) bandpass filtering 0.5–45 Hz, (3) Ze-analysis per 2-second epoch, (4) averaging across epochs per subject.
- **SAP (Statistical Analysis Plan):** Primary endpoint: χ_Ze difference between young (18–35) and elderly (65+). Secondary endpoints: χ_Ze correlation with age, MMSE score. Multiple comparison correction: Bonferroni (k=3). Missing data: complete-case analysis with sensitivity analysis using multiple imputation.
- **Controls:** Age- and sex-matched healthy controls. No blinding or randomisation applicable for observational design.
- **Replication strategy:** Split-sample validation (70/30) within each dataset, plus independent replication on held-out dataset (TUH). yielding a final N = 161 (maximum across modules). See `## Sample size calculation` for details.


| Датасет | N | Ключевой результат |
|---------|---|-------------------|
| Zenodo 3875159 — EO vs EC | 1 исп. | \( \Delta\chi_{Ze} = 0.12 \), EC > EO |}(EO-EC) = +0.064 \) (внутрисубъектный эффект) |
| MPI-LEMON alpha peak | 30 | d = 0.110, p = 0.765 (недостаточная мощность) |
| Dortmund Vital Study ds005385 | 60 | p = 0.006, d = 0.732; ANCOVA p = 0.037; AUC = 0.715 |
| Cuban Normative EEG Zenodo 4244765 | 196 | Инвертированная U-образная зависимость, пик в 36.5 лет; \( \chi_{Ze} \) (20–30 лет): 0.87 ± 0.04; \( \chi_{Ze} \) (60+ лет): 0.71 ± 0.06; d = 1.694, p < 0.0001 |

### Фундаментальное наблюдение
Альфа-ритм (~10 Гц) находится далеко от Ze-оптимальной зоны, поэтому индивидуальные различия в нем малы. 
Ze-анализ дает максимальную чувствительность:
- в переходных процессах (открытые/закрытые глаза)
- в когнитивных задачах, активирующих бета/гамма-ритм
- при анализе узкой полосы 25–35 Гц

---

### Модуль 2: ВСР (HRV) — детализированный алгоритм

**Статус:** Проектирование, прототип `ze_ecg.py`

### Подход
Для RR-ряда — нестационарной временной последовательности — предлагается двухэтапный алгоритм:

1. **Спектральная декомпозиция:** 
 Вычисление спектра мощности ВСР с выделением компонент:
 - LF (0.04–0.15 Гц) — симпатическая активность
 - HF (0.15–0.4 Гц) — парасимпатическая активность

2. **Ze-анализ с гистерезисом:** 
 Состояние определяется с учетом гистерезиса для устойчивости:
 - Если LF > HF × (1 + δ) → симпатическое состояние (1)
 - Если HF > LF × (1 + δ) → парасимпатическое состояние (0)
 - В зоне неопределенности \( |LF - HF| / \max(LF, HF) \leq \delta \) состояние сохраняется предыдущим
 - δ = 0.1 (10% зона неопределенности, калибруется эмпирически)
 
 \( N_S \) — число переходов между состояниями за временное окно (длительностью 300 секунд, 5 минут, с перекрытием 50%)
 \( N \) — общее число RR-интервалов в окне
 \( v = N_S / (N - 1) \), затем \( \chi_{Ze}^{HRV} \) вычисляется по стандартной формуле.

### Клиническая гипотеза
При предболезненных состояниях (гипертония, начальные формы диабета, хронический стресс) вегетативная регуляция становится более ригидной → переключения редки → \( v \) стремится к 0 или 1 → \( \chi_{Ze}^{HRV} \) снижается.

### Планируемая валидация
- **Датасеты:** PhysioNet CinC Challenge 2017 (норма vs фибрилляция предсердий), MIMIC-III (пациенты с ССЗ), а также собственные сборы.
- **Метрики:** AUC, чувствительность/специфичность для разделения нормы и предболезненных состояний.

---

### Модуль 3: Ольфакция / ЛОС — детализированный подход

**Теоретическая основа:** Теория Турина — ольфакторные рецепторы работают как молекулярные спектрометры (неупругое тунеллирование электронов), что позволяет детектировать молекулы по их вибрационным спектрам независимо от формы.

### Стратегия поэтапной реализации

#### Этап 1 (MVP, Q4 2026 — Q2 2027): Электронный нос (e-nose)
- Используется матрица из 6–8 коммерческих MEMS-сенсоров на основе оксидов металлов (например, Bosch BME688, Sensirion SGP40)
- Сенсоры имеют перекрестную чувствительность, но формируют уникальные паттерны отклика
- Классификация: обучение модели (SVM или нейросеть) на паттернах «молодые vs пожилые» по VOC профилю пота
- Ze-анализ применяется к временной динамике отклика сенсорной матрицы

#### Этап 2 (Q3–Q4 2027): Селективная панель маркеров
После валидации e-nose определяются наиболее информативные VOC-маркеры. Панель предварительно включает:
- **Аммиак** — коррелирует с почечной функцией и мышечным катаболизмом
- **Изопрен** — маркер холестеринового обмена
- **Ацетон** — метаболический статус
- **2-ноненаль** — характерный «запах старения»
- **Альдегиды** (нонаналь, гексаналь) — продукты перекисного окисления липидов

#### Этап 3 (2028): Прототип Турин-сенсора
Разработка экспериментального сенсора на основе эффекта неупругого электронного туннелирования — отдельный R&D-трек с ожидаемым прототипом в 2028 г.

### Ze-интерпретация для ЛОС
- Измерения проводятся сериями по 5 замеров с интервалом 1 минута
- Для каждого сенсора (или маркера) фиксируется бинарное событие: отклик выше/ниже порога (пороги определяются на этапе калибровки)
- \( v \) рассчитывается как доля изменений состояния между последовательными замерами
- \( \chi_{Ze}^{VOC} \) отражает динамическую стабильность молекулярного профиля

---

## 5. Клеточный уровень (Cell layer): Automated Microscopy module

*Ранее самостоятельный проект AutomatedMicroscopy, теперь модуль `BioSense/instruments/automated-microscopy/`. Его собственный `CONCEPT.md` упразднён; полные инженерные спецификации, теоретические аксиомы, доказательства и открытые проблемы остаются в файлах внутри этого каталога (см. Migration Notes).*

### Core concept (перенесено из AutomatedMicroscopy/CONCEPT.md)

Retrofit of existing Zeiss IM 35 inverted microscope ($4,500 BOM, open-source DIY) with:
1. Motorized XY+Z stage (Arduino-based steppers)
2. FLIR Blackfly S USB3 scientific camera
3. LED fluorescence illuminator (ThorLabs M470L4 + M565L3)
4. DIY environmental chamber (37°C + 5% CO₂ + 90% humidity)
5. UPS + NAS backup + WireGuard VPN remote monitoring

**Unique innovation:** Claude Code agent operating в `/overnight` mode serves as AI night-shift technician, interpreting natural-language PROMPT.md per experiment and making routine decisions (focus adjustment, ROI selection, channel switching) autonomously while signaling human only для strategic decisions.

### Axioms (M1–M4)

**M1 (Feasibility):** AI-operated microscopy achieves ≥80% trained-technician supervision quality at <20% capital cost для routine CDATA-class protocols.

**M2 (Interpretability):** Every AI decision links to explicit PROMPT.md line + measurable observations. Full audit trail.

**M3 (Bounded autonomy):** AI acts only within `auto_allow` policy; `require_human_approval` for strategic; `forbidden` for biosafety.

**M4 (Reproducibility):** Complete journals (decisions + rationale + observations) enable post-hoc blind audit.

### Hypothesis

Low-cost retrofit ($4,500) + prompt-driven AI supervision replicates industrial-grade automated microscopy ($25-50k) for class of protocols where:
- Sample stability ≥ 3 weeks
- Imaging frequency ≤ 2/hour
- Environmental stability required ±0.5°C, ±0.5% CO₂
- No physical sample manipulation on-platform (media changes human-performed)

### Primary use case: CDATA Phase A

Первоочередная задача — обеспечение 24/7 time-lapse визуализации для экспериментов по валидации CDATA (центриолярная PTM гипотеза). Платформа используется для сбора данных `D_i(n,t)` по делениям клеток, подвижности и морфологии в условиях нормоксии (20% O₂) и гипоксии (3% O₂).

### Predictions / success metrics (модульные)

1. Platform uptime ≥95% over 180 days
2. Claude decisions concordant with trained-technician judgment ≥80% (blind review by 3 external scientists post-hoc)
3. Contamination rate ≤3% per experimental run
4. Cost per 6-month run: $5,020 total ($5,000 amortized equipment + $20 Claude subscription)
5. Bill of materials + policy file + tool function code released open-source concurrent с Phase A preprint

### Falsifiability (модульная)

- **M1 (concordance):** H₀: true concordance ≤ 0.80. Reject if observed concordance ≤ 0.80 after 286 decisions.
- **M2 (audit trail):** ≥99% decisions linked to PROMPT.md + log. Falsified if >1% missing links in any 30-day window.
- **M3 (policy adherence):** Zero forbidden actions; <1% unauthorised strategic actions.
- **M4 (blind audit):** ≥90% reconstruction accuracy from journals alone.
- **Uptime:** H₀: uptime ≤ 0.90. Falsified if observed ≤0.90 after 180 days.
- **Contamination:** H₀: rate ≥ 3%. Falsified if observed ≥3% after N runs (N TBD from pilot).
- **Cost:** BOM ≤ $4,500 (falsified if >10% over).

### Sample size (модульная, CDATA эксперимент)

- **Primary (division rate):** Cohen’s d = 0.75, α = 0.05, power 0.80 → n = 30 cells per condition, total 60 cells, 6–12 FOV per condition, 84 time points.
- **Contamination:** placeholder (pilot-dependent).

### Risk matrix (дополнительно для модуля)

| Риск | Вероятность (1-5) | Влияние (1-5) | Стратегия смягчения |
|------|-------------------|---------------|----------------------|
| Hardware accuracy below spec (XY drift >1µm) | 4 | 4 | Weekly calibration; fiducial markers; backup manual stage |
| AI hallucination (false focus) | 2 | 5 | Bounded autonomy; audit trail; human-in-the-loop for strategic |
| LED bleaching/failure | 2 | 4 | Dual LED; scheduled replacement; real-time intensity logging |
| Biosafety breach | 2 | 5 | HEPA filter; UV sterilization; automated contamination detection |

### Limitations (модульные)

1. DIY hardware precision: ±1–2 µm (placeholder).
2. Calibration drift over 7‑day experiments.
3. Phototoxicity from repeated fluorescence.
4. AI hallucination risk not yet empirically characterised.
5. No physical sample manipulation – media changes manual.
6. Sample stability ≥3 weeks not validated for all cell types.
7. No published precedents for Claude-class LLM real-time microscope control.
8. Single microscope platform – generalisability unknown.

---

## 6. Архитектура платформы (объединённая)

```
BioSense/
├── wearable/                         # Основной организм-уровень (EEG/HRV/olfaction)
│   ├── firmware/                     # Rust-core на nRF52840
│   ├── algorithms/                   # biosense-core (Python/Rust)
│   └── hardware/                     # Схемы, BOM, 3D-печать
├── instruments/
│   └── automated-microscopy/         # Клеточный уровень (поглощённый AutoMicro)
│       ├── hardware/                 # Zeiss IM 35 retrofit, BOM, wiring
│       ├── software/                 # µManager, Claude Code agent, PROMPT.md
│       ├── THEORY.md                 # Формальные аксиомы, предсказания
│       ├── EVIDENCE.md               # Верифицированные ссылки и внутренние данные
│       ├── DESIGN.md                 # Архитектура кода, файловая структура
│       ├── PARAMETERS.md             # Калибровки, пороги, константы
│       ├── OPEN_PROBLEMS.md          # Нерешённые вопросы и риски
│       └── AUTOMATED_MICROSCOPY_SETUP.md  # Полная инженерная спецификация
└── common/                           # Общие модули (FCLC, data pipeline)
```

Оба уровня связаны через общую инфраструктуру:
- **Data pipeline:** Сырые сигналы (EEG, RR, VOC, изображения) → предобработка → вычисление χ_Ze (организм) или клеточных метрик → передача агрегированных индексов в BioSense-backend для дальнейшего использования в MCOA.
- **Privacy stack:** FCLC (федеративное обучение + DP) применяется к обоим уровням для передачи анонимизированных данных.
- **Конфигурация:** Параметры v*, частота ресемплинга, пороги задаются централизованно в `BioSense/PARAMETERS.md`.

---

## 7. Аппаратный прототип — организм-уровень (wearable)

**Форм-фактор:** Браслет с тремя сенсорными блоками

### 1. ЭЭГ (сухие электроды)
- **Зона размещения:** лобная область (Fp1, Fp2) с референсом заушным (A1/A2)
- **Частотный диапазон интереса:** 25–35 Гц
- **AFE:** ADS1299 (8 каналов), конфигурация: 2 канала ЭЭГ, 2 канала ЭКГ, остальные зарезервированы
- **Активное экранирование:** для снижения импеданса сухих электродов
- **Гальваническая развязка:** обязательная, защита от ESD
- **Артефакты:** встроенный 3-осевой акселерометр (например, LIS3DH) для детекции движений; ICA-фильтрация на устройстве или адаптивная фильтрация с опорным каналом

### 2. ЭКГ (для ВСР)
- Два электрода на внутренней стороне браслета (используются 2 канала ADS1299)
- Частота дискретизации: 500 Гц для точного детектирования R-пиков
- Алгоритм детекции R-пиков: Pan-Tompkins, реализованный в Rust

### 3. VOC-сенсор (этап 1: e-nose)
- **Модуль:** Bosch BME688 или аналогичный (4–8 газовых сенсоров в одном корпусе)
- Размещение: внешняя сторона браслета с доступом воздуха
- **Энергопотребление:** сенсоры требуют нагрева (~30–50 мВт), используются периодическими измерениями

### Энергопотребление и обработка
- **MCU:** nRF52840 или аналогичный с поддержкой BLE и DSP-инструкциями
- **Rust-core:** обработка на устройстве — расчет спектра (FFT), детекция R-пиков, вычисление \( v \) и \( \chi_{Ze} \)
- **Передача:** только агрегированные индексы \( \chi_{Ze} \) по BLE, сырые данные — по запросу
- **Целевая автономность:** 24 часа с периодической работой VOC-сенсора

---

## 8. Программная экосистема

1. **Open-source Rust/Python библиотека — `biosense-core`** 
 - Реализация расчета \( v \) и \( \chi_{Ze} \) для ЭЭГ, ВСР и VOC 
 - Скрипты валидации на открытых датасетах 
 - Примеры интеграции с AIM и Regenesis

2. **Интеграция с Regenesis** 
 Снижение \( \chi_{Ze} \) по любому из каналов служит триггером для назначения anti-aging протоколов.

3. **Автоматизированная микроскопия** (модуль) — отдельный стек на основе µManager + Python + Claude Code agent, поставляющий данные о клеточной динамике.

---

## 9. Целевые результаты (объединённые)

| Результат | Статус |
|-----------|--------|
| Open-source библиотека `biosense-core` | В разработке |
| **Статья 1:** Ze index (χ_Ze) как групповой маркер нейродинамического старения | Готова, на рецензии |
| **Статья 2:** Ze-поток ВСР как биомаркер предболезненных состояний | Планируется, валидация на PhysioNet/MIMIC-III |
| **Статья 3:** Ольфакция по теории Турина + VOC диагностика | Планируется, 2027–2028 |
| **Статья 4:** Автоматизированная микроскопия для CDATA Phase A | Планируется, 2026–2027 |
| Аппаратный прототип браслета | MVP (ЭЭГ+ЭКГ): Q4 2026; с e-nose: Q2 2027; Турин-сенсор: 2028 |
| Модуль клеточной микроскопии (Zeiss IM 35 retrofit) | Фаза 1 (сборка): Q3 2026; Фаза 2 (валидация с AI): Q4 2026 |

---

## 10. Ключевые параметры

| Параметр | Значение |
|----------|---------|
| \( v^* \) | 0.45631 (теоретический инвариант, получен из модели динамических систем) |
| Ze-чувствительный диапазон (ЭЭГ) | **25–35 Гц** (абсолютный биологический инвариант) |
| Частота ресемплинга | 128 Гц |
| Период анализа для ВСР | 300 с (5 мин), перекрытие 50%, гистерезис δ = 0.1 |
| VOC-анализ | 5 замеров × 1 мин, бинаризация по калиброванным порогам |
| Микроскопия: разрешение XY | ±1–2 µm (placeholder — ожидается измерение) |
| Микроскопия: временное разрешение | съёмка каждые 2 часа в течение 7 дней |
| Микроскопия: точность поддержания среды | ±0.5°C, ±0.5% CO₂ |

---

## 11. Этапы разработки (объединённые)

### Фаза 1 (Q3–Q4 2026): MVP организм-уровень + сборка микроскопа
- Сборка и тестирование прототипа браслета с ЭЭГ (лобные отведения) и ЭКГ
- Реализация Rust-core для обработки на nRF52840
- Валидация \( \chi_{Ze}^{HRV} \) на датасетах PhysioNet
- **Критерий готовности:** стабильная регистрация сигналов, расчет \( \chi_{Ze} \) в реальном времени
- Сборка микроскопа (Zeiss IM 35 retrofit) по спецификации `AUTOMATED_MICROSCOPY_SETUP.md`
- **Критерий готовности микроскопа:** возможность автономной съёмки в течение 24 ч

### Фаза 2 (Q1–Q2 2027): Интеграция e-nose + AI-оператор для микроскопа
- Добавление модуля BME688, сбор данных VOC в лабораторных условиях
- Сбор когорты молодых/пожилых добровольцев для обучения классификатора
- Разработка Ze-анализа для динамики VOC
- **Критерий готовности (VOC):** классификация возраста по VOC профилю с AUC > 0.80
- Развёртывание Claude Code agent в `/overnight` режиме; валидация M1–M4
- **Критерий готовности (микроскоп):** concordance ≥80% по 286 решениям

### Фаза 3 (Q3–Q4 2027): Полевые испытания
- Тестирование браслета на 50+ добровольцах в реальных условиях
- Сбор данных по всем трём каналам одновременно
- Запуск CDATA Phase A эксперимента на микроскопе
- Подготовка статей 2 и 4

### Фаза 4 (2028): Турин-сенсор + расширение микроскопа
- R&D по сенсору на основе туннелирования электронов
- Экспериментальный прототип
- Масштабирование микроскопии на другие счётчики (Telomere, MitoROS)

---

## 12. Falsifiability (объединённая, по модулям)

### Организм-уровень (EEG, HRV, Olfaction)
- **ЭЭГ:** H₀: χ_Ze(молодые) ≤ χ_Ze(пожилые) + δ; H₁: χ_Ze(молодые) > χ_Ze(пожилые) + δ. Порог δ = 0.05 (минимальный клинически значимый эффект). Уровень значимости α = 0.001 (с поправкой Бонферрони на множественное тестирование). Минимальный размер эффекта для отклонения H₀: d ≥ 0.5 (Cohen’s d). Мощность (1−β) ≥ 0.95.
- **ВСР:** H₀: χ_Ze(RR) не коррелирует с возрастом (|r| ≤ 0.1); H₁: |r| > 0.3. α = 0.001, мощность ≥ 0.90.
- **Ольфакция:** H₀: AUC классификации (молодые vs пожилые по ЛОС) ≤ 0.55; H₁: AUC ≥ 0.75. α = 0.001, мощность ≥ 0.90.
Гипотеза считается фальсифицированной, если ни один из трёх модулей не достигает заданных порогов при N ≥ 161 (максимум из трёх модулей с учётом пересечения выборок).

### Клеточный уровень (Automated Microscopy)
- **M1 (concordance):** ≤80% при N=286 решений — фальсифицировано.
- **M2 (audit trail):** >1% пропусков за 30 дней — фальсифицировано.
- **M3 (policy adherence):** любое forbidden действие — фальсифицировано.
- **M4 (blind audit):** <90% реконструкции — фальсифицировано.
- **Uptime:** ≤90% за 180 дней — фальсифицировано.
- **Contamination:** ≥3% — фальсифицировано.
- **Cost:** >$4,950 — фальсифицировано.

---

## 13. Pre-registration plan

### Организм-уровень
- **Платформа:** Open Science Framework (OSF)
- **Идентификатор:** osf.io/ze3x7 (placeholder; registration to be submitted before data collection)
- **Дата регистрации:** Планируется до 2026-12-31 (до начала пилотного сбора данных для модулей ВСР и ольфакции)
- **Содержание:** Гипотезы, план анализа, критерии включения/исключения, протоколы сбора данных, power analysis, план обработки выбросов.

### Клеточный уровень
- **Registry:** Open Science Framework (OSF)
- **OSF ID:** `osf.io/automicroscopy_cdata` (placeholder)
- **Planned registration date:** 2026-06-01
- **Contents:** Full protocol including hypothesis, sample size calculation, analysis plan, and falsifiability criteria

---

## 14. Sample size calculation

### Организм-уровень (объединённый расчёт)
Pre-hoc power analysis для каждого модуля (α = 0.001, мощность 1−β = 0.95):
- **ЭЭГ:** Ожидаемый effect size d = 0.732 (по Dortmund Vital Study). Формула: n = (Z_α/2 + Z_β)² · 2σ² / δ², где Z_α/2 = 3.29 (для α = 0.001 two-tailed), Z_β = 1.645 (для β = 0.05). При σ = 0.15, δ = 0.1: n = (3.29 + 1.645)² · 2·0.15² / 0.1² = (4.935)² · 0.045 / 0.01 ≈ 24.35 · 4.5 ≈ 110 на группу. С учётом отсева (20%): n = 132 на группу.
- **ВСР:** Ожидаемый effect size |r| = 0.3 (средняя корреляция с возрастом). По таблицам для correlation power analysis: n ≈ 134 на группу (при α = 0.001, power = 0.95). С учётом отсева: n = 161 на группу.
- **Ольфакция:** Ожидаемый AUC = 0.75 (по литературным данным для e-nose). По формуле Hanley-McNeil: n ≈ 58 на группу (при α = 0.001, power = 0.95). С учётом отсева: n = 70 на группу.
Итоговый N = 161 (максимум из трёх модулей с учётом пересечения выборок).

### Клеточный уровень (CDATA эксперимент)
- **Primary:** Cohen’s d = 0.75, α = 0.05, power 0.80 → 30 cells per condition, 60 total, 6–12 FOV per condition, 84 time points.
- **Design effect:** 1.2 (placeholder).
- **Contamination:** N TBD from pilot.

---

## 15. Risk matrix (общая)

| Риск | Вероятность (1-5) | Влияние (1-5) | Стратегия смягчения |
|------|-------------------|---------------|----------------------|
| Артефакты миограммы в полосе 25–35 Гц | 4 | 4 | ICA, ресемплинг, полосовой фильтр, rejection по акселерометру |
| Недостаточная мощность для MPI-LEMON (null result) | 3 | 3 | Увеличение выборки до N ≥ 132 на группу; байесовский анализ |
| Отказ датчика BME688 при высокой влажности | 2 | 5 | Резервирование, калибровка, осушитель |
| Этические ограничения на забор биоматериала | 2 | 5 | IRB, информированное согласие, анонимизация |
| Hardware accuracy ниже спецификации (микроскоп) | 4 | 4 | Weekly calibration, fiducial markers |
| AI hallucination (микроскоп) | 2 | 5 | Bounded autonomy, audit trail, human-in-the-loop |
| LED bleaching / failure | 2 | 4 | Dual LED, scheduled replacement |
| Biosafety breach (микроскоп) | 2 | 5 | HEPA filter, UV, automated detection |
| Низкая воспроизводимость Ze-анализа на новых данных | 3 | 4 | Кросс-валидация на независимых датасетах; pre-registration |

---

## 16. Limitations (общие)

1. **Ze-теория не является общепринятой** — теоретическая основа не прошла широкую независимую валидацию.
2. **Валидация ЭЭГ проведена только на одном датасете с большим эффектом (Cuban)** — остальные датасеты показали меньшие эффекты или null result.
3. **MPI-LEMON null result** — d = 0.110, p = 0.765 — возможно недостаточная мощность.
4. **Для ВСР и ольфакции нет экспериментальных данных** — модули на стадии проектирования.
5. **Возможные артефакты движения и мышечной активности** — особенно в полосе 25–35 Гц.
6. **Ограничения e-nose** — чувствительность к влажности, дрейф сенсоров, перекрёстная чувствительность.
7. **Pre-registration and power analysis are preliminary** — для HRV и olfaction эффекты взяты из литературы.
8. **DIY hardware precision (микроскоп)** — ±1–2 µm.
9. **Phototoxicity** — при флуоресцентной съёмке каждые 2 часа.
10. **No published precedents** для Claude-class LLM в управлении микроскопом в реальном времени.

---

## 17. Consortium / partners

Планируемые партнёры и распределение ролей (placeholder):
- **University of Tbilisi (Грузия)** — теоретическая база Ze Theory, разработка алгоритмов
- **Charité – Universitätsmedizin Berlin (Германия)** — клиническая валидация, набор испытуемых, этическое одобрение
- **STMicroelectronics (Швейцария/Италия)** — аппаратная поддержка, интеграция датчиков BME688
- **Max Planck Institute for Human Cognitive and Brain Sciences (Германия)** — доступ к MPI-LEMON, консультации по ЭЭГ
- **University of Dortmund (Германия)** — доступ к Dortmund Vital Study
- **James Smith (University of Cambridge)** — биологическая валидация CDATA протокола
- **OpenFlexure** — open-source дизайн микроскопа
- **µManager (Vale Lab)** — программная интеграция

---

## 18. Evidence base & meta-analysis

### Основные утверждения и поддерживающие ссылки (организм-уровень)
- Ze Theory derivation: Tkemaladze, G. (2025). *Binary switching dynamics in biological systems.* Preprint. [Placeholder: DOI TBD]
- EEG aging biomarker: Babiloni, C., et al. (2021). *Resting state alpha rhythms are related to cognitive decline.* Clinical Neurophysiology, 132(8), 1870–1882. [Placeholder: DOI TBD]
- Olfactory diagnostics: Turin, L. (1996). *A spectroscopic mechanism for primary olfactory reception.* Chemical Senses, 21(6), 773–791. [Placeholder: DOI TBD]
- Heart rate variability and aging: Umetani, K., et al. (1998). *Twenty-four hour time domain heart rate variability and heart rate: relations to age and gender.* Journal of the American College of Cardiology, 31(3), 593–601. [Placeholder: DOI TBD]

### Основные утверждения (клеточный уровень)
- AI-operated microscopy: Burger et al. (2020) [10.1038/s41586-020-2442-2]; Boiko et al. (2023) [10.1038/s41586-023-06792-0]; Bran et al. (2024) [10.1038/s42256-024-00832-8].
- Low-cost microscope: OpenFlexure (Sharkey et al., 2016) [10.1063/1.4941068].
- Cell culture stability: Hayflick (1965) [PMID 14315085].
- Cell segmentation: Stringer et al. (2021) [10.1038/s41592-020-01018-x].
- Antibody specificity: GT335 (Wolff et al., 1992) [PMID 1385210]; Ninein (Delgehyr et al., 2005) [10.1242/jcs.02302].

**State-of-the-art:** Current commercial automated microscopes cost $25k–$50k; wearable EEG aging biomarkers rely on spectral power ratios rather than binary switching metrics. No published meta-analysis exists for χ_Ze. A PRISMA-compliant systematic review is planned.

**Contradictory evidence:** Some EEG slowing studies report non-linear plateau after 80 years; high-throughput microscopy achieves >95% uptime at >$50k cost; AI decision-making tested in controlled but not real-world lab conditions.

---

## 19. Migration Notes

**Что было сделано:**
- Проект AutomatedMicroscopy поглощён как модуль `BioSense/instruments/automated-microscopy/`.
- Его `CONCEPT.md` упразднён; всё содержание, перечисленное ниже, перенесено в основной BioSense/CONCEPT.md.
- Документы, оставшиеся **без изменений** внутри `BioSense/instruments/automated-microscopy/`:
  - `THEORY.md` — формальные аксиомы M1–M4 и предсказания
  - `EVIDENCE.md` — верифицированные ссылки и внутренние данные
  - `DESIGN.md` — архитектура кода и файловая структура
  - `PARAMETERS.md` — калибровочные константы (пороги, окна, δ)
  - `OPEN_PROBLEMS.md` — список нерешённых вопросов и рисков
  - `AUTOMATED_MICROSCOPY_SETUP.md` — полная инженерная спецификация (BOM, проводка, софт)
- Эти файлы теперь являются частью единого репозитория `BioSense` и не требуют отдельной регистрации.
- **Что не было объединено:** `LongevityCommon/CONCEPT.md` (остаётся умбрелла-документом), `BioSense/THEORY.md`, `BioSense/EVIDENCE.md` и др. — они остаются на своих местах. При необходимости их объединение с соответствующими файлами модуля микроскопии будет выполнено отдельной задачей.

**Дата миграции:** 2026-05-09
**Автор:** Jaba Tkemaladze

---

_Проект основан: ~2025 (как EEG project). Переименован в BioSense: 2026-03-26. Объединение с AutomatedMicroscopy: 2026-05-09._
