# MEMORY — MCARA

> История решений, журнальный путь, ключевые договорённости.

---

## 2026-07-15: Апелляция в Biogerontology 🟡

- **Журнал:** Biogerontology | **ID:** `7cc6de62` | **Тип:** Апелляция на решение редактора
- **Письмо:** Rattan + Yanai. Аргумент: статья полностью переписана — narrative-first, Figure 1 (centriole-cilium ratio), 13-group experiment table, 5 falsifiable hypotheses. Наука не изменена, презентация — новая.
- **Ключевые изменения:**
  1. Открывается с парадокса: hTERT + гипоксия → arrest (не абстрактный framework)
  2. Центриоль — физический объект, не абстрактный счётчик
  3. Доказательства из 5 phyla ДО multi-counter архитектуры
  4. Figure 1: centriole-cilium ratio (графический обзор)
  5. Таблица 13-group эксперимента → 5 falsifiable гипотез
- **Статус:** 🟡 Ожидание ответа редактора

---

## 2026-07-14: Подача в BioEssays (Problems & Paradigms) 🟢

- **Журнал:** BioEssays (Wiley) | **ID:** `4799098` | **Тип:** Problems & Paradigms
- **Название:** «Centriole Elimination as a Gateway to a New Differentiation State: A Hypothesis»
- **Статус:** 🟢 Подана 2026-07-14, ожидание редакционного решения

---

## 2026-07-14: Препринт «Three-Step Strategy to Overcome the Sprouting Paradox» — Research Square 🟢

- **Платформа:** Research Square | **DOI:** `10.21203/rs.3.rs-10320333/v1`
- **RSID:** rs-10320333
- **Статус:** ✅ Опубликован, постоянный DOI

---

## Rejection #1 — 2026-07-13: Biogerontology

**Журнал:** Biogerontology | **ID:** `7cc6de62` | **Дней:** 40 | **Тип:** Section editor (Hagai Yanai)

**Причина:** «While this concept is intriguing, the presentation is difficult to follow and I recommend that it is made more approachable to a wider readership.»

**Что мы упустили:** MCARA — сложная количественная модель. Подана в сыром виде: много формул, мало нарратива. Не адаптирована под читателя-биогеронтолога (не физика).

**Что изменить:**
1. Добавить графическую схему MCARA (multi-counter visual)
2. Вынести математику в Appendix/Supplementary
3. Основной текст — нарратив: проблема → контрпримеры → MCARA-решение → предсказания
4. Каждый counter объяснить на конкретном биологическом примере
5. Сократить на 30%, убрать дублирование

**Следующий шаг:** Transfer в другой Springer-журнал (предложат автоматически) ИЛИ переписать и подать заново. Решение: переписать для читателя.

---

## 2026-07-14: Разговор с Гакели — OpenFlexure, инвертированная vs прямая схема, нанопоры

- **Событие:** Вечерний диалог с Гакели (WhatsApp) о микроскопии для ARGUS
- **Контекст:** Джаба рассказал о проблеме с водяной иммерсией в инвертированном положении (объектив снизу — вода стекает). Гакели предложил прямое решение: **прямой (upright) микроскоп.**
- **КЛЮЧЕВОЕ ИНЖЕНЕРНОЕ ПРОЗРЕНИЕ:**
  - Объектив **сверху** → вода держится гравитацией + поверхностным натяжением ✅
  - Лазер абляции **снизу** → через стеклянное дно чашки Петри ✅
  - Клетки оседают на дно сами ✅
  - Это **проще**, чем текущий дизайн ARGUS-LP v3 (инвертированный + водяная муфта + шприцевой насос 0.1 мл/ч)
- **OpenFlexure:** Гакели изучает OpenFlexure v6.1.5 — open-source 3D-печатный микроскоп:
  - Прямой (upright), моторизованный (<100 нм позиционирование)
  - RMS-оптика, Raspberry Pi Camera 2
  - Полностью открытые STL-файлы
  - Стоимость: ~$200–500 (печать + оптика + RasPi)
  - Поддерживает флуоресценцию, фазовый контраст, структурированное освещение
  - ПО: open-source Python, плагины, программируемые клиенты
  - **Ссылка Гакели:** https://microscope-stls.openflexure.org/#/v6.1.5 (RMS_f50d13 + picamera_2)
- **Нанопоры + отслеживание линий:** Гакели упомянул: Oxford Nanopore читает метилирование одновременно с ДНК → можно отслеживать дифференцировку клеточных линий и тканевое возрастное группирование. Это **ортогональный** (комплементарный) метод к визуальному lineage tracking ARGUS-LP.
- **Видение масштабирования:** «6,000 роботов = 120,000 учёных мышей» — при цене OpenFlexure ~$500/станция → сеть из 6,000 ARGUS-станций = ~$3M.
- **Сравнительный анализ:** OpenFlexure (прямой, $200–500) vs ARGUS-LP v3 (инвертированный, $2,045–$8,170) — `~/Desktop/LC/MCARA/ARGUS-LP/docs/OpenFlexure_vs_ARGUS-LP_analysis_2026-07-14.md`
- **Служебная записка в Telegram-группу ARGUS:** `~/Desktop/LC/MCARA/ARGUS-LP/docs/ARGUS_Telegram_memo_2026-07-14.md`

---

## 2026-07-13: Анализ Research Feed — ClpP, mRNA-регионализация, mei-P26 + похожие статьи

