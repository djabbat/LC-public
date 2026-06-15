# MASTER — MCAOA (Multi-Component Aging Oscillation Analysis)

**Версия:** 2026-06-15 (аудит pi)
**Проект:** LC/MCAOA

## Назначение
MCAOA — зонтичный проект для моделирования многокомпонентных осцилляций старения. Объединяет 5 компонентов (CDATA, EpigeneticDrift, MitoROS, Proteostasis, Telomere) в единую модель.

## Структура

```
MCAOA/
├── CONCEPT.md            ← этот документ (зонтичный концепт)
├── MASTER.md             ← этот файл (перекрёстные ссылки)
├── crates/mcoa_*/        ← общие крейты (core, api, cli, simulation, compare, tests)
├── CDATA/                ← Centriolar Damage Accumulation Theory of Aging
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
| **CDATA** | — (корень) | Базовая модель центриолярного старения |
| **CellLineageTree** | CDATA | Древо клеточных линий |
| **ARGUS** | CDATA + CellLineageTree | ML-модель lineage tracing |
| **EpigeneticDrift** | CDATA (8 ссылок) | Эпигенетические часы |
| **MitoROS** | CDATA (8 ссылок) | Митохондриальная дисфункция |
| **Proteostasis** | CDATA (8 ссылок) | Протеостаз |
| **Telomere** | CDATA (9 ссылок) | Теломерная динамика |

## Общие крейты

| Крейт | Назначение |
|-------|-----------|
| `mcoa_core` | Общие структуры, типы, константы |
| `mcoa_api` | REST API для MCAOA |
| `mcoa_cli` | CLI-интерфейс |
| `mcoa_simulation` | Симуляция осцилляций |
| `mcoa_compare` | Сравнение моделей |
| `mcoa_tests` | Общие тесты |

## Правило ослабления связей

При изменении CDATA проверить:
1. `EpigeneticDrift/CONCEPT.md` (8 ссылок)
2. `MitoROS/CONCEPT.md` (8 ссылок)
3. `Proteostasis/CONCEPT.md` (8 ссылок)
4. `Telomere/CONCEPT.md` (9 ссылок)

Стремиться к минимизации прямых ссылок — использовать `mcoa_core` как промежуточный слой.

## Статус подпроектов

| Подпроект | CONCEPT | Код | Статус |
|-----------|:-------:|:---:|--------|
| CDATA | 2KB | ✅ | 🟡 Готовится к сабмиту |
| CellLineageTree | 36KB | ✅ | 🟡 Активен |
| EpigeneticDrift | 44KB | ✅ | 🟢 Submitted |
| MitoROS | 71KB | ✅ | 🟢 Активен |
| Proteostasis | 45KB | ✅ | 🟢 Активен |
| Telomere | 49KB | ✅ | 🟢 Активен |

## Ближайшие действия
- CDATA: завершить доработку языка, сабмит
- CellLineageTree: валидация модели
- Ослабить прямые ссылки на CDATA → использовать mcoa_core
