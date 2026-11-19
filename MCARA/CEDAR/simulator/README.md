# CEDAR-v2.1 Simulator (reference implementation)

> ⚠️ **Provenance (2026-11-16).** The original `cdata_sim` sources are lost (the editable install pointed at a deleted directory, and `github.com/djabbat/CEDAR-sim` does not exist). This directory is an **independent re-implementation** of the published architecture, extended with the v2.1 modules. Every interpretive choice is listed in [`MODEL_SPEC_v2.1.md`](MODEL_SPEC_v2.1.md). Reproduction of the published v2.0 targets: median 50.5 vs 48, IQR 9.5 vs 8, p53 route 26.7% vs 28.5%, amplification 49.9% vs 52% — fitted, not exact.
>
> Results: `results/v21_results.md` · regenerate with `python3 scripts/run_v21_results.py` (≈50 s) · tests: `python3 -m pytest tests -q`

**Stochastic, ABC-SMC-calibrated model of centriole-driven stem cell exhaustion with dual Aurora A-mediated p53 inactivation.**

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Python 3.10+](https://img.shields.io/badge/python-3.10+-blue.svg)](https://www.python.org/downloads/)

## Reference

Tqemaladze J. "CEDAR-v2: A Stochastic, ABC-SMC-Calibrated, GSA-Validated Model of Centriole-Driven Stem Cell Exhaustion With Dual Aurora A-Mediated p53 Inactivation." 2026.

## Installation

```bash
# NOTE (2026-11-16): this URL does not exist yet — the original code was lost.
# Use the local reference implementation instead:
pip install -e .
```

Or from the local reference implementation:

```bash
cd ~/Desktop/LC/MCARA/CEDAR/simulator
pip install -e .
```

## Quick Start

```python
from cdata_sim import CEDARModel, calibrate, sobol_first_total

from cdata_sim.calibration import summary

# v2.0 baseline (ATM-only architecture)
model = CEDARModel(seed=42)
stats = summary(model.simulate(10_000, regime="v2.0_atm_only"))
print(f"Hayflick median: {stats['median_exit']:.1f} generations")

# v2.1 scenarios (PIDDosome brake + autophagy correction)
print(summary(model.simulate(10_000, regime="v2.1_wt"))["frac_piddosome"])

# Sensitivity (Saltelli estimators)
results = sobol_first_total(lambda p: p["kappa"], {"kappa": (0.5, 1.1)}, n_base=64)
for name, st in sorted(results["ST"].items(), key=lambda x: -x[1])[:5]:
    print(f"  {name}: S_T = {st:.3f}")
```

## Model Overview

CEDAR-v2 models stem cell exhaustion through centriolar apparatus dynamics:

- **Aurora A → p53 inactivation**: Two parallel mechanisms — Ser315 (degradation) and Ser215 (transcriptional silencing)
- **ATM → p53(Ser15) → centrosome**: Spindle checkpoint activation → senescence
- **CCP5/AGBL5**: Deglutamylase activity decline drives damage accumulation and centrosome amplification
- **CEP295 → Polo/PLK1**: Mother centriole maturation and PCM assembly

14 parameters calibrated via ABC-SMC against 2,400 observations (3 cell types × 4 endpoints).

## License

GPL v3. See [LICENSE](LICENSE).