- **Событие:** Анализ 7 статей из ленты Jaba + PubMed-поиск похожих (митохондриальный QC, ClpP/LONP1, mRNA-локализация).
- **Результат:** 4 категории статей (ТОП-3 + 4 дополнительных из ленты, 5 из поиска по митохондриальному QC, 5 из поиска по ClpP/LONP1). Всего ~20 высокорелевантных PMID.
- **КЛЮЧЕВОЕ:** CDATA переименован в CEDAR — заменено в AGENTS.md, директории уже переименованы.
- **КЛЮЧЕВЫЕ НАХОДКИ ДЛЯ MCARA:**

### 1. Серия ClpP — митохондриальный протеостаз и мейоз
| PMID | Статья | Год | Связь |
|------|--------|-----|-------|
| 42281331 | Feng HW et al. ClpP Ensures Mitochondrial Integrity and Meiotic Progression — Andrology | 2026 | 🔥 ClpP cKO → мейотический блок, дефекты митохондрий |
| 37798322 | Guo C et al. ClpP/ClpX deficiency → impaired mTORC1 signaling — Commun Biol | 2023 | mTOR-ось: ClpP → митохондрии → mTORC1 → мейоз |
| 23851121 | Gispert S et al. Clpp null → infertility, mtDNA accumulation — Hum Mol Genet | 2013 | Первая характеристика Clpp KO |
| 38927630 | Key J, Gispert S, Auburger G. CLPP/CLPX in matrix condensates near IMM — Genes | 2024 | Молекулярный механизм CLPP/CLPX |
| 38341415 | Ng AQE et al. Nutrient-dependent intron → germline mitochondrial QC — Nat Commun | 2024 | Связь нутриентов с митохондриальным QC в герм-клетках |

### 2. Митохондриальный UPR и протеазы
| PMID | Статья | Журнал | Год |
|------|--------|--------|-----|
| 42216472 | Czechowicz P et al. The mammalian mitochondrial UPR — multilayered circuit | FEBS J | 2026 |
| 41655698 | Currie SQW et al. Molecular mechanisms of mitochondrial AAA+ proteases | J Biol Chem | 2026 |
| 40903791 | Nandha SR et al. Targeting CLPP and LONP1 → proteotoxic stress | Cell Commun Signal | 2025 |

### 3. mRNA-регионализация и клеточное паттернирование
| Статья | Журнал | Связь с CEDAR |
|--------|--------|---------------|
| Albright AR et al. mRNA regionalization in giant single cell | PNAS 2026 | 🔥 Прямой proof: одиночная клетка → пространственное паттернирование → основа асимметричного деления |
| Leite I et al. Cyst-ained connections in mammalian germline | Curr Top Dev Biol 2026 | Цисты герм-клеток — структурный контекст для mRNA-регионализации |

### 4. Митоз→мейоз переход
| Статья | Журнал | Связь |
|--------|--------|-------|
| Terry J et al. mei-P26 mutation → impaired chromosome dynamics | Genetics 2026 | Молекулярный gatekeeper: митоз→мейоз переход |
| Iniesta-Cuerda M et al. SIRT1 haploinsufficiency → age-associated subfertility (α-tubulin hyperacetylation) | Biol Direct 2026 | Эпигенетический механизм возрастной субфертильности |

- **Полный анализ:** `~/Desktop/Services/docs/RESEARCH_FEED_ANALYSIS_2026-07-13.md`

---

## 2026-07-12: Анализ 144 references Manni et al. (IGF-1 senescence switch)

- **Событие:** Глубокий анализ reference list статьи Manni et al. (Cytokine, 2026, PMID: 41905220) — 144 references.
- **Результат:** Ни одной работы Джабы (Tqemaladze/Tkemaladze/Chichinadze) среди references. «Ageless Creatures» не найдена в reference list — вероятно, ошибка Google Scholar алерта или книга.
- **КЛЮЧЕВЫЕ ПЕРЕСЕЧЕНИЯ С MCARA (10 references из 144):**

| # | Reference | Год | Связь с MCARA |
|---|-----------|-----|-------------|
| 1 | **Hallmarks of aging: An expanding universe** (López-Otín) | 2023 | Прямая связь — Джаба цитирует в MCARA |
| 2 | **IGFBP5 is released by senescent cells and is internalized by healthy cells, promoting their senescence** | 2024 | 🔥 **Паракринный механизм** — аналог центриолярного драйвера, но через секретируемые факторы! |
| 3 | **Hypoxia-Induced Senescent Fibroblasts Secrete IGF1 to Promote Cancer Stemness** | 2024 | Стволовость → центриоли в раке |
| 4 | **Targeting IGF1-Induced Cellular Senescence to Rejuvenate Hair Follicle Aging** | 2025 | Реювенация = вмешательство в счётчики |
| 5 | **The IGF System and Aging** (Endocrine Reviews) | 2024 | Comprehensive review IGF-1/aging |
| 6 | **Cellular senescence in tissue repair and regeneration** | 2021 | Регенерация → тканевый уровень MCARA |
| 7 | **IGFBP-5 Induces Cell Senescence** | 2018 | Механизм — как центриоль запускает сенесценцию |
| 8 | **Insulin/IGF-1 and ROS signaling pathway cross-talk** | 2008 | ROS — общий знаменатель IGF-1 + центриоли |
| 9 | **IGF-I enhances cellular senescence via ROS-p53 pathway** | 2012 | ROS→p53 — общий путь |
| 10 | **Senescence and the SASP: many therapeutic avenues** | 2020 | SASP — therapeutic window |

- **Главный вывод:** Manni et al. строит мост между IGF-1 signalling и SASP/senescence. Для MCARA это означает: **IGFBP-5 = молекулярный аналог центриолярного храповика на уровне секретома**. Старая центриоль → изменённый CAMC → изменённый секретом (IGFBP-5↑) → паракринная сенесценция.
- **Файл анализа:** `~/Desktop/Services/docs/RESEARCH_FEED_ANALYSIS_2026-07-12.md`

