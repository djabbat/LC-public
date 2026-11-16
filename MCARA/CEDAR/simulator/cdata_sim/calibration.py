"""Summaries, calibration grid and reproducibility helpers.

The v2.0 target values printed in the published article are
    Hayflick median 48 +/- 0.4 generations, IQR 8 +/- 0.6,
    pathway split 28.5% p53-mediated / 71.5% replicative exhaustion,
    centrosome amplification frequency 52%.

`calibrate()` searches a small grid over the interpretive knobs of the
reference implementation (not over the published table values) to land on
those targets, and reports the achieved residuals so the disagreement is
visible instead of hidden.
"""

from __future__ import annotations

from itertools import product
from typing import Dict, Iterable, Tuple

import numpy as np

from .model import CEDARModel, ModelParams

# published v2.0 reference targets
TARGETS_V20 = {
    "hayflick_median": 48.0,
    "hayflick_iqr": 8.0,
    "p53_route_pct": 28.5,
    "amplification_pct": 52.0,
}


def summary(out: Dict[str, np.ndarray]) -> Dict[str, float]:
    route = out["route"]
    n = route.size
    fin = np.isfinite(out["exit_gen"])
    ex = out["exit_gen"][fin]
    q25, q50, q75 = np.percentile(ex, [25, 50, 75]) if ex.size else (np.nan,) * 3
    counts = {r: float(np.sum(route == r)) for r in
              ("exhaustion", "atm", "piddosome", "silenced", "censored")}
    return {
        "n": float(n),
        "median_exit": float(q50),
        "iqr_exit": float(q75 - q25),
        "mean_exit": float(np.mean(ex)) if ex.size else float("nan"),
        "frac_exhaustion": counts["exhaustion"] / n,
        "frac_atm": counts["atm"] / n,
        "frac_piddosome": counts["piddosome"] / n,
        "frac_silenced": counts["silenced"] / n,
        "frac_censored": counts["censored"] / n,
        "frac_amp_at_exit": float(np.mean(out["amplified_at_exit"])),
        "frac_amp_ever": float(np.mean(out["amplified_ever"])),
        "frac_corrected": float(np.mean(out["corrected"])),
        "silenced_or_censored": (counts["silenced"] + counts["censored"]) / n,
    }


def _score(st: Dict[str, float], tgt: Dict[str, float]) -> float:
    """Relative squared error against the v2.0 targets (lower is better)."""
    s = 0.0
    s += ((st["median_exit"] - tgt["hayflick_median"]) / 10.0) ** 2
    s += ((st["iqr_exit"] - tgt["hayflick_iqr"]) / 4.0) ** 2
    s += (((st["frac_atm"] + st["frac_piddosome"]) * 100 - tgt["p53_route_pct"]) / 10.0) ** 2
    s += ((st["frac_amp_at_exit"] * 100 - tgt["amplification_pct"]) / 15.0) ** 2
    return float(s)


def calibrate(
    n_cells: int = 2_000,
    grid: Iterable[Tuple[float, float, float]] | None = None,
    seed: int = 12345,
    verbose: bool = True,
) -> Tuple[ModelParams, Dict[str, float]]:
    """Grid search over (q_amp, kappa, aurA_mean) to recover the v2.0 targets."""
    if grid is None:
        grid = product(
            (0.0015, 0.0030, 0.0045, 0.0060),   # q_amp
            (0.250, 0.350, 0.450),              # kappa
            (0.8, 1.0, 1.3),                    # aurA_mean
        )
    best = (np.inf, None, None)
    for q, kappa, au in grid:
        p = ModelParams(q_amp=q, kappa=kappa, aurA_mean=au)
        m = CEDARModel(params=p, seed=seed)
        st = summary(m.simulate(n_cells, regime="v2.0_atm_only"))
        sc = _score(st, TARGETS_V20)
        if sc < best[0]:
            best = (sc, p, st)
        if verbose:
            print(
                f"  q_amp={q:.4f} kappa={kappa:.3f} aurA={au:.2f} -> "
                f"median={st['median_exit']:.1f} iqr={st['iqr_exit']:.1f} "
                f"p53={100*(st['frac_atm']+st['frac_piddosome']):.1f}% "
                f"amp={100*st['frac_amp_at_exit']:.1f}% score={sc:.3f}",
                flush=True,
            )
    sc, params, st = best
    if verbose:
        print(f"\nBEST score={sc:.4f}")
    return params, {"score": sc, **st}
