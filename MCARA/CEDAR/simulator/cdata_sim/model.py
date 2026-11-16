"""Core stochastic model (CEDAR-v2.0 architecture + v2.1 modules).

State per cell (vectorised over cells with numpy):
    D     damage burden
    Ms    structural maintenance (centriole/PCM integrity)
    Mf    functional maintenance
    A     division asymmetry (v2.0: additive decay + recovery)
    Nmat  mature centrosome count (1 = normal, >=2 = amplified)
    plk1  PLK1 activity
    aurA  Aurora A activity (fixed per cell; log-normal across the population)
    p53c  consecutive-generation counter of p53 pathway activity (delay tau)

Exits:
    'exhaustion'  A < A_exit                         (v2.0)
    'atm'         ATM -> p53 -> arrest               (v2.0, v2.1 marks route)
    'piddosome'   PIDD1-RAIDD-CASP2 -> MDM2 -> p53 -> p21 arrest (v2.1 only)
    'silenced'    p53 engaged but transactivation blocked by p-Aurora A(Ser215);
                  cell keeps cycling (pre-malignant branch)  (v2.1 only)
"""

from __future__ import annotations

from dataclasses import dataclass, field, replace
from typing import Dict

import numpy as np

# ----------------------------------------------------------------------------
# parameter set (published v2.0 table, plus v2.1 additions)
# ----------------------------------------------------------------------------

REGIMES: Dict[str, Dict[str, float]] = {
    # name: overrides applied on top of ModelParams()
    "v2.0_atm_only": {"piddosome": 0.0, "autophagy": 0.0},
    "v2.1_wt": {"piddosome": 1.0, "autophagy": 1.0},
    "v2.1_pidd1_ko": {"piddosome": 0.0, "autophagy": 1.0},
}


@dataclass
class ModelParams:
    # --- degradation / damage (published v2.0 values) ---
    mu_P: float = 0.020
    r_0: float = 0.040
    r_age: float = 0.002
    lam_age: float = 0.025
    k_cat: float = 0.100
    mu_s: float = 0.015
    mu_f: float = 0.015
    eta_s: float = 0.040
    eta_f: float = 0.050
    omega: float = 0.030
    alpha_CEP: float = 0.050
    beta_0: float = 0.040
    alpha_AurA_215: float = 4.0
    alpha_AurA_315: float = 3.0
    # --- asymmetry / senescence (published v2.0 values) ---
    gamma_D: float = 0.005
    gamma_S: float = 0.015
    gamma_F: float = 0.015
    gamma_N: float = 0.005
    D_healthy: float = 0.3
    M_healthy: float = 0.7
    R_recovery: float = 0.035
    K_s: float = 1.5
    K_f: float = 1.5
    kappa: float = 0.350
    kappa_D: float = 0.100
    D_senescence: float = 1.0
    sigma_N: float = 0.200
    tau: float = 2.5
    # --- interpretive knobs of this reference implementation ---
    A_init: float = 1.0
    A_exit: float = 0.55          # published: exhaustion when A < 0.55
    q_amp: float = 0.004          # per-generation centrosome-amplification event rate
    aurA_mean: float = 1.0        # population mean Aurora A activity
    aurA_cv: float = 0.35
    p53_threshold: float = 1.0    # active p53 above this engages the arrest counter
    Nmat_max: float = 8.0
    G_max: int = 250              # hard generation cap
    s_D: float = 0.15             # damage scale in the ATM term (interpretive)
    decay_scale: float = 1.0      # scalar on the asymmetry-decay sum (interpretive)
    r_mult_cv: float = 0.30       # per-cell heterogeneity of damage production
    init_cv: float = 0.10         # per-cell heterogeneity of Ms/Mf/A at birth
    # --- v2.1 modules ---
    piddosome: float = 1.0        # PIDDosome competence P (1 = WT, 0 = PIDD1/CASP2 KO)
    N_crit_P: float = 1.6         # mature centrosome count sensed by PIDD1
    k_P: float = 4.0              # sensing steepness
    lam_p21: float = 0.60         # p21 arrest rate
    autophagy: float = 1.0        # autophagy correction edge on/off
    p_lc3b: float = 0.12          # per-generation probability of entering LC3B+ state
    k_auto: float = 0.45          # correction-rate multiplier (RAC1-ATG16L1 axis)
    G215_silencing: float = 0.5   # transactivation gate below which arrest is blocked

    def copy(self, **kw) -> "ModelParams":
        return replace(self, **kw)


# ----------------------------------------------------------------------------
# helpers
# ----------------------------------------------------------------------------

def _sigmoid(x: np.ndarray) -> np.ndarray:
    return 1.0 / (1.0 + np.exp(-np.clip(x, -60, 60)))


ROUTES = ("exhaustion", "exhaustion_silenced", "atm", "piddosome", "censored")


