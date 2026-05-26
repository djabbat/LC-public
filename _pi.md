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
| **Activated** | Клинический пилот (Тбилиси) | `AIM/` |

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
├── AIM/             — ИИ-медицина (Ze-HRV, Activated clinic)
├── BioSense/        — носимые устройства
├── FCLC/            — федеративное обучение
├── MCAOA/           — (включая CDATA, Telomere, MitoROS и др.)
├── Ze/              — Ze Vectors Theory
├── scripts/         — утилиты
├── server/          — серверная часть
├── web/             — веб-интерфейсы
└── docs/            — документация
```
