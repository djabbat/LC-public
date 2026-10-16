# CEDAR — Empirical Foundations

**Last verification date:** 2026‑10‑15
**Status:** Current as per CORRECTIONS_2026‑04‑22, updates 2026‑04‑25, 2026-07-29, 2026-10-14 and 2026-10-15 literature review.

## 🔥 NEW 2026-10-15 — Явно-электронные O(N²) движки (EVM) + жанр eFF — для масштабирования молекулярной симуляции CEDAR

> **Триггер:** письмо Ivaylo Anastasov 2026-10-15 (Anastasov_Theory, EVM). **Верификация:** Europe PMC + Crossref 2026-10-15. **Мастер-анализ:** `~/Desktop/Services/docs/ANALYSIS_2026-10-15_Anastasov_DeterministicPhysics.md`.

### Находка: EVM — дифференцируемый явно-электронный движок O(N²)
- **EVM** (Emergent Valence Mechanics): электроны как классические заряды, силы = Coulomb (1/r²) + стерическое ядро (1/r¹²) + RAKTS phase-exclusion (1/r⁶, только одинаковые фазы) + spin-pairing; интегрирование Velocity Verlet, всё в PyTorch → **полностью дифференцируемо**.
- Заявление: QM9 (~134k молекул) → 99.96% структурная стабильность, средний RMSD ~0.013 Å, «обход квантового бутылочного горла» (DFT O(N³–N⁴) → O(N²) на одной GPU).

### Двухсвидетельное подтверждение жанра (Noepedia-стандарт):
| Свидетель | Работа | PMID/DOI | Verified |
|---|---|---|---|
| 1. **eFF** (классическое поле с явными электронами) | Su & Goddard, *J Chem Phys* 2009 | 10.1063/1.3272671 | ✅ |
| 1b. eFF приложение | Kim, Su, Goddard, *PNAS* 2011 | 10.1073/pnas.1110322108 | ✅ |
| 1c. **Reactive eFF для углеводородов 2025** (свежая активность жанра) | Zhou Z et al., *JCTC* | 10.1021/acs.jctc.5c00633 (PMID 40548681) | ✅ |
| 1d. **Явно-электронный ML-потенциал** (мост eFF ↔ ML) | Cools-Ceuppens, Dambre, Verstraelen 2021, *JCTC* | 10.1021/acs.jctc.1c00978 (arXiv:2109.13111) | ✅ |
| 2. **ML force fields / дифференцируемая физика** (обучение NN на законах физики) | активная область 2025–2026 | 19+ работ (Europe PMC) | ✅ |
| 2b. **Дифференцируемые гибридные силовые поля 2026** (автономный поиск электролитов) | Wang X et al. 2026 | 10.1016/j.jechem.2026.05.049 (arXiv:2604.07979) | ✅ |

### Что это даёт CEDAR / MCARA
| Аспект | Значение |
|---|---|
| **Масштаб молекулярной симуляции** | Явно-электронный O(N²) дифференцируемый движок — кандидат для симуляции процессов старения (proteostasis, MitoROS, агрегация белков) за пределами DFT; жанр подтверждён eFF + ML-FF (два независимых направления). |
| **Дифференцируемость → AI-платформа** | Обучение NN прямо на физических законах движка согласуется с AI-интеграцией CEDAR/ARGUS-OS1 (симулятор ↔ машинное обучение в одном контуре). |
| **Бенчмарк-инфраструктура** | QM9-пайплайн EVM — готовый пример high-throughput валидации геометрий; можно использовать как независимый генератор стартовых конформаций. |
| **Эпистемический стандарт** | «Явные электроны» больше не аутсайдерская идея: рецензированный eFF (2009–2025) + критика 2026 — честная рамка «возможность + пределы». |

### Слабые места (мета-анализ) — критически важно
1. **QM9-тест EVM слабый:** 500 шагов с замороженными ядрами + 100 свободных; «99.96% стабильности» = RMSD ≤ 0.15 Å (молекула «не развалилась»), а НЕ предсказание энергий/свойств; **энергии не сравнивались**.
2. **Ручная инжекция «оптимальной статической фазы»** для CH₄/NH₃ = подгонка параметров, не эмерджентность; эксклюзивные радиусы (C/N/O/F=2.0) эмпирические; статические фазы без динамики спина.
3. **Жанр в целом под критикой 2026:** Ovad & Slavíček, «Electron force field for radiation chemistry? A critical assessment», *PCCP* 2026, DOI 10.1039/d6cp01937d (PMID 42751854) — дешёвый явно-электронный подход **не дотягивает по точности** для реактивной/радиационной химии.
4. EVM самопубликован (Zenodo), **не рецензирован**.

### Итог
- **Использовать EVM как:** быстрый генератор стабильных геометрий/конформаций + прототип дифференцируемой молекулярной физики; **НЕ как** источник энергий/спектроскопии (не доказано).
- **Цитировать жанр:** eFF (10.1063/1.3272671; 10.1073/pnas.1110322108; 10.1021/acs.jctc.5c00633) + ML-FF как два свидетеля идеи; критику (10.1039/d6cp01937d) — как честный предел.
- **Оценка силы:** идея жанра — K5 (два рецензированных направления); EVM-реализация — K2; применимость к aging-молекулярным симуляциям CEDAR — открытый вопрос (нужен пилот-тест на белок/агрегат).

### 🧪 Пилот EVM (ВЫПОЛНЕН 2026-10-15) — глицин + Gly-Gly: пептидная связь стабильна
> Скрипт + полный отчёт: `scripts/evm_pilot/cedar_pilot_evm.py`, `scripts/evm_pilot/REPORT.md`. Окружение: torch 2.13 CPU, rdkit 2026.09.1, движок автора как есть.

| Молекула | Своб. шагов | RMSD Å | Статус ≤0.15 | Пептидная C–N Å |
|---|---|---|---|---|
| Контроль (H₂O)₂ | 1000 | — | ✅ O–O = 2.747 (эмп. ~2.78–2.82) | — |
| **Глицин** (40 e⁻) | 500 | **0.028** | ✅ STABLE | 1.48→1.46 |
| **Gly-Gly** (70 e⁻) | 500 | **0.101** | ✅ STABLE | 1.44–1.48 (эмп. амид ~1.33–1.46) |

**Что даёт:** пептидная связь/аминокислоты стабильны в явно-электронном движке на 500 свободных шагах (тест в 5× жёстче QM9-протокола авторов) → EVM пригоден как **быстрый фильтр стабильности + генератор стартовых конформаций** для пептидных фрагментов (протеостаз).

**Слабые места из пилота:** (1) RMSD Gly-Gly растёт линейно (~0.0002 Å/шаг) → **не для долгих MD**; (2) пептидная C–N длиннее эмпирической → **амидный резонанс не воспроизводится** (предел классических явных электронов, совпадает с критикой 2026); (3) статические фазы → не для переходных состояний; (4) движок не рецензирован.

**Следующий шаг (TODO):** пилот на коротком пептиде 3–5 остатков / Aβ(16–22) как минимальная модель агрегации + сравнение с GROMACS.

## 🔥 NEW 2026-10-14 — Whole-Mammal Lineage Tracing: DNA Typewriter (Science) + PEtracer (Cell) + 2-cell asymmetry (Cell 2024)

> **Триггер:** статья iXBT Live (2026-10-14) «От оплодотворенной клетки до органов». **Все PMID верифицированы NCBI E-utilities 2026-10-14.**
> **Полный разбор:** `docs/ANALYSIS_2026-10-14_WholeMammal_LineageTracing.md` · refs: `CellLineageTree/refs/`.

### Ключевые работы

| # | Работа | Журнал | PMID/DOI | Verified | Сила |
|---|--------|--------|----------|:--------:|:----:|
| 1 | **Yu Q et al. (Shendure, UW)** — DNA Typewriter: филогения **1,28 млн** клеток мыши из **одного** эмбриона, зигота→поздний органогенез (~13,5 дн); маркировка дочерей 1-го деления; **клональная доминантность при гаструляции** | Science, 2026-10-08 | **42848909** / 10.1126/science.ael0508 | ✅ | **Strong** |
| 2 | **Colgan WN et al. (Weissman, Whitehead)** — **PEtracer**: ~75% делений, **>1,4 млн клеток**, **16 эмбрионов**, E7.5–E10.0 | Cell, 2026-10-08 | **42849436** / 10.1016/j.cell.2026.09.050 | ✅ | **Strong** |
| 3 | **Junyent S et al. (Zernicka-Goetz)** — **Человек**: большинство эпибласта из одного бластомера 2-клеточной стадии; bottleneck интернализации 8→16 | Cell, 2024-05-23 | **38744282** / 10.1016/j.cell.2024.04.029 | ✅ | **Strong** |
| 1b | Препринт DNA Typewriter (in vivo reconstruction) | bioRxiv 2026-07-30 | **42619846** | ✅ | препринт |
| 2b | Препринт PEtracer | bioRxiv 2026-07-17 | **42146547** | ✅ | препринт |

### Что это даёт CEDAR / CDLT

| Аспект | Значение |
|---|---|
| **Осуществимость lineage-реконструкции млекопитающего** | Полная филогения 1,28 млн клеток из одного эмбриона (Shendure) + 16 эмбрионов/~75% делений (Weissman) — прямой внешний прецедент масштаба платформы CDLT |
| **Каскадная асимметрия (Jaba, 2026-07-05) — прямое человеческое подтверждение** | 2-клеточная стадия уже асимметрична: большинство эпибласта из одного бластомера; bottleneck 1–3 клеток на 8→16 → траектория развития в целом асимметрична, не только отдельные деления |
| **Пластичность развития имеет границы** | Zernicka-Goetz (статья iXBT): «теперь можно исследовать границы пластичности развития» — ранний эмбрион не полностью эквипотенциален → согласуется с «центриоль как морфогенетический замок» (CONCEPT §1) и «точкой невозврата» |
| **Bottleneck интернализации** | 1–3 клетки на 8→16 (человек): повреждение одной клетки может клонально экспандировать в тело → место концентрации центриолярного счётчика |
| **Power Analysis CDLT** | Weissman n=16/~75% покрытие → реконструкция содержательна при неполном покрытии; CDLT нужны ≥2 эмбриона/условие для вариабельности |
| **Комплементарность методов** | Оптический (CDLT/Aubrey, живой) vs молекулярный (DNA Typewriter/PEtracer, постфактум) — два независимых свидетеля lineage (эпистемический стандарт Noepedia) |
| **Маркировка первого деления** | DNA Typewriter: burst редактирования помечает дочерей 1-го деления → прототип отслеживания асимметрии мать/дочь in vivo |

### Слабые места (мета-анализ)
1. **Shendure: n=1 эмбрион** для полной реконструкции — межиндивидуальная вариабельность не оценена.
2. **Weissman: ~75% делений, окно E7.5–E10.0** — не от зиготы; ~25% ветвей не разрешены.
3. **Методы разные (DNA Typewriter vs PEtracer), окна разные** — нет прямого кросс-сравнения; нужна отдельная работа.
4. **Молекулярный, постфактум** (parsimony-дерево, не живая динамика) — CDLT-оптический комплементарен, нужна калибровка.
5. **Нет центриолярной информации в обоих методах** — это и есть пробел, который заполняет CDLT (opportunity).
6. **Механизм клональной доминантности не объяснён** (стохастика vs детерминизм) — открытое окно для центриолярного объяснения CEDAR.
7. **Человек: малое n, этика, частично ретроспективно** (Zernicka-Goetz 2024) — подтверждающая роль.
8. **Препринты (42619846, 42146547)** — не рецензированы; цитировать журнальные версии.

### Итог
- **CEDAR Counter #1:** 8.8/10 (без изменений) + внешний прецедент масштаба lineage-реконструкции (Science 2026 + Cell 2026, оба 2026-10-08).
- **Каскадная асимметрия** поддержана прямым человеческим доказательством (38744282).
- **Пластичность развития имеет границы** — согласуется с «центриоль как морфогенетический замок».

---

## Phylogenetic Analysis: Centriole Elimination Across Mammals (2026-08-03)

> **Finding:** Systematic review of 15+ mammalian species reveals three distinct patterns of centriole elimination, refuting the critique that placental mammals universally lost this mechanism.

| Pattern | Species | Timing | Transferable to soma? |
|---------|---------|--------|:---:|
| **Rodent-type** | Mouse, rat, hamster | No elimination — maternal inheritance | ❌ |
| **Primate-type** | Human, rhesus macaque | Pre-meiotic elimination | ⚠️ Unknown mechanism |
| **Ungulate-type** | Cattle, pig, sheep, rabbit | Post-fertilization elimination during mitosis | ✅ Best model |

**Key reference:** Uzbekov R, Avidor-Reiss T (2024) The proximal centriole age in spermatozoa determines its fate in the zygote. Open Biol 14:230458. PMID 38442864. [Older centrioles preferentially eliminated — age-dependent selection.]

**Implication for CEDAR/MCARA:** The ungulate-type mechanism (post-fertilization, mitotic) is the most directly transferable to somatic cell reprogramming. Human elimination factors exist (primate-type) but are uncharacterized. Both branches confirm that centriole elimination is present in placental mammals — it was never lost, just diversified.


## 🔥 NEW: Centriole–Autophagy Interface (2026)

### Coelho, Yu & Glover (2026) — TIAM1/PLK4/центриоли/аутофагия
- **DOI:** `10.64898/2026.07.02.735969` | bioRxiv, Caltech
- TIAM1 (RAC1 GEF) — интерфейс между сборкой центриолей и аутофаголизосомальной системой
- TIAM1 depletion → abnormal PLK4 distribution + enlarged LAMP1 compartments
- **Вывод:** контроль качества центриолей через лизосомы — новый механизм для MCARA

### Sensing Centrosome Amplification (2026)
- **PMID:** `42324259` | Nat Commun, 2026 Jun 21
- Интерфейс между центриольной дупликацией и аутофагией

### Autophagy Controls Centrosome Number (2017)
- **PMID:** `28209922` | Oncotarget, 2017
- Аутофагия регулирует число центросом

---

## 1. Supporting Evidence (support for axioms and mechanisms)

[Same tables as v5.2 – unchanged.]

## 2. Internal Data (simulations, calibration)

[Same as v5.2 – unchanged.]

## 3. Disconfirming Evidence and Limitations (honest assessment)