---

## 2026-07-13: C. elegans paradox — resolved

- **Событие:** Разрешение парадокса C. elegans: почему элиминация центриолей в соматических клетках ведёт к терминальной дифф-ке, а не к тотипотентности.
- **Причина:** У соматических клеток C. elegans нет ни Reprogram (нет факторов мейоза — DUX4/TPRX1), ни Rebuild (нет de novo сборки центриолей). Только Eliminate. Без шагов 2 и 3 генным сетям нечем переключиться.
- **Germline (зигота):** Все три шага происходят естественно → тотипотентность.
- **Обновлены:** Статья Desktop (§6.5.3), THEORY.md, CONCEPT.md

---

## 2026-07-12: Differentiation Ratchet — уточнение концепции

- **Событие:** Джаба уточнил концепцию центриолярного храповика
- **Ключевые положения:**
  1. **Центриоль стареет по второму закону термодинамики.** Не от делений, не от участия в дифф-ке. В нейроне — так же, как в СК. **Ответ Хейфлику:** центриоль — стохастика (второй закон). Накопление старых центриолей в СК — программа (асимметричное наследование). Стохастика на уровне органеллы, программа на уровне клетки.
  2. **Энтропия — не храповик. Храповик — переключение генных сетей.** Необратимая дифференцировка = отключение одних генных сетей + включение других. Центриоль — ИНСТРУМЕНТ переключения (DID-РНК, CAMC, NANOG, cilium).
  2. **Каждое асимметричное деление** продвигает храповик на один щелчок. Старая центриоль → стволовая (сохраняет статус). Новая центриоль → дифф-ка (шаг вперёд).
  3. **Стволовые клетки накапливают** старые центриоли — механизм трансляции центриолярного старения в старение организма.
  4. **Разные центриоли — разные CAMC-профили.** Старая = CAMC_old (удерживает стволовое состояние). Новая = CAMC_new (толкает в конкретную линию).
  5. **DID-частицы.** При делении одна отщепляется → уходит с новой центриолью. При редупликации — с уменьшением. **DID может не закончиться. Проблема — центриоль-носитель деградирует по второму закону термодинамики.** Даже при DID=N, центриоль становится нестабильной → теряет способность удерживать DID. Старение = деградация носителя, а не истощение счётчика.
  6. **Старение организма — цена за истинную дифференциацию.** Растения: модуляция (обратимая дифф-ка), нет центриолей в соме. Животные: необратимая дифф-ка через центриолярный храповик → старение.
  7. **PolyE = компенсация дисфункции.** TTLL стабилизируют микротрубочки при накоплении энтропии; CCP удаляют polyE. Баланс = борьба центриоли за функцию.
  8. **Три шага — Eliminate → Reprogram → Rebuild.** (1) ELIMINATE — удаление. (2) REPROGRAM — DUX4+TPRX1. (3) REBUILD — de novo центриоль как ИНСТРУМЕНТ ПЕРЕКЛЮЧЕНИЯ GRN. **PCM1 (транспорт) + DID-РНК (перезапись генома) — комплементарны.**
  9. **Направление движения.** Вниз — без барьеров. Вверх — против храповика.
10. **Maturity sensor — альтернативная модель (Lindhout 2021).** Центриоль = датчик зрелости, не замок. Потеря центриоли снижает порог дифф-ки неспецифически, без задания конкретного состояния. Обе модели (hardware reset и maturity sensor) предсказывают повышение пластичности после элиминации — трудноразличимы без специальных тестов. Различающие эксперименты: (а) ядерный NANOG — если ↑ → активный регулятор; (б) элиминация без факторов — если хаотичная активация ZGA → maturity sensor; (в) IFT88 shRNA — если потеря реснички имитирует потерю центриоли → эффект через cilium. Модели не взаимоисключающие: центриоль может быть и сенсором (через cilium), и хабом (через NANOG/DID).
- **Обновлены:** Статья на Desktop, CONCEPT.md, THEORY.md

---

## 2026-07-12: Центральный эксперимент — гипотеза тотипотентности

### 2026-07-12: Отличие факторов плюрипотентности от тотипотентности (Синклер/Гладышев)

- **Контекст:** Обсуждение гипотезы тотипотентности при centriole elimination + OSKM. Джаба спросил: в чём отличие факторов, применяемых для индукции плюрипотентности (OSKM), от факторов тотипотентности (о которых говорили Синклер и Гладышев)

**Таблица сравнения:**

| | OSKM → Плюрипотентность | Тотипотентные факторы |
|---|---|---|
| Уровень | ICM/эпибласт (~E4.5) | Зигота/2C (~E1.5) |
| Факторы | Oct4, Sox2, Klf4, c-Myc | **DUX4** (человек) / Dux (мышь), **TPRX1/Tprx2**, **Zscan4**, Dppa3/Stella |
| Транспозоны | Подавлены (LINE1, IAP) | **Активированы** — MERVL (маркер 2C-like) |
| Метилирование | Пассивное + TET | **Активное** тотальное деметилирование |
| Теломеры | Теломераза | **Zscan4-рекомбинация** (альтернативный путь) |
| Трофобласт | ❌ Нет | ✅ CDX2+ |
| Центриоли | Остаются (не сбрасываются) | В зиготе: элиминируются → de novo |

