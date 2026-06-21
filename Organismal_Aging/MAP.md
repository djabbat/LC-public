# Organismal Aging — MAP

**Дата:** 2026-06-21

## Структура Organismal_Aging/ (в составе LC/)

```
Organismal_Aging/
├── CONCEPT.md                       ← Концепт (v2.0)
├── THEORY.md                        ← Математический формализм
├── PARAMETERS.md                    ← Числовые параметры
├── MAP.md                           ← Этот файл
├── STATE.md                         ← Текущий статус
├── MEMORY.md                        ← История решений
├── TODO.md                          ← Задачи
├── EVIDENCE.md                      ← Доказательная база
├── README.md                        ← Описание
├── _pi.md                           ← Правила для pi
├── OPEN_PROBLEMS.md                 ← Открытые вопросы
├── DESIGN.md                        ← Архитектура ПО
│
├── docs/                            ← Документация
│   ├── ARCHITECTURE.md
│   ├── EXTENSION_GUIDE.md           ← Как добавить вид/ткань/счётчик
│   └── GLOSSARY.md
│
├── scripts/                         ← Скрипты
│   ├── run_simulation.sh
│   └── benchmark.sh
│
└── _archive/                        ← Архив
```

## Интеграция в LC

Organismal_Aging — **детализированная документация** для компонента-интегратора внутри LC.

LC (корень):
- `sim_core/` — код ядра (Rust)
- `argus_bridge/` — мост к ARGUS-LP
- `infogest_bridge/` — мост к INFOGEST
- `biosense/` — сенсоры
- `fclc/` — безопасность
- `hap/` — Health-Age Profiling
- `sim_gui/` — веб-интерфейс

---

*Organismal Aging MAP v1.0 — 2026-06-21.*
