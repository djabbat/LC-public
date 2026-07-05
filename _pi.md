# pi — LC

> ## 🔴 ЖЕЛЕЗНОЕ ПРАВИЛО: ПАМЯТЬ ПРЕЖДЕ ДЕЙСТВИЯ
> 
> **Перед любым действием (чтение файла, запуск команды, конвертация, ответ пользователю) — ОБЯЗАТЕЛЬНО прочитать:**
> 1. `_pi.md` — этот файл (правила)
> 2. `MAP.md` — карта проекта
> 3. `MEMORY.md` — история решений
>
> Нарушение → игнорирование памяти → потеря контекста → ошибки.
>
> ---

**Parent:** Desktop
**Created:** 2026-04-28 (CONCEPT v5.6)
**EIC Grant:** Challenges 2026, Area #2 Biotechnology for Healthy Ageing

## Описание

LC — интегративная экосистема для биомаркер-управляемых интервенций в старении как Total Chronic Disease. 5 научных подпроектов + социальная прослойка.

## Подпроекты

| Подпроект | Роль | Путь |
|---|---|---|
| **MCAOA** | Теоретическая мета-рамка (Multi-Counter Architecture of Aging) | `MCAOA/` |
| **CDATA** | Молекулярно-клеточная гипотеза (Centriolar Damage) | `MCAOA/CDATA/` |
| **Ze Theory** | Математический анзац (dτ/dt = −α·I(Z)) | `Ze/` |
| **BioSense** | Носимые биомаркеры (wearable platform) | `BioSense/` |
| **FCLC** | Федеративное обучение + DP + k-anonymity | `FCLC/` |
| **HAP** | Health Analytics Platform (клинический пилот) | `HAP/` |
| **Organismal_Aging** | Организменное старение (интегративный уровень) | `Organismal_Aging/` |

## Ключевые файлы

- `README.md` — полный концепт (эквивалент CONCEPT)
- `MAP.md` — карта подпроектов и связей
- `PARAMETERS.md` — параметры
- `TODO.md` — задачи
- `STATE.md` — статус
- `_pi.md` — этот файл

## Структура

```
LC/
├── README.md, MAP.md, STATE.md, TODO.md, PARAMETERS.md, _pi.md
├── BioSense/        — носимые устройства
├── FCLC/            — федеративное обучение
├── HAP/             — Health Analytics Platform
├── MCAOA/           — (включая CDATA, Telomere, MitoROS и др.)
├── Organismal_Aging/ — организменное старение
├── Ze/              — Ze Vectors Theory
├── realtime/        — real-time обработка
├── shared-types/    — общие типы
├── sim_core/        — ядро симуляций
├── server/          — серверная часть
├── web/             — веб-интерфейсы
├── scripts/         — утилиты
└── docs/            — документация
```
\n## 🚫 Правило: не писать про внутренние ревью во внешних документах\nНикогда не упоминать внутренние процессы (peer review, MBPR, TA-review, 9-state, audit loop) в документах для партнёров, грантов, инвесторов. Во внешних документах — только результат.\n