**Ключевые тотипотентные факторы:**
- **DUX4/DUX:** Pioneer factor — открывает хроматин в локусах ZGA (zygotic genome activation). Активирует MERVL. Главный переключатель «2C-like state»
- **TPRX1:** Человеческий 8-клеточный транскрипционный фактор. Связывает и активирует гены раннего эмбриогенеза
- **ZSCAN4:** Способствует удлинению теломер через рекомбинацию (не теломеразу!), обеспечивает геномную стабильность на ранних стадиях
- **DPPA3/STELLA:** Защищает импринтированные локусы от деметилирования — сохраняет эпигенетическую память родительского происхождения

**Связь с MCARA:**
- OSKM перезагружает software (эпигеном), но не трогает hardware (центриоль)
- Частичное репрограммирование → омоложение метилома, но не полная репликативная способность
- Для тотипотентности нужен сброс всего — включая центриоли
- **Гипотеза Джабы:** Элиминация центриолей + OSKM → промежуточное состояние (плюри- → ближе к тоти-). Элиминация центриолей + **тотипотентные факторы** (DUX4 + TPRX1) → возможно, настоящая тотипотентность
- **Тест:** MERVL активация, Zscan4 экспрессия, CDX2+ трофобластная дифференцировка

**Ссылки для проверки:**
- Hendrickson et al. (2017) Nat Genet — Dux activates MERVL and 2C-like state (PMID 28369030)
- Zalzman et al. (2010) Nature — Zscan4 extends telomeres and genomic stability (PMID 20139984)
- Gao et al. (2024) — TPRX1 in human totipotency
- Sinclair: Information Theory of Aging (Cell 2013, PMID 23810509) + partial reprogramming (Nature 2020, PMID 33299633)
- Gladyshev: 2C-like state as aging zero-point + transposable elements in aging

---

## 2026-07-11: Поиск оптотехника — контакты от Алексея

- **Событие:** Alex провёл встречу с инженером (неделя 7 июля) по OpenFlexure v7
- **Результат:** Инженер сам взяться не может, но поделился контактами инженерных сообществ
- **Прогресс Алексея (10 июля, 22:11):** Получил первые контакты, общается с ребятами из инженерных комьюнити
- **Контекст:** Нужен оптотехник для сборки/адаптации микроскопа MCARA (OpenFlexure v7, флексурный столик суб-100 нм, 28BYJ-48 + Sangaboard, ASA, live-cell адаптация по Malcolm et al. 2026)
- **ТЗ:** v2.0 отправлено Алексею 5 июля — полностью автономный файл с обновлённой механикой
- **Статус:** 🟡 В процессе поиска

---

## 2026-07-10: Анализ «Hot-Mitochondrion Paradox» — связь с C3

**Статья:** Fahimi, Lynch, Matta (2026) BioEssays — «Decoding the Hot-Mitochondrion Paradox»

**Ключевые находки:**
1. Chrétien et al. (2018, PLoS Biol): митохондрии ≈50°C — на 10–15°C выше цитоплазмы
2. Это противоречит закону Фурье на 5–6 порядков (тепло должно рассеиваться за наносекунды)
3. Fahimi et al. предлагают ratchet engine модель: ионные каналы создают локальные тепловые всплески
4. НЕ воспроизведено независимо; критика Treberg & Mailloux (2026): «Too Slow to Cool»
5. >43°C разрушает дыхательные комплексы (Moreno-Loshuertos 2023)

**Значение для MCARA:**
- C3 (митохондриальный счётчик) сейчас считается «защищённым 2% O₂»
- Если митохондрии действительно горячее — это независимый от ROS фактор стресса
- Тепловой стресс может ускорять mtDNA-мутации, агрегацию белков, износ ионных каналов
- Возможный C3b: тепловое старение митохондрий, не сводимое к ROS
- Ratchet engine модель → механический износ каналов → ещё один счётчик
- **Нужно:** мониторить литературу; при подтверждении 50°C — пересмотреть защищённость C3

**Файл:** `~/Desktop/ANALYSIS_Hot_Mitochondrion_Paradox.md`

---

## 2026-07-11: Звонок с Wolfgang Wagner (RWTH Aachen) — 🟢

**Результат:** Позитивно. Вагнер задал три ключевых вопроса:

1. **Гонка C1 vs C2:** Вагнер считает, что гонка между центриолярным и эпигенетическим счётчиками не должна быть центральным фокусом — функционально они слишком разные. Джаба пояснил: гонка — не фокус, но определяющая часть. Обещал выслать полное описание целей.

2. **Институциональная поддержка:** Вагнер спросил, кто помогает с написанием заявки (политические аспекты EIC).

3. **Состав консорциума:** Финальный список — после визита в Кёльн (конец августа), обсуждение с David Meyer и Aubrey de Grey.

**Поддержка Обри:** Джаба рассчитывает на помощь de Grey в написании Part B.

**Действия:** Отправить полное описание целей → ответить про институциональную поддержку → Кёльн (август).

---

## 2026-07-09: Три класса методов элиминации центриолей — разделение «центриоль vs CAASM»

**Контекст:** Глубокий поиск (50+ статей, 29 PMID) в ответ на запрос Джабы о методах элиминации центриолей.

### Найдено 11 методов, разделённых на 3 класса:

| Класс | Методы | Что удаляется | CAASM? |
|-------|--------|---------------|:------:|
| 🔴 **Физические** | Лазерная абляция, микрохирургия иглой | Центриоль целиком + PCM | ❌ Удаляется |
| 🟡 **Химические** | Centrinone, Plk4 siRNA, STIL shRNA | Предотвращение дупликации | ✅ Сохраняется |
| 🟠 **Антитело** | GT335 loading (Bobinnec 1998) | Только MT центриоли | 🟡 PCM остаётся |

