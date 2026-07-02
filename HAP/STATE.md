# STATE — HAP Project

**Date:** 2026-07-02
**Status:** 🟢 Активный

## ⚠️ BSPC — формальный отказ (1 июл 2026)

- **1 июл 2026** — получен формальный отказ от BSPC (BSPC-D-26-11119)
- **Редактор:** Mathias Baumert (Executive Editor)
- **Причина:** prescreen — «threshold for acceptance is high»
- **Это 9-й отказ** (JAD, PNEC, BioSystems, CSF, MedHyp, BMB, BBS, BSPC + CSF dual)
- **Текущая активная подача:** Mathematical Biosciences (MBS-D-26-00817, 27 июн)

> **📄 Статьи и публикации:** см. `~/Desktop/Services/publications/PUBLICATIONS_TRACKER.md`

## Технический статус

| Компонент | Статус |
|-----------|:------:|
| ODE-симуляция (6 переменных) | ✅ Готово |
| Morris + Sobol sensitivity | ✅ θ_L доминирует (ST=0.75) |
| Stochastic robustness | ✅ CV<1%, robustness=0.992 |
| Phase portraits | ✅ Сгенерированы |
| Bifurcation analysis | ✅ Saddle-node при L_basal≈0 |
| Графики | ✅ 10 шт. |
| Доказательная база | ✅ 6 категорий подтверждены |
| GitHub | ✅ https://github.com/djabbat/hap-dynamics |

## Сессия 2026-06-15 (итог): Полный цикл ревизии завершён

### Сделано
- [x] Morris + Sobol sensitivity: θ_L dominates (ST=0.75)
- [x] Stochastic: white noise CV=0.78%, colored (OU) CV=1.59%
- [x] 2D parameter scan (L_basal × θ_L, 900 runs)
- [x] Phase portraits (L-A, S-A)
- [x] Рукопись v3: 471 строка, 26 references (все верифицированы)
- [x] NHAM → HAP, Afaf удалена, Longevity Horizon скрыт
- [x] El Fettahi disclosure + email correspondence
- [x] 3 раунда peer review (IF 18+ уровень)
- [x] Рецензия №3: вердикт ACCEPT (с рекомендациями)
- [x] Новые референсы: PMID 40362260, 39566821, 41465592, 41459016 + Phytomedicine 2024

## Сессия 2026-05-30: Создание проекта + прототип симуляции + evidence search

### Что сделано
- [x] Создан проект с 8 core-файлами
- [x] Написан прототип ODE симуляции (Python, 6 state variables)
- [x] Симуляция воспроизводит HAP Predictions (ablation до/после τ_crit)
- [x] Bifurcation analysis — saddle-node при L_basal≈0, k_A_L≈0
- [x] Сгенерированы 10 графиков (trajectory, ablation, bifurcation)
- [x] **PubMed search по 15 запросам** — все категории подтверждают HAP
- [x] **Evidence report** — docs/evidence_hap_confirmation.md (6 разделов, 20+ новых PMID)
- [x] Afaf вышла из проекта (Biomarker Review). Решение: заморозить, делать соло при возврате.

### Ключевые находки evidence search

| Категория | Статус | Новых статей |
|-----------|:------:|:------------:|
| NAFLD ↔ depression | ✅ Confirmed | 6 (2022-2026) |
| Bile acid → FXR/TGR5 → mood | ✅ Confirmed | 4 (2024-2026) |
| Drosophila: fat body + ecdysone → affect | ✅ Confirmed | 6 (2017-2026) |
| C. elegans: no liver = no affect | ✅ Confirmed | 3 (2025-2026) |
| Liver Tx → mood | ✅ Confirmed | 4 (2015-2026) |
| Critical window: steroids → affect | ✅ Confirmed | 3 (2016-2022) |