1. **Lack of direct C1+C2 data for HSC:** Critical. No publication demonstrates simultaneously (C1) linear PTM increase on HSC centrioles with division number *in vivo* and (C2) asymmetric inheritance of this PTM‑loaded centriole in HSC. This is the main blocking barrier and the target of this proposal.
2. **Alternative explanations for HSC slowdown:** Cell cycle lengthening may be due to systemic factors (niche, inflammation) rather than intrinsic centriolar damage. The proposal includes experiments to isolate cell‑autonomous effects (e.g., CCP1 KO).
3. **Measurement difficulty:** Quantitative measurement of polyglutamylation on a single centriole in rare HSC populations is challenging. We will use super‑resolution microscopy (STED) and the validated GT335 antibody.
4. **Uncertainty of epigenetics link:** Previously the Sobol analysis suggested epigenetic dominance. This has been resolved by the mechanistic coupling model (see CONCEPT.md §2026‑04‑25 update). The coupling will be tested on real data.
5. **Limited tissue coverage:** Strongest C2 evidence obtained for neural progenitors and lymphocytes. The present proposal extends to HSC, the model of blood aging.
6. **Lack of own preliminary experimental data.** The current proposal does not include any preliminary data generated by the applicant. This is both a limitation and a motivation: the budget is designed to generate the first direct evidence for C1 and C2 in HSC. The PI’s prior intramural pilot work validated protocols (GT335 staining in mouse bone marrow) but did not produce definitive quantitative data. The ninein‑asymmetry and PTM‑division correlation data are the primary deliverables of Aims 1 and 2.

7. **SAS-6/картуэлл не персистентен (Meyer review, 2026-08-13).** Разбирается при созревании/митозе (Huang 2025, PMID 39614048) и теряется при цилиогенезе (Li 2017, PMID 28743734; Serwas 2017, PMID 28411189). Мишени: триплет-стенка, внутренний скэффолд (Le Guennec 2020, PMID 32110738), appendage-кольца. Прямой тест: pulse-labeling мать vs дочь (Фаза 0).

**Risk mitigation:** The team includes Dr. X (Yale) and Dr. Y (Stanford), who have published on centriole inheritance and HSC transplantation, respectively. Their letters of support confirm they will provide expert guidance and share validated protocols. This reduces the risk of technical failure.

## 4. Sample Size and Power Analysis

We performed formal power calculations for the two primary falsification tests (FT1.1 and FT1.2) using standard methods.

### FT1.1: Spearman correlation between PTM level and division number (P1)
- Effect size: expected ρ = 0.6 (based on indirect evidence from neural progenitors and the theoretical necessity of a strong correlation).
- Significance level α = 0.05 (two‑sided).
- Power = 0.80.
- Fisher z‑transformation yields required n ≥ 20 independent HSCs (exact calculation: n = (Z_α/2 + Z_β)² / (0.5·ln((1+ρ)/(1‑ρ)))² + 3 = (1.96+0.84)² / (0.5·ln(1.6/0.4))² + 3 ≈ 19.3). To account for technical failures (e.g., incomplete centriole labeling) and multiple testing across two age groups (young and old), we will collect 30 HSCs per age group, providing >90% power to detect ρ ≥ 0.6.

### FT1.2: Binomial test for asymmetric inheritance (P2)
- Null hypothesis: probability of mother centriole inheritance by stem daughter = 0.5 (symmetric).
- Alternative: p = 0.7 (expected minimum).
- α = 0.05 (one‑sided).
- Power = 0.80.
- Using exact binomial power calculation (binom.test), required n = 31 division events. We will image at least 40 divisions per age group to compensate for unusable observations.

These sample sizes are achievable with the 30+30 mice budgeted (approximately 20–40 HSC divisions can be recorded per mouse after enrichment). Pre‑registration will include these targets.

## 5. Risk Matrix

| Risk ID | Risk Description | Likelihood | Impact | Mitigation |
|---------|-----------------|------------|--------|------------|
| R1 | GT335 staining fails to detect polyGlu in HSC centrioles due to low abundance | Medium | High | Use validated protocol from pilot; include positive controls (brain sections); test STED to enhance signal. |
| R2 | HSC yield from mouse bone marrow insufficient for planned sample size | Low | High | Use 30+30 mice; isolate from pooled femur/tibia; if low, repeat isolation from additional animals (contingency budget). |
| R3 | Asymmetric inheritance (C2) not observed in HSC (Ninein positive) | Low | Critical | Alternative: use CEP170; if still negative, consider that HSC may differ from neural/T‑cells; then CEDAR would be falsified for blood. |
| R4 | Live‑cell imaging of HSC divisions causes phototoxicity, affecting division outcome | Medium | Medium | Use low‑light imaging; limit acquisition to first two divisions; validate with brightfield controls. |
| R5 | CCP1‑KO mice show embryonic or early postnatal lethality | Low | High | Use conditional KO (Vav‑iCre); Dr. Y has experience; back‑up: inducible KO. |
| R6 | Pre‑registration delays due to IRB/IACUC approval timing | Medium | Low | Submit IACUC protocol immediately after award; pre‑register study design on OSF before data collection. |

## 6. New Literature — Asymmetric Male Meiosis & Centriole Inheritance (2026-07-05)

> Полный список: `~/Desktop/PhD/docs/literature_search_2026-07-05.md`
> Основная статья: Meng X, Baird RB, **Yamashita YM** — *Asymmetric male meiosis and its implications in heredity* — Curr Top Dev Biol 168:211–243 (2026) — DOI: `10.1016/bs.ctdb.2026.01.005` — PMID: 42097813

### Статьи, напрямую релевантные для CEDAR (C1/C2 + центриоли + асимметрия)

| # | Article (кратко) | Journal | Год | OA | Value для CEDAR |
|---|------------------|--------|-----|:--:|---------------------|
| 1 | **Thomas A, Meraldi P** — Centrosome age breaks spindle size symmetry even in «symmetrically» dividing cells | J Cell Biol | 2024 | ✅ | **Прямое доказательство:** возраст центросомы → асимметрия веретена → поддерживает C2 |
| 2 | **Royall LN et al.** — Asymmetric inheritance of centrosomes maintains stem cell properties in human NPCs | eLife | 2023 | ✅ | **C2 в нейральных прогениторах человека** — PMID: 37882444 |
| 3 | **Skinner MW et al.** — Meiotic divisions and round spermatid formation do NOT require centriole duplication in mice | PLoS Genet | 2025 | ✅ | Мейоз без центриолей → подрывает догму обязательности центриолей |
| 4 | **Skinner MW et al.** — Spermatocytes segregate chromosomes despite centriole duplication failure | EMBO Rep | 2024 | ✅ | Сегрегация хромосом без дупликации центриолей |
| 5 | **Chen C, Yamashita YM** — Centrosome-centric view of asymmetric stem cell division | Open Biol | 2021 | ✅ | Обзор асимметрии центросом в стволовых клетках — PMID: 33435817 |
| 6 | **Salzmann V, ..., Yamashita YM** — Centrosome-dependent asymmetric inheritance of midbody ring in Drosophila GSC | Mol Biol Cell | 2014 | ✅ | Центросомно-зависимая асимметрия в GSC — PMID: 24227883 |
| 7 | **Segura RC et al.** — Asymmetry of centrosomes in Drosophila neural stem cells requires PP4 | Mol Biol Cell | 2025 | ✅ | Механизм: PP4 требуется для асимметрии центросом |
| 8 | **Bonente D et al.** — Inactivation and Elimination of Centrioles During Development in Drosophila | Cells | 2025 | ✅ | Элиминация центриолей в развитии — PMID: 40558492 |
| 9 | **Zhao X et al.** — PCM1 coordinates centrosome asymmetry with polarized endosome dynamics | Nat Commun | 2025 | ✅ | PCM1 → асимметрия центросом → судьба дочерних клеток |
| 10 | **Gönczy P, Balestra FR** — Sperm-contributed centrioles segregate stochastically into blastomeres of C. elegans | Genetics | 2023 | ✅ | Стохастическая сегрегация центриолей сперматозоида — PMID: 36988082 |
| 11 | **Bener MB, Inaba M** — Stem cells resume asymmetric division upon niche re-entry through centrosome orientation checkpoint | Commun Biol | 2026 | ✅ | Центросомный чекпоинт ориентации → асимметричное деление |
| 12 | **Al-Yazeedi T et al.** — Lagging X chromatids specify asymmetric organelle partitioning in XX spermatocytes | Genetics | 2022 | ✅ | Асимметричное распределение органелл в мужском мейозе |

### Основная статья (Meng/Baird/Yamashita 2026) — ключевые положения для CEDAR

1. **Мужской мейоз может быть скрыто асимметричным** — даже у видов с «симметричным» сперматогенезом
2. **Asymmetry мужского мейоза → фундамент для meiotic drive** (нарушение Менделевского расщепления)
3. **Полярные тельца** могут формироваться факультативно в мужском мейозе
4. **Латентная асимметрия** может быть использована для адаптивного смещения наследования
5. **125 references** — золотая жила для CEDAR (Akera, Chmatal, Lindholm, Pelletier, Herbette...)

## 7. Stem Cell Exit & Epigenetic Barriers (2026-07-05)

> **Основная статья:** Park EJ, Levin-Ferreyra F, **Di Stefano B** — *Mechanisms coordinating exit from the stem cell state in mammals* — Genes Dev 40:982-1011 (2026) — DOI: `10.1101/gad.353584.125` — PMID: **42156139** — PMCID: PMC13267984 — 🔓 CC BY-NC 4.0

### 7.1. Ключевые находки

| # | Article (кратко) | Journal | Год | OA | PMID | Value для CEDAR |
|---|------------------|--------|-----|:--:|------|---------------------|
| 1 | **Park EJ, Di Stefano B** — Mechanisms coordinating exit from stem cell state | Genes Dev | 2026 | ✅ | 42156139 | 🔑 **Основной обзор:** 5 уровней stem cell exit. «Точка невозврата» — ключевой вопрос |
| 2 | **Strawbridge SE et al.** — Exit from naive pluripotency WITHOUT asymmetric division | Stem Cell Reports | 2026 | ✅ | 41687620 | ⚠️ **КРИТИЧЕСКОЕ ОГРАНИЧЕНИЕ** — см. подробный анализ ниже |

### 🔬 Детальный разбор: Strawbridge et al. (2026) — границы применимости CEDAR

**Strawbridge SE, Corridori C, Blanchard GB, Smith A, Kugler H, Martello G** — *Exit from naive pluripotency proceeds with variable latency but without asymmetric division to generate population heterogeneity*

- **Journal:** Stem Cell Reports, Vol. 21 (2026) | **DOI:** `10.1016/j.stemcr.2026.102813` | **PMCID:** PMC12985371
- **Лабы:** Austin Smith (Exeter) — создатель системы 2i/LIF; Graziano Martello (Padua)
- **Метод:** Rex1-GFPd2 репортёр наивной плюрипотентности + long-term single-cell imaging (LTSCI) + генеалогический анализ

**6 ключевых результатов:**

| # | Результат | Value для CEDAR |
|---|----------|---------------------|
| 1 | **Нет ре-экспрессии репортёра** — дифференцировка необратима | 🟢 Consistent: CEDAR предсказывает необратимость через накопление повреждений |
| 2 | **Только симметричные деления** — ни одного асимметричного. Сестринские клетки выходят синхронно | 🔴 **Ограничение:** в ES-клетках нет асимметричного наследования → CEDAR не универсальна |
| 3 | **Коллапс наивной идентичности резкий (abrupt)** — переключатель, не плавный процесс | 🟡 Model «таймер + переключатель»: центриоль может быть таймером, но не переключателем |
| 4 | **Вариабельная задержка 0–3 поколения** перед abrupt collapse | 🟡 Что определяет задержку? Повреждения центриоли — кандидат |
| 5 | **Разброс >15 часов** между клетками популяции | 🟡 Асинхронный выход — популяционная стратегия (safeguard pluripotency) |
| 6 | **Не зависит от клеточного цикла** | 🟡 CEDAR связывает счётчик с делениями; здесь связь с циклом отсутствует |

**Таксономия применимости CEDAR:**
- ✅ Взрослые стволовые клетки (HSC, NSC, GSC, ISC) — асимметричное деление доказано → CEDAR применима
- ❌ Эмбриональные стволовые клетки (ES cells) — выход без асимметрии → CEDAR **не применима**
- ⏳ Плюрипотентные СК in vivo (эпибласт) — вопрос открыт (Strawbridge: in vitro)

**Вывод:** CEDAR не универсальна, но это не фатально. Для диссертации важно честно ограничить область применимости: **взрослые стволовые клетки с асимметричным делением.**

### 🧠 Контраргумент Джабы (2026-07-05): два уровня асимметрии

Strawbridge показал отсутствие асимметрии только на уровне **одного деления** в одной системе (ES-клетки, 2i/LIF, naive→formative). Но **траектория дифференцировки в целом асимметрична:**
- Тотипотентная зигота → плюрипотентные → мультипотентные → унипотентные
- Каждый шаг — потеря потенциала, разные судьбы дочерних популяций
- Это **каскадная асимметрия**, не сводимая к одному делению

**CEDAR не требует, чтобы каждое деление было асимметричным.** Она утверждает, что накопление центриольных повреждений на длинной дистанции создаёт разницу между клетками. Variable lag (0–3 поколения) у Strawbridge — окно для накопления таких повреждений перед abrupt collapse.
| 3 | **Espinosa-Martínez M et al.** — Molecular basis of cell memory: epigenetic cycle | Sci Adv | 2024 | ✅ | 38416817 | Механизм клеточной памяти — альтернатива центриольному счётчику |
| 4 | **Silvério-Alves R et al.** — GATA2 mitotic bookmarking for haematopoiesis | Nat Commun | 2023 | ✅ | 37580379 | Bookmarking в HSC — передача идентичности через митоз |
| 5 | **Palma LG et al.** — Chromatin activity of IκBα mediates exit from naïve pluripotency | eLife | 2025 | ✅ | 41123589 | Хроматин-опосредованный выход из плюрипотентности |
| 6 | **Levin-Ferreyra F et al.** — TE activity captures human pluripotent cell states | EMBO Rep | 2025 | ✅ | 39668246 | Тот же автор (Di Stefano lab) — транспозоны как маркеры |
| 7 | **McCreery KP et al.** — Mechano-osmotic signals control chromatin state in PSCs | Nat Cell Biol | 2025 | ✅ | 41023488 | Механо-осмотическая регуляция хроматина |
| 8 | **Ma B et al.** — Asymmetric histone inheritance in olfactory stem cells | Nat Commun | 2026 | ✅ | 41872193 | Asymmetry гистонов → судьба клеток |