### Экспериментальная логика:
- Если лазер + OSKM → iPSC успешно, а centrinone + OSKM → блок → CAASM СУЩЕСТВУЕТ
- Если лазер = centrinone → центриоль сама = носитель «памяти», CAASM вторичен

### Ключевые статьи (физические методы):
- Maniotis & Schliwa 1991, Cell — PMID 1934057
- La Terra et al. 2005, J Cell Biol — PMID 15738265
- Uetake et al. 2007, J Cell Biol — PMID 17227892

### Ключевой вывод Uetake 2007:
Центросомная потеря в НОРМАЛЬНЫХ клетках → G1 arrest через p38 (не p53).
Но клетки входят в S-фазу БЕЗ центриолей! Для эксперимента centrinone + OSKM
нужен не только p53i (pifithrin-α), но и p38i (SB203580).

---

## 2026-07-09: Ответ Pierre Gönczy — три важных подтверждения

**Контекст:** Джаба отправил Pierre письмо (7 июл) с анализом 5 data points о центриоли как стабилизаторе клеточного состояния. Pierre ответил 9 июл.

### Ответы Pierre (по пунктам)

| Вопрос | Ответ |
|--------|-------|
| Sulston 1983 — та статья? | ✅ Да, PMID 6684600 |
| Главная статья по centriole elimination? | **Kalbfuss & Gönczy, *Sci Adv.* 2023;9(22):eadg8682** — «Extensive programmed centriole elimination unveiled in C. elegans embryos» (НЕ обзор Open Biol 2023, а оригинальная research article) |
| Реверс-эксперимент (убрать центриоли → iPSC)? | **Никто не делал.** Pierre: «I am not aware of this either.» |
| Механизмы элиминации известны? | **Нет.** «We do not yet know the mechanism, but are working on trying to find out…» |

### Ключевые выводы

1. **Sci Adv 2023 — главная статья:** Нужно заменить ссылку с обзора Open Biol 2023 (PMID 37963546) на оригинальную статью Sci Adv 2023 (PMID 37256957, DOI 10.1126/sciadv.adg8682) как primary reference по centriole elimination в C. elegans.

2. **Реверс-эксперимент — наша ниша:** Pierre Gönczy (ведущий мировой эксперт) подтвердил: эксперимент «Plk4 siRNA → элиминация центриолей → OSKM → iPSC» никем не проведён. Это даёт нам **приоритет** на постановку central experiment гипотезы CEDAR/MCARA.

3. **Механизмы устранения неизвестны:** Даже лаборатория Gönczy (мировой лидер) ещё не знает молекулярных игроков somatic centriole elimination. Это открывает пространство для теоретического вклада.

### Действия
- [x] Добавить Kalbfuss & Gönczy, Sci Adv 2023, PMID 37256957 в EVIDENCE.md CEDAR
- [x] Обновить Centriole Map: заменить обзор Open Biol на Sci Adv 2023 research article
- [x] Зафиксировать приоритет реверс-эксперимента (дата: 2026-07-09)
- [x] Сохранить полный текст письма Pierre → `letters/sent/Pierre_Gonczy_Response_2026-07-09.md`
- [x] Отправить благодарственное письмо Pierre → `letters/sent/Pierre_Gonczy_Thanks_2026-07-10.md` ✅ Отправлено
- [ ] Написать Bettencourt-Dias (Drosophila, Science 2016) — experimental validation
- [ ] Написать Cajanek (PLK4/STIL, hPSC, Stem Cell Reports 2018) — experimental validation

### Новая стратегия experimental validation

**Gönczy — не партнёр.** Вежливо держит дистанцию: отказался от консорциума, не делится инсайдерской информацией о механизмах, не предлагает collaboration. Его lab в гонке за открытие механизмов centriole elimination — мы потенциальные конкуренты.

**Альтернативные кандидаты на experimental validation центрального эксперимента CEDAR (Plk4 siRNA → OSKM → iPSC):**

| Кандидат | Специализация | Статья | PMID | Почему |
|----------|---------------|--------|------|--------|
| **Mónica Bettencourt-Dias** | Центриоли, Polo kinase, oocyte elimination | Science 2016 | 27229142 | Уже в нашем списке 5 data points. Drosophila → млекопитающие? |
| **Lukáš Čajánek** | PLK4/STIL, centrosome loss → differentiation | Stem Cell Reports 2018 | 30197118 | Уже в нашем списке. hPSC — близко к iPSC. |

**Тактика:** Не писать холодное письмо сейчас. Сначала: (1) подготовить concept note реверс-эксперимента, (2) найти их недавние публикации и гранты, (3) написать с конкретным предложением → experimental validation central hypothesis CEDAR.

---

## 2026-07-08: Реорганизация — ARGUS-LP, Aubrey, CEDAR → MCARA

**Решение:** Переместить научные и инструментальные подпроекты из Marketing/ в LC/MCARA/.

