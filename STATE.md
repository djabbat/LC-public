# LC · STATE (v7.2 — Organismal Aging, пост-28 peer review)

**Дата:** 2026-06-21
**CONCEPT:** v7.2 (финальная архитектура)
**Peer Review:** 28 циклов пройдено
**Оценка:** 8.2/10

---

## §0. ТЕКУЩЕЕ СОСТОЯНИЕ

**LC = Organismal Aging — платформа для тестирования гипотез старения.**

Архитектура: plug-and-play. Ядро: MCAOA (уровень #2) + ткани/Ze (уровень #3). Уровень #1 — сменный модуль.

---

## §1. КОД

| Компонент | Статус | Тесты |
|---|---|---|
| **sim_core** (Rust) | ✅ v0.1.1 | 70 passed |
| ├── centriole/ | ✅ 4 теста | Уровень #1 |
| ├── counters/ | ✅ 7 тестов | Уровень #2 (5 счётчиков) |
| ├── tissue/ | ✅ 5 тестов | Уровень #3 (8 тканей) |
| ├── organism/ | ✅ 6 тестов | Интеграция |
| ├── species/ | ✅ 5 тестов | Человек, мышь, C. elegans |
| ├── learning/ | ✅ 4 теста | Байесовский контур |
| ├── macrobiome/ | ✅ 4 теста | INFOGEST, диеты |
| ├── provenance/ | ✅ 5 тестов | 7 типов источников |
| ├── intervention/ | ✅ 2 теста | CR, рапамицин |
| ├── migration/ | ✅ 7 тестов | mcoa_core + cell_dt |
| └── integration/ | ✅ 10 тестов | Кросс-модульные |
| **CLI** (`oa`) | ✅ | simulate, audit, compare, species |
| **Python viz** | ✅ | plot_simulation.py |
| **README** | ✅ | EN, GitHub-ready |
| **argus_bridge** | 🔴 | Не начат |
| **sim_api** | 🔴 | Не начат |
| **sim_gui** | 🔴 | Не начат |

---

## §2. ДОКУМЕНТАЦИЯ

| Файл | Строк | Статус |
|---|---|---|
| CONCEPT.md | ~480 | ✅ v7.2 |
| THEORY.md | ~300 | ✅ v3.0 |
| EVIDENCE.md | ~130 | ✅ v2.0 |
| PARAMETERS.md | ~100 | ✅ v7.0 |
| OPEN_PROBLEMS.md | ~40 | ✅ |
| MAP.md | ~90 | ✅ |
| DESIGN.md | ~90 | ✅ |
| README.md | ~140 | ✅ EN |

---

## §3. ПРОВЕДЁННЫЕ PEER REVIEW

28 циклов (2026-06-21). 30 PMID верифицированы. 3 критические ошибки исправлены.

---

## §4. ДЕДЛАЙНЫ

- EIC Pathfinder: 28 Oct 2026
- Czech Embassy: 15 Sep 2026
- EU4Business: 30 Oct 2026

---

*LC STATE v7.2 — 2026-06-21.*