### 7.2. Ключевые положения Park/Di Stefano (2026) для CEDAR

1. **5 уровней stem cell exit** — CEDAR (центриольный счётчик) находится на стыке: центриоль → сигналинг → хроматин → судьба
2. **Pol II паузинг** — промотор-проксимальное накопление Pol II на генах lineage specification → механизм «быстрого ответа» при выходе
3. **Tfe3/Folliculin** — ядерно-цитоплазматическая локализация TF: «Контроль локализации TF вместо экспрессии может обеспечить гибкость» — аналогия с асимметрией центросом?
4. **CTCF/TAD структуры** — поддержание плюрипотентности через 3D-геном — при выходе структуры перестраиваются
5. **Бивалентные гены (H3K27me3/H3K4me3)** — Yagi et al. (2025): потеря H3K4me3 → блок гемопоэза

### 7.3. Новый препринт Tqemaladze (2026)

- **Tqemaladze J** — *The Centrosomal Ledger—a Unified Model of Structural...* — DOI: `10.20944/preprints202601.1235.v1`

### 7.4. CAASM Evidence — центросома как платформа индукторов дифференцировки (2026-07-05)

> **Status:** 🟡→🟢 M3 upgraded from hypothetical (2/10) to plausible with evidence (5/10).

| # | Белок | Role | Connection с центросомой | PMID | OA |
|---|-------|------|---------------------|------|:--:|
| 1 | **Oct4 (POU5F1)** | Мастер-регулятор плюрипотентности | **Физически локализуется на центросоме в митозе** — нетранскрипционная функция: сборка веретена и сегрегация хромосом | **41725553** | ✅ |
| 2 | **CEP170** | Центросомный белок | **Связывает центросомную функцию с кортикальным развитием мозга** — новый механизм нейрогенеза | **41888776** | ✅ |
| 3 | **LGALS3BP** | Белок межорганеллярных контактов | **Связывает центросомы и митохондрии** — поддерживает энергетический фитнес через центросомно-митохондриальный контакт | **42055624** | ✅ |
| 4 | **PCM1** | Белок перицентриолярного материала | **Координирует асимметрию центросом с динамикой эндосом** → детерминация судьбы дочерних клеток | **41315244** | ✅ |
| 5 | **NuSAP–CEP57–CEP152** | Комплекс центриолярного engagement | NuSAP safeguard — **целостность центриоли → правильный engagement → функция центросомы** | **41616107** | ✅ |
| 6 | **Первичная цилия** | Сигнальный хаб | **Цилия как ворота TGF-β суперсемейства** — центриоль→базальное тельце→цилия→TGF-β/BMP (Herrera-Cid 2026, PMID: **42233350**); обзор: цилии как хабы сигнальной трансдукции (Li 2025, PMID: **41310849**) | **42233350**, **41310849** | ✅ |

**Value:** CAASM — не одна структура, а **коллекция белков**, физически ассоциированных с центросомой и контролирующих клеточную судьбу.

### 7.5. Decisions проблем Peer Review (2026-07-05)

> Полный документ: `docs/SOLUTIONS_2026-07-05.md`. Аудит: `docs/PEER_REVIEW_2026-07-05.md`.

| Проблема | Решение | Status |
|----------|---------|:------:|
| M3/CAASM — гипотетический | 5 белков + 2 сигнальных обзора с centrosome localization evidence | 🟢 |
| Strawbridge (2026) — ES без асимметрии | Таксономия: CEDAR для взрослых СК; ES — отдельный механизм | 🟢 |
| Causality не установлена | CCP1-KO эксперимент (P3) — дизайн готов | 🟡 (требует funding) |
| Tissue-specificity | Таблица 8 тканей с n*, α, β | 🟢 |
| Γ matrix — placeholder | Метод парных perturbation experiments | 🟡 |
| ¬R reversibility | iPSC-репрограммирование → тест сброса polyGlu | 🟡 |
| D_critical пороги | Model тканевых порогов толерантности | 🟢 |

**Общая оценка CEDAR: 6.7/10 → 7.5/10**

### 7.6. Модуляция vs необратимая дифференцировка — модель желудка (2026-07-05)

> **Джаба:** «Разница между модуляцией (временное и обратимое изменение морфологии) и необратимой дифференцировкой — стволовые клетки в стенке желудка.»

**SPEM (Spasmolytic Polypeptide-Expressing Metaplasia):** chief cells желудка обратимо трансдифференцируются в SPEM — это модуляция, а не необратимая дифференцировка. Ключевые статьи:

| Article | Год | PMID | Суть |
|--------|-----|------|------|
| **Divenko M, Mills JC** — Stomach at the crossroads: NR signaling | 2026 | **41732976** | Интерфейс модуляции/дифференцировки в желудке |
| **Cornish SK, Choi E, Mills JC, Goldenring JR** — Origins of pyloric metaplasia | 2026 | **42319154** | Chief cell → SPEM как модель обратимой трансдифференцировки |
| **Zhang T et al.** — SPEM: hidden linchpin in gastric carcinogenesis | 2025 | **41347923** | SPEM beyond Correa cascade |
| **Li X et al.** — Cellular lineage origins of SPEM | 2025 | **41195276** | Дебаты о происхождении SPEM |

**Предсказание CEDAR:** В обратимой модуляции (chief→SPEM) центриолярные повреждения не достигают D_critical. При хроническом воспалении D_c накапливается → модуляция становится необратимой (SPEM→неоплазия). Желудок — идеальная модель для тестирования: сравнить GT335 на центриолях chief cells при острой (обратимой) и хронической (необратимой) SPEM.

### 7.7. Топ-статьи из рекомендаций Semantic Scholar (2026-07-05)

**Центросома как сигнальный хаб (усиление M2/M3):**

| Article | Journal | Год | Value |
|--------|--------|-----|----------|
| **Pan M et al.** — Centrosome-Signaling Pathway Crosstalk | Cytoskeleton | 2025 | Центросома ↔ сигнальные пути |
| **Kiermaier E et al.** — Amplified centrosomes—more than just a threat | EMBO Rep | 2024 | Центросомы как сигнальные хабы |
| **Garcia-Carpio I et al.** — Extra centrosomes induce PIDD1-mediated inflammation | EMBO J | 2023 | Центросомы → воспаление |
| **Purkerson MM et al.** — Extra centrosomes as signaling hubs | Cancer Metastasis Rev | 2024 | Центросомы = сигнальные платформы |
| **Coelho PA et al.** — Sensing centrosome amplification: centriole duplication and autophagy | Nat Commun | 2026 | Центросомы → аутофагия |

**Клеточные переходы состояний:**

| Article | Journal | Год |
|--------|--------|-----|
| **Kukreja K et al.** — Cell state transitions decoupled from cell division | Nat Cell Biol | 2024 |
| **González-Suárez P, Smit ME** — Flexibility or commitment: division vs differentiation | J Exp Bot | 2025 |
| **Yamashita YM** — Asymmetric Stem Cell Division and Germline Immortality | Annu Rev Genet | 2023 |

**Центриоли и РНК:**

| Article | Journal | Год |
|--------|--------|-----|
| **Fang J et al.** — PCM scaffold enables RNA localization to centrosomes | Mol Biol Cell | 2025 |
| **Martinez A, Pearson CG** — Centriolar satellites regulate CEP350 mRNA localization | bioRxiv | 2026 |
| **Pachinger C et al.** — Centriolar satellites: translation of centrosomal/ciliary proteins | J Cell Biol | 2025 |

### 7.11. Markerы центриолярной энтропии (2026-07-05)

> polyGlu — не сама энтропия, а **признак** накопленной энтропии. Более старые центриоли имеют больше polyGlu. Дополнительные маркеры:

| Marker | Изменение с возрастом | Value |
|--------|----------------------|----------|
| **PolyGlu (GT335)** | ↑ | Признак структурной дезорганизации центриолярных микротрубочек |
| **Длина центриоли** | ↑ | Центриоли удлиняются с возрастом — прогрессирующая структурная дезорганизация |
| **Длина первичной цилии** | ↓ | Цилия укорачивается → нарушение сенсорного сигналинга (Hh, Wnt, TGF-β) |

**Hardware vs Software:** Центриоль — **hardware** (физическая структура, не ремонтируется, только дуплицируется или элиминируется). Эпигенетика — **software** (перепрограммируется факторами Яманаки). Это объясняет, почему частичное репрограммирование омолаживает эпигеном, но не восстанавливает репликативную ёмкость — hardware накапливает энтропию, которую software-патчи не исправляют.

**Почему клетка не может выбросить старую центриоль?** Стволовые клетки нуждаются в центриолях для деления. Постмитотические клетки (нейроны, кардиомиоциты) сохраняют центриоли как базальные тельца первичных цилий, необходимых для сенсорного сигналинга. Элиминация центриолей происходит только при гаметогенезе и в нескольких специализированных линиях (стволовые клетки планарий, реснитчатый эпителий). [Kalbfuss & Gönczy, Sci Adv 2023 — PMID: 37256957; обзор: Open Biol 2023 — PMID: 37963546].

**Отрицательная энтропия (негэнтропия):** Li et al. (2026) — "Neg-entropy is the true drug target for chronic diseases" — Acta Pharm Sin B, PMID: **41584335**. Терапевтическая цель — не снижение энтропии вообще, а восстановление негэнтропийных механизмов.

> **polyGlu = маркер/признак энтропии (компенсаторный ответ).** Количество polyGlu на центриоли показывает, сколько энтропии накопила центриоль и насколько активен компенсаторный ответ. Асимметричное наследование старых центриолей — элемент механизма необратимой дифференцировки. **Накопление энтропии в стволовых клетках через наследование старых центриолей — плата за возможность необратимой дифференцировки.**

**Литературная поддержка (энтропийная теория старения — emerging mainstream):**

| Article | Journal | Год | PMID |
|--------|--------|-----|------|
| **Wu Z et al.** — The entropic view of aging: from thermodynamics to biology | Life Med | 2026 | **42388853** |
| **Cummings SR, Hong N, Cohen AA** — Entropy and Human Aging | Aging Cell | 2025 | **41230623** |
| **Hong N, Cohen AA** — Aging as Entropy: A Quantifiable Framework | Endocrinol Metab | 2025 | **41299832** |
| **De Man R et al.** — Single-cell atlas: increased transcriptional entropy with age | Nat Commun | 2026 | **41571679** |
| **Hong N et al.** — Entropy of Muscle Histology Predicts Mobility | Aging Cell | 2026 | **41724675** |

**Value:** CEDAR даёт энтропийной теории старения конкретный клеточный механизм — центриоль как физический носитель энтропии.

### 7.8. Асимметричные деления: обе дочерние — стволовые, но с разной прогениторностью (2026-07-05)

> **Джаба:** «Асимметричные деления, когда обе дочерние клетки остаются стволовыми, но разными по прогениторности.»

**Ключевая находка — Bener & Inaba (2026):**

| Article | Journal | Год | PMID | Суть |
|--------|--------|-----|------|------|
| **Bener MB, Slepchenko BM, Inaba M** — Asymmetric division sustains stem cell heterogeneity in Drosophila testicular niche | Commun Biol | 2026 | **42056503** | 🔑 **Асимметричное деление поддерживает гетерогенность СК** — обе дочерние остаются стволовыми, но с разными свойствами |
| **Bunel B et al.** — Unequal mitochondrial segregation promotes asymmetric fates during neurogenesis | Nat Commun | 2025 | **41398154** | Неравная сегрегация митохондрий → асимметричные судьбы при нейрогенезе |
| **Rodríguez-Correa E et al.** — Kinetics-based model of haematopoiesis reveals extrinsic regulation of skewed lineage output from stem cells | Nat Cell Biol | 2026 | **42185517** | HSC производят дочерние клетки с разным lineage bias |
| **Ma B et al.** — Asymmetric histone inheritance regulates olfactory stem cell fates during regeneration | Nat Commun | 2026 | **41872193** | Asymmetry гистонов → разные судьбы дочерних СК |
| **Bener MB, Inaba M** — Plasticity and stringency: rethinking stem cell division modes | Biochem Soc Trans | 2026 | **41626805** | Обзор: асимметричное деление ≠ всегда СК + диф. клетка |

**Value для CEDAR:** Это именно тот тип асимметрии, который предсказывает CEDAR. **polyGlu — не «повреждение», а маркер зрелости/функциональности центриоли.** Материнская центриоль (polyGlu-high) = более стабильная платформа для сборки CAASM (гипотетической структуры индукторов необратимой дифференцировки) → на ней собрано больше индукторов → дочерняя клетка получает более ограниченный прогениторный потенциал (готова к дифференцировке). Дочерняя центриоль (polyGlu-low) = меньше CAASM → дочерняя клетка сохраняет полный потенциал (более «стволовая»). **Обе — стволовые, но с разной степенью коммитированности.** Проблема возникает при ГИПЕР-глутамилировании в старости: избыточный polyGlu нарушает функцию центриоли — тогда обе дочерние клетки страдают.

**Предсказание CEDAR (исправленное):** В делениях GSC дрозофилы (модель Bener/Inaba) материнская центриоль (Ninein+, GT335-high) должна нести БОЛЬШЕ белков-индукторов дифференцировки (CAASM) и наследоваться дочерней СК с более ограниченным потенциалом. Дочерняя центриоль (GT335-low) — меньше CAASM → полный потенциал.

### 7.9. Экспериментальные дизайны (2026-07-05)

> Полный документ: `docs/EXPERIMENTAL_DESIGNS_2026-07-05.md`

| Эксперимент | Цель | Model | Budget | Status |
|-------------|------|--------|:------:|:------:|
| **CCP1-KO** | Causality: polyGlu → HSC aging? | Vav-iCre × CCP1^fl/fl, n=50 мышей | ~$35K | Дизайн готов |
| **Центросомный IP-MS** | CAASM: белки на центросоме при дифференцировке | HSC/MPP/Lin+, молодые vs старые, TMT-MS | ~$34K | Дизайн готов |
| **GT335 SPEM** | Модуляция vs необратимость | Chief cells желудка: DMP-777 vs L-635 | ~$5K | Дизайн готов |

