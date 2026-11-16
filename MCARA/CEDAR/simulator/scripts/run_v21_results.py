"""Reproduce the CEDAR-v2.0 baseline and produce the v2.1 scenario table.

Usage:  python3 scripts/run_v21_results.py [--out results/v21_results.md]

Everything printed here is generated from cdata_sim (reference implementation).
See MODEL_SPEC_v2.1.md for the modelling choices that the published article
does not specify.
"""

from __future__ import annotations

import argparse
import json
import time
from pathlib import Path

import numpy as np

import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from cdata_sim.calibration import TARGETS_V20, summary  # noqa: E402
from cdata_sim.model import CEDARModel, ModelParams  # noqa: E402
from cdata_sim.regularization import held_out_error, wasserstein_to_target  # noqa: E402
from cdata_sim.sensitivity import sobol_first_total  # noqa: E402

# calibrated reference set (reproduces the v2.0 targets to within a few percent)
BASE = dict(
    decay_scale=2.45,
    s_D=0.0060,
    q_amp=0.0079,
    kappa=0.85,
    r_mult_cv=0.20,
    aurA_cv=0.62,
)

SEEDS = tuple(range(10))
N = 10_000


def agg(model: CEDARModel, regime: str | None = None) -> dict:
    rows = []
    for s in SEEDS:
        model.rng = np.random.default_rng(s)
        rows.append(summary(model.simulate(N, regime=regime)))
    keys = [k for k, v in rows[0].items() if isinstance(v, float)]
    return {k: (float(np.mean([r[k] for r in rows])), float(np.std([r[k] for r in rows]))) for k in keys}


