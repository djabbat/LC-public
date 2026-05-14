# pi.md — LongevityCommon

Памятка агента pi. Создан 2026-05-11.

## Суть

Hypothesis-stage framework: 5 подпроектов (MCOA, CDATA, Ze, BioSense, FCLC) + социальный слой (server/web/realtime). Все AUC — exploratory, не confirmatory. Pre-registered тесты χ_Ze — NULL (deprecated). Публикации НЕ peer-reviewed.

## EIC Pathfinder Challenges 2026

- Deadline: **28 Oct 2026**, бюджет до €4M
- Variant C: WP1-5 (FCLC + Ze + CDATA + BioSense + Aqtivirebuli), €3.0M / 36 mo
- Партнёры: Geiger (Ulm), Janke Curie, COSIC/Preneel KU Leuven

## Структура

18 поддиректорий. См. CONCEPT.md для authority order.

## Связь с сервером

- `AIM/` → симлинк → `~/hive_queen/AI` (на сервере)
- Локальная копия (`~/Desktop/LongevityCommon/`) — сервер (`~/LongevityCommon/`) — git-оригинал

## Команды

```bash
# Пуш на сервер
cd ~/Desktop/LongevityCommon && git push server main
```

---

## Заметки
## 2026-05-14

## Rust/Phoenix rule for gabro — 2026-05-14
Все новый код для gabro пишется ТОЛЬКО на Rust (core/backend) или Phoenix/Elixir (frontend/UI).
Другие языки — только по прямому указанию pi.
Rust library: ~/Desktop/Services/gabro-erudition/ — self-optimizing erudition layer.

## 2026-05-13

## 2026-05-14 — Gabro Intelligence Imperative

**Каждое действие pi — делать gabro умнее.** Это absolute priority.

Gabro — локальный Triada agent (amigdala + PEL + TBPR + gabro).
Ключевые improvements:
1. SYSTEM_PROMPT — identity + Triada + tools + правила
2. `_project_tool_query()` — pre-gather данных через bash/sqlite/git/ps
3. `_build_smart_prompt()` — сборка system + lang + data + CoT
4. Chain-of-thought: 1) данные 2) анализ 3) вывод
5. Правило "не выдумывай" — только из [DATA] секции
6. Auto-switch на deepseek-r1:14b для сложных запросов

Никаких изменений без улучшения gabro.


## 2026-05-13

## 2026-05-13 — Критическое правило: PMID всегда проверять
Перед вставкой любого PMID/DOI — проверить:
1. Что DOI/PMID реально существует (resolvable)
2. Что содержание статьи соответствует контексту (не fabricated как с DOI:10.1038/s41598-019-53456-3 — микрофлюидика вместо stage)
3. Fabricated reference = REVISE_MAJOR минимум. 3+ = TOXIC_WITHDRAW.
