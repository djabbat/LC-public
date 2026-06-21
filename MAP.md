# LC — MAP (v7.0: Organismal Aging — единый проект)

**Дата:** 2026-06-21 (полная реорганизация — объединение подпроектов)

## Новая структура (единый проект)

```
LC/  =  Organismal Aging
│
├── _pi.md                          ← правила для pi
├── CONCEPT.md                       ← ЕДИНЫЙ концепт (этот документ — главный)
├── TODO.md                          ← задачи
├── PARAMETERS.md                    ← параметры
├── MAP.md                           ← этот файл
├── STATE.md                         ← текущий статус
├── MEMORY.md                        ← история решений
├── README.md                        ← описание
├── DESIGN.md                        ← архитектура ПО
├── THEORY.md                        ← единая теория (центриоль + MCAOA + Ze)
├── EVIDENCE.md                      ← доказательная база (все PMID)
├── OPEN_PROBLEMS.md                 ← открытые вопросы
├── LICENSE                          ← Apache 2.0
├── .gitignore
│
├── sim_core/                        ← ЯДРО СИМУЛЯТОРА (Rust)
│   ├── Cargo.toml
│   └── src/
│       ├── centriole/               ← Уровень #1 (← cell_dt crates)
│       ├── counters/                ← Уровень #2 (← mcoa crates)
│       ├── tissue/                  ← Уровень #3: ткани + Z_conflict
│       ├── organism/                ← Интеграция: онтогенез, кривая старения
│       ├── species/                 ← Видовая параметризация
│       ├── microbiome/              ← Микробиом
│       ├── macrobiome/              ← Макробиом (питание, среда)
│       ├── learning/                ← Самообучение (Байес)
│       └── spatial/                 ← 3D-модель
│
├── argus_bridge/                    ← Мост к ARGUS-LP (робот)
├── infogest_bridge/                 ← Мост к INFOGEST (пищеварение)
│
├── biosense/                        ← Сенсоры (← BioSense)
├── fclc/                            ← Конфиденциальные вычисления (← FCLC)
├── hap/                             ← Health-Age Profiling (← HAP)
│
├── sim_cli/                         ← CLI (Rust)
├── sim_api/                         ← REST API (Rust)
├── sim_gui/                         ← Веб-интерфейс (Phoenix LiveView)
├── sim_py/                          ← Python bindings (PyO3)
│
├── calibration/                     ← Калибровка
│   ├── gtex/
│   ├── uk_biobank/
│   └── all_of_us/
│
├── validation/                      ← Валидация
│   └── ze_hierarchy/               ← Ze-Hierarchy (← Ze/Ze-Hierarchy)
│
├── tests/                           ← Интеграционные тесты
├── benches/                         ← Бенчмарки
│
├── docs/                            ← Документация
├── scripts/                         ← Скрипты
│
├── _archive/                        ← Архив
│   ├── subprojects_concepts/        ← Старые CONCEPT.md подпроектов
│   └── v_pre_2026-06-21/            ← Предыдущая версия
│
├── server/                          ← Серверная инфраструктура
├── web/                             ← Веб (существующий)
├── realtime/                        ← Real-time сервисы
├── services/                        ← Сервисы
├── shared-types/                    ← Общие типы
├── refs/                            ← Референсы
└── _originals/                      ← Оригиналы документов
```

## Бывшие подпроекты → новые компоненты

| Бывший подпроект | Путь | Новый путь | Статус |
|---|---|---|---|
| **MCAOA** | `MCAOA/` | `sim_core/counters/` | Интегрируется |
| **CDATA** | `MCAOA/CDATA/` | `sim_core/centriole/` | Интегрируется |
| **Ze** | `Ze/` | `sim_core/tissue/` + `validation/ze_hierarchy/` | Интегрируется |
| **BioSense** | `BioSense/` | `biosense/` | ✅ Работает |
| **FCLC** | `FCLC/` | `fclc/` | ✅ Production |
| **HAP** | `HAP/` | `hap/` | 🟡 Сабмит |
| **Organismal_Aging** | `Organismal_Aging/` | → КОРЕНЬ LC | Интегрируется |
| **Aubrey/ARGUS** | `~/Desktop/Marketing/Aubrey/` | `argus_bridge/` + `docs/argus/` | 🟡 Мост |

---

*Обновлено 2026-06-21 (v7.0 — единый проект Organismal Aging).*
