# EIC Pathfinder Part B — консолидированный отчёт независимого peer review

**Дата:** 2026-04-21
**Рецензент:** DeepSeek-reasoner с официальной рубрикой EIC Pathfinder
**Дедлайн подачи:** 2026-05-12 (21 день)

## Общий вердикт по текущей версии

| Документ | Вердикт | EX | IM | IMPL |
|----------|---------|----|----|------|
| EIC_PartB_v2_FULL | **REJECT** | 2/5 | 3/5 | 2/5 |
| Section 2 (Impact) | MAJOR REVISION | — | — | — |
| Section 3 (Implementation) | **REJECT** | 2/5 | 2/5 | 2/5 |
| WP1 | MAJOR REVISION | — | — | — |
| WP2 | MAJOR REVISION | — | — | — |
| WP3 (centriole rejuvenation) | **MAJOR REVISION** (на грани REJECT) | — | — | — |
| WP4 | MAJOR REVISION | — | — | — |
| WP5 (clinical pilot) | MAJOR REVISION | — | — | — |

**Интегральный прогноз: В текущем виде заявка будет отклонена EIC.**

## Критические блокеры (требуют устранения ДО подачи)

### 1. Консорциум не соответствует правилам EIC (Flag 6)
В тексте упоминаются только Грузия, Германия (MPI), Куба. Правила EIC Pathfinder требуют:
- Минимум 3 независимых юридических лица
- Из минимум 3 разных EU member states ИЛИ associated countries
- Грузия **не** является associated country Horizon Europe
- Клинический пилот в Грузии без EU partner — административный основание для отсева

**Действие:** срочно пригласить EU-based партнёров. Кандидаты уже обозначенные:
- Hartmut Geiger (Ulm) — WP3 HSC эксперимент (`project_cdata_copi_candidates`)
- Shashviashvili (Грузия) — ok, но нужен ещё EU clinical partner

### 2. Недостаточная peer-reviewed поддержка ключевых гипотез

Reviewers специально отметили:
- **CDATA** (centriolar damage accumulation) — заявляется как прорыв, но опирается на 10 PubMed работ Tkemaladze/Chichinadze (см. `pubmed_authoritative.md`) + группу работ, которые в ранее упомянутых источниках помечены как PMID mismatches (Conduit & Raff 2010, Janke 2020, Terman 2004 — см. `CORRECTIONS_NEEDED_CDATA.md`)
- **χ_Ze** — помечается reviewers как "speculative, validated misrepresentation" (согласно `project_fclc_peer_review` v10 — 1.86/5 REJECT тоже упоминал это)
- **Polyglutamylation асимметрия в HSC** — one primary reference (Mao 2024 in *Nature Aging*) упоминается, больше нет

**Действие:** Round 2 CORRECTIONS_NEEDED должен быть отработан вручную; нужна мета-аналитическая секция с 20+ PubMed-верифицированными ссылками на асимметрическое наследование, polyglutamylation aging, центриолярные PTMs.

### 3. Section 3 (Implementation) полный REJECT

Основания:
- Полное отсутствие ссылок на рецензируемую литературу
- Нет подтверждённого консорциума
- Нереалистичный бюджет
- Нет risk management секции
- Decision Gates без качественных критериев
- Use of "внутри страны" fallbacks — неприемлемо для EIC

**Действие:** Section 3 переписать с нуля после формирования консорциума.

### 4. WP3 (центральный биологический прорыв) — слабейший

- Нет peer-reviewed обоснования центральной гипотезы
- Нет plan B если primary experiment fails
- Нет описания модели мышей, контроля, статистической мощности
- WP3 — экзистенциальный риск для проекта

**Действие:** сотрудничество с Geiger (Ulm) срочно; добавить подробный protocol с pre-registration на OSF; power calculation; два backup experiments.

### 5. WP4 (25-35 Hz нейроморфная архитектура)

- Нет peer-reviewed обоснования для диапазона 25-35 Hz
- Выбор нейроморфной архитектуры не аргументирован
- Нет market analysis / competitive assessment

