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
├── MCAOA/                           ← Multi-Counter Architecture (5 подпроектов)
│   ├── CDATA/                       ←   + 4 подподпроекта (simulator, Aubrey, CellLineageTree, articles)
│   ├── EpigeneticDrift/
│   ├── MitoROS/
│   ├── Proteostasis/
│   └── Telomere/
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
| **MCAOA** | `MCAOA/` | 7/7 |
| **CDATA** | `MCAOA/CDATA/` | 7/7 |
| **EpigeneticDrift** | `MCAOA/EpigeneticDrift/` | 7/7 |
| **MitoROS** | `MCAOA/MitoROS/` | 7/7 |
| **Proteostasis** | `MCAOA/Proteostasis/` | 7/7 |
| **Telomere** | `MCAOA/Telomere/` | 7/7 |
| **CDATA/simulator** | `MCAOA/CDATA/simulator/` | 7/7 |
| **CDATA/Aubrey** | `MCAOA/CDATA/Aubrey/` | 7/7 |
| **CDATA/CellLineageTree** | `MCAOA/CDATA/CellLineageTree/` | 7/7 |
| **CDATA/articles** | `MCAOA/CDATA/articles/` | 7/7 |
| **Ze_CHSH** | `Ze/Ze_CHSH/` | 7/7 |
| **Ze_D** | `Ze/Ze_D/` | 7/7 |
| **Ze-Hierarchy** | `Ze/Ze-Hierarchy/` | 7/7 |
| **Ze_Model** | `Ze/Ze_Model/` | 7/7 |
| **ze_sim** | `Ze/website/ze_sim/` | 7/7 |
| **Ze/simulator** | `Ze/simulator/` | ⚠️ 1/7 |
| **BioSense** | `BioSense/` | 7/7 |
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

*Обновлено 2026-07-04 (Цикл 8 аудита — фактическая структура).*
