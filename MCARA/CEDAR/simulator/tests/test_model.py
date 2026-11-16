"""Smoke and behaviour tests for the CEDAR-v2.1 reference implementation."""

import numpy as np
import pytest

from cdata_sim.calibration import summary
from cdata_sim.model import CEDARModel, ModelParams

BASE = dict(
    decay_scale=2.45, s_D=0.0060, q_amp=0.0079, kappa=0.85, r_mult_cv=0.20, aurA_cv=0.62
)


def run(regime, n=2000, seed=3, **over):
    m = CEDARModel(params=ModelParams(**BASE, **over), seed=seed)
    return m.simulate(n, regime=regime)


def test_all_cells_exit():
    out = run("v2.0_atm_only")
    assert np.all(np.isfinite(out["exit_gen"]))
    assert set(np.unique(out["route"])) <= {"exhaustion", "exhaustion_silenced", "atm", "piddosome", "censored"}


def test_hayflick_in_plausible_range():
    st = summary(run("v2.0_atm_only"))
    assert 40.0 <= st["median_exit"] <= 60.0
    assert 5.0 <= st["iqr_exit"] <= 15.0


def test_piddosome_absent_in_v20():
    st = summary(run("v2.0_atm_only"))
    assert st["frac_piddosome"] == 0.0


def test_piddosome_present_in_v21_wt_and_absent_in_knockout():
    wt = summary(run("v2.1_wt"))
    ko = summary(run("v2.1_pidd1_ko"))
    assert wt["frac_piddosome"] > 0.05
    assert ko["frac_piddosome"] == 0.0


def test_knockout_reduces_p53_route():
    wt = summary(run("v2.1_wt"))
    ko = summary(run("v2.1_pidd1_ko"))
    assert (wt["frac_atm"] + wt["frac_piddosome"]) > (ko["frac_atm"] + ko["frac_piddosome"]) + 0.05


def test_autophagy_reduces_instantaneous_amplification():
    with_auto = summary(run("v2.1_pidd1_ko"))
    without = summary(run("v2.0_atm_only"))
    assert with_auto["frac_amp_at_exit"] < without["frac_amp_at_exit"]


def test_high_aurora_a_silences_arrest():
    low = summary(run("v2.1_wt"))
    high = summary(run("v2.1_wt", aurA_mean=3.0))
    assert (high["frac_atm"] + high["frac_piddosome"]) < (low["frac_atm"] + low["frac_piddosome"])


def test_determinism_given_seed():
    a = summary(run("v2.1_wt"))
    b = summary(run("v2.1_wt"))
    assert a["median_exit"] == b["median_exit"]
    assert a["frac_atm"] == b["frac_atm"]
