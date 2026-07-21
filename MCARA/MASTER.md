# MASTER — MCARA (Multi-Component Aging Oscillation Analysis)

**Версия:** 2026-06-15 (аудит pi)
**Проект:** LC/MCARA

## Назначение
MCARA — зонтичный проект для моделирования многокомпонентных осцилляций старения. Объединяет 5 компонентов (CEDAR, EpigeneticDrift, MitoROS, Proteostasis, Telomere) в единую модель.

## Структура

```
MCARA/
├── CONCEPT.md            ← этот документ (зонтичный концепт)
├── MASTER.md             ← этот файл (перекрёстные ссылки)
├── crates/mcara_*/        ← общие крейты (core, api, cli, simulation, compare, tests)
├── CEDAR/                ← Centriolar Damage Accumulation Theory of Aging
│   ├── CellLineageTree/  ← реконструкция клеточных линий
│   ├── Aubrey/           ← ARGUS-LP (статья)
│   └── articles/         ← публикации
├── EpigeneticDrift/      ← эпигенетический дрейф
├── MitoROS/              ← митохондриальные АФК
├── Proteostasis/         ← протеостаз (белковый гомеостаз)
└── Telomere/             ← теломерная длина
```

## Перекрёстные связи

| Компонент | Зависит от | Предоставляет |
|-----------|-----------|---------------|
| **CEDAR** | — (корень) | Базовая модель центриолярного старения |
| **CellLineageTree** | CEDAR | Древо клеточных линий |
| **ARGUS** | CEDAR + CellLineageTree | ML-модель lineage tracing |
| **EpigeneticDrift** | CEDAR (8 ссылок) | Эпигенетические часы |
| **MitoROS** | CEDAR (8 ссылок) | Митохондриальная дисфункция |
| **Proteostasis** | CEDAR (8 ссылок) | Протеостаз |
| **Telomere** | CEDAR (9 ссылок) | Теломерная динамика |

## Общие крейты

| Крейт | Назначение |
|-------|-----------|
| `mcara_core` | Общие структуры, типы, константы |
| `mcara_api` | REST API для MCARA |
| `mcara_cli` | CLI-интерфейс |
| `mcara_simulation` | Симуляция осцилляций |
| `mcara_compare` | Сравнение моделей |
| `mcara_tests` | Общие тесты |

## Правило ослабления связей

При изменении CEDAR проверить:
1. `EpigeneticDrift/CONCEPT.md` (8 ссылок)
2. `MitoROS/CONCEPT.md` (8 ссылок)
3. `Proteostasis/CONCEPT.md` (8 ссылок)
4. `Telomere/CONCEPT.md` (9 ссылок)

Стремиться к минимизации прямых ссылок — использовать `mcara_core` как промежуточный слой.

## Статус подпроектов

| Подпроект | CONCEPT | Код | Статус |
|-----------|:-------:|:---:|--------|
| CEDAR | 2KB | ✅ | 🟡 Готовится к сабмиту |
| CellLineageTree | 36KB | ✅ | 🟡 Активен |
| EpigeneticDrift | 44KB | ✅ | 🟢 Submitted |
| MitoROS | 71KB | ✅ | 🟢 Активен |
| Proteostasis | 45KB | ✅ | 🟢 Активен |
| Telomere | 49KB | ✅ | 🟢 Активен |

## Ближайшие действия
- CEDAR: завершить доработку языка, сабмит
- CellLineageTree: валидация модели
- Ослабить прямые ссылки на CEDAR → использовать mcara_core