def fmt(name: str, st: dict) -> str:
    m = lambda k: f"{st[k][0]:.1f}"  # noqa: E731
    p53 = 100 * (st["frac_atm"][0] + st["frac_piddosome"][0])
    return (
        f"| {name} | {m('median_exit')} ± {st['median_exit'][1]:.2f} | {m('iqr_exit')} | "
        f"{p53:.1f} | {100*st['frac_exhaustion_total'][0]:.1f} | "
        f"{100*st['frac_piddosome'][0]:.1f} | {100*st['frac_silenced'][0]:.1f} | "
        f"{100*st['frac_amp_at_exit'][0]:.1f} | {100*st['frac_corrected'][0]:.1f} |"
    )


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=None)
    args = ap.parse_args()
    t0 = time.time()

    p = ModelParams(**BASE)
    model = CEDARModel(params=p)

    print("# CEDAR-v2.1 reference results")
    print(f"\nBase parameter set: {BASE}")
    print(f"Cells per run: {N}; seeds: {len(SEEDS)}\n")

    # ---------------- 1. v2.0 baseline reproduction -------------------------
    base20 = agg(model, "v2.0_atm_only")
    print("## 1. v2.0 baseline reproduction (ATM-only, no autophagy, P=0)")
    print("| metric | reference | published v2.0 | deviation |")
    print("|---|---|---|---|")
    for key, tgt_key in (
        ("median_exit", "hayflick_median"),
        ("iqr_exit", "hayflick_iqr"),
        ("amp_pct", "amplification_pct"),
    ):
        if key == "amp_pct":
            val, tgt = 100 * base20["frac_amp_at_exit"][0], TARGETS_V20[tgt_key]
        else:
            val, tgt = base20[key][0], TARGETS_V20[tgt_key]
        print(f"| {key} | {val:.2f} | {tgt} | {100*(val-tgt)/tgt:+.1f}% |")
    p53_20 = 100 * (base20["frac_atm"][0] + base20["frac_piddosome"][0])
    print(f"| p53 route, % | {p53_20:.2f} | {TARGETS_V20['p53_route_pct']} | "
          f"{100*(p53_20-TARGETS_V20['p53_route_pct'])/TARGETS_V20['p53_route_pct']:+.1f}% |")

    # ---------------- 2. v2.1 scenarios ------------------------------------
    print("\n## 2. v2.1 scenarios")
    print("| scenario | Hayflick median | IQR | p53-route % | exhaustion % | PIDDosome % | "
          "silenced % | amplified at exit % | corrected % |")
    print("|---|---|---|---|---|---|---|---|---|")
    scenarios = {
        "v2.0 baseline (ATM only)": ("v2.0_atm_only", {}),
        "v2.1 WT (P=1, autophagy on)": ("v2.1_wt", {}),
        "v2.1 PIDD1-KO / CASP2-KO (P=0)": ("v2.1_pidd1_ko", {}),
        "v2.1 WT + Aurora A x3 (Tet-On)": ("v2.1_wt", {"aurA_mean": 3.0}),
        "v2.1 WT + Aurora A x3 + alisertib-like (x0.35)": ("v2.1_wt", {"aurA_mean": 1.05}),
        "v2.0 + Aurora A x3 (no PIDDosome, no autophagy)": ("v2.0_atm_only", {"aurA_mean": 3.0}),
    }
    results = {}
    for name, (regime, over) in scenarios.items():
        m = CEDARModel(params=p.copy(**over)) if over else model
        st = agg(m, regime)
        results[name] = st
        print(fmt(name, st))

    print("\n## 3. Route partition, v2.0 vs v2.1 (WT)")
    for label in ("v2.0 baseline (ATM only)", "v2.1 WT (P=1, autophagy on)"):
        st = results[label]
        print(
            f"- **{label}**: exhaustion {100*st['frac_exhaustion'][0]:.1f}%, "
            f"exhaustion(silenced) {100*st['frac_silenced'][0]:.1f}%, "
            f"ATM-arrest {100*st['frac_atm'][0]:.1f}%, "
            f"PIDDosome-arrest {100*st['frac_piddosome'][0]:.1f}%, "
            f"censored {100*st['frac_censored'][0]:.1f}%"
        )

    # ---------------- 4. sensitivity ---------------------------------------
    print("\n## 4. Variance-based sensitivity, v2.1 WT")

    def f_median(par):
        m = CEDARModel(params=p.copy(decay_scale=par["decay_scale"], s_D=par["s_D"],
                                     kappa=par["kappa"], N_crit_P=par["N_crit_P"],
                                     lam_p21=par["lam_p21"], k_auto=par["k_auto"]),
                       seed=4)
        return summary(m.simulate(4000, regime="v2.1_wt"))["mean_exit"]

    def f_silenced(par):
        m = CEDARModel(params=p.copy(decay_scale=par["decay_scale"], s_D=par["s_D"],
                                     kappa=par["kappa"], N_crit_P=par["N_crit_P"],
                                     lam_p21=par["lam_p21"], k_auto=par["k_auto"]),
                       seed=4)
        return 100 * summary(m.simulate(4000, regime="v2.1_wt"))["frac_silenced"]

    bounds = {
        "decay_scale": (2.0, 2.9),
        "s_D": (0.004, 0.010),
        "kappa": (0.5, 1.1),
        "N_crit_P": (1.3, 2.2),
        "lam_p21": (0.2, 1.0),
        "k_auto": (0.2, 0.8),
    }
    sens_med = sobol_first_total(f_median, bounds, n_base=96, seed=9)
    sens_sil = sobol_first_total(f_silenced, bounds, n_base=96, seed=9)
    print("| parameter | S1 (median) | ST (median) | S1 (silenced %) | ST (silenced %) |")
    print("|---|---|---|---|---|")
    for name in bounds:
        print(f"| {name} | {sens_med['S1'][name]:+.3f} | {sens_med['ST'][name]:+.3f} | "
              f"{sens_sil['S1'][name]:+.3f} | {sens_sil['ST'][name]:+.3f} |")

    # ---------------- 5. held-out / distance --------------------------------
    print("\n## 5. Regularisation checks")
    ho = held_out_error(p, n_cells=3000)
    print(f"- Held-out fold medians: {[round(x,1) for x in ho['fold_medians']]} "
          f"(sd {ho['sd']:.2f}, CV {ho['cv_pct']:.1f}%)")
    model.rng = np.random.default_rng(0)
    out = model.simulate(N, regime="v2.0_atm_only")
    w1 = wasserstein_to_target(out["exit_gen"])
    print(f"- Wasserstein-1 to a normal surrogate with the published median/IQR: {w1:.2f} generations")
    print(f"\nRuntime: {time.time()-t0:.1f} s")

    if args.out:
        path = Path(args.out)
        path.parent.mkdir(parents=True, exist_ok=True)
        payload = {
            "base_params": BASE,
            "published_targets": TARGETS_V20,
            "scenarios": {k: {m: v for m, v in st.items()} for k, st in results.items()},
            "sensitivity_median": sens_med,
            "sensitivity_silenced": sens_sil,
            "held_out": ho,
            "wasserstein": w1,
            "n_cells": N,
            "seeds": list(SEEDS),
            "version": "2.1.0-ref",
        }
        path.with_suffix(".json").write_text(json.dumps(payload, indent=2, default=float))
        print(f"Wrote {path.with_suffix('.json')}")


if __name__ == "__main__":
    main()
