# Pilot Report — Dynamic Biomarkers Systematic Map

**Date:** 2026-06-15
**Status:** Pilot completed (PubMed only)
**Decision:** NARROW to HRV + Cortisol

---

## 1. Full Search (All 5 Domains)

| Параметр | Значение |
|----------|----------|
| **База** | PubMed |
| **Поисковая строка** | v2.2 (IF 18+ upgrade) — HRV, cortisol, IL-6, CRP, TNF-α, tryptophan/kynurenine, EEG/ERP, EDA |
| **Дата** | 2026-06-15 |
| **Всего хитов** | **5,651** |
| **Вердикт** | Превышает порог NARROW (>200) |

---

## 2. Narrowed Search (HRV + Cortisol only)

| Параметр | Значение |
|----------|----------|
| **База** | PubMed |
| **Поисковая строка** | v2.4 (HRV + cortisol + sAA) |
| **Дата** | 2026-06-15 |
| **Всего хитов** | **3,499** |
| **Оценка после title screen (~40%)** | ~1,400 |
| **Оценка после full-text (~25%)** | ~350 |

---

## 3. Поисковая строка (v2.4 — PubMed)

```
(("Heart Rate"[MeSH] OR "heart rate variability"[MeSH] OR "HRV"[tiab] OR "RMSSD"[tiab]
  OR "Hydrocortisone"[MeSH] OR "cortisol"[tiab] OR "salivary alpha-amylase"[tiab] OR "sAA"[tiab]))

AND

(("Emotional Regulation"[MeSH] OR "Stress, Psychological"[MeSH] OR "Cognition"[MeSH]
  OR "affect* regulation"[tiab] OR "emotion regulation"[tiab] OR "cognitive reappraisal"[tiab]
  OR "acute stress"[tiab] OR "social stress"[tiab] OR "uncertainty"[tiab] OR "TSST"[tiab]
  OR "cognitive load"[tiab] OR "threat"[tiab] OR "challenge"[tiab]))

AND

(("repeated measures"[tiab] OR "time series"[tiab] OR "recovery"[tiab]
  OR "reactivity"[tiab] OR "transition*"[tiab] OR "dynamic*"[tiab]
  OR "EMA"[tiab] OR "ecological momentary"[tiab] OR "ambulatory"[tiab]
  OR "wearable"[tiab] OR "daily diary"[tiab]))

AND

("Humans"[MeSH] AND "Adult"[MeSH])

AND

("2000"[Date - Publication] : "2026"[Date - Publication])

NOT

("animal*"[ti] OR "review"[pt] OR "meta-analysis"[pt] OR "comment"[pt] OR "editorial"[pt])
```

**PubMed URL:** https://pubmed.ncbi.nlm.nih.gov/?term=%28%28%22Heart+Rate%22%5BMeSH%5D+OR+%22heart+rate+variability%22%5BMeSH%5D+OR+%22HRV%22%5Btiab%5D+OR+%22RMSSD%22%5Btiab%5D+OR+%22Hydrocortisone%22%5BMeSH%5D+OR+%22cortisol%22%5Btiab%5D+OR+%22salivary+alpha-amylase%22%5Btiab%5D+OR+%22sAA%22%5Btiab%5D%29%29+AND+...

---

## 4. Title Screen Sample (First 20 — narrowed search)

| # | PMID | Title | Eligible? |
|---|------|-------|:---------:|
| 1 | 42285693 | Psychophysiological and emotional stress responses in higher education | ✅ |
| 2 | 42281109 | HRV and Body Motion as Digital Biomarkers of Task Workload | ✅ |
| 3 | 42281047 | Using Heart Rate to Measure Stress in Healthcare Workers | ✅ |
| 4 | 42276802 | Effect of supplemental hydrocortisone during stress (clinical trial) | ❌ Clinical |
| 5 | 42276683 | Phasic HRV Predicts Outcomes in GAD + Depression | ❌ Clinical |

**Оценка:** ~30-50% проходят title screen (нужна точная пилот-калибровка с Афаф, 50 случайных, κ ≥ 0.70)

---

## 5. Decision (per Protocol §15)

| Критерий | Порог | Факт | Решение |
|-----------|-------|------|---------|
| NARROW if >200 hits | >200 | 3,499 | ✅ NARROW до HRV + cortisol |
| GO — T2 if ≥50 eligible | ≥50 | TBD | ⏳ После title screen |
| GO — T1 if ≥80 eligible + ≥30% ESM | ≥80 | TBD | ⏳ После title screen |

**Решение:** Сузить до 2 доменов (HRV + кортизол). IL-6, CRP, TNF-α, триптофан/кинуренин, EEG/ERP, EDA — deferred to Discussion.

---

## 6. Embase — Search String Adaptation (EMTREE)

*Embase требует подписки Elsevier. Прямой API недоступен. Строка адаптирована для запуска при получении доступа.*

