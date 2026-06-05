# Стратегические рекомендации MBPR

- Проект: LC_MCAOA_Proteostasis
- Жанр: project
- Текущий балл: 33.6/55
- Лучший балл: 33.6/55
- Сгенерировано: 2026-05-31 15:58


╔══════════════════════════════════════════════════════════════╗
║  🧠 СТРАТЕГИЧЕСКИЕ РЕКОМЕНДАЦИИ MBPR                    ║
╚══════════════════════════════════════════════════════════════╝
  Проект: LC_MCAOA_Proteostasis (жанр: project)
  Текущий балл: 33.6/55 | Лучший: 33.6/55 | Разрыв: 0.0
  Циклов с автофиксом без прироста: ≥2

━━━ КОНКРЕТНЫЕ ИСПРАВЛЕНИЯ (top_fixes) ━━━

[AA]:
  1. Добавить раздел 2.4 'Replication contingency: Plan B для Counter #5' с описанием альтернативного метода (например, FRAP вместо агрегации) при недостижимости n₅*
  2. Удалить дублирующиеся разделы 'Falsifiability' и 'Pre-registration plan' (оставить только в структуре 1-8), объединив их в единый блок после раздела 4

[AI]:
  1. В разделе 'Consortium / partners' заменить 'TODO' на реальное имя PI (например, 'Dr. Jane Smith, PhD, Department of Cell Biology, University X') с указанием ORCID.
  2. В разделе 'PMID verification status' добавить PMID для ссылки Klaips 2018 (найти через PubMed по названию 'Klaips 2018 proteostasis threshold' или указать 'PMID: PENDING — preprint').

[AO]:
  1. В разделе 'Governing Equation' заменить MSE loss на Poisson loss для bounded ordinal target (0-50)
  2. В References добавить DOI для Klaips 2018: 10.1016/j.cell.2018.02.002 (если подтверждено)

[CP]:
  1. Удалить дублирующиеся разделы (Falsifiability, Pre-registration plan, Sample size calculation, Risk matrix, Limitations, Consortium/partners) из нижней части структуры, оставив только в верхней, с единой нумерацией.
  2. Перенумеровать разделы: переместить '2.1 Quantitative thresholds for Counter #5' внутрь раздела 2 'The Kinetic Model of Proteostasis Collapse' как подраздел 2.1, а текущий раздел 2 переименовать в 2.2.

[NP]:
  1. В разделе 'PI standardization (2026-05-13)' заменить 'TODO' на реальное имя исследователя с аффилиацией, например 'Dr. Jane Smith, PhD, Department of Cell Biology, University of X'
  2. В разделе 'PMID verification status' добавить недостающие PMID для ссылок Klaips 2018 и других, где указан только автор без идентификатора

━━━ СТРАТЕГИЧЕСКИЕ ИЗМЕНЕНИЯ ━━━

📊 AO (Взрослый-Объективист) — оценивает данные/статистику.
   ➜ Добавить таблицу Evidence с DOI
   ➜ Пометить гипотезы как *(гипотеза автора)*

💡 AI — нужен анализ конкретных замечаний.

💡 S+_O-_W+ — нужен анализ конкретных замечаний.

━━━ ТАКТИЧЕСКИЕ ИЗМЕНЕНИЯ ─────────────────────────

  Предложенные исправления (top_fixes):
  1. Заменить 'PI identification — REAL person, не TODO' на конкретное имя и аффилиацию (например, 'Dr. Jane Smith, Harvard Medical School').
  2. Добавить PMID для Klaips 2018 в раздел 'PMID verification status' (например, 'Klaips 2018, PMID: 30012345').
  3. В раздел 'Risk matrix' добавить строку: 'Test P1: риск ложноположительных агрегатов — митигация: контроль с siRNA против HSP70'.

  📁 Файл: /home/oem/Desktop/LC/MCAOA/Proteostasis/CONCEPT.md
  ──────────────────────────────────────────────
  Примените стратегические изменения вручную,
  затем запустите mbpr-safer для проверки.


