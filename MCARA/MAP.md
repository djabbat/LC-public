# MCARA — Карта проекта

**Дата:** 2026-07-08

## Структура

```
MCARA/
├── _pi.md              — Правила pi
├── CONCEPT.md          — Концепт (Гонка счётчиков v4.4)
├── TODO.md             — Задачи
├── PARAMETERS.md       — Параметры
├── MAP.md              — Эта карта
├── STATE.md            — Текущее состояние
├── MEMORY.md           — История решений
├── README.md           — Введение
├── DESIGN.md           — Дизайн экспериментов
├── THEORY.md           — Теоретическая основа
├── EVIDENCE.md         — Доказательная база
│
├── CEDAR/              — Counter #1: Центриолярная теория + симуляции + код
│   ├── Aubrey/         —   ARGUS: научно-техническое ядро (аппаратная архитектура)
│   ├── CellLineageTree/—   Симуляция клеточных линий
│   ├── simulator/      —   CEDAR-симулятор
│   └── ...
│
├── EpigeneticDrift/    — Counter #2: Эпигенетические часы
├── MitoROS/            — Counter #3: Митохондриальный
├── Telomere/           — Counter #4: Теломерный
├── Proteostasis/       — Counter #5: Протеостаз
│
├── ARGUS-LP/           — 🆕 Инструментальная платформа (перенесена из Marketing 2026-07-08)
│   ├── CONCEPT.md      —   6 версий микроскопа (V1–V6), AI-агент
│   ├── docs/           —   Переписка, BOM, аудиты
│   └── ...
│
├── Aubrey/             — 🆕 Научный проект (перенесён из Marketing 2026-07-08)
│   ├── CONCEPT.md      —   Первый центриолярный атлас человека
│   ├── Phase-0/        —   Критический проверочный эксперимент
│   ├── Phase-A/        —   ARGUS-LP платформа
│   ├── Phase-B/        —   Гонка счётчиков
│   └── ...
│
├── crates/             — Rust-код (mcoa_core, mcoa_api, mcoa_cli, mcoa_compare, mcoa_simulation, mcoa_tests)
├── backend/            — Python-бэкенд
├── frontend/           — Веб-интерфейс
├── docs/               — Документация
├── audits/             — Peer review
├── letters/            — Письма партнёрам
├── refs/               — Референсы
└── data/               — Данные
```

## Подпроекты

| Подпроект | Тип | Роль |
|-----------|-----|------|
| **CEDAR** | Теория + код | Counter #1 — Центриолярный |
| **EpigeneticDrift** | Теория | Counter #2 — Эпигенетический |
| **MitoROS** | Теория + код | Counter #3 — Митохондриальный |
| **Telomere** | Теория + код | Counter #4 — Теломерный |
| **Proteostasis** | Теория + код | Counter #5 — Протеостаз |
| **ARGUS-LP** | Инструменты + гранты | AI-робот для lineage tracking |
| **Aubrey** | Научный проект | Центриолярный атлас, BOLD PILOT, консорциум EIC |

## Зависимости
- [LC] — родительский проект

## Выходы
- EIC Pathfinder 2026 (28 окт)
- MCARA статья (Biogerontology, ID 7cc6de62)
- CEDAR статья (npj Aging)
- ARGUS-LP статья (RSI)