```
('heart rate variability'/exp OR 'heart rate'/exp OR hrv:ti,ab,kw OR rmssd:ti,ab,kw
 OR 'hydrocortisone'/exp OR cortisol:ti,ab,kw OR 'alpha amylase'/exp OR 'salivary alpha amylase':ti,ab,kw OR saa:ti,ab,kw)

AND

('emotion'/exp OR 'mental stress'/exp OR 'cognition'/exp
 OR 'affect regulation':ti,ab,kw OR 'emotion regulation':ti,ab,kw OR 'cognitive reappraisal':ti,ab,kw
 OR 'acute stress':ti,ab,kw OR 'social stress':ti,ab,kw OR uncertainty:ti,ab,kw
 OR tsst:ti,ab,kw OR 'cognitive load':ti,ab,kw OR threat:ti,ab,kw OR challenge:ti,ab,kw)

AND

('repeated measures':ti,ab,kw OR 'time series':ti,ab,kw OR recovery:ti,ab,kw
 OR reactivity:ti,ab,kw OR transition*:ti,ab,kw OR dynamic*:ti,ab,kw
 OR ema:ti,ab,kw OR 'ecological momentary':ti,ab,kw OR 'ambulatory monitoring'/exp
 OR wearable:ti,ab,kw OR 'daily diary':ti,ab,kw)

AND

([adult]/lim AND [humans]/lim AND [english]/lim AND [2000-2026]/py)

NOT

('animal'/exp OR 'review'/exp OR 'meta analysis'/exp OR 'editorial'/exp)
```

| Ключ EMTREE | PubMed аналог |
|-------------|---------------|
| `/exp` | `[MeSH]` — explode (включает подтермины) |
| `:ti,ab,kw` | `[tiab]` — title, abstract, keywords |
| `[adult]/lim` | `Adult[MeSH]` |
| `[humans]/lim` | `Humans[MeSH]` |
| `[2000-2026]/py` | `2000:2026[Date - Publication]` |
| `'mental stress'/exp` | `Stress, Psychological[MeSH]` |

---

## 7. Multi-Database Estimates (Projected)

| База | Статус | Хиты (оценка) | Примечание |
|------|--------|:------------:|------------|
| **PubMed** | ✅ Проверен | **3,499** | Точный подсчёт 2026-06-15 |
| **Embase** | ⏳ Нужен доступ | ~4,500 | Обычно +20-40% к PubMed для биомедицинских тем |
| **Scopus** | ⏳ Афаф | ~2,500 | Меньше PubMed, больше социальных наук |
| **PsycINFO** | ⏳ Афаф | ~1,500 | Психологические конструкты, emotion regulation |
| **Web of Science** | ⏳ Ограничен | ~2,000 | Citation tracking |
| **ProQuest** | ⏳ Афаф | ~300 | Диссертации |
| **Google Scholar** | ⏳ | 200 | Первые 200 (лимит по протоколу) |
| **Semantic Scholar** | ✅ Проверен | 272 | Простой запрос (API limit) |
| **Backward citation** | ⏳ | ~100 | Snowballing |
| **Всего (до дедупликации)** | | **~14,600** | |
| **После дедупликации** | | **~9,500** | ~35% overlap |
| **После title screen (~40%)** | | **~3,800** | |
| **После full-text (~25%)** | | **~350-500** | Приемлемо для systematic map |

---

## 8. Decision (per Protocol §15)

| Критерий | Порог | Оценка | Решение |
|-----------|-------|--------|---------|
| NARROW if >200 hits | >200 | 3,499 (PubMed) | ✅ NARROW до HRV + cortisol |
| GO — T2 if ≥50 eligible | ≥50 | ~350-500 projected | ✅ Вероятно GO |
| GO — T1 if ≥80 eligible + ≥30% ESM | ≥80 | TBD | ⏳ После полного пилота |

**Решение:** Предварительно **GO (T2)**. После получения Embase/Scopus/PsycINFO данных — финальное решение.

---

## 9. Next Steps

| # | Действие | Кто | Статус |
|---|----------|-----|--------|
| 1 | PubMed pilot | Jaba | ✅ 2026-06-15 |
| 2 | Embase — запросить доступ | Jaba | ⏳ |
| 3 | Scopus pilot | Jaba | ⏳ |
| 4 | PsycINFO pilot | Jaba | ⏳ |
| 5 | ProQuest pilot | Jaba | ⏳ |
| 6 | Объединённый pilot report (все базы) | Jaba | ⏳ |
| 7 | Decision GO / PAUSE | Jaba | ⏳ После всех пилотов |
| 8 | Rayyan project + загрузка references | Jaba | ⏳ |
| 9 | Pilot calibration (50 abstracts) | Jaba | ⏳ |

---

## 7. Protocol Updates (2026-06-15)

- §6: Biomarker domains narrowed to HRV + cortisol (+ sAA). EDA/SCR removed from primary.
- §9: Search string v2.2 → v2.4 (HRV + cortisol only). Removed IL-6, CRP, TNF-α, tryptophan/kynurenine, EEG/ERP, EDA blocks.
- Protocol version: v2.3 → v2.4 (solo, narrowed scope)
