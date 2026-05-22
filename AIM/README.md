<!-- AUTO-TRANSLATED from README.md via DeepSeek 2026-05-13. Source language: russian. Original (README.md) is canonical; re-run scripts/translate_core_files.py after edits. -->

# AIM v8.0 — Integrative Medicine Assistant

**Помощник врача интегративной медицины.** Не AI. Не ставит диагнозы. Агрегирует знания экосистемы LC.

## What It Is

A physician's tool — provides a structured knowledge summary from the entire LC ecosystem as applied to a specific patient.

## Launch

```bash
# gabro-aim API service (Rust, порт 4820)
gabro-aim

# Show status of all projects
python3 dashboard/status.py

# Generate ecosystem graph
dot graph/ecosystem.dot -Tpng -o graph/ecosystem.png

# Check consistency
python3 validate/counter_numbering.py
python3 validate/ze_vstar.py
python3 validate/concept_versions.py
```

## Структура

- `registry.json` — machine-readable registry (canon)
- `validate/` — cross-project validation scripts
- `dashboard/` — status dashboard
- `graph/` — graph in DOT/Mermaid
- `MAP.md` — dependency map (human-readable)
- `CONCEPT.md` — полное описание концепции

## gabro-aim (API сервис)

Rust-сервис на порту 4820, предоставляет REST API к знаниям AIM для gabro экосистемы.

```bash
# Статус экосистемы
curl http://localhost:4820/aim/status

# 5 MCAOA counters
curl http://localhost:4820/aim/counters

# Интерпретация χ_Ze
curl 'http://localhost:4820/aim/chi_ze?v=0.62'

# Поиск по проектам
curl 'http://localhost:4820/aim/search?q=mito'
```

Полный список эндпоинтов — в `CONCEPT.md §5.5`.

## Archive

`_archive/v7_ai_code/` — complete AIM v7.0 code (AI medical system). Removed from concept on 2026-05-09.