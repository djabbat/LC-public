# CONCEPT — LC (LongevityCommon)

**Дата:** 2026-06-16 (глубокий аудит pi, уровень 9)
**Версия:** 6.0 (переписан на основе реального кода)
**Кодовая база:** 360 файлов (158 Rust + 146 Phoenix/Elixir + 56 Python)

## Назначение
Интегративная экосистема для биомаркер-ориентированных интервенций в старении. Объединяет вычислительные модели, веб-интерфейсы и экспериментальные протоколы.

## Архитектура (реальная)

```
LC/
├── MCAOA/                    ← Multi-Counter Architecture of Aging
│   ├── crates/mcoa_core/     ← общие структуры (Rust lib)
│   ├── crates/mcoa_api/      ← REST API (Rust, actix-web)
│   ├── crates/mcoa_cli/      ← CLI (Rust, clap)
│   ├── crates/mcoa_simulation/ ← симуляции осцилляций
│   ├── crates/mcoa_compare/  ← сравнение моделей (bin: compare_all, compare_mcoa_cdata)
│   ├── crates/mcoa_tests/    ← интеграционные тесты (test1_dominance, test4_aubrey)
│   ├── CDATA/                ← Центриолярная теория (10 crates!)
│   │   ├── crates/cell_dt_core/      ← ядро: модели клеточного деления
│   │   ├── crates/cell_dt_validation/ ← валидация: calibration, biomarkers, sensitivity, datasets, MCMC
│   │   ├── crates/cell_dt_cli/       ← CLI
│   │   ├── crates/cell_dt_gui/       ← GUI (Rust)
│   │   ├── crates/cell_dt_python/    ← Python bindings
│   │   ├── crates/cell_dt_modules/   ← 5 модулей: inflammaging, asymmetric_division, mitochondrial, tissue_specific, aging_engine
│   │   ├── backend/          ← REST API (Rust)
│   │   └── frontend/         ← Phoenix LiveView
│   ├── EpigeneticDrift/      ← Эпигенетические часы (Rust backend + Phoenix frontend)
│   ├── MitoROS/              ← Митохондриальные АФК (Rust backend + Phoenix frontend)
│   ├── Proteostasis/         ← Протеостаз (Rust backend + Phoenix frontend)
│   └── Telomere/             ← Теломерная динамика (Rust backend + Phoenix frontend)
├── Ze/                       ← Ze-теория (восприятие времени)
│   ├── Ze_CHSH/              ← Квантовый CHSH-эксперимент
│   ├── Ze_D/                 ← D-модель
│   ├── Ze_Model/             ← Математическая модель
│   ├── Ze-Hierarchy/         ← Иерархическая симуляция
│   └── website/              ← Веб-интерфейс (Phoenix)
├── BioSense/                 ← Биосенсоры (wearable, EEG)
│   ├── backend/              ← Rust API
│   ├── frontend/             ← Phoenix LiveView
│   ├── data/                 ← Cuban EEG датасет
│   └── instruments/          ← Автоматическая микроскопия
├── HAP/                      ← Health-Age Profiling
├── FCLC/                     ← Конфиденциальные вычисления (fclc.longevity.ge)
└── server/ web/ realtime/    ← Серверная инфраструктура
```

## Технологический стек
- **Rust:** 158 файлов (actix-web, serde, rusqlite, ndarray, statrs)
- **Phoenix/Elixir:** 146 файлов (LiveView, Ecto, Postgrex)
- **Python:** 56 файлов (скрипты, калибровка, визуализация)
- **Базы данных:** SQLite (dev), PostgreSQL (prod plan)

## Ключевые протоколы
- **CDATA:** Центриолярная теория старения (Tkemaladze 2023, PMID 36583780)
- **MCAOA:** Многокомпонентная архитектура старения (6 счётчиков)
- **Ze:** Теория восприятия времени (v* = 0.456)
- **FCLC:** Федеративные конфиденциальные вычисления (v13.4 PASS)

## Статус компонентов

| Компонент | Код | Тесты | Документация | Статус |
|-----------|:---:|:-----:|:------------:|--------|
| CDATA | 10 crates | ✅ | 2KB CONCEPT | 🟡 Активен |
| MCAOA | 6 crates | ✅ | MASTER.md | 🟢 |
| EpigeneticDrift | backend+frontend | ✅ | 44KB CONCEPT | 🟢 |
| MitoROS | backend+frontend | ✅ | 71KB CONCEPT | 🟢 |
| Proteostasis | backend+frontend | ✅ | 45KB CONCEPT | 🟢 |
| Telomere | backend+frontend | ✅ | 49KB CONCEPT | 🟢 |
| Ze | 5 подпроектов | ✅ | CONCEPT'ы | 🟡 |
| BioSense | backend+frontend | ✅ | 2.8KB CONCEPT | 🟡 |
| HAP | статья | — | 3.5KB CONCEPT | 🟢 Сабмит |
| FCLC | сервер | ✅ | 1.9KB CONCEPT | 🟢 Production |

## Связи с другими проектами
- **Aubrey:** CDATA теория центриолей
- **Services:** mbpr peer review, pel предсказания
- **PhD:** диссертация на основе CDATA
- **WLRAbastumani:** longevity-протоколы для курорта

## Текущие метрики
- Rust: 158 файлов (оценка ~25,000 строк)
- Phoenix: 146 файлов (оценка ~12,000 строк)
- Python: 56 файлов (оценка ~4,000 строк)
- Git: 17 коммитов, активность: растущая
- Сервисы: Ze :4001, BioSense :4101, FCLC :4002 (все локально)