## 8. Pre‑registration Plan (updated 2026‑04‑25)

All prospective falsification tests will be pre‑registered on the Open Science Framework (OSF) before data collection. The pre‑registration will include the study design, sample size justification (see §4), analysis plan, and primary outcome measures.

| Test ID | Planned Registration Date | Target | Sample Size |
|---------|--------------------------|--------|-------------|
| FT1.1 (PTM vs divisions) | 2026‑09‑01 | Confirmation of C1 | 30 HSCs per age group |
| FT1.2 (asymmetry in HSC) | 2026‑09‑01 | Confirmation of C2 | 40 divisions per condition |
| FT2.1 (PTM‑signaling correlation) | 2026‑10‑15 | Axiom 2 validation | 25 HSCs per group |
| FT6.1 (CCP1 KO) | 2026‑11‑01 | Prediction P6 | 5 mice per genotype per time point |
| FT6.2 (CCP1 rescue) | 2026‑11‑01 | Prediction P9 | 5 mice per condition |

Actual OSF identifiers will be inserted upon submission. Any deviations from pre‑registered plans will be documented as amendments.

---

## 9. Evidence Update (2026-07-06) — Systematic Meta-Analysis

### 9.1. Hallmarks of Stem Cell Aging — Centrosomes MISSING

**Rando TA, Brunet A, Goodell MA** — *Hallmarks of Stem Cell Aging* — Cell Stem Cell 32:1038–1054 (2025) — PMID: **40562035** | PMCID: **PMC12227283** | 🔓 Open Access

Five hallmarks: (1) depth of quiescence, (2) self-renewal propensity, (3) fate of progeny, (4) resilience, (5) population heterogeneity.

**Key finding:** Centrosomes, centrioles, microtubules, and polyglutamylation are NOT mentioned anywhere in the 17-page review. CEDAR is not cited. This represents a gap — CEDAR proposes the missing sixth hallmark.

### 9.2. Mother Centrosome → T Cell Fate (Mammalian)

**Barandun N, Meier B, Stehli G, Gräbnitz F, Zangger N, Oxenius A** — *Targeted localization of the mother centrosome in CD8+ T cells undergoing asymmetric cell division promotes memory formation* — Cell Reports (2025) — DOI: `10.1016/j.celrep.2024.115127`

Direct evidence in mammals: mother centrosome inheritance determines memory vs effector fate in CD8+ T cells. Supports C2 (asymmetric inheritance) and M3 (CAASM).

### 9.3. Senescence Not Predicted by Telomere Length

**Passanisi S, Spencer SL** — *Replicative senescence induction in single cells is not predicted by telomere length, dysfunction, or oxidation* — iScience (2026) — DOI: `10.1016/j.isci.2026.114801`

At single-cell level, neither telomere length nor oxidation predicts senescence. Strong independent support for multi-counter MCARA model.

### 9.4. UBE2G1 — Proteostasis Meets Signaling

**Proteostasis meets signaling: UBE2G1 in HSC aging** — Haematologica (2026) — DOI: `10.3324/haematol.2026.300724`

**Elevated Ube2g1 → segmental aging** — Haematologica (2026) — DOI: `10.3324/haematol.2025.288847`

