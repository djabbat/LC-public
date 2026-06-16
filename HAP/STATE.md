# STATE — HAP Project

**Date:** 2026-06-16
**Status:** 🔴 Третий desk reject (Psychoneuroendocrinology). Ищем альтернативный журнал.

## 2026-06-16 — Psychoneuroendocrinology: Editorial Desk Reject

- **PNEC-D-26-00481** — editorial desk reject (16 июн, 3:56 AM)
- **Редактор:** Elizabeth (Birdie) Shirtcliff (Editor-in-Chief)
- **Причина:** «topic falls outside of the scope of this journal»
- **Хронология отказов:**
  1. J. Affective Disorders (JAFD-D-26-06247) — desk reject, «lack of novelty» / mismatch
  2. Medical Hypotheses — трансфер в Psychoneuroendocrinology (не отказ, но не приняли)
  3. Psychoneuroendocrinology (PNEC-D-26-00481) — desk reject, «не в скоупе»
- **Все три отказа — по скоупу, не по качеству.** Статья прошла 3 раунда внутренней ревизии, вердикт ACCEPT.
- **Нужен:** журнал, принимающий теоретические + математические работы в биологии

### Следующий шаг
- [ ] Elsevier Journal Finder (http://journalfinder.elsevier.com)
- [ ] Рассмотреть: J. Theoretical Biology, Biosystems, PLOS ONE, PeerJ, Frontiers in Physiology

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

### Файлы на Desktop
- `manuscript_v3_hap_final.md` — рукопись (471 строка)
- `peer_review_round3_IF18.md` — рецензия на русском
- `pasport.pdf` — скан паспорта

### Следующий шаг
- [ ] Подача в Medical Hypotheses через Editorial Manager

## Сессия 2026-06-15 (продолжение): Sensitivity + Stochastic + Manuscript revision

### Сделано
- [x] Morris + Sobol sensitivity: θ_L dominates (ST=0.75), топ-4 — печёночные
- [x] Stochastic robustness: CV<1% (±5% perturbation), robustness=0.992
- [x] Рукопись v3: NHAM убран, Afaf удалена, Longevity Horizon скрыт
- [x] Рецензия (IF 18+): 10 критических правок выполнены
- [x] El Fettahi disclosure в §1.1
- [x] Невалидные PMID удалены, новые источники добавлены (Zhao 2025, Wang 2026)
- [x] Фазовые портреты сгенерированы
- [x] Альтернативная P(L,B) добавлена
- [x] Limitations расширены до 11 пунктов

### Следующий шаг
- [ ] Подача в Medical Hypotheses

## Сессия 2026-06-15: J. Affective Disorders → отказ, переподача в Medical Hypotheses

### Событие
- **JAFD-D-26-06247** — desk reject (5 дней), без рецензий
- Причина: «Lack of sufficient novelty» (mismatch тематики)
- Решение: подать в **Medical Hypotheses** (специализируется на смелых теориях)
- Cover letter: `~/Desktop/LC/HAP/docs/cover_letter_Medical_Hypotheses.md`

## Сессия 2026-05-30: Создание проекта + прототип симуляции + evidence search

### Что сделано
- [x] Создан отдельный проект ~/Desktop/HAP/ с 8 core-файлами
- [x] Написан прототип ODE симуляции (Python, 6 state variables)
- [x] Симуляция воспроизводит HAP Predictions (ablation до/после τ_crit)
- [x] Bifurcation analysis — saddle-node при L_basal≈0, k_A_L≈0
- [x] Сгенерированы 10 графиков (trajectory, ablation, bifurcation)
- [x] **PubMed search по 15 запросам** — все категории подтверждают HAP
- [x] **Evidence report** — docs/evidence_hap_confirmation.md (6 разделов, 20+ новых PMID)
- [x] **Письмо Afaf с результатами** — на Desktop (HAP_to_Afaf/)

### Что в процессе
- [x] **Parameter sensitivity analysis** (Morris + Sobol) — theta_L главный параметр
- [x] **Stochastic parameter perturbation** — robustness = 0.992 (±5% шум)
- [ ] **Medical Hypotheses** — подача статьи
- Afaf вышла из проекта (Biomarker Review / Dynamic Biomarkers Systematic Map)
- Причина: протокол стал слишком широким для двух человек
- Письмо: `LC/HAP/Biomarker_Review/email_from_Afaf_2026-06-15.md`
- **Решение Джабы:** заморозить Biomarker Review
  - Нового коллаборатора не искать
  - Когда вернёмся — делать соло с суженным протоколом (1-2 домена)
  - Удалить имя Afaf из протокола, OSF, всех документов (при разморозке)
- **Фокус сейчас → HAP-симуляция + Medical Hypotheses**

### Ключевые находки evidence search
| Категория | Статус | Новых статей |
|-----------|:------:|:------------:|
| NAFLD ↔ depression | ✅ Confirmed | 6 (2022-2026) |
| Bile acid → FXR/TGR5 → mood | ✅ Confirmed | 4 (2024-2026) |
| Drosophila: fat body + ecdysone → affect | ✅ Confirmed | 6 (2017-2026) |
| C. elegans: no liver = no affect | ✅ Confirmed | 3 (2025-2026) |
| Liver Tx → mood | ✅ Confirmed | 4 (2015-2026) |
| Critical window: steroids → affect | ✅ Confirmed | 3 (2016-2022) |
