# MEMORY.md — HAP Project

**Назначение:** Что нужно помнить между сессиями. Новое сверху.

## 2026-06-18 — Medical Hypotheses: Handling Editor → Instant Reject

- **18 июн 2026, 14:57** — YMEHY-D-26-00985: Editor-in-Chief Dr Sachin Sarode назначен handling editor
- **18 июн 2026, 15:00** — Rejected (через 3 минуты!), desk reject без рецензий
- **Причина:** Стандартная формулировка («reviewers recommend against publishing»), но reviewers не назначались → desk reject
- **Ирония:** Отзыв был запрошен 17.06, но не обработан — рукопись всё равно получила отказ
- **5-й отказ подряд** (JAD, PNEC, BioSystems, CSF, Medical Hypotheses) — все по скоупу/формату
- **Следующий:** Journal of Biological Dynamics (T&F, subscription, IF ~2.8, не Elsevier)
- **Cover letter:** `~/Desktop/cover_letter_JBD.md` уже готов

## 2026-06-17 — CSF Rejected (dual submission) + Medical Hypotheses withdrawal

- **17 июн 2026** — CSF (CHAOS-D-26-05728) rejected: Elsevier обнаружил dual submission
- **Причина:** Medical Hypotheses (YMEHY-D-26-00985) не закрыл рукопись при трансфере в Psychoneuroendocrinology
- **Medical Hypotheses** отзыв отправлен (17.06) через Editorial Manager
- **Apology letter** отправлен Marcel Clerc (CSF Editor-in-Chief)
- **Урок:** При трансфере внутри Elsevier всегда проверять, что рукопись закрыта в исходном журнале
- **Решение:** Подача в Journal of Biological Dynamics (T&F, не Elsevier, subscription, IF ~2.8)
- **Cover letter:** `~/Desktop/cover_letter_JBD.md`
- **BBS:** «Sensation, Feeling, Abstraction» (BBS-D-26-00814) — тоже rejected 17.06 (scope)

## 2026-06-16 — Третий desk reject: Psychoneuroendocrinology

- **16 июн 2026** — отказ Psychoneuroendocrinology (PNEC-D-26-00481)
- **Редактор:** Elizabeth (Birdie) Shirtcliff (Editor-in-Chief)
- **Причина:** «falls outside of the scope of this journal»
- **Это 3-й desk reject подряд** (J. Affective Disorders → Medical Hypotheses → Psychoneuroendocrinology)
- **Все три — по скоупу, не по качеству** (статья прошла 3 раунда внутренней ревизии, вердикт ACCEPT)
- **Проблема:** HAP — междисциплинарная работа (эволюционная биология + математическое моделирование + аффективная нейронаука), её трудно уместить в узкий журнальный скоуп
- **Решение:** искать журналы, специально принимающие теоретические/математические работы (J. Theoretical Biology, Biosystems) или мультидисциплинарные (PLOS ONE, PeerJ)

## 2026-06-15 — Afaf отказалась от Biomarker Review

- **15 июн 2026** — Afaf El Fettahi вышла из Biomarker Review (Dynamic Biomarkers Systematic Map)
- **Причина:** протокол (v2.4) стал слишком масштабным для дуэта: 3,499 hits → ~350-500 eligible → 5 biomarker domains → много баз данных → quality assessment → synthesis
- **Рекомендация:** найти методиста по systematic reviews или лабораторию
- **Письмо:** `Biomarker_Review/email_from_Afaf_2026-06-15.md`
- **Следствия:**
  - Удалить имя Afaf из протокола, OSF регистрации, всех документов
  - Нужен новый коллаборатор ИЛИ сузить протокол до 1-2 biomarker domains
  - Embase access теперь нужно искать самостоятельно

## 2026-06-15 — J. Affective Disorders Rejected → Medical Hypotheses

- **15 июн 2026, 7:31 AM** — отказ J. Affective Disorders (JAFD-D-26-06247)
- **Редактор:** Benjamin Goldstein (Deputy Editor)
- **Причина:** «Lack of sufficient novelty» (desk reject, без рецензий)
- **Причина отказа:** Несовпадение тематики — J. Affective Disorders клинический журнал, не для ODE-моделей
- **Решение:** Подать в **Medical Hypotheses** (Elsevier, IF ~4.7) — журнал специально для смелых теорий
- **Cover letter подготовлен:** `~/Desktop/LC/HAP/docs/cover_letter_Medical_Hypotheses.md`

## 2026-05-30 — Создание проекта HAP

- HAP отделён от PhD в самостоятельный проект (~/Desktop/HAP/)
- Перенесена симуляция (src/) и документация (docs/)
- Созданы все core-файлы
- Прототип симуляции работает и воспроизводит HAP Predictions

## 2026-05-30 — Письмо Afaf отправлено

- Afaf предложила: nonlinear dynamics, allostasis, feedback loops (не quantum)
- План: симуляция → данные (не наоборот)
- Письмо-ответ отправлено: согласие на direction, предложение начать с симуляции
- Ожидается ответ

## Постоянные правила

- **HAP Strong Version** уже опубликован (DOI: 10.65649/d76f6c48) — это фундамент
- **HAP фундаментальнее NHAM** — HAP даёт necessary condition, NHAM — механизм
- **Симуляция прежде данных** — не искать данные, пока модель не готова
- **Afaf** — главный коллаборатор по второй статье
