# CDATA-v2 Simulator

**Stochastic, ABC-SMC-calibrated model of centriole-driven stem cell exhaustion with dual Aurora A-mediated p53 inactivation.**

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Python 3.10+](https://img.shields.io/badge/python-3.10+-blue.svg)](https://www.python.org/downloads/)

## Reference

Tqemaladze J. "CDATA-v2: A Stochastic, ABC-SMC-Calibrated, GSA-Validated Model of Centriole-Driven Stem Cell Exhaustion With Dual Aurora A-Mediated p53 Inactivation." 2026.

## Installation

```bash
pip install git+https://github.com/djabbat/CDATA-sim.git
```

Or from source:

```bash
git clone https://github.com/djabbat/CDATA-sim.git
cd CDATA-sim
pip install -e .
```

## Quick Start

```python
from cdata_sim import CDATAModel, ABCSMC, SobolGSA

# Simulate a lineage tree
model = CDATAModel(seed=42)
trees = model.simulate_tree(max_generations=40, n_cells=100)
stats = model.compute_statistics(trees)
print(f"Hayflick median: {stats['hayflick_median']:.1f} generations")

# Run global sensitivity analysis
gsa = SobolGSA(n_samples=10000, seed=42)
results = gsa.run()
for name, st in sorted(results["ST"].items(), key=lambda x: -x[1])[:5]:
    print(f"  {name}: S_T = {st:.3f}")
```

## Model Overview

CDATA-v2 models stem cell exhaustion through centriolar apparatus dynamics:

- **Aurora A → p53 inactivation**: Two parallel mechanisms — Ser315 (degradation) and Ser215 (transcriptional silencing)
- **ATM → p53(Ser15) → centrosome**: Spindle checkpoint activation → senescence
- **CCP5/AGBL5**: Deglutamylase activity decline drives damage accumulation and centrosome amplification
- **CEP295 → Polo/PLK1**: Mother centriole maturation and PCM assembly

14 parameters calibrated via ABC-SMC against 2,400 observations (3 cell types × 4 endpoints).

## License

GPL v3. See [LICENSE](LICENSE).
