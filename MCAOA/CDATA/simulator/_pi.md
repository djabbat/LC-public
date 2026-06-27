# _pi.md — CDATA-v2 Simulator

> 🔴 **ПРАВИЛО: перед любым действием — прочитать этот файл.**

## Идентификация
- **Проект:** CDATA-v2 Simulator
- **Родительский проект:** LC (LongevityCommon) → MCAOA → CDATA
- **Тип:** Python-пакет (pyproject.toml, setuptools)
- **GitHub:** https://github.com/djabbat/CDATA-sim (публичный, Apache 2.0 / GPL v3)
- **Язык:** Python 3.10+
- **Лицензия:** GPL v3

## Назначение
Стохастическая симуляционная модель истощения стволовых клеток через динамику центриолярного аппарата. Калибровка ABC-SMC, глобальный анализ чувствительности (Sobol GSA).

## Связи
- **LC/MCAOA/CDATA** — научная теория и концепт
- **LC/MCAOA/CDATA/Aubrey** — применение модели в грантовых заявках
- **PhD** — часть диссертационной работы

## Правила для pi
1. Все изменения — через git
2. Тесты перед коммитом: `python -m pytest tests/`
3. Не менять сигнатуры публичного API без обновления README
4. Python-стиль: black + isort

## Быстрые команды
```bash
# Запуск тестов
cd ~/Desktop/CDATA-sim && python3 -m pytest tests/ -v

# Установка в dev-режиме
cd ~/Desktop/CDATA-sim && python3 -m pip install -e ".[dev]"

# Запуск симуляции
cd ~/Desktop/CDATA-sim && python3 -c "from cdata_sim import CDATAModel; m = CDATAModel(seed=42); trees = m.simulate_tree(max_generations=60, n_cells=200); print(m.compute_statistics(trees))"
```
