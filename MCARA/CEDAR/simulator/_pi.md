# _pi.md — CEDAR-v2 Simulator

**Parent:** LC/MCARA/CEDAR

> 🔴 **ПРАВИЛО: перед любым действием — прочитать этот файл.**

## Идентификация
- **Проект:** CEDAR-v2 Simulator
- **Родительский проект:** LC (LongevityCommon) → MCARA → CEDAR
- **Тип:** Python-пакет (pyproject.toml, setuptools)
- **GitHub:** https://github.com/djabbat/CEDAR-sim (публичный, Apache 2.0 / GPL v3)
- **Язык:** Python 3.10+
- **Лицензия:** GPL v3

## Назначение
Стохастическая симуляционная модель истощения стволовых клеток через динамику центриолярного аппарата. Калибровка ABC-SMC, глобальный анализ чувствительности (Sobol GSA).

## Связи
- **LC/MCARA/CEDAR** — научная теория и концепт
- **LC/MCARA/CEDAR/Aubrey** — применение модели в грантовых заявках
- **PhD** — часть диссертационной работы

## Правила для pi
1. Все изменения — через git
2. Тесты перед коммитом: `python -m pytest tests/`
3. Не менять сигнатуры публичного API без обновления README
4. Python-стиль: black + isort

## Быстрые команды
```bash
# Запуск тестов
cd ~/Desktop/LC/MCARA/CEDAR/simulator && python3 -m pytest tests/ -v

# Установка в dev-режиме
cd ~/Desktop/LC/MCARA/CEDAR/simulator && python3 -m pip install -e ".[dev]"

# Запуск симуляции
cd ~/Desktop/LC/MCARA/CEDAR/simulator && python3 -c "from cedar_sim import CEDARModel; m = CEDARModel(seed=42); trees = m.simulate_tree(max_generations=60, n_cells=200); print(m.compute_statistics(trees))"
```