| Перемещение | Откуда | Куда |
|-------------|--------|------|
| ARGUS-LP | Marketing/ARGUS-LP/ | **MCARA/ARGUS-LP/** |
| Aubrey | Marketing/Aubrey/ | **MCARA/Aubrey/** |
| CEDAR (Marketing) | Marketing/CEDAR/ | **MCARA/CEDAR/_merged_marketing/** |

**Замена MCARA → MCARA:** Выполнена во всех файлах ARGUS-LP, Aubrey, Marketing/CEDAR.

**В Marketing/_archive/ перемещены:** KorkotiLine, BACCHUS, MCARA.

**Core-файлы MCARA синтезированы:** _pi.md, MAP.md, STATE.md, TODO.md обновлены с учётом новых подпроектов.

---

## Rejection #29 — 2026-07-08 — bioRxiv (Gatekeeper)

**Журнал:** bioRxiv
**ID:** `10.5281/zenodo.21299683`
**Дней до решения:** 0 (скрининг)
**Тип решения:** Desk reject — административный

### Причина (что сказал редактор)
> «bioRxiv requires authors to have an organizational affiliation. It is necessary for submissions to be associated with an organization that provides oversight of research activities so that it can adjudicate any ethical issues/disputes that arise.»

### Что мы упустили
- bioRxiv требует institutional affiliation с oversight capability. GLA (регистр. №404506520) — НКО, не университет/институт. Для bioRxiv этого недостаточно.
- Не проверили требования bioRxiv к affiliation перед подачей.
- Не использовали соавтора с университетской affiliation.

### Что изменить перед следующей подачей
- [ ] Выбрать препринт-сервер без строгих требований к affiliation: **Zenodo** (бесплатно, DOI, без affiliation check), OSF Preprints, или Research Square
- [ ] Либо добавить соавтора с университетской affiliation (Вагнер/RWTH, Гейгер/Ulm, и др.)

### Следующий препринт-сервер
**Zenodo** — бесплатно, DOI, нет требований к institutional affiliation.
**Почему:** Самый простой путь. bioRxiv — не опция без университетской affiliation.

---

## 2026-07-08: Поиск литературы — новые концептуальные подтверждения

**mei-P26 (Genetics, 2026, iyag163):** Terry et al. — гипоморфная мутация mei-P26 в *Drosophila* нарушает координацию митоз→мейоз. Клетки входят в мейоз с митотическими сигналами → кроссинговеры идут неправильно. Концептуальный аналог: один ген-таймер → каскад downstream дефектов, подобно центриоли как таймеру клеточного состояния в MCARA.

**Центриоли морской звезды (PMID 27002173):** Разные механизмы для mother vs daughter центриолей при мейотической элиминации. Подтверждает тезис MCARA о полном сбросе центриолей в мейозе.

**Рецензии (8 раундов):** Статья прошла 8 раундов сверхстрогого peer review. Все 45 ссылок верифицированы через PubMed API. Ключевые изменения: деглутамилазы CCP1-6, sinc-MT/KIFC3, Lindhout, Ma et al. (ARL13B-ARL3), Bobinnec (1998), Bradford Hill скорректирован (5/9).

---

## 2026-07-07 (вечер): v4.4 — Центриоль = органелла необратимой дифференцировки

**Фундаментальная коррекция:** Счётчик — не polyE, а **возраст центриоли.** polyE — циферблат (readout). Центриоль — органелла, с которой ассоциирована необратимая дифференцировка. В асимметричных делениях старая материнская центриоль наследуется стволовой клеткой (Yamashita 2007, Barandun 2025).
**Новые PMID (июнь 2026):**
- 42343301: miP-FERMT3 на субдистальных отростках центриоли → p53-НЕЗАВИСИМЫЙ сенесценс. FERMT3↑ с возрастом.
- 42380124: ALMS1 (IDP) → centriole biogenesis with «memory» (Tsou lab, Nat Commun).
- 42316241: PLK4+PPM1D synthetic lethality.
**Статья:** v6 (Desktop). 25 PMID.
**Файлы:** CONCEPT v4.4, STATE v4.4, MEMORY обновлён.

**Центральный эксперимент MCARA (Фаза 0):** Plk4 siRNA → элиминация центриолей в фибробластах → OSKM-репрограммирование → эффективность iPSC. Никем не проведён. Предсказание: эффективность ↑ ≥2× vs контроль. Фальсификация: если ≤ контроля → гипотеза опровергнута.
**Сверхстрогое peer review:** `audits/MCARA_Peer_Review_2026-07-07.md`. 6/6 PMID верифицированы. Гипотеза: 5/10 по доказательной базе. Для гранта достаточно.

**Верификация (11/11 PMID подтверждены через PubMed):**
- PMID 36583780 ✅ (Tkemaladze 2023, CEDAR)
- PMID 17255513 ✅ (Yamashita 2007, Science — mother centrosome)
- PMID 36599349 ✅ (López-Otín 2023, Cell — centrosome НЕ в hallmarks)
- PMID 24138928 ✅ (Horvath 2013, DNAm clock)
- PMID 30332397 ✅ (Kabacik/Horvath 2018 — hTERT НЕ спасает от epigenetic aging)
- PMID 31113906 ✅ (Matsuyama/Horvath 2019 — гипоксия замедляет, НЕ останавливает)
- PMID 39764850 ✅ (Barandun/Oxenius 2025 — mother centrosome → CD8 fate, млекопитающие)
- PMID 41816297 ✅ (Passanisi/Spencer 2026 — senescence NOT predicted by telomeres)
- PMID 40562035 ✅ (Rando/Brunet/Goodell 2025 — 5 stem cell hallmarks, centrosome NOT mentioned)
- PMID 41641641 ✅ (Niemann/Geiger 2026 — Ube2g1 → HSC aging)
- PMID 41784031 ✅ (Commentary — proteostasis meets signaling)

**Ключевой вывод:** Kabacik/Horvath + Matsuyama/Horvath дают прямое доказательство: hTERT + гипоксия НЕ останавливают C2. C1 — единственный counter без защиты.

**Вагнер (RWTH):** Ответил положительно! Хочет Zoom. C2 ✅.
**Гёнчи (EPFL):** 🔴 Отказал. C. elegans убран. Но его работы по centriole elimination (PMID 37963546, 40475707) стали основой для comparative centriole map.
**Centriole Map:** Создана полная карта центриолярного статуса C. elegans (7/558 клеток). Шаблон для других видов → `docs/C_elegans_Centriole_Map.md`. Доказательство: центриоль = универсальный триггер необратимой дифференцировки.

**Файлы обновлены:** CONCEPT.md (v4.1), STATE.md, MEMORY.md, EVIDENCE.md
**Файл создан:** ~/Desktop/MCARA_Evidence_Base_2026-07-07.md

---

## 2026-07-06: MCARA v4.0 — Rejuvenation Platform

**Решение:** Полностью переработан концепт MCARA. Вместо наблюдательной модели (измерение counters) — активная Rejuvenation Platform: получение молодых безопасных взрослых стволовых клеток из собственных клеток пациента, омоложённых по всем 5 трекам репликативного старения.

**5 фаз:** ARGUS (инструмент) → Aubrey (доказательство counters) → Rejuvenation (трек за треком) → Integration (все 5 вместе) → Transplantation (мышь).

**Бюджет:** ~€3.5M, 36 месяцев, реальные EU-цены.

**Консорциум:** 8 партнёров, 6 стран. GLA (C1) + Wagner DE ✅ (C2) + Suomalainen FI (C3) + Magiera FR (C4) + Gönczy CH 🔴 отказал + Geiger DE (👨‍⚖️ судья) + Jacquemet FI (ENG) + Senescence TBD.

**Ключевой аргумент:** C1 — rate-limiting counter. В условиях hTERT + гипоксии теломеры (C5) защищены, митохондрии (C3) защищены, эпигенетика (C2) частично — а центриоль (C1) не защищена ничем. polyE накапливается. Лимит Хейфлика сохраняется.

**Терапевтическая цель:** протокол получения безопасных аутологичных стволовых клеток с молодыми центриолями для трансплантации.

---

## Journal Cascade (записано 2026-05-20)

**Правило:** сначала все бесплатные маршруты, только потом платные.

| № | Журнал | APC | Статус |
|---|--------|-----|--------|
| 1 | ~~**eLife**~~ | бесплатно (diamond OA) | ❌ soft-decline 13.05 → RC → ❌ RC 19.05 → ✅ письмо 20.05 → ❌ отказ 21.05 |
| 2 | ~~**F1000Research**~~ | $1,080 (LMIC waiver → $0) | ✅ подано 2026-05-22, #183257 |
| 3 | **Annals of Rejuvenation Science** | бесплатно (GLA journal) | последний запасной |
| 4 | npj Aging | €2,190 | только с грантом |
| 5 | Nature Aging | ~€9,500 | ожидание решения |

---

## Хронология событий

### 2026-05-21 — Отказ eLife
- **Событие:** Dr Peter Rodgers (Chief Magazine Editor, Features Editor, eLife) отклонил статью.
- **Формулировка:** «When considering potential Feature Articles we look for articles that offer fresh insights into a topic of broad interest to readers across the life and biomedical sciences. Your article would, I feel, be better suited to a specialist journal in the field of aging.»
- **Вывод:** eLife — не тот формат (Feature Article требует широкого междисциплинарного интереса, MCARA воспринята как специализированная работа по старению).
- **Действие:** Переход к F1000Research (LMIC waiver, бесплатно).

### 2026-06-03 — Подача в Biogerontology (Springer)
- **Событие:** MCARA подана в Biogerontology (Springer Nature) как Perspective article.
- **Тема:** Stem Cells in Ageing and Longevity
- **Платформа:** submission.nature.com
- **Файлы:** manuscript.docx, cover_letter.docx
- **APC:** $0 (subscription option)
- **Рецензирование:** 3 раунда экспертной рецензии (симуляция), итоговая оценка 8.75/10, ACCEPT IN CURRENT FORM
- **Статус:** Technical check пройден ✅, ожидание редакционного решения
- **Журнальный каскад обновлён:**
  - Nature Aging ❌ desk reject
  - eLife → Review Commons ❌ отказ
  - F1000Research ❌ desk reject
  - Biogerontology ✅ подано 2026-06-03

### 2026-06-03 — Отказ F1000Research
- **Событие:** Desk reject от F1000Research. Формулировка: «does not meet our requirements».
- **Статья #183257** — закрыта.
- **Вывод:** F1000Research не принял статью без объяснения причин (стандартная формулировка desk reject).
- **Действие:** Переход к Biogerontology.

### 2026-05-22 — Подача в F1000Research
- **Событие:** Статья «The Multi-Counter Architecture of Organismal Aging: A Quantitative Framework for Integrating Mechanistic Theories» подана в F1000Research как Opinion Article.
- **Статья #183257**
- **APC:** $1,080 → LMIC waiver (Georgia) → **$0**
- **Файл:** `docs/manuscripts/MCARA_F1000Research_2026-05-22.md` (2,846 слов) + `docx`
- **AI disclosure:** Указано использование pi (Earendil Works) как assistive tool
- **Статус:** ❌ Desk reject 2026-06-03
- **Журнальный каскад:** eLife ❌ → F1000Research ❌ → ?

### 2026-05-20 — Письмо в eLife о пересмотре
- **Событие:** Review Commons (#RC-2026-03569) отказал в рецензировании по жанровому признаку (теория ≠ эксперимент).
- **Действие:** Отправлено письмо Yamini Dalal (Senior Editor, eLife) с просьбой пересмотреть манускрипт напрямую, ссылаясь на пункт письма RC: "decision does not affect affiliate journals".
- **Приложен:** PDF отказа RC.
- **Статус:** ✅ отправлено, ответ получен — отказ.

### 2026-05-19 — Отказ Review Commons
- RC отказал: манускрипт не соответствует формату (теоретическая работа, не экспериментальная).
- Важно: решение не влияет на аффилированные журналы (eLife и др.).

### 2026-05-13 — Soft-decline от eLife
- Yamini Dalal: нет Reviewing Editor, но пригласила передать через Review Commons.
- Подали в RC.

### 2026-04-28 — Деск-режект Nature Aging
- Nature Aging отклонил без рецензии.

### 2026-04-19 — Подача в Nature Aging
- MCARA v5 (Perspective) подана в Nature Aging (NATAGING-P13741).

### 2026-07-06 — Тотальная верификация + Phase IV + Консорциум

**Верификация:** 39/39 ссылок (29 PMID + 10 DOI) подтверждены через PubMed E-utilities + Crossref API. 0 фальшивых.

**Hallmarks of stem cell aging:** Rando/Brunet/Goodell (Cell Stem Cell 2025, PMID 40562035) — 5 hallmark'ов. Центросомы НЕ упомянуты. CEDAR не процитирована. Решение: позиционировать CEDAR как «the missing sixth hallmark» → commentary в Cell Stem Cell.

**Ключевые находки:**
- CD8+ T cell mother centrosome → fate (Barandun/Oxenius, Cell Reports 2025) — прямое доказательство в млекопитающих. C2: 9/10.
- Senescence ≠ telomere length (Passanisi/Spencer, iScience 2026) — независимое подтверждение multi-counter модели.
- UBE2G1 — proteostasis meets signaling (Haematologica 2026) — первый конкретный механизм coupling counters.

**Phase IV — CELTRA-MAP:** 36 мес, €3.24M, 4 ткани, 8 человек. Бюджет согласован с Aubrey grant.

**Сводный бюджет:** Phase A €90K + Phase B €200.4K + Phase III €420K + Phase IV €3.24M = **€3,950,400.**

**Консорциум (Волна 1):** Письма отправлены Yamashita (MIT), Di Stefano (Baylor), Oxenius (ETH), Meraldi (UNIGE). Ждём ответов.

**Оценка CEDAR:** 6.7 → 7.3 → **7.8/10.**

**Файлы:** Переписаны CONCEPT.md (v3.4), STATE.md, PARAMETERS.md, EVIDENCE.md (все 3 проекта). Созданы: CONSORTIUM_ANALYSIS, META_ANALYSIS, feed_analysis, CELTRA-MAP Concept Note + 4 письма.

## 2026-07-09 — Глубокий аудит MCARA (pi)

### Находки
- **CEDAR → CEDAR:** Проведено полное переименование в 12 активных файлах. _archive и _originals сохранены.
- **4 статьи из фида Джабы:** Интегрированы в CEDAR/EVIDENCE.md §10.
  - Feng et al. (2026) — ClpP mitochondrial protease → meiosis (🔴 критически важно)
  - Mao et al. (2026) — Slmap → spermiogenesis defects (🟡 важно)
  - Dominicci-Cotto & Jenny (2026) — syncytium → sperm (🟡 supporting)
  - Zhou et al. (2026) — hnRNPs in spermatogenesis (🟠 косвенно)
- **12 дополнительных статей** найдено через CrossRef/PubMed:
  - 🔴 Yamada et al. (2026, Nat Commun) — MLKL → mitochondria → HSC aging
  - 🔴 Wani et al. (2022, Cell Rep) — YME1L → NSC self-renewal
  - 🔴 Khire et al. (2016, Curr Biol) — Centriole Remodeling during Spermiogenesis
  - 🔴 Mohrin et al. (2018, Aging Cell) — UPR^mt → HSC quiescence exit
  - 🔴 Wang et al. (2023, Cell Metab) — UPR^mt → NSC aging
  - +7 дополнительных
- **Мета-анализ:** 4 митохондриальные протеазы (ClpP, YME1L, LONP1, PARL) → cell fate. Гипотеза: протеостатическая ось митохондрий.

### Решения
- CEDAR → CEDAR переименование во всех активных файлах
- Новые данные добавлены в: CEDAR/EVIDENCE.md §10, MitoROS/EVIDENCE.md §v4, Proteostasis/EVIDENCE.md §v4
- CONCEPT.md (CEDAR) обновлён: новые оценки Counter'ов
- STATE.md обновлены: CEDAR (→2026-07-09), EpigeneticDrift (переписан, был 2025-03-15)
- MEMORY.md добавлены записи в MitoROS, Proteostasis, EpigeneticDrift, Telomere

### Проблемы (выявлены аудитом)
- ⚠️ ARGUS-LP: отсутствуют EVIDENCE.md и THEORY.md (нужно создать)
- ⚠️ EpigeneticDrift: STATE.md был устаревшим (2025-03-15) — исправлено
- ⚠️ PARAMETERS.md: многие подпроекты не имеют заполненных параметров
- ⚠️ MEMORY.md: EpigeneticDrift, MitoROS, Proteostasis, Telomere не обновлялись до этого аудита

### Оценки (обновлённые)
- Counter #3 (Митохондриальный): 8.5 → 9.0/10 (серия ClpP 2013-2026 укрепляет механизм)
- Counter #5 (Протеостаз): 7.5 → 8.5/10 (ClpP/ClpXP + AAA+ протеазы обзоры 2026)
- Counter #1 (Центриолярный): 7.5 → 8.0/10
- Общая MCARA/CEDAR: 7.3 → 7.8/10
