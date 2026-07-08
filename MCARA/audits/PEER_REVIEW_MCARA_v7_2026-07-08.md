# PEER REVIEW: MCARA v7 → Biogerontology

**Рецензент:** pi (GATE mode)
**Дата:** 2026-07-08
**Статья:** «The Centriole as the Organelle of Irreversible Differentiation»
**ID:** 7cc6de62 · Подана 3 июн 2026 · With Editor 5 недель

---

## OVERALL ASSESSMENT: CONDITIONAL ACCEPT (Major Revisions)

**Score: 7.2/10**

Статья представляет оригинальную, хорошо аргументированную гипотезу. Cross-species evidence (C. elegans, Drosophila, planarians, human) убедительна. Эксперимент фальсифицируем. Однако отсутствие прямых экспериментальных данных и несколько спекулятивных связей требуют доработки.

---

## ПО КАТЕГОРИЯМ

### 1. НОВИЗНА: 8/10 ✅

**Сильные стороны:**
- Гипотеза «центриоль = органелла необратимой дифференцировки» — оригинальна
- polyE как индекс отношения центриоль/цилия — новая измеримая величина
- Концепция «центриоль/цилия ratio» как структурный индекс коммитмента
- Объединение 4 модельных организмов под единую рамку

**Слабость:**
- Статья позиционируется как hypothesis, но структурирована как research article с experiment design. Это создаёт tension: рецензент ожидает experimental data, а их нет. Нужно явно обозначить жанр: «Hypothesis Article» или «Perspective».

**Рекомендация:** Добавить в заголовок или abstract: «A Hypothesis and Experimental Framework».

### 2. ОБОСНОВАННОСТЬ: 6/10 ⚠️

**Сильные стороны:**
- 25 PMIDs верифицированы
- Ключевые эксперименты (Renzova 2018, Yamashita 2007, Raheja 2026) релевантны
- Логическая цепочка: centriole age → polyE → centriole/cilium ratio → commitment

**Слабые стороны (КРИТИЧЕСКИЕ):**
1. **polyE-to-ratio hypothesis — центральная, но не подтверждена.** Автор честно признаёт: «No direct experimental data exist.» Но на этой гипотезе держится весь механизм. Без неё статья — speculation.
2. **Связь FERMT3 → senescence → centriole age непрямая.** Raheja 2026 показывает, что FERMT3 вызывает senescence. Но что FERMT3 экспрессия КОРРЕЛИРУЕТ с centriole age — не показано.
3. **C. elegans evidence — eliminator mechanism неизвестен.** Автор цитирует Kalbfuss & Gönczy 2023 — «mechanism of elimination unknown.» Это ослабляет аргумент.

**Вопрос рецензента:** Что именно causal: центриольный возраст ИЛИ истощение пула стволовых клеток? Может, центриоль — passenger, а не driver?

### 3. ЭКСПЕРИМЕНТАЛЬНЫЙ ДИЗАЙН: 7/10 ⚠️

**Сильные стороны:**
- Чёткий gate-test
- FACS enrichment Cep135⁻ — хорошо
- Ортогональные методы (centrinone + STIL shRNA)
- Временная p53 супрессия — решает проблему Renzova
- Falsification criterion: ≤2× → hypothesis disconfirmed

**Слабые стороны:**
1. **5-10× увеличение iPSC — слишком смелое.** Без preliminary data. Даже OSKM + hypoxia дают ~5× improvement (Yoshida 2009). Автор предсказывает эффект порядка OSKM+hypoxia только от удаления центриоли. Обоснование недостаточно.
2. **p53 suppression window (48h) — слишком короткий?** Центриоль теряется за 3 дня центринона. OSKM занимает 7-14 дней до первых колоний. Окно в 48 часов p53-супрессии — клетка может не успеть завершить репрограммирование.
3. **Контроль: PLK4-resistant mutant.** Отличная идея. Но нет деталей: какая именно мутация? Renzova 2018 использует PLK4^as (analog-sensitive), не resistant. Уточнить.
4. **SILAC centriole age — «optional, high-risk».** Если это ключевое доказательство концепции, оно не может быть опциональным. Нужно либо включить в основной дизайн, либо явно обозначить как Phase 2.

