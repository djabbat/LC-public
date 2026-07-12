# LC — MAP (v7.1: фактическая структура)

**Дата:** 2026-07-04 (Цикл 8 аудита pi — приведение к реальности)

## Фактическая структура (июль 2026)

```
LC/
├── _pi.md                          ← правила для pi
├── CONCEPT.md                       ← концепт LC
├── TODO.md                          ← задачи
├── PARAMETERS.md                    ← параметры
├── MAP.md                           ← этот файл
├── STATE.md                         ← текущий статус
├── MEMORY.md                        ← история решений
├── README.md                        ← описание
├── DESIGN.md                        ← архитектура ПО
├── THEORY.md                        ← теория
├── EVIDENCE.md                      ← доказательная база
├── LICENSE                          ← Apache 2.0
├── .gitignore
│
├── MCARA/                           ← Multi-Counter Architecture (5 подпроектов)
│   ├── ARGUS-LP/                    ←   грантовый подпроект
│   ├── CEDAR/                       ←   + 5 подподпроектов (Aubrey-Platform, simulator, CellLineageTree, articles, ARGUS-Hardware)
│   ├── EpigeneticDrift/
│   ├── MitoROS/
│   ├── Proteostasis/
│   ├── Telomere/
│   └── Aubrey/                      ←   Centriolar Atlas + Phase-0/A/B + grants
│
├── Ze/                              ← Ze Vectors Theory (4 подпроекта)
│   ├── Ze_CHSH/
│   ├── Ze_D/
│   ├── Ze-Hierarchy/
│   ├── Ze_Model/
│   ├── simulator/                   ← Rust-симулятор (Cargo)
│   ├── website/ze_sim/              ← Веб-симулятор
│   ├── bristlebot_sim/              ← Python-симуляция
│   ├── simulations/                 ← Python-симуляции
│   └── ze-web/                      ← Веб-конфиг
│
├── BioSense/                        ← Носимые биомаркеры (7/7)
├── FCLC/                            ← Федеративное обучение (7/7)
├── HAP/                             ← Health Analytics Platform (7/7)
├── Organismal_Aging/                ← Организменное старение (7/7)
│
├── sim_core/                        ← Ядро симулятора (Rust workspace)
├── realtime/                        ← Real-time сервисы
├── shared-types/                    ← Общие типы
│
├── server/                          ← Серверная инфраструктура
├── web/                             ← Веб-интерфейсы
├── docs/                            ← Документация
├── scripts/                         ← Скрипты
├── _archive/                        ← Архив
└── _originals/                      ← Оригиналы
```

## Подпроекты (с _pi.md)

| Подпроект | Путь | Core |
|-----------|------|:----:|
| **MCARA** | `MCARA/` | 7/7 |
| **ARGUS-LP** | `MCARA/ARGUS-LP/` | 7/7 |
| **Aubrey** | `MCARA/Aubrey/` | 7/7 |
| **Aubrey/Phase-0** | `MCARA/Aubrey/Phase-0/` | 7/7 |
| **Aubrey/Phase-A** | `MCARA/Aubrey/Phase-A/` | 7/7 |
| **Aubrey/Phase-B** | `MCARA/Aubrey/Phase-B/` | 7/7 |
| **Aubrey/EIC_Pathfinder_Open** | `MCARA/Aubrey/grants/EIC_Pathfinder_Open/` | 7/7 |
| **CEDAR** | `MCARA/CEDAR/` | 7/7 |
| **EpigeneticDrift** | `MCARA/EpigeneticDrift/` | 7/7 |
| **MitoROS** | `MCARA/MitoROS/` | 7/7 |
| **Proteostasis** | `MCARA/Proteostasis/` | 7/7 |
| **Telomere** | `MCARA/Telomere/` | 7/7 |
| **CEDAR/Aubrey-Platform** | `MCARA/CEDAR/Aubrey-Platform/` | 7/7 |
| **CEDAR/ARGUS-Hardware** | `MCARA/CEDAR/Aubrey-Platform/ARGUS-Hardware/` | 7/7 |
| **CEDAR/simulator** | `MCARA/CEDAR/simulator/` | 7/7 |
| **CEDAR/CellLineageTree** | `MCARA/CEDAR/CellLineageTree/` | 7/7 |
| **CEDAR/articles** | `MCARA/CEDAR/articles/` | 7/7 |
| **Ze_CHSH** | `Ze/Ze_CHSH/` | 7/7 |
| **Ze_D** | `Ze/Ze_D/` | 7/7 |
| **Ze-Hierarchy** | `Ze/Ze-Hierarchy/` | 7/7 |
| **Ze_Model** | `Ze/Ze_Model/` | 7/7 |
| **ze_sim** | `Ze/website/ze_sim/` | 7/7 |
| **Ze/simulator** | `Ze/simulator/` | ⚠️ 1/7 |
| **BioSense** | `BioSense/` | 7/7 |
| **BioSense/automated-microscopy** | `BioSense/instruments/automated-microscopy/` | 7/7 |
| **BioSense/CubanEEG** | `BioSense/data/cuban/oldgandalf-.../` | 7/7 |
| **FCLC** | `FCLC/` | 7/7 |
| **HAP** | `HAP/` | 7/7 |
| **Organismal_Aging** | `Organismal_Aging/` | 7/7 |

## План реорганизации (v7.0 от 2026-06-21)

> ⚠️ Следующая архитектура — **ПЛАН**, не реализована:
> - `sim_core/` — единое ядро (центриоль + counters + tissue + organism + ...)
> - `biosense/`, `fclc/`, `hap/` — переименование (lowercase)
> - `sim_cli/`, `sim_api/`, `sim_gui/`, `sim_py/` — интерфейсы
> - `calibration/`, `validation/`, `argus_bridge/`, `infogest_bridge/` — мосты

---

*Обновлено 2026-07-10 (Цикл 13 аудита — переименования Aubrey→Aubrey-Platform, ARGUS→ARGUS-Hardware, +недостающие подпроекты).*