# ----------------------------------------------------------------------------
# model
# ----------------------------------------------------------------------------

class CEDARModel:
    """Vectorised single-cell lineage simulator."""

    def __init__(self, params: ModelParams | None = None, seed: int = 0):
        self.p = params or ModelParams()
        self.rng = np.random.default_rng(seed)

    # -- one population -----------------------------------------------------
    def simulate(self, n_cells: int = 10_000, regime: str | None = None) -> Dict[str, np.ndarray]:
        p = self.p
        if regime is not None:
            p = p.copy(**REGIMES[regime])
        rng = self.rng
        n = int(n_cells)

        D = np.zeros(n)
        r_mult = np.exp(p.r_mult_cv * rng.standard_normal(n))
        Ms = np.clip(1.0 + p.init_cv * rng.standard_normal(n), 0.3, 2.0)
        Mf = np.clip(1.0 + p.init_cv * rng.standard_normal(n), 0.3, 2.0)
        A = np.clip(p.A_init + p.init_cv * rng.standard_normal(n), 0.3, 1.2)
        Nmat = np.ones(n)
        plk1 = np.ones(n)
        aurA = p.aurA_mean * np.exp(p.aurA_cv * rng.standard_normal(n))
        aurA = np.clip(aurA, 0.2, 8.0)
        p53c = np.zeros(n, dtype=np.int32)

        exit_gen = np.full(n, np.nan)
        route = np.full(n, "", dtype=object)
        amp_seen = np.zeros(n, dtype=bool)
        corrected = np.zeros(n, dtype=bool)
        silenced_ever = np.zeros(n, dtype=bool)
        blocked_hist = np.zeros(n, dtype=bool)
        nmat_at_exit = np.full(n, np.nan)

        alive = np.ones(n, dtype=bool)

        for g in range(1, p.G_max + 1):
            idx = np.flatnonzero(alive)
            if idx.size == 0:
                break
            k = idx.size
            d, ms, mf, a = D[idx], Ms[idx], Mf[idx], A[idx]
            nm, pk, au = Nmat[idx], plk1[idx], aurA[idx]

            # --- damage: production, catalytic removal, hyperglutamylation ---
            ccp5 = np.exp(-p.lam_age * g)
            d = d + p.r_0 * r_mult[idx] * (1.0 + p.r_age * g) - p.k_cat * ccp5 * d + p.omega * d * (1.0 - ccp5)
            d = np.clip(d, 0.0, None)

            # --- maintenance: decay, damage coupling, saturating synthesis ---
            ms = ms * (1.0 - p.mu_s) - p.eta_s * d + p.alpha_CEP / (1.0 + ms / p.K_s)
            mf = mf * (1.0 - p.mu_f) - p.eta_f * d + p.beta_0 * pk / (1.0 + mf / p.K_f)
            ms = np.clip(ms, 0.02, 3.0)
            mf = np.clip(mf, 0.02, 3.0)

            # --- PLK1: decay, replenished while the centrosome cycle is intact ---
            pk = pk * (1.0 - p.mu_P) + 0.020 * (ms / (1.0 + ms / p.K_s))
            pk = np.clip(pk, 0.02, 2.0)

            # --- centrosome number: amplification events + inheritance noise ---
            q_amp = p.q_amp * (1.0 + 0.35 * au) * np.clip(1.5 - mf, 0.2, 2.0) * (1.0 + 0.5 * d)
            amp_event = rng.random(k) < q_amp
            nm = np.where(amp_event, nm * 2.0, nm)
            nm = nm * (1.0 + p.sigma_N * 0.05 * rng.standard_normal(k))
            nm = np.clip(nm, 1.0, p.Nmat_max)

            # --- v2.1: autophagy correction edge (LC3B+ -> Nmat back to 1) ---
            if p.autophagy > 0.0:
                p_lc3b = np.clip(p.p_lc3b * (1.0 + 1.5 * np.clip(1.0 - mf, 0.0, 1.0)), 0.0, 0.9)
                lc3b = rng.random(k) < p_lc3b
                corr = lc3b & (nm > 1.0) & (rng.random(k) < np.clip(p.k_auto * (p.autophagy), 0, 1))
                nm = np.where(corr, 1.0, nm)
                if corr.any():
                    corrected[idx[corr]] = True
            amp_seen[idx] |= nm > 1.5

            # --- asymmetry: additive decay + recovery (v2.0 scheme) ---
            decay = p.decay_scale * (
                p.gamma_D * np.clip(d - p.D_healthy, 0.0, None)
                + p.gamma_S * np.clip(p.M_healthy - ms, 0.0, None)
                + p.gamma_F * np.clip(p.M_healthy - mf, 0.0, None)
                + p.gamma_N * np.clip(nm - 1.0, 0.0, None)
            )
            healthy = (d < p.D_healthy) & (ms > p.M_healthy) & (mf > p.M_healthy)
            a = a - decay + np.where(healthy, p.R_recovery, 0.0)
            a = np.clip(a, 0.0, 1.2)

            # --- p53 inputs -------------------------------------------------
            # ATM axis: centrosome amplification + direct damage, opposed by Aurora A(Ser315)
            atm = p.kappa * np.clip(nm - 1.0, 0.0, None) + p.kappa_D * np.clip(
                d - p.D_senescence, 0.0, None
            ) / p.s_D
            p53 = atm / (1.0 + p.alpha_AurA_315 * au)

            # transactivation gate (Aurora A -> p53 Ser215)
            G215 = 1.0 / (1.0 + (p.alpha_AurA_215 / 3.0) * au)

            # v2.1 PIDDosome axis: an Independent, ATM-independent p21 brake.
            # Sensing is instantaneous (no tau delay), and the Aurora A(Ser215)
            # gate blocks p53 transactivation, so high Aurora A silences the brake.
            A_PID = p.piddosome * _sigmoid(p.k_P * (nm - p.N_crit_P))
            p_p21 = 1.0 - np.exp(-p.lam_p21 * A_PID * G215)

            engaged = p53 > p.p53_threshold
            p53c[idx] = np.where(engaged, p53c[idx] + 1, 0)

            tau_int = int(np.ceil(p.tau))
            arrest_due = p53c[idx] >= tau_int

            # route attribution: the PIDDosome brake is its own exit event
            pid_arm = rng.random(k) < np.clip(p_p21, 0.0, 1.0)
            atm_arm = arrest_due & ~pid_arm

            silencing = G215 < p.G215_silencing
            blocked = arrest_due & silencing
            atm_exit = atm_arm & ~silencing

            exhaust = a < p.A_exit

            # --- apply exits ---------------------------------------------------
            # exhaustion wins; a blocked (silenced) arrest is recorded as a
            # sub-label of the exhaustion route, exactly as v2.0 folds the
            # silenced branch into replicative exhaustion.
            blocked_hist[idx] = blocked_hist[idx] | blocked
            ex_exhaust = exhaust
            ex_exhaust_sil = ex_exhaust & blocked_hist[idx]
            ex_exhaust_plain = ex_exhaust & ~blocked_hist[idx]
            ex_atm = (~ex_exhaust) & atm_exit & (~pid_arm)
            ex_pid = (~ex_exhaust) & pid_arm

            for mask, name in (
                (ex_exhaust_plain, "exhaustion"),
                (ex_exhaust_sil, "exhaustion_silenced"),
                (ex_atm, "atm"),
                (ex_pid, "piddosome"),
            ):
                sel = idx[mask]
                if sel.size:
                    exit_gen[sel] = g
                    route[sel] = name
                    nmat_at_exit[sel] = nm[mask]
                    alive[sel] = False
                    if name == "exhaustion_silenced":
                        silenced_ever[sel] = True

            # --- write back --------------------------------------------------
            D[idx], Ms[idx], Mf[idx], A[idx] = d, ms, mf, a
            Nmat[idx], plk1[idx], aurA[idx] = nm, pk, au

        # censored (still cycling at G_max) -> counted as silenced-alive
        cens = alive
        if cens.any():
            route[cens] = "censored"
            exit_gen[cens] = float(p.G_max)
            nmat_at_exit[cens] = Nmat[cens]

        amp_seen |= Nmat > 1.5
        return {
            "exit_gen": exit_gen,
            "route": route,
            "nmat_at_exit": nmat_at_exit,
            "amplified_ever": amp_seen,
            "amplified_at_exit": nmat_at_exit > 1.5,
            "corrected": corrected,
            "aurA": aurA,
            "final_D": D,
            "final_Ms": Ms,
            "final_Mf": Mf,
            "final_A": A,
        }

    # -- many seeds ---------------------------------------------------------
    def validate(self, n_cells: int = 10_000, seeds=(0, 1, 2, 3, 4, 5, 6, 7, 8, 9), regime=None):
        from .calibration import summary as _summary

        med, iqr, p53s, amps = [], [], [], []
        for s in seeds:
            self.rng = np.random.default_rng(s)
            out = self.simulate(n_cells, regime=regime)
            st = _summary(out)
            finite = np.isfinite(out["exit_gen"])
            med.append(st["median_exit"])
            q75, q25 = np.percentile(out["exit_gen"][finite], [75, 25])
            iqr.append(q75 - q25)
            p53s.append((st["frac_atm"] + st["frac_piddosome"]) * 100)
            amps.append(st["frac_amp_at_exit"] * 100)
        f = lambda v: (float(np.mean(v)), float(np.std(v)))  # noqa: E731
        return {
            "hayflick_median": f(med),
            "hayflick_iqr": f(iqr),
            "p53_route_pct": f(p53s),
            "amplification_pct": f(amps),
        }