**Действие:** либо обосновать литературой, либо упростить до конвенциональной FPGA архитектуры.

### 6. WP5 (клинический пилот)

- Нет EU clinical partner (только Shashviashvili в Грузии)
- Бюджет не detailed
- Нет plan консультаций с EMA
- Protocol не связан с центральной CDATA/χ_Ze гипотезой — пилот по железодефициту

**Действие:** либо заменить пилот на более релевантный (centriolar biomarkers в старении), либо аргументировать связь IDA с χ_Ze (возможно? iron → oxidative stress → centriolar damage).

## Средней критичности замечания

- **Over-claiming в количествах** (ε≤1.0, r>0.85, p<0.01) без preliminary calibration data
- **Weak dissemination / exploitation strategy** — только "open-source release" не достаточно
- **Self-citation ≥15%** risk для MCAOA Perspective (по рубрике Nature)
- **Longevity Horizon / Georgian Scientists** не считаются peer-reviewed (см. `pubmed_authoritative.md`)

## Рекомендованные действия (prioritized)

**На этой неделе (до 2026-04-27):**
1. Сформировать EU-complaint консорциум: ответ от Hartmut Geiger; plus 2nd EU partner (кандидаты из existing рецензентов?)
2. Отработать все CORRECTIONS_NEEDED из /tmp/audit_v2/reports/ — исправить реальные PMID или удалить некорректные ссылки
3. Нанять внешнего English editor для Part B (см. `user_tkemaladze_profile` — "needs English editor for IF>5")

**Неделя 2 (2026-04-28 ... 2026-05-03):**
4. WP3 переписать: full protocol, pre-registered на OSF, power calc, plan B/C
5. Section 3 Implementation с нуля: новый консорциум, risk management, Gantt, budget breakdown
6. Добавить 20+ peer-reviewed references для CDATA обоснования (это требует meta-analysis — в работе)

**Неделя 3 (2026-05-04 ... 2026-05-11):**
7. Revised full draft внутренний peer review
8. Submit 2026-05-12

## Путь к приемлемой версии: оценка реалистичности

При предпринимаемых полных изменениях заявка может достичь уровня **MAJOR ACCEPTABLE** (EX 3-4/5, IM 3-4/5, IMPL 3-4/5).
Это всё ещё ниже конкурентного порога EIC Pathfinder (успешные заявки обычно 4-5/5).

**Альтернатива:** перенести подачу на следующий cycle EIC Pathfinder (вероятно 2026-Q3/Q4) с полной доработкой. Это даст время на:
- Hartmut Geiger fibroblast experiment — получить preliminary data (6 месяцев)
- Mao 2024 data replication/extension
- Создание настоящего EU-based консорциума с clinical partner
- Набор pilot data на BioSense хотя бы 50 пациентов

При срочной подаче 2026-05-12 — **вероятность успеха: 1-3%**
При подаче через 6-12 месяцев с preliminary data — **вероятность успеха: 8-15%**

## Полные тексты рецензий

- [EIC_PartB_v2_FULL_review.md](./EIC_PartB_v2_FULL_review.md)
- [EIC_PartB_v2_Section2_IMPACT_review.md](./EIC_PartB_v2_Section2_IMPACT_review.md)
- [EIC_PartB_v2_Section3_IMPLEMENTATION_review.md](./EIC_PartB_v2_Section3_IMPLEMENTATION_review.md)
- [EIC_PartB_WP1_detailed_review.md](./EIC_PartB_WP1_detailed_review.md)
- [EIC_PartB_WP2_detailed_review.md](./EIC_PartB_WP2_detailed_review.md)
- [EIC_PartB_WP3_detailed_review.md](./EIC_PartB_WP3_detailed_review.md)
- [EIC_PartB_WP4_detailed_review.md](./EIC_PartB_WP4_detailed_review.md)
- [EIC_PartB_WP5_detailed_review.md](./EIC_PartB_WP5_detailed_review.md)