First concrete molecular mechanism coupling proteostatic (#5) and signaling (#4) counters in HSCs.

### 9.5. Mitochondrial Drivers of Stem Cell Aging

**Bautista J, López-Cortés A** — *Mitochondrial drivers of stem cell aging and inflammaging* — npj Aging (2026) — DOI: `10.1038/s41514-026-00422-5`

Comprehensive review linking mitochondrial dysfunction to inflammaging in stem cells. Direct support for Counter #3.

### 9.6. Somatic piRNA/PIWI in Stem Cells

**Manju et al.** — *Somatic piRNA and PIWI-mediated post-transcriptional gene regulation in stem cells and disease* — Front Cell Dev Biol (2024) — DOI: `10.3389/fcell.2024.1495035`

PIWI expressed in Hydra, planarians, BMSCs — but no functional data in human HSCs. Confirms EXPLORATORY status of Counter #6.

### 9.7. CD38 + Mitochondrial Calcium in HSC Aging

*CD38 and the mitochondrial calcium uniporter contribute to age-related HSC dysfunction* — Immunometabolism (2024) — DOI: `10.1097/in9.0000000000000048`

### 9.8. Additional Supporting References

| Article | Journal | Год | DOI/PMID |
|--------|--------|-----|----------|
| Gilloteaux et al. — Mitochondrial ultrastructure in differentiated SH-SY5Y | Ultrastruct Pathol | 2026 | 42159247 |
| Eckhart et al. — Holocrine secretion | Cells | 2026 | 10.3390/cells15121058 |
| Geyer & Hagelueken — Protein storage in oocytes | Nature | 2026 | 42204324 |
| The systemic costs of HSC aging | Development | 2025 | 10.1242/dev.205103 |
| Stem Cells in Aging and Anti-Aging | Stem Cell Rev Rep | 2026 | 10.1007/s12015-026-11093-w |
| Rethinking holocrine secretion | Am J Physiol Cell Physiol | 2026 | 10.1152/ajpcell.00191.2026 |
| Protein Storage in Oocytes | Annu Rev Cell Dev Biol | 2025 | 10.1146/annurev-cellbio-101323-031045 |
| Oocyte-intrinsic aging drives aneuploidy | Reprod Biol | 2026 | 10.1016/j.repbio.2026.101205 |
| Synthetic oocyte aging method | Nature Aging | 2025 | 10.1038/s43587-025-01010-0 |
| Peanut Procyanidin A Delays HSC Aging through Cox5a | Aging Dis | 2026 | 10.14336/ad.2025.1354 |
| Lipid deregulation impacts HSC functions | Blood | 2025 | 10.1182/blood-2025-975 |
| Outside the niche: Gut microbiota → HSC dysfunction | Cell Stem Cell | 2026 | 10.1016/j.stem.2026.06.004 |
| Piwi-piRNA via PNUTS/Senataxin | Mol Cell | 2025 | 10.1016/j.molcel.2025.10.006 |
| CCHCR1 links P-body to centrosome | Cell Mol Biol Lett | 2025 | 10.1186/s11658-025-00780-0 |
| Cell lineage-resolved embryonic map | Nat Commun | 2025 | 10.1038/s41467-025-58878-0 |
| Asymmetric histone inheritance in olfactory SC | Nat Commun | 2026 | 41872193 |
| 3D Genome Architecture in Stem Cell Commitment | Adv Genet | 2025 | 41036476 |

---

## 10. SPERMIOGENESIS & MITOCHONDRIAL PROTEASES — New Evidence (2026-07-09)

> **Date анализа:** 2026-07-09 | **Источник:** 4 статьи из фида Джабы + широкий поиск по CrossRef/PubMed (20+ дополнительных статей)
> **Метод:** Верификация DOI через CrossRef API, абстрактов через PubMed/CrossRef, поиск родственной литературы.

---

### 10.1. ЧЕТЫРЕ ОСНОВНЫЕ СТАТЬИ

| # | Article | Journal | DOI | Type | Ссылок |
|---|--------|--------|-----|-----|--------|
| 1 | **Zhou K, Ren K, An Y, Xia L, Cheng X** — The role of heterogeneous nuclear ribonucleoproteins in mammalian spermatogenesis: mechanisms and clinical implications | *Reproduction* (2026) | `10.1093/reprod/xaag066` | REVIEW | 66 |
| 2 | **Mao B, Li SY, Dong WY, Wang YF** — *Slmap* Is Required For Spermiogenesis in *Drosophila melanogaster* | *Arch Insect Biochem Physiol* (2026) | `10.1002/arch.70186` | RESEARCH | 39 |
| 3 | **Dominicci-Cotto C, Jenny A** — From syncytium to sperm: Coordinated membrane and cytoskeletal remodeling in *Drosophila* spermiogenesis | *Curr Top Dev Biol* (2026) 168:317–370 | `10.1016/bs.ctdb.2026.03.005` | REVIEW | 232 |
| 4 | **Feng HW, Gao YL, Jiang BJ, Zubair M, Liu DT, …, Huo LJ** — ClpP Ensures Mitochondrial Integrity and Spermatocyte Meiotic Progression in Mice | *Andrology* (2026) | `10.1111/andr.70274` | RESEARCH | 17 |

**Верификация:** Все 4 DOI подтверждены через CrossRef API + Semantic Scholar. Article №3 имеет PubMed ID: 42097816. Все статьи реальны, 2026 года.

---

### 10.2. АНАЛИЗ ПО СТАТЬЯМ

#### 🔴 Article №4 — Feng et al. (2026): ClpP → митохондриальная целостность → мейоз

**Суть:** ClpP (казеинолитическая пептидаза P) — митохондриальная протеаза контроля качества. Conditional KO (*Ddx4*-Cre^ERT2) в сперматоцитах мыши вызывает:
- Уменьшение размера семенников, отсутствие зрелых сперматозоидов
- **«Гигантские митохондрии»** в сперматоцитах *Clpp* cKO
- Нормальный мейотический синапсис НО нарушенная рекомбинация: **↓RAD51 фокусов, аномальная локализация MLH1**
- Элиминация пахитенных/диплотенных сперматоцитов и сперматид

**Value для MCARA/CEDAR:**

| Аспект | Value | Counter |
|--------|----------|--------|
| **ClpP = митохондриальный протеостаз** | Митохондриальная протеаза контролирует cell fate (мейотическую рекомбинацию). Прямая демонстрация: **митохондриальный протеостаз → судьба клетки.** | Counter #3 + #5 |
| **«Гигантские митохондрии»** | Морфологический фенотип, идентичный стареющим митохондриям в HSC. ClpP deficiency → mitochondrial swelling. | Counter #3 |
| **Ретроградный сигналинг** | ClpP (митохондриальный матрикс) → RAD51 фокусы (ядро). Митохондриальная протеаза контролирует ядерные процессы. | **НОВЫЙ МЕХАНИЗМ:** Counter #3 ↔ #1 |
| **ClpP + ClpX = комплекс** | Guo et al. (2022, препринт; 2023, Hum Reprod): ClpP/ClpX deficiency → impaired mTORC1. mTORC1 = ключевой путь старения. | Counter #3 + #4 |

**🔥 КЛЮЧЕВОЙ ВЫВОД:** ClpP — первый конкретный пример mitochondrial protease, чья потеря вызывает: (а) морфологические дефекты митохондрий, (б) нарушение ядерных процессов (рекомбинация), (в) клеточную гибель. **Прямая аналогия с MCARA Counter #3**: потеря митохондриального протеостаза → функциональный дефицит стволовых клеток.

#### 🟡 Article №2 — Mao et al. (2026): Slmap → спермиогенез → аксонема + митохондрии

**Суть:** Slmap (sarcolemma-associated protein) knockdown в ранней зародышевой линии дрозофилы →:
- Аберрантные паракристаллические отложения в обоих митохондриальных дериватах
- Слияние митохондриальных дериватов и сперматид
- **Потеря центральной пары микротрубочек аксонемы** ← ДЕФЕКТ ЦЕНТРИОЛЯРНОГО ПРОИСХОЖДЕНИЯ
- Кластеризация аксонем
- Upregulation генов individualization complex (*chic*, *ctp*, *WASp*, *Lasp*)
- Транскриптомика: затронуты гены cuticle development, oxidative phosphorylation
- **Апоптоз через Hid-Diap1 путь**

**Value для MCARA/CEDAR:**

| Аспект | Value | Connection |
|--------|----------|------|
| **Мембранный белок → митохондрии + центриоли** | Slmap влияет на митохондриальные дериваты И на аксонему. Межкомпартментная связь. | **Counter #1 ↔ #3 crosstalk** |
| **Паракристаллические отложения** | Сбой протеостаза в митохондриях → структурная патология. | Counter #3 + #5 |
| **Потеря центральной пары аксонемы** | Прямой центриолярный дефект. | **Counter #1** |
| **Hid-Diap1 апоптоз** | Структурные дефекты → caspase-зависимая клеточная смерть. | CEDAR: D_c → apoptosis |

#### 🟡 Article №3 — Dominicci-Cotto & Jenny (2026): Синцитий → сперматозоиды

**Суть:** Капитальный обзор (232 ссылки) мембранного и цитоскелетного ремоделирования при Drosophila спермиогенезе: от синцития до индивидуальных сперматозоидов. Coordinated membrane remodeling + actin dynamics + microtubule reorganization.

**Value для CEDAR:**
- Синцитий → индивидуальные клетки — аналогия: СК в нише → дифференцированные клетки.
- Актиновый цитоскелет (Arp2/3, WASp) — regulated by centrosomes through Cdc42/Rac1/RhoA.
- **232 references** — золотая жила. Рекомендуется ручной просмотр при доступе к полному тексту.

#### 🟠 Article №1 — Zhou et al. (2026): hnRNPs в сперматогенезе

**Суть:** Обзор роли hnRNPs (РНК-связывающих белков) в сперматогенезе млекопитающих. 66 ссылок.

**Value:** Умеренная косвенная поддержка Counter #4 (эпигенетический/РНК-метаболизм).

---

### 10.3. ДОПОЛНИТЕЛЬНАЯ ЛИТЕРАТУРА (широкий поиск)

#### 🔴 КРИТИЧЕСКИ ВАЖНЫЕ

| # | Article | Journal | Год | DOI/PMID | Value |
|---|--------|--------|-----|----------|----------|
| 1 | **Yamada Y et al.** — Non-necroptotic MLKL function damages mitochondria and promotes HSC aging | *Nat Commun* | 2026 | `10.1038/s41467-026-71060-4` | 🔥 **Прямая связь: митохондриальные повреждения → HSC aging.** MLKL → mitochondria → impaired self-renewal. RIPK3-MLKL = ключевой медиатор HSC-старения. |
| 2 | **Wani GA et al.** — Metabolic control of adult NSC self-renewal by mitochondrial protease YME1L | *Cell Rep* | 2022 | `10.1016/j.celrep.2022.110370` | 🔥 **YME1L → NSC self-renewal.** Параллель с ClpP: mitochondrial protease → stem cell fate — УНИВЕРСАЛЬНЫЙ ПРИНЦИП. |
| 3 | **Mohrin M et al.** — UPR^mt activated upon HSC exit from quiescence | *Aging Cell* | 2018 | `10.1111/acel.12756` | 🔥 **UPR^mt → HSC.** Митохондриальный протеостаз меняется при HSC quiescence→proliferation. |
| 4 | **Wang CL et al.** — UPR^mt regulates hippocampal NSC aging | *Cell Metab* | 2023 | `10.1016/j.cmet.2023.04.012` | 🔥 **UPR^mt → NSC aging.** |
| 5 | **Khire A et al.** — Centriole Remodeling during Spermiogenesis in Drosophila | *Curr Biol* | 2016 | PMID: **28094036** | 🔥 **Центриоли ремоделируются при спермиогенезе.** Poc1 enrichment необходимо. |

#### 🟡 ВАЖНЫЕ

| # | Article | Journal | Год | DOI | Value |
|---|--------|--------|-----|-----|----------|
| 6 | **Schumacher S et al.** — PARL is required for spermatogenesis | *Commun Biol* | 2024 | `10.1038/s42003-023-05703-3` | Третья митохондриальная протеаза → сперматогенез. PARL → PINK1. |
| 7 | **Morganti C, Ito K** — Mitochondrial Contributions to HSC Aging | *Int J Mol Sci* | 2021 | `10.3390/ijms222011117` | Обзор митохондрий в старении HSC. |
| 8 | **Bautista J, López-Cortés A** — Mitochondrial drivers of stem cell aging | *npj Aging* | 2026 | `10.1038/s41514-026-00422-5` | Современный обзор. |
| 9 | **Tcheng M et al.** — LONP1 reliance in AML stem cells | *J Clin Invest* | 2026 | `10.1172/jci196687` | 🔥 LONP1 — ещё одна митохондриальная протеаза → cancer SC. |
| 10 | **Varuzhanyan G, Chan DC** — Mitochondrial dynamics during spermatogenesis | *J Cell Sci* | 2020 | `10.1242/jcs.235937` | Контекст для ClpP и Slmap. |
| 11 | **Guo C et al.** — ClpP/ClpX deficiency impairs mTORC1 during spermatogenesis | *bioRxiv* | 2022 | `10.1101/2022.12.16.520702` | Препринт → Feng 2026. |
| 12 | **Manandhar G et al.** — Centriole degeneration during mouse spermiogenesis | *Cell Motil Cytoskeleton* | 1999 | PMID: 10401572 | Классика. |

---

### 10.4. МЕТА-АНАЛИЗ: Митохондриальные протеазы → судьба стволовых клеток

**Наблюдение:** Четыре митохондриальные протеазы независимо показаны как критические для cell fate:

| Протеаза | Локализация | Model | Фенотип потери | Ссылка |
|----------|-------------|--------|----------------|--------|
| **ClpP** | Матрикс | Сперматоциты мыши | Гигантские митохондрии, ↓RAD51, гибель | Feng 2026 |
| **YME1L** | Внутр. мембрана (i-AAA) | Нейральные СК | Нарушение self-renewal | Wani 2022 |
| **LONP1** | Матрикс | Лейкемические СК | Повышенная зависимость | Tcheng 2026 |
| **PARL** | Внутр. мембрана (ромбоид) | Сперматогенез | Нарушение, ↓PINK1 | Schumacher 2024 |

**🔥 ГИПОТЕЗА ДЛЯ MCARA:** Митохондриальные протеазы (ClpP, YME1L, LONP1, PARL) образуют **протеостатическую ось**, чья дисфункция при старении → накопление повреждённых белков → mitochondrial dysfunction → нарушение cell fate. Универсальный механизм: HSC (Yamada 2026), NSC (Wani 2022, Wang 2023), сперматоциты (Feng 2026), лейкемические СК (Tcheng 2026).

**Connection с CEDAR:** Два компартмента, где протеостаз критичен для cell fate:
- **Центриоль:** polyGlu — признак энтропии; нет механизма ремонта
- **Митохондрии:** ClpP/YME1L/LONP1 — протеостатический контроль; дисфункция → потеря cell fate control

**Предсказание:** В стареющих HSC должно наблюдаться: (а) ↓ClpP и YME1L, (б) накопление убиквитинилированных белков в митохондриях, (в) «гигантские митохондрии» — параллельно с (г) накоплением polyGlu на центриолях.

---

### 10.5. СВЯЗЬ С CEDAR (Cell Death And Tissue Architecture)

> **Примечание:** CEDAR — отдельный концепт (Cell Death And Tissue Architecture), не CEDAR. Connection через апоптотические пути.

**Mao et al. (2026):** Slmap → Hid-Diap1 апоптоз. Структурная патология → programmed cell death.

**Feng et al. (2026):** ClpP deficiency → элиминация сперматоцитов.

**Yamada et al. (2026):** MLKL → non-lethal mitochondrial damage → functional decline (не апоптоз/некроптоз).

---

### 10.6. ИТОГОВАЯ ОЦЕНКА

| Компонент | Было | Стало | Δ | Причина |
|-----------|------|-------|-----|--------|
| **Counter #3 (Митохондриальный)** | 8/10 | **8.5/10** | +0.5 | ClpP + YME1L + MLKL → stem cell fate |
| **Counter #5 (Протеостаз)** | 6.5/10 | **7.5/10** | +1.0 | ClpP/YME1L/LONP1: mitochondrial proteases |
| **Counter #1 (Центриолярный)** | 7.5/10 | **8.0/10** | +0.5 | Khire 2016 + Slmap axoneme defects |
| **CEDAR (апоптоз)** | — | **7.0/10** | NEW | Hid-Diap1 + ClpP → structural damage → death |
| **M3/CAASM** | 6/10 | **6.5/10** | +0.5 | ClpP: mito protease → nuclear recombination |

**Общая оценка MCARA/CEDAR: 7.3/10 → 7.8/10 (+0.5)**

---

## Добавлено 2026-07-13 — Глубокий разбор Research Feed + поиск похожих

### PCM1 — центросомная асимметрия → судьба клетки (Zhao et al., Nat Commun 2025)
| Утверждение | PMID/DOI | Проверено | Сила |
|-------------|----------|:---:|:---:|
| Pcm1 асимметрично ассоциирован с Cep83 (маркер материнской центриоли) в zebrafish RGPs | 41315244 | ✅ 2026-07-13 | Strong |
| Pcm1 на Notch-лиганд-содержащих эндосомах → связь центросомной асимметрии с сигналингом | 41315244 | ✅ 2026-07-13 | Strong |
| PARD3-PCM1-CEP83-RAB11 ассоциации консервативны (human brain organoids) | 41315244 | ✅ 2026-07-13 | Strong |
| Потеря pcm1 → повышенная нейрональная дифференцировка за счёт self-renewal | 41315244 | ✅ 2026-07-13 | Strong |

### SIRT1 → центриолярные белки при старении (Iniesta-Cuerda et al., Biol Direct 2026)
| Утверждение | PMID/DOI | Проверено | Сила |
|-------------|----------|:---:|:---:|
| SIRT1 недостаточность → потеря ацетилирования центриолярных белков CEP170, CEP350 | 41882697 | ✅ 2026-07-13 | Strong |
| α-тубулин гиперацетилирован в мидписе сперматозоида при старении и SIRT1↓ | 41882697 | ✅ 2026-07-13 | Strong |
| Midpiece α-tubulin hyperacetylation ↔ mitochondrial dysfunction + ROS↑ | 41882697 | ✅ 2026-07-13 | Moderate |

### SAS-6: контекст-зависимость центриолярного биогенеза (Grzonka & Bazzi, eLife 2024)
| Утверждение | PMID/DOI | Проверено | Сила |
|-------------|----------|:---:|:---:|
| mESC не требуют SAS-6 для формирования центриолей (в отличие от эмбрионов) | 38407237 | ✅ 2026-07-13 | Strong |
| PLK4 активность даёт robustness — SAS-6-независимый биогенез | 38407237 | ✅ 2026-07-13 | Strong |
| Дифференцировка → потеря SAS-6-независимого пути → уязвимость центриолей | 38407237 | ✅ 2026-07-13 | Moderate |

### Caspar — би-модальный регулятор PGC судьбы (Das et al., Biol Open 2025)
| Утверждение | PMID/DOI | Проверено | Сила |
|-------------|----------|:---:|:---:|
| Caspar контролирует PGC fate через Osk-зависимый и центросомный пути | 40614258 | ✅ 2026-07-13 | Moderate |
| Центросомная дисфункция проявляется ДО PGC формирования (NC 6-9) | 40614258 | ✅ 2026-07-13 | Moderate |

### mRNA-регионализация в одиночной клетке (Albright et al., PNAS 2026)
| Утверждение | Источник | Проверено | Сила |
|-------------|----------|:---:|:---:|
| мРНК разных генов распределены по разным регионам гигантской одноклеточной водоросли | PNAS 2026, Albright AR, Marshall WF | ✅ 2026-07-13 | Strong |
| Одиночная клетка способна к пространственному паттернированию, аналогичному эмбриональному | PNAS 2026 | ✅ 2026-07-13 | Strong |

### mei-P26 — gatekeeper митоз→мейоз (Terry et al., Genetics 2026)
| Утверждение | Источник | Проверено | Сила |
|-------------|----------|:---:|:---:|
| Гипоморфная мутация mei-P26 → задержка в митозе → вход в мейоз с митотическими сигналами | Genetics 2026, Terry J, Crown N | ✅ 2026-07-13 | Strong |
| Нарушение кроссинговера из-за аберрантной мейотической динамики хромосом | Genetics 2026 | ✅ 2026-07-13 | Moderate |

---

## 11. Repair Mechanisms & Their Limitations (2026-08-02)

> **Trigger:** Julia Mahamid objection — "I suspect there may be repair mechanisms, as such damage cannot propagate endlessly in an organism's lifetime."
> **Full defense:** `docs/REPAIR_OBJECTION_DEFENSE.md`
> **THEORY.md:** §4.1 — updated ¬R axiom with limited repair distinction

### 11.1. What Repair EXISTS

| Mechanism | Target | Capability | Limitation | Reference |
|-----------|--------|------------|------------|----------|
| **Autophagy (TIAM1/PLK4)** | Centriolar proteins during DUPLICATION | Quality control of NEW centrioles | Does NOT repair existing centriolar wall | Coelho, Yu & Glover (2026), DOI: `10.64898/2026.07.02.735969` |
| **Autophagy (centrosome amplification sensing)** | Supernumerary centrioles | Eliminates EXTRA centrioles | Does NOT repair structural damage | Nat Commun (2026), PMID: **42324259** |
| **Ubiquitin-Proteasome System** | PCM components | Degrades damaged PCM proteins | Cannot access tubulin inside triplet wall | Multiple refs |
| **Chaperones (Hsp70/Hsp90)** | Misfolded PCM proteins | Refolds non-covalent damage | Cannot reverse covalent modifications (carbonylation, 4-HNE) | Multiple refs |
| **Centriole elimination (germline)** | Entire centrioles | Complete reset in oocyte/early embryo | NOT in adult somatic cells | Manandhar et al. (1999), PMID: 10401572; Gönczy & Balestra (2023), PMID: 36988082 |
| **Centriole elimination (C. elegans embryo)** | Entire centrioles | Programmed elimination in specific lineages | Tissue-specific, not universal | Kalbfuss & Gönczy (2023), PMID: 37256957 |
| **Centriole inactivation (Drosophila)** | Entire centrioles | Developmentally programmed | Specific stages only | Bonente et al. (2025), PMID: 40558492 |

### 11.2. What Repair DOES NOT Exist

| Desired Mechanism | Why Impossible |
|-------------------|---------------|
| **Extraction of damaged tubulin from centriolar wall** | Triplets are closed cylindrical structures. Damaged tubulin is embedded in the lattice. Extraction → disassembly → loss of centriole integrity. |
| **Reversal of carbonylation** | Carbonylation is a covalent, irreversible modification. No known enzyme reverses protein carbonylation. |
| **Reversal of 4-HNE adducts** | 4-HNE forms Michael adducts with Cys, His, Lys residues. Partially reversible by ALDH2/GST, but adducts buried in MT lattice are inaccessible. |
| **In-situ replacement of damaged subunits** | Microtubule dynamics (GTP-cap) allow subunit exchange at ends, but centriolar MTs are stable (no dynamic instability). No exchange in the wall. |

### 11.3. Quantitative Model: Damage Accumulation vs Limited Repair

```
dD/dt = k_damage × (ν + τ) − k_autophagy × D − k_UPS × D_accessible − k_elim × D

where:
- k_damage ≈ 0.01-0.05 D_critical/year (oxidative + replicative)
- k_autophagy ≈ 0.001-0.01/year (only during duplication, ~1-2% of cell cycle)
- k_UPS ≈ 0.005/year (only PCM-accessible fraction ~10-20%)
- k_elim ≈ 0 in adult somatic cells

→ dD/dt ≈ k_damage × (ν + τ) [net positive — damage accumulates]
→ D(t) reaches D_critical in 60-100 years (human lifespan)
```

### 11.4. Evolutionary Resolution

**Selection shadow (Medawar 1952, Williams 1957):** Natural selection cannot optimize mechanisms whose failure manifests after reproductive age. Centriolar damage accumulates slowly enough that:
- Pre-reproductive period (0-15 years): D << D_critical — centrioles function normally
- Reproductive period (15-40 years): D < D_critical — reproduction successful
- Post-reproductive (40+ years): D → D_critical — aging phenotypes emerge

Selection "sees" only the reproductive window. Complete repair was never selected for.

### 11.5. Germline Reset — Breaking the Chain Between Generations

```
Generation N:   Zygote (D≈0) → Adult (D→D_critical) → Gametes (D→0 via elimination)
Generation N+1: Zygote (D≈0) → Adult (D→D_critical) → Gametes (D→0)
```

The species does not age. Individuals do.

### 11.6. Experimental Distinction: Repair vs Elimination

| Experiment | If Repair Exists | If Only Elimination Exists |
|-----------|-----------------|---------------------------|
| GT335 in post-mitotic neurons over time | GT335 stable or ↓ (active deglutamylation) | GT335 ↑ (no removal) |
| Carbonyl detection on centriolar tubulin | ↓ after stress removal | ↑ monotonically |
| Centriole length in aged vs young | Stable (repair maintains) | ↑ (elongation = entropy) |
| **Observed:** | GT335 ↑ with age. Length ↑. Carbonyl ↑. → **Elimination model supported.** | |

---

*Added 2026-08-02. Integrated with THEORY.md §4.1 and docs/REPAIR_OBJECTION_DEFENSE.md.*

---

## Добавлено 2026-07-25 — Chk1: Неканонические роли на центросоме

### Chk1→β-tubulin-T285 → spindle quality (M1 mechanism)

| Утверждение | PMID/DOI | Проверено | Сила |
|-------------|----------|:---:|:---:|
| Chk1 фосфорилирует β-tubulin T285 на митотических центросомах → необходимо для плотности и нуклеации MT веретена | **41844775** — Boutakoglou/…/Zachos. *Commun Biol*. 2026;9:608 | ✅ 2026-07-25 | **Strong** |
| ATRIP→ATR→TopBP1→Chk1 на центросоме — НЕ ядерный DNA damage путь, а центросомный путь созревания веретена | (ibid.) | ✅ 2026-07-25 | Strong |
| T285A phospho-dead → неправильное веретено, задержка анафазы, ошибки сегрегации, неравные дочерние клетки | (ibid.) | ✅ 2026-07-25 | Strong |
| Chk1-P→Aurora B S331 → активация Aurora B (та же лаба) | `10.1083/jcb.201104023` — Petsalaki/…/Zachos. *J Cell Biol*. 2011 | ✅ 2026-07-25 | Strong |

**Значение для CEDAR:** (1) Механизм M1 (spindle defects → genomic instability) получает конкретный молекулярный путь. (2) Chk1 на центросоме — пример «перепрофилирования» DNA damage киназы для контроля митоза. (3) Возрастное снижение Chk1 → ухудшение качества веретена → ускорение Counter #1.

### Chk1→mitophagy → cardiac aging (Counter #3)

| Утверждение | PMID/DOI | Проверено | Сила |
|-------------|----------|:---:|:---:|
| CHK1 overexpression → HSP90-митофагия → замедление кардиального старения | **42229233** — Jing P et al. *Redox Biol*. 2026;95:104242 | ✅ 2026-07-25 | **Strong** |
| CHK1 ↓ с возрастом; Chk1-cKO → ухудшение функции сердца in vivo | (ibid.) | ✅ 2026-07-25 | Strong |

---

## 12. Planarian Centriole Evidence — Gene Loss vs Organelle Loss (2026-08-06)

> **Trigger:** Jochen Rink (MPI-NAT) response to CEDAR inquiry + pointer to Martín-Durán et al. (2017)
> **Key question:** Does the discovery of hidden orthologs of centrosome genes refute Azimzadeh's finding that planarian neoblasts lack centrioles?

### 12.0. The Original Evidence — Azimzadeh et al. (2012)

**Azimzadeh J, Wong ML, Downhour DM, Sánchez Alvarado A, Marshall WF** — *Centrosome loss in the evolution of planarians.* Science. 2012;335(6067):461-463. PMID: **22223737**.

Two distinct claims made in this paper:

| Claim | Type | Method | Status after Martín-Durán 2017 |
|-------|------|--------|------|
| **Claim A:** Centrosome-related genes are absent in planarian genomes | Genomic | BLAST against human centrosome gene set | ⚠️ **Weakened** — hidden orthologs exist |
| **Claim B:** Neoblasts and dividing embryonic cells lack centrioles | Structural | TEM, immunofluorescence (γ-tubulin, centrin) | ✅ **Stands** — no genomic finding refutes TEM |

### 12.1. The Challenge — Martín-Durán et al. (2017)

**Martín-Durán JM, Ryan JF, Vellutini BC, Pang K, Hejnol A** — *Increased taxon sampling reveals thousands of hidden orthologs in flatworms.* Genome Res. 2017;27(7):1263-1272. PMID: **28400424**. PMCID: PMC5495077.

**Method:** Leapfrog pipeline — transitive homology BLAST using intermediate «bridge» species to detect orthologs invisible to direct BLAST due to rapid sequence evolution.

**Key findings relevant to CEDAR:**

| Finding | Detail |
|---------|--------|
| 3427 hidden orthologs recovered | From 35 flatworm transcriptomes |
| Centrosome-related genes found | Including **SDCCAG8** in *S. mediterranea* (Fig. 2C) |
| Standard tools miss them | OrthoFinder recovered only 75-83%; HaMStR recovered only 2-4 of 75 |
| **Authors' caveat** | «…which suggests that they might not be coexpressed on those planarian cells that assemble centrosomes and thus might have evolved alternative functions.» |

### 12.2. Resolution — Two Separate Questions

| Question | Answer | Evidence |
|----------|--------|----------|
| **Are centrosome genes present in the planarian genome?** | Yes, as hidden orthologs | Martín-Durán 2017 — Leapfrog pipeline |
| **Do planarian neoblasts have centrioles?** | No | Azimzadeh 2012 — TEM (structural, not genomic) |

**Principle:** Gene presence ≠ organelle presence. Hidden orthologs may have been co-opted for non-centrosomal functions over ~500 Myr of planarian evolution. The genomic argument (Claim A) is weakened; the structural argument (Claim B) is unchanged.

### 12.3. What This Means for CEDAR

| Aspect | Impact |
|--------|--------|
| **Loss of centrosome genes as evidence** | Can no longer be cited as support without qualification. Must cite Martín-Durán 2017 alongside Azimzadeh 2012. |
| **TEM evidence of acentriolar neoblasts** | Remains the strongest direct evidence. Unaffected by hidden ortholog discovery. |
| **Hidden orthologs as alternative-function genes** | Consistent with CEDAR: if centrioles were lost evolutionarily, centrosome genes would either degrade or adapt to new roles. Hidden orthologs with alternative functions support rather than refute this. |
| **Planarian embryogenesis** | Rink confirms: nobody has looked at centrioles in zygotes/cleavage-stage embryos. Gap remains open. |

### 12.4. Jochen Rink — Personal Communication (2026-08-05)

**Three questions answered:**
1. **Centriole from sperm in zygote?** — No published data. Fertilisation inside parent; specific efforts needed.
2. **Sex/asex centriole gene expression?** — Fissiparous reproduction bypasses embryos entirely; no single-cell bottleneck.
3. **TEM of zygotes/early embryos?** — No. Zygotes are «needles in the haystack» among yolk cells. Serial sectioning required.

**Rink also flagged:** Martín-Durán et al. (2017) questions the centrosome component loss reported in Azimzadeh. See above analysis.

---

## 13. Tollervey et al. (2025) — Cryo-ET Centrosome Atlas & Related Literature (2026-08-02)

> **Full analysis:** `docs/TOLLERVEY_2025_ANALYSIS.md`
> **Trigger:** Julia Mahamid correction — правильная статья Tollervey et al. Dev Cell 2025, не Fung et al.

### 12.1. Tollervey et al. (2025) — Key Findings

**Tollervey F, Rios MU, Zagoriy E, Woodruff JB, Mahamid J** — *Molecular architectures of centrosomes in C. elegans embryos visualized by cryo-electron tomography.* Dev Cell. 2025. PMID: **39721584**, PMCID: PMC11948214.

| # | Finding | Value for CEDAR |
|---|---------|----------------|
| 1 | Pseudo-timeline centriole assembly via cryo-ET | Methodological proof: cryo-ET CAN visualize structural changes |
| 2 | **Mother vs daughter centriole — distinct structural features** | Already structurally different within one cell cycle |
| 3 | 13 protofilaments (centriolar MT) vs 11 (PCM MT) | Architectural difference — potential oxidation target |
| 4 | Atypical γ-TuRC with 11-fold symmetry | Nucleation mechanism — potentially oxidation-sensitive |
| 5 | Porous, disordered PCM network | PCM as dynamic condensate — organization may change with age |
| ⚠️ | **NO young vs aged comparison** | Only developmental timeline. Age comparison = our experiment |

### 12.2. Guichard Lab — U-ExM Centriole Architecture

#### Laporte MH et al. (2024) Cell — CRITICAL
> *Time-series reconstruction of the molecular architecture of human centriole assembly.* Cell. 2024. PMID: **38604175**, PMCID: PMC11060037.

| Finding | Value for CEDAR |
|---------|----------------|
| Naked cartwheel forms FIRST (no MTs) | Cartwheel assembly = surface-catalysed from the very beginning |
| Bloom phase: MT blade assembly + radial separation + cartwheel growth | Cartwheel grows while attached to template — template distortion → distorted growth |
| 6 structural modules mapped (24 proteins) | Complete architectural map — any deviation measurable |
| **U-ExM method established** | Method ready at Guichard lab, BUT cartwheel disassembles at maturation in vertebrate mother centrioles (Guichard 2026) — valid readout for C. elegans / procentrioles only; for mature human centrioles measure persistent scaffold (triplets, inner scaffold POC1A/B/POC5, A-C linker) |

#### Bournonville L et al. (2025) Nat Commun — A-C Linker
> *The A-C linker controls centriole structural integrity and duplication.* Nat Commun. 2025. PMID: **40707486**.

| Finding | Value for CEDAR |
|---------|----------------|
| A-C linker proteins: **CCDC77, WDR67, MIIP** | Concrete molecular TARGET of oxidative damage |
| Depletion → triplet cohesion loss → proximal breakage | Oxidised A-C linker → triplet cohesion loss → cartwheel distortion |
| A-C linker + inner scaffold = joint architecture maintenance | Two structural modules, both oxidation-sensitive |

#### Brunet M et al. (2025) EMBO J — Alms1/Cartwheel Regulation
> *Drosophila Alms1 proteins regulate centriolar cartwheel assembly by enabling Plk4-Ana2 amplification loop.* EMBO J. 2025. PMID: **40021845**.

| Finding | Value for CEDAR |
|---------|----------------|
| **Alms1a/b → Plk4-Ana2 amplification → Sas-6 recruitment** | Molecular pathway: surface → cartwheel |
| Acute Alms1 loss → NO procentriole formation | Template quality essential for cartwheel assembly |
| Alms1a — PCM protein, loaded proximally at procentriole onset | PCM protein = interface between damaged template and cartwheel assembly |

### 12.3. Sullenberger et al. (2020) Cells — Centriole Maturation
> *With Age Comes Maturity: Biochemical and Structural Transformation of a Human Centriole in the Making.* Cells. 2020. PMID: **32526902**.

- Three centriole types: procentrioles, daughter, mother — structurally, biochemically, functionally distinct
- Age-dependent structural changes ALREADY documented (within cell cycle)
- **NO comparison of young vs aged organism** — gap = our hypothesis

### 12.4. Woodruff et al. (2017) Cell — PCM as Condensate
> *The Centrosome Is a Selective Condensate that Nucleates Microtubules by Concentrating Tubulin.* Cell. 2017. PMID: **28575670**.

- SPD-5 forms spherical condensates (in vitro reconstitution)
- PCM = selective phase → condensate properties may change under oxidative stress
- Woodruff JB = co-author of Tollervey 2025

### 12.5. Raheja et al. (2026) — Centriole → Senescence Link
> *A microprotein encoded by FERMT3 modulates endothelial cell protein catabolism and induces cell cycle arrest and senescence.* Cell Commun Signal. 2026. PMID: **42343301**.

- miP-FERMT3 localises to centriole subdistal appendages (ninein+, CEP170+)
- Induces centrosome amplification → cell cycle arrest → senescence
- **FERMT3 expression/protein abundance ↑ with age** in mouse and human endothelial cells
- Direct link: centriole-localised protein → senescence → aging

### 12.6. Field Readiness for CEDAR Experiment

| Component | Readiness | Who |
|-----------|-----------|-----|
| Method (U-ExM for cartwheel) | ✅ Ready | Guichard lab (Laporte 2024 Cell) |
| Cryo-ET centrosome protocol | ✅ Ready | Mahamid lab (Tollervey 2025 Dev Cell) |
| Molecular map (24 proteins) | ✅ Ready | Guichard lab |
| A-C linker targets identified | ✅ Ready | Bournonville 2025 Nat Commun |
| Cartwheel assembly pathway | ✅ Ready | Brunet 2025 EMBO J |
| Young vs aged comparison | ❌ NOT DONE | **Our experiment** |
| Carbonylation on centriolar proteins | ❌ NOT DONE | Needs redox proteomics |

**Conclusion:** The field is methodologically ready. All methods and molecular maps exist. The hypothesis is the missing piece — and Guichard lab has everything needed to test it.

---

## Добавлено 2026-09-30 — Repurposed Cell-Cycle Oscillators & Centriole as Organelle Hub

> Полный разбор: `docs/LITERATURE_REVIEW_2026-09-30_Oscillators_OrganelleHub.md`

### Новые подтверждения для CEDAR (C1/C2 + CEDAR-γ)

| Claim | PMID/DOI | Article | Verified | Strength |
|-------|----------|---------|:---:|:---:|
| Митотический осциллятор перепрофилируется на массовую продукцию центриолей без деления («митоз без митоза») | **28982797** | Al Jord et al. Calibrated mitotic oscillator drives motile ciliogenesis. *Science*. 2017 | ✅ 2026-09-30 | Strong |
| Альтернативный клеточный цикл координирует дифференцировку мультицилиарных клеток — «state-lock» через специализированный цикл | **38811726** | Choksi et al. An alternative cell cycle coordinates multiciliated cell differentiation. *Nature*. 2024 | ✅ 2026-09-30 | Strong |
| Амплификация центриолей возможна без центросомы (независимость de novo пути) | **31506528** | Vladar et al. Dynamics of centriole amplification in centrosome-depleted progenitors. *Sci Rep*. 2019 | ✅ 2026-09-30 | Strong |
| PIDDosome-p53 — детектор сверхчисленных центросом (QC числа центриолей) | **28130345** | Fava et al. The PIDDosome activates p53 in response to supernumerary centrosomes. *Genes Dev*. 2017 | ✅ 2026-09-30 | Strong |
| Избыточные центросомы → PIDD1-опосредованное воспаление/иммунонадзор | **37530438** | Extra centrosomes induce PIDD1-mediated inflammation. *Cell Rep*. 2023 | ✅ 2026-09-30 | Strong |
| Центросомная дисфункция — связь сенесценции и противоопухолевого иммунитета | **32606370** | Wu et al. Centrosome dysfunction: link between senescence and tumor immunity. *Signal Transduct Target Ther*. 2020 | ✅ 2026-09-30 | Strong |
| Амплификация центросом → RAC1 → автофагия (мост с MitoROS/протеостазом) | **42633651** | Connecting the dots: centrosome-RAC1 → autophagy. *Autophagy*. 2026 | ✅ 2026-09-30 | Moderate |

### MTOC как организатор митохондрий (ооцит, Yin & Spradling 2026)

| Claim | PMID/DOI | Article | Verified | Strength |
|-------|----------|---------|:---:|:---:|
| Balbiani body ре-ассемблируется вокруг MTOC; MTOC организует митохондриальный кластер → митохондриальная селекция | DOI 10.64898/2026.09.02.748954 | Yin Q, Spradling AC. *bioRxiv*. 2026 (препринт) | ✅ 2026-09-30 | Moderate (препринт) |
| Персистентные мейотические центросомы переживают quiescence (вопреки Szollosi 1972); Bb-MTOC — их потомок | (ibid.) | (ibid.) | ✅ 2026-09-30 | Moderate (препринт) |
| Независимое подтверждение структуры в растущих ооцитах (конвергенция) | **42664359** | Zollo et al. A transient RNP compartment. *Sci Adv*. 2026 | ✅ 2026-09-30 | Strong |
| Морфология/распределение/активность митохондрий при развитии ооцита | **38599901** | Mitochondrial morphology, distribution and activity during oocyte development. *Trends Endocrinol Metab*. 2024 | ✅ 2026-09-30 | Strong |

### Значение для CEDAR (концептуальные выводы)
1. **Два режима центриолярной динамики:** (а) шаблонная дупликация в делящихся клетках — носитель энтропии/деградации; (б) de novo/деутеросомная амплификация в постмитотических клетках — сброс геометрии. de novo-сброс недоступен самоподдерживающимся стволовым клеткам (согласуется с Renzova 2018: потеря центриолей → хаотичная дифференцировка).
2. **Подкрепление «germline reset»:** единственное полное обращение = устранение + de novo; ооцит делает то же для митохондрий (Bb-селекция) — природа «чинит» сбросом, а не починкой.
3. **Quiescence ≠ накопление:** персистентные центросомы ооцитов сохраняются через месяцы покоя → основная энтропия копится при делениях (division-компонента), а не от времени (time-компонента). Калибровка β vs division-компонент.
4. **QC числа существует (PIDDosome), QC качества единственной старой центриоли — нет:** старая «правильная» центриоль не отбраковывается → согласуется с селективным удержанием в стволовой клетке.
5. **Γ₁₃-мост (с MitoROS):** MTOC организует митохондрии → деградация центриоли → нарушение митохондриального контроля качества → рост D₃. Конкретный механизм связи Counter #1 ↔ Counter #3.

---

## Добавлено 2026-10-05 — Центросомная наследственность эмбриона × ооцитарная точность (аларт Semantic Scholar, 6 статей)

> **Полный разбор:** `docs/ANALYSIS_2026-10-05_CentrosomeInheritance_OocyteFidelity.md`
> **Источник:** еженедельный аларт Semantic Scholar (email_index 3-0-8…3-5-13). Все PMID/DOI верифицированы PubMed/Crossref 2026-10-05.
> **Итог:** CEDAR Counter #1 → **8.5/10**; Germline reset — молекулярно обоснована; M3/CAASM → 7.5/10; MitoROS Counter #3 → 9.0/10.

### Шесть статей аларта (идентификация)

| # | Статья | Тип | PMID/DOI | Статус |
|---|--------|-----|----------|--------|
| 1 | Lozano-Prieto, Chichón, Sánchez-Madrid — Centrosome isolation/proteomics in T-lymphoblasts | Протокол | 42681007 | ✅ ANALYSIS_2026-10-01 |
| 2 | Ju, Zhu, …, Qi — Dynamic centrosomes review | Обзор | 42806000 | ✅ ANALYSIS_2026-10-01 |
| 3 | Ortiz-Álvarez & Al Jord — Repurposing cell cycle oscillators for MCC differentiation | Глава | 42782440 | 🆕 |
| 4 | Yin & Spradling — Mouse oocyte Balbiani body reassembles | Препринт | DOI 10.64898/2026.09.02.748954 | ✅ MitoROS 2026-09-30 |
| 5 | Aparicio — Building Fidelity at the Beginning of Life | Тезис | DOI 10.33540/3770 | 🆕 |
| 6 | Sathananthan — Mitosis in human embryo: sperm centrosome | Обзор-классика | 9225167 | 🆕 |

### Ключевые подтверждения для CEDAR (C1/C2 + germline reset + M1)

| Claim | PMID/DOI | Статья | Verified | Strength |
|-------|----------|--------|:---:|:---:|
| CDK1→SAS-6 фосфорилирование разрушает фазовое разделение → картуэль → **элиминация центриолей в оогенезе** (первоисточник механизма Ju/Zhu) | **40410380** | Qi F et al. — Dynamic SAS-6 phosphorylation aids centrosome duplication and elimination in C. elegans oogenesis. *EMBO Rep* 26:3411 | ✅ 2026-10-05 | Strong |
| CDK1-cyclin B — разборка картуэля в **человеческих** клетках → disengagement | **36356903** | Huang F et al. *J Biol Chem* 298:102658 | ✅ 2026-10-05 | Strong |
| **huoMTOC** (TACC3, CCP110, CKAP5, DISC1) — ацентриолярный MTOC ооцита человека → кинеетохорная нуклеация | **36395215** | Wu T et al. *Science* 378:eabq7361 | ✅ 2026-10-05 | Strong |
| Ацентросомное веретено: RanGTP/HURP жидкоподобный домен | **31249032** | So C et al. *Science* 364:eaat9557 | ✅ 2026-10-05 | Strong |
| Дистальная **атипичная** центриоль сперматозоида функциональна — вторая центриоля зиготы (ремоделирование дистальной) | **29880810** | Fishman EL et al. *Nat Commun* 9:2210 | ✅ 2026-10-05 | Strong |
| КРС — модель человеческой центросомной наследственности; пересмотр «одной центриоли» | **37713143** | Avidor-Reiss & Uzbekov. *J Assist Reprod Genet* 40:2539 | ✅ 2026-10-05 | Strong |
| Цикл биогенеза центриоли = **автономный осциллятор**, сопряжённый с клеточным циклом (соседняя глава того же тома, что и №3) | **42782439** | Ow C, Aydogan MG. *Adv Exp Med Biol* 1521:15–42 | ✅ 2026-10-05 | Strong |
| Перепрофилирование цикловых осцилляторов → массовая центриольная амплификация в МЦК без деления | **42782440** | Ortiz-Álvarez & Al Jord. *Adv Exp Med Biol* 1521:43–55 | ✅ 2026-10-05 | Strong |
| Отцовская центриоль — основа всех центриолей эмбриона до бластоцисты; яйцеклетка с неактивной центросомой; диспеpмия → триполярность | **9225167** | Sathananthan AH. *Histol Histopathol* 12:827 | ✅ 2026-10-05 | Strong |
| Первое митотическое деление — «опасный мост»; центросомы ex novo из обоих родителей | **37027836** | Coticchio G et al. *Hum Reprod* 38:1019 | ✅ 2026-10-05 | Strong |
| Переключение мейоз→митоз (веретено, центросомы, хромосомы) | **42647160** | Cantwell H. *Mol Biol Cell* 37:pe4 | ✅ 2026-10-05 | Moderate (perspective) |

### Концептуальные выводы для CEDAR
1. **Germline reset — молекулярный каркас готов:** элиминация (CDK1→SAS-6 LLPS) + ацентросомное окно (huoMTOC/aMTOC) + отцовская ре-инициация (sperm aster) + амплификация (осцилляторы МЦК). Природа «чинит» центриоли ТОЛЬКО сбросом.
2. **«D≈0» зиготы уточняется:** зигота начинает с отцовской центриоли (проксимальная каноническая + ремоделированная дистальная). **Новый фальсификатор:** polyGlu/окислительные маркеры на проксимальной центриоли зиготы коррелируют с возрастом отца? (КРС-модель.)
3. **Центриоль как часы:** цикл биогенеза центриоли — автономный осциллятор (Ow 2026), сопряжённый с клеточным циклом → уточнение уравнения времени CEDAR (S₀+β·t + осцилляторная сопряжённость).
4. **M1/QC:** триполярность при диспеpмии + центросомы критичны со 2-го деления (Aparicio/Wu) — поддержка QC числа и M1.
5. **M3/CAASM +:** huoMTOC-белки и отцовская PCM — ацентриолярные MTOC-платформы индукторов.
6. **Протокол для «Центросомный IP-MS» (CEDAR §7.9):** Lozano-Prieto 2026 (T-лимфобласты, near-physiological) — готовый метод выделения центросомальных фракций для TMT-MS молодых vs старых клеток.

### Слабые места (резюме)
- Ооцитарный сброс ≠ соматический (нет элиминации во взрослых стволовых клетках; huoMTOC — только мейоз).
- SAS-6 LLPS — преимущественно беспозвоночные; перенос на человека косвенный.
- **Возраст отцовской центриоли не изучен** — возможный перенос отцовской энтропии в зиготу (пробел группы).
- Тезис Aparicio — серая литература (первоисточники: Wu 2022, So 2019); МЦК-амплификация ≠ точная 9-кратная реконструкция.

---

## Добавлено 2026-10-07 — CA в диабете (Lee 2026): стресс-триггер CA, quenching-gap и уточнение SASP

> **Первоисточник:** Lee SC, Xu P, Wu QG (2026). Front Cell Dev Biol 14:1877870 — DOI 10.3389/fcell.2026.1877870 (Perspective, OA).
> **Полный разбор:** `~/Desktop/PhD/docs/ANALYSIS_2026-10-07_Lee2026_CA_Diabetes_DeepDive.md` · **Все PMID верифицированы** (2026-10-07).

### Что это даёт CEDAR

| Аспект | Утверждение | PMID | Verified | Сила |
|---|---|---|---|---|
| **Стресс-индуцированная CA (не только возрастная)** | Диабет/AGEs/глюкоза триггерят CA (AGEs→PLK4↑) — CA как *индуцируемый*, а не только аккумулируемый дефект → поддержка «integrator»-версии (damage + stress) и H₂O₂-ветки Эксперимента B | 37788590 (Zhang 2023); 31618077 (He 2020); 10.3389/fcell.2026.1877870 | ✅ | Moderate (Perspective; человеч. причинность не доказана) |
| **CA → геномная нестабильность = мост Counter #1 ↔ #2** | Мультиполярный митоз → анеуплоидия/микронуклеусы/kataegis; CEDAR: D_cent → мутации | 19506557; 18555779; 28132847 | ✅ | **Strong** |
| **CA → inflammation (ECASP/variant-SASP)** | PIDDosome→NF-κB + NADPH-ROS→NRF2→ECASP; PLK4-KD ↓IL-6/TNF-α/IL-1β (причинность in vitro) | 37530438 (Garcia-Carpio); 30458137 (Arnandis); 36660875 (Wu 2023) | ✅ | **Strong** |
| **⚠️ STING-уточнение** | Variant SASP от CA: **HIF-1α-зависим, НЕ зависит от микронуклеусов–cGAS-STING** | **41296514** (Ho 2026, Mol Biol Cell) | ✅ 2026-10-07 | Strong — не цитировать STING-ветвь как универсальную |
| **⚠️ Коррекция Wu 2023** | Correction to «variant SASP induced by centrosome amplification» (Aging Cell) | **42203735** | ✅ | — |
| **CA → аберрантный цилиогенез** | Ацилиация/мультицилиация; потеря Hh/Wnt/PDGFR → дерепрессия пролиферации → **поддержка Line 2 (цилиарная сигнальная энтропия)** | 22840514 (Mahjoub); 42691060 (Ran 2026, гипергликемия→цилиарная диссэссемблия) | ✅ | Strong |
| **Quenching-gap = наше место** | Авторы: «mechanisms must exist to quench these cycles» — не названы. CEDAR даёт: автофагия (42324259), PIDDosome-p53 QC числа (28130345), элиминация/сброс деглутамилазами | — | ✅ | Opportunity |
| **QC числа есть, QC качества старой центриоли — нет** | PIDDosome-p53 детектирует *сверхчисленные* центросомы; старая «правильная» центриоль не отбраковывается → селективное удержание в стволовой клетке согласуется | 28130345 (Fava 2017) | ✅ (2026-09-30) | Strong |

### Независимое подтверждение AGE→CA (вне группы Lee)
| PMID | Работа | Value |
|---|---|---|
| **41401801** | Hu 2025, J Agric Food Chem — **CML (диетический AGE) → CA → гепатокарциногенез** | ✅ Независимая лаба |

### Интервенция (для фазы Eliminate / Proposal)
| PMID | Работа | Value |
|---|---|---|
| **42220890** | Centrinone-based **PLK4-деградаторы** (RSC Chem Biol 2026) | Precision-средство сброса CA |
| **42288516** | PLK4i RP-1664, нейробластома (Nat Commun 2026) | PLK4i в доклинике |
| **40865591** | **β-гидроксибутират ингибирует дупликацию центриолей** (Cell Signal 2025) | Метаболический рычаг (кетоз) → кандидат для H₂O₂-ветки как контр-интервенция |

### Итог
**CEDAR усиливается по двум осям:** (1) CA — конкретный *индуцируемый* путь, соединяющий Counter #1 с мутационным счётчиком и inflammaging (ECASP/variant-SASP); (2) нерешённый quenching-gap — прямое поле для CEDAR (автофагия + QC + элиминация). Общая оценка CEDAR остаётся **8.5/10** (Counter #1) с добавленным **+0.2** за стресс-индуцируемость и **+0.2** за цилиарную ветвь (Line 2).

---

## Добавлено 2026-10-10 — OPTN-трафик центросома↔мидбоди (Chakraborty 2026, bioRxiv): независимое подтверждение асимметрии мать/дочь и MT-независимой PCM-сборки

> **Первоисточник:** Chakraborty A, Kundu K, Weil D, Kundu B (2026). *Tracing the dynamic recruitment trajectory of Optineurin between the centrosome and other macromolecular assemblies, signifying its role in cell cycle progression.* bioRxiv, DOI `10.64898/2026.09.30.755612` (препринт, posted 2026-10-06). Semantic Scholar `00b9f15d34716bf6cba9cc085bbb1edb299366ce`. **Компаньон:** Chakraborty 2025, *Cytoskeleton*, DOI `10.1002/cm.70015`.
> **Полный разбор:** `~/Desktop/LC/MCARA/CEDAR/docs/ANALYSIS_2026-10-10_OPTN_Centrosome_Trafficking.md` · **Все PMID/DOI верифицированы 2026-10-10** (PubMed/OpenAlex/Semantic Scholar).
> **Команда:** IIT Delhi + Sorbonne/CNRS/Inserm (D. Weil, лаб. «Développement Adaptation et Vieillissement» — старения).

### Что это даёт CEDAR

| Аспект | Утверждение | Ссылка | Verified | Сила |
|---|---|---|---|---|
| **Асимметрия мать/дочь = новый молекулярный маркёр** | OPTN предпочтительно на **материнской** центриоли на границе G1/S (асимметрия 100×[(H−L)/H]), балансировка в S-фазе — совпадает с сепарацией дочерней центриоли. Подтверждает тезис «Mother and Daughter Centrioles Are Not Equivalent» (собств. работа) | `10.64898/2026.09.30.755612` + `10.65649/75nq9t08` | ✅ 2026-10-10 | **Strong** (независимая лаба, HeLa+HEK293) |
| **MT-независимая сборка PCM (скаффолд) + MT-зависимый выход** | Рекрутмент/удержание OPTN в PCM не требует МТ (холод 30′, нокодазол 0.5 мкМ); выход — по МТ (радиальная «звезда» вдоль отрастающих МТ при washout, 54.2% колок. с β-тубулином). Устоявшаяся парадигма: Khodjakov & Rieder 1999; Woodruff 2017; Nasa 2017 | 28575670; 28188792; `10.64898/2026.09.30.755612` | ✅ | **Strong** — стабильный фундамент CEDAR |
| **Зрелость PCM — условие рекрутмента** | OPTN появляется на центросоме по мере созревания PCM в G1 (12 ч), НЕ на ранних split centrioles (>0.5 мкм) | ibid. | ✅ | Strong |
| **Митотический дефект ↔ ND (ALS/POAG)** | OPTN-деплеция → митотические дефекты; OPTN-мутации → ALS/POAG; дефектное деление → нейродегенерация (Joseph 2020) | 22365832 (Kachaner 2012); 20428114 (Maruyama 2010); 11834836 (Rezaie 2002) | ✅ | Supportive |
| **Мидбоди = таймер/счётчик (не «мусор»)**: мидбоди-инхеритенс → ре-вход в квисценцию в HSC | OPTN трафикуется через мидбоди; Fukushima 2026: мидбоди-релиз ↔ задержка следующего деления HSC | **42702233** | ✅ | Strong (независимо) |
| **PTM-регуляция центросомного трафика** | Фосфорилирование OPTN Plk1 (Kachaner 2012) + Cyclophilin A (2023, PMID 36691345) — конвергентный паттерн «выпуск из центросомы в митозе» | 22365832; 36691345 | ✅ | Supportive |

### Независимое подтверждение тезисов CEDAR (конвергенция маркёров асимметрии)
| Маркёр/механизм | Система | Год | PMID/DOI |
|---|---|---|---|
| **OPTN (G1/S, мать)** | Человек (HeLa/HEK293) | 2026 | `10.64898/2026.09.30.755612` |
| Cnn → асимметрия размера центросомы | Drosophila нейробласты | 2010 | 21145745 |
| Polo/Centrobin → асимметрия в раннем митозе | Drosophila | 2020 | 32760088 |
| **PP4 → асимметрия центросом** | Drosophila нейральные стволовые | 2025 | 40072519; 39868139 |
| Bld10/Cep135 → асимметрия | Drosophila | 2014 | 24954048 |
| Мать→GSC судьба (Yamashita) | Drosophila | 2007 | 17255513 |
| Мать→нейральные прогениторы человека | Человек | 2023 | Royall eLife 12:e83157 |
| Мать→CD8+ T-клетки памяти | Мышь | 2025 | 39764850 |

### Слабые места (честная оценка)
- **Препринт без peer-review** (bioRxiv 06.10.2026) — цитировать осторожно.
- **Только описательная микроскопия:** нет нокдауна/нокина, нет функционального следствия (авторы сами: «Ideally, a direct functional assignment...»).
- **Нет живого тайм-лапса одного митоза** — траектория реконструирована по разным клеткам на стадиях (авторы честно это отмечают, Fig 3b).
- **Небольшие n** (18–42 центросом/стадия; 30 клеток line-scan), только t-test без поправок на множественность.
- **Нормировка на γ-тубулин** (сам колеблется в митозе) — возможен артефакт.
- **Механизм PTM/PCM-якоря не доказан** → открытое окно для CEDAR.

### Итог
**CEDAR подтверждён независимой лабораторией по трём осям:** (1) асимметрия мать/дочь (новый маркёр OPTN, человек), (2) MT-независимая PCM-сборка (скаффолд + конденсат), (3) мидбоди-таймер/квисценция HSC. Общая оценка CEDAR: **8.8/10** (+0.3 за независимое подтверждение асимметрии у человека). ⚠️ Не смешивать динамический трафик OPTN (реверсибельный) с необратимым энтропийным накоплением в центриоли — это разные процессы; OPTN = маркёр/рид-аут, а не механизм накопления.

## §. Эпистемический стандарт (Noepedia Harvest — 2026-10-12)
> Мастер: `~/Desktop/Services/docs/NOEPEDIA_HARVEST_2026-10-12.md`
1. **K-лестница (K0–K9) вместо float-уверенности:** у каждого claim — ступень (адрес/PMID → сертификат → кандидат → два независимых свидетеля → процедура); карта на GRADE (PMID 18436948) + Murad-пирамиду (PMID 27339128) + NASA TRL.
2. **Два независимых свидетеля на ключевые claims:** ≥2 независимых источника/модальности (PMID + механизм + данные), не два PMID из одного пула.
3. **REQUIRED_ROLE (сформированный неизвестный):** пробелы пути = «отношение требует участника, имя неизвестно» (киназа/субстрат/лиганд/регулятор) — OPEN с ограничениями + правилом валидации, а не «нет данных» (абдукция arXiv 2604.08016, 2607.18564; orphan-канон). Без валидации — апорофения.
4. **Provenance для AI-гипотез:** метка происхождения (модель/шаг/источник); потеря метки = дефект (HART arXiv 2603.05828).

---

## 🔥 NEW 2026-10-12 — Terminal Acentriolar Lock (TAL) across tissues: сердце + мозг + скелетная мышца (мульти-тканевая конвергенция H4f)

> **Триггер:** два текста Джабы (сердце + мозг). **Мастер-анализ (PMID верифицированы Europe PMC 2026-10-12):** `~/Desktop/PhD/docs/ANALYSIS_2026-10-12_HeartBrain_AcentriolarLock_Comparative.md`

### Природный эксперимент расширен: «центриоль как гейт/счётчик деления» подтверждается в 4+ тканях
| Ткань | Судьба центросомы/центриолей | Новый ЦОМТ | PMID (верифицированы) |
|-------|------------------------------|-----------|:---------------------:|
| **Кардиомиоциты (млекопитающие)** | Полная редукция после рождения | Ядерная оболочка (NE-MTOC, AKAP6) | 26247711, 32946803, 33295871 |
| **Нейроны (мозг)** | Центросома теряет MTOC-роль; центриоли в большинстве сохранены, но «выключены»; в зернистых клетках мозжечка — ресничка разрушена навсегда, центриоль кэпируется CEP97 | Гольджи + дендритные ncMTOC (augmin/γ-TuRC) | 42481875, 42405626, 38587100, 39705308, 39137043 |
| **Скелетная мышца (миотубы)** | Центросома теряется при слиянии; при дедифференцировке заново обретается (p53-зависимо) | — (саркомерный/субсинаптический MTOC) | 42786221 |
| **Адипоциты** | Целостность центриоли гейтит дифференцировку (POC5) | — | 42507085 |
| **Регенераторы (рыбы/амфибии/рептилии)** | Центросомы сохранены → пролиферация/регенерация | — | 27812722, 42448327, 41986369, 42097640, 41559073 |

### Ключевые новые факты для CEDAR (H4f — centriole-as-division-gate)
1. **⭐⭐ Скелетная мышца — прямой функциональный тест (42786221, npj Regen Med 2026, Simon lab):** у тритона аброгация центросом блокирует дедифференцировку мышц и формирование бластемы; млекопитающие миотубы **заново обретают центросомы через ингибирование p53**. → «Регенератору нужна центросома» — показано функционально, а не только коррелятивно.
2. **⭐⭐ «Кэппинг» центриоли CEP97 в зернистых клетках мозжечка (39705308, PNAS 2024; 39137043, JCB 2024):** ресничка разрушается навсегда (деконструкция), материнская центриоль кэпируется → не может служить базальным тельцом. **Молекулярный спектр состояний центриоли:** сохранение → кэппинг → деконструкция реснички → полная элиминация (кардиомиоцит). Это уточняет «градацию счётчика» в CEDAR.
3. **⭐⭐ Регенерация коры аксолотля (41559073, Nat Commun 2026, Fei JF lab):** эпендимоглиальные клетки — единственный источник регенерации ЦНС; после абляции >95% постмитотических корковых нейронов все подтипы восстанавливаются хронологически с ламинарным распределением. Контраст с мозгом млекопитающих (пул НСК есть, но замок — 42118098).
4. **PLK4: локализация, а не уровень экспрессии — маркер пластичности (42786221)** — новый рид-аут; мост к линии centrinone/PLK4 (MCARA) и к центросомной амплификации (Lee 2026).

### Слабые места (честная оценка)
- **Птицы — НЕТ прямых данных** (см. анализ §1.2): заявление «птицы теряют центриоли, как млекопитающие» не имеет первоисточника; архитектура птичьих CM (Shiels 2022, 36189815) ближе к эктотермам.
- **«Все млекопитающие»/«КРС» — переобобщение** (данные: мышь/крыса/человек hPSC).
- **«Кэппинг» документирован пока для зернистых клеток**; универсальность «инактивации» по всем нейронам — гипотеза (клетки Пуркинье — атрибуция НЕ подтверждена).
- **Регенерация мозга у холоднокровных идёт через НСК/EGC, а не через зрелые нейроны** (переоценка в тексте).
- Каузальность «потеря центросомы → G0» в сердце: Zebrowski «can promote», Ng — острая фармакологическая (см. ANALYSIS_2026-10-15 §3).

### Rebuild-прецедент для LERR (важно для Track C / StemEntropy)
- **p53-зависимое обретение центросом в дедифференцирующихся миотубах (42786221)** = валидированный аналог фазы Rebuild без импорта; согласуется с идеей Centriole Import (2026-10-15) и с правилом «лицензирование ≤2, иначе мультиполярность/CIN» (Leone & Engel 2019, 31301302).

---

## 🔥 NEW 2026-10-16 — Demon Core Transfer → pipeline-аудит (cell_dt_audit)

> **Мастер:** `docs/DEMON_CORE_TRANSFER_2026-10-16.md`. **Источник принципов:** Noepedia Demon Core
> Harvest (DAIS `docs/NOEPEDIA_DEMON_CORE_HARVEST_2026-10-16.md`, DCT01–DCT20, DEMON_CORE_SPEC).
> **Код:** `crates/cell_dt_audit` (Rust; зависимость только hmac/sha2). **Статус:** реализован +
> 7 тестов (6 unit + 1 первый сквозной прогон), 0 clippy-замечаний.

### Перенос исполнительной дисциплины демона на конвейер CEDAR
Волна-2 Noepedia (K-лестница, два свидетеля, REQUIRED_ROLE) уже в CEDAR. Волна-3 добавляет
**как конвейер обязан останавливаться, не выдумывать, не затирать историю и не самолегитимизироваться**:

| Принцип (демон-ядро, DCT) | Реализация в `cell_dt_audit` | Тест |
|---|---|---|
| Non-destructive MARK / история ≠ текущее (DCT11–12) | `AuditRecord` (stage/epoch/scope) + `PipelineAudit::mark_stage`; `history_of()` хранит все прогоны | эпохи 1→2, история из 2 записей |
| STOP/NEED + STOP CARD (DEMON §7–8) | `RegimeState` (GO/ASK/BLOCK/NEED), `NeedClass` (12), `StageStopCard` | стадия без политики → BLOCK, не фабрикует |
| Transaction membrane (DCT08–09) | `PipelineAudit::membrane_check` — прогон не авторизуется параметром, который сам вводит | `SELF_AUTHORIZATION` отклонён; внешний параметр разрешён |
| Dependency policy в сети (DCT05–07) | `StageDependencyPolicy` (ROLE=POLICY) + `check_dependency` | без политики → NEED/MISSING_RULE; с политикой → проход |
| Composition semantics (DCT19–20) | `CompositionPolicy` + `evaluate_composition` (ALTERNATIVE/JOINT) | один датасет challenged: ALTERNATIVE выживает, JOINT падает |
| TRACE CONTINUITY + подпись журнала (DCT10–12, P2) | `AuditJournal` (archive/compact/`check_trace_continuity`) + `JournalGuard` (HMAC-SHA256) | честный трейл верифицируется; изменение ломает подпись; компактирование сохраняет адреса |

### Отличия от DAIS (сознательные)
- **HMAC-SHA256 вместо Ed25519**: локальный трейл анализа — tamper-evidence достаточно;
  сертификация/мульти-сторонний аудит потребовал бы Ed25519 (как в DAIS) — зафиксировано как слабое место §5.
- **In-memory, без БД/сети** (проект «analysis only»); интеграция с реальными стадиями
  `cell_dt_validation`/`cell_dt_cli` — следующий шаг.

### Слабые места (честно, детали в мастер-документе §5)
- Dependency-граф захардкожен в коде (не data-driven); политики сравниваются строково;
  `StageStopCard` проще, чем в DAIS (нет списков provenance/scope в карте);
  «молчание ≠ поддержка» сейчас структурно, автоматического конвейер-проверятеля ещё нет;
  реальные MCMC-прогоны через аудит-слой не проверены (кран стоит отдельно).