### 4. ПЕРЕКРЁСТНЫЕ ДОКАЗАТЕЛЬСТВА: 7/10 ⚠️

**Сильные стороны:**
- C. elegans: элиминация центриолей = потеря пластичности ✅
- Drosophila: удержание центриоли = стерильность ✅
- Planarians: нет центриолей = максимальная пластичность ✅
- Human: FERMT3 → senescence ✅

**Слабые стороны:**
1. **Planarian evidence — логическая ошибка.** Автор утверждает: «Neoblasts are centrosome-free → maximally plastic.» Но correlation ≠ causation. Neoblasts также не имеют многих other organelles. Это weak evidence.
2. **Fertilization paragraph (5.4) — слишком короткий.** Maternal centriole elimination — важный аргумент. Нужен как минимум один PMID (Schatten & Sun 2011, Dev Dyn).

### 5. ЯСНОСТЬ И СТРУКТУРА: 8/10 ✅

Хорошо написано. ASCII-диаграмма centriole/cilium состояний отличная. Evidence table с 25 PMIDs — очень полезно. Есть limitations section.

**Мелкие замечания:**
- Abstract: «polyE is an index of centriole-to-cilium length ratio» — не раскрыто, ПОЧЕМУ polyE отражает именно ratio. Добавить одно предложение.
- Section 6.1: «the ratio may be the physical signal» — «may be» слишком часто. Заменить на более уверенную формулировку или убрать.

---

## MAJOR REVISIONS (ОБЯЗАТЕЛЬНО)

| # | Требование | Приоритет |
|---|-----------|:---------:|
| R1 | **Обозначить жанр:** добавить «A Hypothesis Article» в заголовок/abstract | 🔴 |
| R2 | **Усилить polyE обоснование:** добавить предсказание — «Мы предсказываем, что U-ExM измерение покажет монотонную зависимость polyE от centriole/cilium ratio с R²>0.7» | 🔴 |
| R3 | **Ответить на causal question:** центриоль — driver или passenger? Добавить counter-argument paragraph | 🔴 |
| R4 | **Обосновать 5-10× предсказание:** quantitative comparison с известными улучшениями (hypoxia, vitamin C, etc.) | 🟡 |
| R5 | **Уточнить PLK4-resistant mutant:** номер мутации, ссылка, почему работает | 🟡 |
| R6 | **Добавить PMID для fertilization:** Schatten & Sun 2011, PMID 21509822 | 🟡 |

## MINOR REVISIONS

| # | Замечание |
|---|-----------|
| m1 | Заменить «may be» на более уверенные формулировки в 6.1 |
| m2 | Planarian evidence: ослабить утверждение, correlation не causation |
| m3 | p53 suppression — увеличить окно до 72h или обосновать 48h |

---

## ИТОГ

Статья готова к публикации после major revisions. Основная проблема — жанровая: это hypothesis article, а не research article. Если редактор ожидает experimental data, будет desk reject. Если Biogerontology принимает hypothesis/perspective articles — шансы хорошие.

**Почему висит 5 недель With Editor?** Возможные причины:
1. Редактор ищет рецензента с expertise в центриолях + aging (редкая комбинация)
2. Редактор колеблется — hypothesis или research article?
3. Высокая загрузка журнала (лето)

**Рекомендация:** Написать редактору inquiry через 1 неделю (если к 15 июл не будет движения). Не подавать параллельно в другой журнал.

---

## POST-SCRIPTUM: СТРАТЕГИЯ ПРИ ОТКЛОНЕНИИ

Если Biogerontology отклонит — **НЕ подавать в топ-журналы** (урок MCAOA → Nature Aging/eLife). Следующие журналы:
1. **GeroScience** (IF ~5) — принимает hypothesis + computational
2. **BioEssays** (IF ~4) — идеально для hypothesis articles
3. **Frontiers in Cell and Developmental Biology** — высокий acceptance rate
