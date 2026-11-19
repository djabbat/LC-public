"""Variance-based sensitivity (Saltelli first-order / total-effect estimators).

Lightweight replacement for SALib: the model output is a scalar (median exit
generation, or the p53-route fraction), the design is the standard A/B matrix
scheme with one parameter resampled per column.
"""

from __future__ import annotations

from typing import Callable, Dict, Sequence

import numpy as np


def sobol_first_total(
    func: Callable[[Dict[str, float]], float],
    bounds: Dict[str, Sequence[float]],
    n_base: int = 64,
    seed: int = 7,
) -> Dict[str, Dict[str, float]]:
    """Return {'S1': {...}, 'ST': {...}} for a uniform parameter box."""
    rng = np.random.default_rng(seed)
    names = list(bounds)
    lo = np.array([bounds[n][0] for n in names], float)
    hi = np.array([bounds[n][1] for n in names], float)
    k = len(names)

    A = lo + (hi - lo) * rng.random((n_base, k))
    B = lo + (hi - lo) * rng.random((n_base, k))

    def run(mat: np.ndarray) -> np.ndarray:
        return np.array([func({n: float(v) for n, v in zip(names, row)}) for row in mat], float)

    yA, yB = run(A), run(B)
    var = np.var(np.concatenate([yA, yB]), ddof=1)
    if var <= 0:
        return {"S1": {n: 0.0 for n in names}, "ST": {n: 0.0 for n in names}}

    S1, ST = {}, {}
    for i, name in enumerate(names):
        AB = A.copy()
        AB[:, i] = B[:, i]
        yAB = run(AB)
        # Jansen (1999) estimators
        S1[name] = float(np.mean(yB * (yAB - yA)) / var)
        ST[name] = float(0.5 * np.mean((yA - yAB) ** 2) / var)
    return {"S1": S1, "ST": ST}
