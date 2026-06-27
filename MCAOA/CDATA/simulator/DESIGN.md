# DESIGN.md — CDATA-v2 Simulator

## Архитектура

```
┌─────────────────────────────────┐
│         CDATAModel              │
│  ┌──────────┐  ┌─────────────┐  │
│  │ Centriole │  │  p53 cascade │  │
│  │ dynamics  │  │  (Aurora A,  │  │
│  │ (CCP5,    │  │   ATM paths) │  │
│  │  CEP295)  │  └─────────────┘  │
│  └──────────┘                    │
│  ┌──────────┐  ┌─────────────┐  │
│  │ Division │  │  Senescence  │  │
│  │  cycle   │  │  /Apoptosis  │  │
│  └──────────┘  └─────────────┘  │
└─────────────────────────────────┘
         │              │
         ▼              ▼
┌─────────────┐  ┌─────────────┐
│   ABCSMC    │  │  SobolGSA   │
│ calibration │  │ sensitivity │
└─────────────┘  └─────────────┘
```

## Компоненты

### `model.py` — CDATAModel
- `simulate_tree(max_generations, n_cells)` → lineage trees
- `compute_statistics(trees)` → hayflick_median, senescence_rate, apoptosis_rate
- 14 параметров (конфигурируемые)
- Стохастический (seed-контролируемый)

### `abc_smc.py` — ABCSMC
- ABC-SMC algorithm (Toni et al., 2009)
- Adaptive ε-threshold
- 10 популяций
- Сохраняет posterior distribution

### `gsa.py` — SobolGSA
- Saltelli sampling (Sobol sequences)
- First-order (S1) и total-effect (ST) индексы
- n_samples=10000 по умолчанию

### `utils.py`
- Генерация отчётов
- Визуализация (опционально matplotlib)

## Зависимости
- `numpy` (≥1.24) — обязательная
- `scipy` — опциональная (dev)
- `matplotlib` — опциональная (dev)
- `pytest` — тестирование

## Принципы
1. Воспроизводимость (seed)
2. Модульность (модель, калибровка, анализ — отдельные классы)
3. Чистый Python (без C-расширений)
