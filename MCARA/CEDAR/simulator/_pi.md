# _pi.md — CEDAR-v2 Simulator

**Parent:** LC/MCARA/CEDAR

> 🔴 **RULE: before any action — read this file.**

## Identification
- **Project:** CEDAR-v2 Simulator
- **Parent project:** LC (LongevityCommon) → MCARA → CEDAR
- **Type:** Python package (pyproject.toml, setuptools)
- **GitHub:** https://github.com/djabbat/CEDAR-sim (public, Apache 2.0 / GPL v3)
- **Language:** Python 3.10+
- **License:** GPL v3

## Purpose
Stochastic simulation model of stem cell depletion through centriolar apparatus dynamics. Calibration ABC-SMC, global sensitivity analysis (Sobol GSA).

## Links
- **LC/MCARA/CEDAR** — scientific theory and concept
- **LC/MCARA/CEDAR/Aubrey** — application of the model in grant proposals
- **PhD** — part of dissertation work

## Rules for pi
1. All changes — via git
2. Tests before commit: `python -m pytest tests/`
3. Do not change public API signatures without updating README
4. Python style: black + isort

## Quick commands
bash
# Run tests
cd ~/Desktop/LC/MCARA/CEDAR/simulator && python3 -m pytest tests/ -v

# Install in dev mode
cd ~/Desktop/LC/MCARA/CEDAR/simulator && python3 -m pip install -e ".[dev]"

# Run simulation
cd ~/Desktop/LC/MCARA/CEDAR/simulator && python3 -c "from cedar_sim import CEDARModel; m = CEDARModel(seed=42); trees = m.simulate_tree(max_generations=60, n_cells=200); print(m.compute_statistics(trees))"

## English-only
All core files — ENGLISH only. Non-English text must be wrapped in `<!-- lang:XX -->...<!-- /lang:XX -->`. Autofix auto-translates unwrapped non-English text.