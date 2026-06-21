# Ze — PARAMETERS

**Дата:** 2026-06-11

---

## Ключевые параметры теории

| Параметр | Формула | Значение | Описание |
|----------|---------|:--------:|----------|
| **v** (Ze velocity) | `(N_T - N_S) / N` | v* = 1−ln2 ≈ **0.3069** | Баланс T/S событий (точное) |
| **τ** (Ze complexity) | `H(stream) / log₂(N)` | — | Нормализованная энтропия Шеннона |
| **Z** (Ze index) | `N_T / N` | Z* ≈ **0.731** | Доля T-событий |
| **χ** (Ze variability) | `(max - min) / mean` | — | Амплитуда осцилляции |

## Параметры симулятора

| Параметр | Значение |
|----------|---------|
| Язык | Rust (ze-core + ze-runner) |
| Бэкенд | :4001 (healthz) |
| Phoenix LiveView | :4000 |
| Ze-Hierarchy HI max | 0.532 |

## Параметры подпроектов

| Подпроект | Тип | Статус |
|-----------|-----|:------:|
| Ze_Model | Теория | With Editor (Found. of Physics) |
| Ze_CHSH | Теория | Submitted (QSMF) |
| Ze_D | Теория | Submitted (Physica A) |
| Ze_Double_Pendulum | Теория | With Editor (Physica D) |
| Ze-Hierarchy | Аппаратный | NLnet Grant принят |
| website | Веб | Требует обновления |
| simulator | Rust | ze-core + ze-runner |

---

*Создано 2026-06-11.*
