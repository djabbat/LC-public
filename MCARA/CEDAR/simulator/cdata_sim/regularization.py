"""Regularisation / held-out checks for the reference implementation.

The published model reports "leave-one-cell-type-out CV error +10.8%" and a
Wasserstein distance < 0.05. Those cell-type strata are not part of this
reference implementation, so this module provides the closest reproducible
proxy: k-fold held-out error of the calibrated parameter set and a
Wasserstein-1 distance between the simulated and target exit-generation
distributions (v2.0 targets, normal approximation around median/IQR).
"""

from __future__ import annotations

from typing import Dict, Sequence

import numpy as np

from .model import CEDARModel


def wasserstein_to_target(
    exit_gen: np.ndarray, median: float = 48.0, iqr: float = 8.0, n: int = 20_000, seed: int = 3
) -> float:
    """W1 distance to a normal surrogate with the published median/IQR."""
    rng = np.random.default_rng(seed)
    sigma = iqr / 1.349
    target = rng.normal(median, sigma, size=n)
    sim = exit_gen[np.isfinite(exit_gen)]
    if sim.size == 0:
        return float("inf")
    sim = rng.choice(sim, size=min(n, sim.size), replace=True)
    xs = np.sort(np.concatenate([sim, target]))
    ws = np.concatenate([np.full(sim.size, 1.0 / sim.size), np.full(target.size, -1.0 / target.size)])
    order = np.argsort(np.concatenate([sim, target]))
    return float(np.sum(np.abs(np.cumsum(ws[order])[:-1])) * np.mean(np.diff(xs)))


def held_out_error(
    params, n_cells: int = 3_000, folds: Sequence[int] = (0, 1, 2, 3, 4)
) -> Dict[str, float]:
    """Median exit generation per fold; dispersion across folds = held-out error."""
    meds = []
    for f in folds:
        m = CEDARModel(params=params, seed=1000 + f)
        out = m.simulate(n_cells)
        meds.append(float(np.nanmedian(out["exit_gen"])))
    meds = np.array(meds)
    return {
        "fold_medians": meds.tolist(),
        "mean": float(meds.mean()),
        "sd": float(meds.std(ddof=1)),
        "cv_pct": float(100.0 * meds.std(ddof=1) / meds.mean()),
    }
