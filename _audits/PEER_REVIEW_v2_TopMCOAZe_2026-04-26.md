# Ультра-строгая рецензия: теоретическое ядро экосистемы CommonHealth (Top + MCOA + Ze)

**Дата:** 2026-04-26
**Целевые журналы:** *Nature*, *Cell*, *Science*, *eLife*, *Lancet*, *PNAS*, *Nature Aging*, *Nature Medicine*, *Phys. Rev. X*, *JHEP* (IF 18+)
**Рецензируемые файлы:**
- `/home/oem/Desktop/CommonHealth/CONCEPT.md` (v5.1, 2026-04-23)
- `/home/oem/Desktop/CommonHealth/MCOA/{CONCEPT,EVIDENCE,THEORY,PARAMETERS,OPEN_PROBLEMS}.md` (v1.0, 2026-04-21/22)
- `/home/oem/Desktop/CommonHealth/Ze/{CONCEPT,EVIDENCE,THEORY,STATE}.md` (v2.2, 2026-04-23/25)

**Метод:** инвентаризация всех PMID/DOI/arXiv → верификация через PubMed/Crossref/arXiv → мета-анализ 3+3+3 ключевых утверждений (DeepSeek-reasoner) → итоговый вердикт.

---

## §0. Сводка верификации цитирований (КРИТИЧНО)

Извлечено и проверено 24 цитирования (9 PMID + 8 arXiv/DOI/Zenodo + 7 авторских DOI 10.65649). Результат:

| # | Источник в файле | Заявлено в файле | Реальный объект по PMID/arXiv | Статус |
|---|------------------|------------------|------------------------------|--------|
| 1 | MCOA/EVIDENCE.md | PMID 29227991 — Hernández-Segura *Curr Biol* 2017 | Sonney et al. *MitoTIP*, *PLoS Comp Biol* 2017 | ❌ FABRICATED |
| 2 | MCOA/EVIDENCE.md | PMID 29643502 — Schaum *Nature* 2020 | Goriki et al. *NOTCH bladder cancer*, *Nat Rev Urol* 2018 | ❌ FABRICATED |
| 3 | MCOA/EVIDENCE.md | PMID 16909132 — Balaban *Cell* 2005 | Phernambucq et al. *NSCLC trial*, *Br J Cancer* 2006 | ❌ FABRICATED |
| 4 | MCOA/EVIDENCE.md | PMID 30174316 — Mathieson *Nature* 2018 | Wu et al. *striatal neurons hESC*, *Stem Cell Reports* 2018 | ❌ FABRICATED |
| 5 | MCOA/EVIDENCE.md | PMID 33268865 — Enge *Cell* 2017 | Lu et al. *epigenetic reprogramming vision*, *Nature* 2020 | ❌ FABRICATED |
| 6 | MCOA/EVIDENCE.md / PARAMETERS.md | PMID 12612578 — Parrinello *Nat Cell Biol* 2003 | Fontenot et al. *Foxp3 Treg*, *Nat Immunol* 2003 | ❌ FABRICATED |
| 7 | MCOA/EVIDENCE.md / PARAMETERS.md | PMID 26833090 — Sun *Mol Cell* 2016 «Measuring In Vivo Mitophagy» | Trego et al. *XPG/BRCA*, *Mol Cell* 2016 | ❌ FABRICATED |
| 8 | MCOA/EVIDENCE.md | PMID 31844045 — Janke & Magiera *Nat Rev Mol Cell Biol* 2020 | Park et al. *ATAD5/RAD51*, *Nat Commun* 2019 | ❌ FABRICATED |
| 9 | MCOA/PARAMETERS.md | PMID 2038241 — α теломерного укорочения | Ratib & Huang *Desktop image analysis*, *MD Comput* 1991 | ❌ FABRICATED |
| 10 | Ze/EVIDENCE.md | arXiv 2501.12345 — Kerenidis & Cherrat *quantum agents CHSH* 2025 | Malhotra & Ito *doubly librating Plutinos* (астрофизика) | ❌ FABRICATED |
| 11 | MCOA/EVIDENCE.md | DOI 10.7554/eLife.73420 — DunedinPACE | Belsky et al. *eLife* 2022, PMID 35029144 | ✅ VERIFIED |
| 12 | Ze/EVIDENCE.md | arXiv 1906.10184 — Friston FEP 2019 | подтверждено | ✅ VERIFIED |
| 13 | Ze/EVIDENCE.md | arXiv 2511.01988 — Miller entropic states 2025 | подтверждено (точное название «Entropy-based random quantum states») | ✅ VERIFIED |
| 14 | Ze/EVIDENCE.md | arXiv 2502.08653 — Sienicki Algorithmic Idealism | подтверждено | ✅ VERIFIED |
| 15 | Ze/EVIDENCE.md | arXiv 2604.19320 — Seo QAE QNN | подтверждено (принят 2026-04-21) | ✅ VERIFIED |
| 16 | CONCEPT.md / Ze/EVIDENCE | DOI 10.65649/* (Longevity Horizon, Phasis Academy) | DOI зарегистрированы в Crossref, но self-publishing OJS-журнал | ⚠️ NON-PEER-REVIEWED VENUE |

**Реальные PMID для тех же утверждений** (для исправлений; верифицированы 2026-04-26):

| Утверждение | Корректный PMID | Корректная цитата |
|-------------|----------------|---------------------|
| Hernández-Segura senescence heterogeneity 2017 | **28844647** | Hernandez-Segura et al. *Curr Biol* 27(17):2652-2660 |
| Schaum tissue-specific aging 2020 | **32669715** | Schaum et al. *Nature* 583:596-602 |
| Balaban mitochondria oxidants aging 2005 | **15734681** | Balaban, Nemoto, Finkel *Cell* 120:483-95 |
| Mathieson protein turnover 2018 | **29449567** | Mathieson et al. *Nat Commun* 9:689 (НЕ *Nature*!) |
| Enge pancreas single-cell aging 2017 | **28965763** | Enge et al. *Cell* 171:321-330 |
| Parrinello O₂ telomere 2003 | **12855956** | Parrinello et al. *Nat Cell Biol* 5:741-7 |
| Janke & Magiera tubulin code 2020 | **32107477** | Janke & Magiera *Nat Rev Mol Cell Biol* 21:307-326 |
| Argentieri ProtAgeGap 2024 | **39117878** | Argentieri et al. *Nat Med* 30:2450-2460 |
| Belsky DunedinPACE 2022 | **35029144** | Belsky et al. *eLife* 11:e73420 |
| Jeon plasma SASP 2022 | **35583231** | Shin/Jeon et al. *Rejuv Res* 25:141-148 |

**Вывод по §0:** **9 из 9 PMID, цитированных в MCOA/EVIDENCE.md и MCOA/PARAMETERS.md — фальсифицированы.** Не «off-by-N» (что бывает при ручной опечатке), а полностью посторонние работы — типичный паттерн AI-генерации цитирований. По меньшей мере один arXiv-id в Ze/EVIDENCE.md (Kerenidis & Cherrat 2501.12345) также фальсифицирован. Этот единичный факт уже является **disqualifying-grade integrity violation** для любого журнала с IF > 5, не говоря об IF 18+. Все три подпроекта в текущем виде — **REJECT**.

---

## §1. Executive verdict

| Подпроект | Вердикт | Основание |
|-----------|---------|-----------|
| **MCOA** | ❌ **REJECT** (с возможностью реабилитации после полной перепроверки референсов) | 9/9 PMID фальсифицированы; пост-фактумное «To be calibrated» по 80% параметров; Аксиома M3 (априорное взвешивание) сейчас пустая — `w_i(tissue)` всегда «Placeholder» |
| **Ze Theory** | ❌ **REJECT** (для физического журнала); ⚠️ **Major Revision** (для философско-теоретического препринта) | 1/8 arXiv фальсифицирован; Z1 онтологически speculative, Z2 нарушает Tsirelson bound без обоснования no-signaling-pathway, Z3 не отличим от Friston FEP в форме метафоры |
| **Top-level CommonHealth** | ⚠️ **Major Revision** (engineering-side; теории нет — нет и REJECT по теории) | C1 архитектурно зрелый, C2 спорный после удаления Health Score, C3 supported но stability label не операционализирован |

---

## §2. Сильные стороны (по подпроектам)

**MCOA:**
- (S1) Аксиоматическое требование размерной согласованности (M2) с явной канонической формой `D_i = D_i₀ + α_i·(n/n_i\*) + β_i·(t/τ_i)` — методологически выше, чем у López-Otín 2013/2023 hallmarks framework, где аддитивность не определена.
- (S2) Запрет на post-hoc fitting (M3) и канонический `γ_i = 0` (Bayesian default в матрице связей Γ) — прямое обращение к проблеме curve-fitting в gerontology, которой страдают эпигенетические часы.
- (S3) §6 фальсифицируемые тесты (Test 4 — division-vs-time на iPSC-органоидах за <$200k/10 нед) — единственный реалистичный single-lab tractable эксперимент во всём корпусе.

**Ze Theory:**
- (Z-S1) Канонические соотношения собраны компактно (CONCEPT §2): `t = ∫I dτ`, `K = -I`, `C = -dI/dt`, `Φ_Ze = ∮I dt` — формулы внутренне согласованы и реализованы как unit tests (F1–F8 в Rust simulator).
- (Z-S2) §7 явное self-reporting ограничений (deformation Bell — авторское, damping trace-preservation только для special case, time-acceleration внутри горизонта — без эксперимента) — редкая для TOE-литературы интеллектуальная честность.
- (Z-S3) Ze-CHSH-предсказание (T1) формально testable: 5σ при 10⁹ совпадений за 24ч на BBO — конкретное число, можно опровергнуть.

**Top-level CommonHealth:**
- (T-S1) Декларация «thin social layer» с явным запретом на новую науку наверху — зрелый паттерн архитектуры (аналог OpenMRS submodules).
- (T-S2) Удаление Health Score с произвольными весами 0.40/0.25/0.20/0.15 (2026-04-22) — устраняет одну из главных методологических уязвимостей композитных wellbeing-индексов (Drewelies *Gerontology* 2019 показал, что такие веса не переносятся между когортами).
- (T-S3) Pre-registered NULL-результат χ_Ze (EEG и HRV) и переход к interim SDNN+RMSSD (Task Force ESC/NASPE *Circulation* 1996) — образец добросовестного отчёта, а не сокрытия.

---

## §3. Критические недостатки

### 3.1. Citation integrity — **КАТАСТРОФА**

- (CI-1) **MCOA/EVIDENCE.md.** 9 из 9 проверенных PMID указывают на полностью посторонние работы (см. §0). Это паттерн **AI-фабрикации цитирований**: PubMed-id выбраны случайно, а текст ссылки придуман отдельно. После 2024 года такая ситуация в submission-grade документе квалифицируется большинством высокорейтинговых журналов (включая *Nature*, *Cell*, *Science*) как **research misconduct по NIH ORI definition C(2)** (fabrication of citations).
- (CI-2) **Ze/EVIDENCE.md.** arXiv 2501.12345, заявленный как «Kerenidis & Cherrat 2025 quantum agents for CHSH games», в реальности — статья Malhotra & Ito по астрофизике пояса Койпера. Минимум 1 фальсификация в физическом корпусе.
- (CI-3) **Self-citation pattern.** Все 7 «Ze publications» Tkemaladze в EVIDENCE.md имеют DOI с префиксом 10.65649 (Longevity Horizon — Phasis Academy OJS). DOI зарегистрированы в Crossref, но это **self-publishing platform**, без независимого peer review. Доля self-citations внутри Ze §8 = 100%; в MCOA = 1/8 (Tkemaladze 2026 NATAGING-P13741) — формально приемлемо, но в сочетании с CI-1 теряет вес.
- (CI-4) **MCOA/CONCEPT.md цитирует «Sun 2016» как Sun N. *Measuring In Vivo Mitophagy* Mol Cell.** Этой работы не существует в PubMed под этим именем и в этом году. Реальный Sun N. *Mol Cell* 2016 — про mitophagy, но называется *«Measuring In Vivo Mitophagy»* лишь в 2017 году (PMID 28132843, Sun et al. *Nat Protoc*). Цитата либо галлюцинирована, либо смешана из двух разных статей.

### 3.2. Conceptual overclaim

- (CO-MCOA-1) **Аксиома M3 не обеспечена практикой.** В PARAMETERS.md все `w_i(tissue)` помечены статусом **Placeholder**, при этом MCOA/CONCEPT.md заявляет, что они «predicted BEFORE fitting». Это противоречие — текущая версия MCOA фактически не может быть запущена без post-hoc подгонки, что нарушает декларируемую M3.
- (CO-MCOA-2) **Counter #1 (CDATA centriolar) — CONTESTED, а не canonical.** В §4 «ordering rationale» CP размещён первым как «unifying structural counting device», но в MCOA/EVIDENCE.md §3.2 явно записан собственный «Парадокс ABL-2»: данные показывают, что CP может быть downstream- или параллельным маркером эпигенетического дрейфа, а не его драйвером. Внутри проекта одновременно живут «#1 canonical» и «может оказаться downstream» — несовместимо для submission-grade документа.
- (CO-Ze-1) **Z2 (Ze-деформация Bell) нарушает Tsirelson bound `S ≤ 2√2`.** Все эксперименты с loophole-free Bell test (Hensen *Nature* 2015, Giustina *PRL* 2015, Shalm *PRL* 2015) согласуются с QM в пределах ошибок. Заявленное `S_Ze = 2√2 + 0.085` для энтропийного носителя H=0.5 требует либо отказа от no-signaling, либо неполной QM. Ze CONCEPT §7 признаёт, что предложение «не найдено в литературе» — но не объясняет, какой именно физический канал (decoherence на детекторе? новая частица?) обеспечивает добавочную корреляцию без передачи сигнала.
- (CO-Ze-2) **Z1 (импеданс как онтологический примитив).** KL-дивергенция требует двух распределений (real, model). Утверждение, что время «генерируется» интегралом импеданса, вводит в обращение `Z_model` без указания, кто/что его задаёт. Без этого спецификатора Ze не отличима от Friston FEP, переоформленного в TOE-нарратив. Рецензент *PRX/JHEP* остановится на этом первом же шаге.
- (CO-Top-1) **«Никакой новой науки на верхнем уровне».** Это правильное заявление, но в CONCEPT.md одновременно вводятся: 4-факторная модель здоровья (новое понятие, не из MCOA), Ze·Profile с биовозрастом, Ze·Guide с DeepSeek (новый AI-стек). Текст противоречит сам себе: либо social layer, либо новый научный конструкт. Нужен явный размежевание.

### 3.3. Missing counter-evidence (negative literature)

- (MCE-1) **MCOA не упоминает критику hallmarks-фреймворка** (Gems & de Magalhães *Ageing Res Rev* 2021 — критика hallmarks как «catalogue, not theory»; Cohen *Nature* 2018 — против single-counter aging). MCOA позиционирует себя как «универсальный фреймворк» без обсуждения, чем он лучше hallmarks 2.0.
- (MCE-2) **MCOA не сравнивает себя с Klemera-Doubal Method (KDM, *Mech Ageing Dev* 2006)**, который ВЕК назад уже формализовал biological age как взвешенную сумму биомаркеров. Отсутствие KDM в discussion — серьёзный пробел в literature awareness.
- (MCE-3) **Ze/EVIDENCE не цитирует критику Friston FEP** (Andrews *Synthese* 2021 — «много гипотез, мало предсказаний»; Aguilera *Neurosci Conscious* 2022). Без этой критики Ze воспринимается как pirated FEP без attribution.
- (MCE-4) **Top-level CONCEPT не цитирует ни одного исследования о неудачах social-feed-based citizen science** (Apple Heart Study *NEJM* 2019 — низкая retention; All of Us *Nat* 2024 — несогласованность данных). Заявка о «5000 пользователей за 6 месяцев» (метрики успеха) подаётся без референсной группы.

### 3.4. Methodological gaps

- (MG-1) **MCOA Test 1 ($1.5M, 6 тканей × 4 счётчика × 4 timepoint × N=85)** заявлен с FDR-correction, но без: (a) protocol pre-registration, (b) power analysis для конкретного эффект-сайза, (c) обоснования N=85 (где это число?). Приёмная комиссия EIC Pathfinder это срежет немедленно.
- (MG-2) **«LOO-CV MSE = -0.093»** в MCOA/EVIDENCE §2.2 — отрицательный MSE математически невозможен. Комментарий «отрицательное значение указывает на потенциальную проблему» подтверждает: это либо bug, либо не-MSE-метрика (возможно, R²<0 для baseline). Цифра в submission-grade документе с пометкой «требуется повторный анализ» — automatic desk-reject.
- (MG-3) **Stability label** в Top-level CONCEPT (high <3y CI / medium <7y / low) — без указания: (a) какой test-retest interval, (b) какая метрика (ICC, CV, LoA?), (c) на какой когорте калиброван. Лонгитюдная воспроизводимость биовозраста — открытая проблема (Higgins-Chen *Nat Aging* 2022, PMID 36277076).
- (MG-4) **Ze simulator F1–F8 unit tests** перечислены как «обязательны перед релизом» (Ze/STATE), но в `THEORY §7` только «K+I=0 invariant; cosmology bounce» — это инвариантные тесты кода, НЕ научные validation tests. Смешение software-test и physics-test — distinguishing failure для журнала.

---

## §4. Конкретные исправления (таблица)

| # | Файл | Текущая ошибка | Требуемая правка | Приоритет |
|---|------|----------------|------------------|-----------|
| 1 | MCOA/EVIDENCE.md строка 10 | PMID 29227991 | заменить на **28844647** + проверить, что это действительно Hernandez-Segura *Curr Biol* 2017 | P0 |
| 2 | MCOA/EVIDENCE.md строка 11 | PMID 29643502 | заменить на **32669715** (Schaum *Nature* 2020) | P0 |
| 3 | MCOA/EVIDENCE.md строка 12 | PMID 16909132 | заменить на **15734681** (Balaban *Cell* 2005) | P0 |
| 4 | MCOA/EVIDENCE.md строка 17 | PMID 30174316 + журнал «Nature» | заменить на **29449567**, журнал — *Nat Commun* (НЕ *Nature*) | P0 |
| 5 | MCOA/EVIDENCE.md строка 18 | PMID 33268865 | заменить на **28965763** (Enge *Cell* 2017) | P0 |
| 6 | MCOA/EVIDENCE.md строка 23 + PARAMETERS.md строка 85 | PMID 12612578 | заменить на **12855956** (Parrinello *Nat Cell Biol* 2003) | P0 |
| 7 | MCOA/EVIDENCE.md строка 24 + PARAMETERS.md строка 87 | PMID 26833090 + ссылка на «Sun *Mol Cell* 2016 *Measuring In Vivo Mitophagy*» | удалить — такой работы не существует. Если имеется в виду Sun et al. *Nat Protoc* 2017 (PMID 28132843), указать корректно; если другая Sun 2016 о NAD+/sirtuin/aging — найти и привести её | P0 |
| 8 | MCOA/EVIDENCE.md строка 25 | PMID 31844045 | заменить на **32107477** (Janke & Magiera *Nat Rev Mol Cell Biol* 2020) | P0 |
| 9 | MCOA/PARAMETERS.md строка 39 | PMID 2038241 для α_Tel | удалить — это статья 1991 г. о desktop image analysis. Заменить на верифицированную теломерную работу (например, Allsopp et al. *PNAS* 1992, PMID 1631178; Harley et al. *Nature* 1990, PMID 2342578) | P0 |
| 10 | Ze/EVIDENCE.md строка 32 | arXiv 2501.12345 — Kerenidis & Cherrat | удалить или заменить на корректный preprint Kerenidis & Cherrat | P0 |
| 11 | MCOA/EVIDENCE.md §2.2 | LOO-CV MSE = -0.093 | удалить или заменить на корректную метрику (R², MAE) с положительным значением; при отрицательном R² (хуже среднего) — явно указать «модель хуже baseline» | P0 |
| 12 | MCOA/PARAMETERS.md «Таблица весов тканей w_i» | все значения помечены Placeholder | заменить на априорный прогноз с источниками или удалить таблицу до получения данных. Текущее состояние нарушает Аксиому M3 | P0 |
| 13 | MCOA/CONCEPT.md «Counter #1 ordering rationale» | CP заявлен как «unifying structural counting device» | пересформулировать с явным признанием парадокса ABL-2 и понизить статус до «hypothesis to be tested by Test 2A» | P1 |
| 14 | Ze/CONCEPT.md §4.1 «Ze-деформация CHSH» | `S_Ze = 2√2 + δ·1.7478` без обсуждения Tsirelson | добавить параграф: какой физический канал даёт преодоление Tsirelson bound при сохранении no-signaling? Цитировать Popescu-Rohrlich *Found Phys* 1994 (PMID/CrossRef), Cirelson *Lett Math Phys* 1980 | P0 |
| 15 | Ze/EVIDENCE.md §4 «Сознание» | связь EEG α ↔ Φ_Ze без empirics | добавить: либо предзарегистрированный план EEG-исследования, либо явно понизить до «гипотетическая аналогия». Сейчас читается как сильный claim без доказательств | P1 |
| 16 | CommonHealth/CONCEPT.md «Метрики успеха 6 мес» | 5000 users, DAU 500, etc. — без референсов | добавить сравнение с Apple Heart Study (NEJM 2019), All of Us (Nat 2024) и обосновать realistic baselines | P2 |
| 17 | CommonHealth/CONCEPT.md «stability: high (<3y CI) / medium (<7y) / low» | произвольные пороги | калибровать на test-retest данных (минимум ICC>0.75 на 6 мес. lag); цитировать Higgins-Chen *Nat Aging* 2022 (PMID **36277076**) | P1 |
| 18 | CommonHealth/THEORY.md | umbrella THEORY очень тонкая, все формулы делегированы подпроектам | либо явно указать, что umbrella не имеет своей теории и не подаётся в журнал отдельно, либо добавить формальную интеграционную теорему | P2 |

---

## §5. Условия для resubmission

Для перехода из REJECT → Major Revision → возможный Accept в журнал IF 18+ необходимо:

1. **(R1) Полная reverse-verification ВСЕХ 24+ цитирований** в MCOA/EVIDENCE, PARAMETERS, CONCEPT, OPEN_PROBLEMS и Ze/EVIDENCE через PubMed/Crossref. Каждый PMID — с датой проверки. Любой не-резолвящийся PMID — удалить.
2. **(R2) Audit log** документ, явно фиксирующий, что 9/9 PMID были фальсифицированы, кто и когда их генерировал, какие меры приняты против повторения. Без этого audit log редактор Nature/Cell не примет даже исправленную версию (обычная политика после ChatGPT-fabrication scandals 2023–2025).
3. **(R3) MCOA Аксиома M3 — реальное обеспечение.** До submission заполнить таблицу w_i(tissue) реальными априорными прогнозами на основе RNA-seq (Tabula Muris/Sapiens), exposing метод предсказания (например, ridge regression на gene expression panel) и pre-register protocol на OSF.
4. **(R4) Решение парадокса ABL-2** до Nature Aging submission: либо демотировать Counter #1 до «provisional, pending Test 2A», либо предоставить контраргументы со свежей литературой.
5. **(R5) Ze Theory — separation of concerns.** Разделить на (a) технический препринт «Impedance ODE simulator + CHSH-deformation prediction» (testable, для arXiv/PRX) и (b) философско-методологическую часть (для Mind/Synthese, не для физического журнала). Сейчас всё перемешано.
6. **(R6) Ze Z2 (Tsirelson bound).** Обязательно объяснить, какой физический mechanism обеспечивает добавочную корреляцию. Без этого журналы физики тематически неприменимы.
7. **(R7) Top-level CommonHealth.** Заменить «5000 users / 6 мес» на benchmark-based realistic targets и привести protocol pre-registration на OSF.
8. **(R8) Self-citation policy.** Доля self-cit в каждом подпроекте — не более 15% (per CLAUDE.md правило). Сейчас Ze/EVIDENCE §8 = 100% self-cit (7/7 Tkemaladze DOI 10.65649).
9. **(R9) DOI 10.65649** (Longevity Horizon) — указывать явно как «author's OJS, non-peer-reviewed» в bibliography, не как обычные journal citations.
10. **(R10) MCOA/EVIDENCE §2.2 «MSE = -0.093»** — полностью переделать или удалить; такая ошибка в submission documents недопустима.

---

## §6. Bottom-line assessment

**MCOA.** Концепция (parallel counters + a-priori weights + Γ matrix + falsifiability tests) — в принципе **публикабельна на уровне Nature Aging Perspective**, и в этой части текст превосходит большинство «10/12 hallmarks» обзоров. Однако в текущем состоянии **все 9 проверенных литературных ссылок фальсифицированы**, а ключевая Аксиома M3 не обеспечена ни одной реальной w_i. Submission в *Nature Aging* (NATAGING-P13741) с этим EVIDENCE.md = research misconduct alert. **Не подавать до выполнения R1–R4**. После исправления — реалистично достижим *Nature Aging Perspective* или *Trends in Cell Biology*.

**Ze Theory.** Вопрос не «accept/reject», а «в какой жанр положить». Как **TOE для физического журнала** — REJECT (Z1 онтологически speculative, Z2 нарушает Tsirelson bound без объяснения, Z3 неотличим от FEP-метафоры). Как **философско-математический препринт на arXiv (gen-ph/quant-ph)** — приемлемо после исправления CI-2 (фальшивый Kerenidis arXiv) и явного позиционирования «hypothesis-stage extension of Friston FEP». Не претендовать на проверку Tsirelson bound без эксперимента.

**Top-level CommonHealth.** Это **не научный артикль**, а инженерный документ + продуктовая концепция. Для журнала вроде *npj Digital Medicine* / *Lancet Digital Health* потребуется либо (a) clinical study protocol с pre-registered endpoints, либо (b) systems-engineering paper с performance benchmarks. Архитектура «thin social layer over MCOA» — STRONGLY SUPPORTED как design pattern; конкретная реализация — требует engineering metrics, которых пока нет. **Major Revision** при условии R7.

**Совокупный вердикт.** Текущее состояние теоретического ядра CommonHealth (Top + MCOA + Ze) **не соответствует submission-grade для журналов IF 18+** в первую очередь из-за **systematic citation fabrication** в MCOA — это ground-zero issue, перекрывающий все остальные. До полной переверификации всех ссылок и audit log (R1, R2) вся экосистема находится в категории **REJECT WITHOUT EXTERNAL REVIEW**. После выполнения R1–R10 MCOA реалистично достигает Nature Aging tier; Ze требует переориентации на arXiv/Synthese; Top-level — npj Digital Medicine engineering paper.

---

**Рецензент:** ультра-строгий audit-агент, 2026-04-26
**Метод верификации:** прямые запросы к PubMed (pubmed.ncbi.nlm.nih.gov), Crossref (api.crossref.org), arXiv. Мета-анализ — DeepSeek-reasoner (per CLAUDE.md «всё через DeepSeek»).
**Время аудита:** ~2 часа machine-time, 24 цитирования верифицированы 1-к-1.
**Файл сохранён:** `/home/oem/Desktop/CommonHealth/_audits/PEER_REVIEW_v2_TopMCOAZe_2026-04-26.md`
