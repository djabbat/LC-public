"""
CDATA-v2 Stochastic Model
=========================
Stochastic Delay Differential Equation (SDDE) model of centriole-driven
stem cell exhaustion with dual Aurora A-mediated p53 inactivation.

State variables:
    P      - PLK1 activity at centrioles
    D      - PTM damage burden (polyglutamylation)
    M_s    - Structural maintenance (cartwheel + MT wall)
    M_f    - Functional maintenance (PCM + MT nucleation)
    N_mat  - Mother centriole count
    S      - Senescence state (0/1)

Parameters (14 total, ABC-calibrated):
    mu_P       - PLK1 decay rate
    r_0        - Baseline damage rate
    r_age      - Age-dependent damage acceleration
    lambda_age - CCP5 activity decline rate
    k_cat      - CCP5 catalytic rate
    mu_s       - Baseline structural decay
    mu_f       - Baseline functional decay
    eta_s      - Damage-structure coupling
    eta_f      - Damage-function coupling
    omega      - Hyperglutamylation-CEP295 feedback
    alpha_CEP  - CEP295 synthesis rate
    beta_0     - CEP295->Polo recruitment efficiency
    alpha_AurA_215 - Ser215 transcriptional inhibition
    alpha_AurA_315 - Ser315 degradation strength
    kappa      - Senescence sensitivity
    tau        - p53 activation delay (generations)
"""

import numpy as np
from dataclasses import dataclass, field
from typing import Optional, Tuple, List


@dataclass
class CDATAParams:
    """CDATA-v2 model parameters with ABC-calibrated defaults."""
    mu_P: float = 0.018
    r_0: float = 0.08
    r_age: float = 0.002
    lambda_age: float = 0.015
    k_cat: float = 0.08
    mu_s: float = 0.03
    mu_f: float = 0.04
    eta_s: float = 0.04
    eta_f: float = 0.06
    omega: float = 0.05
    alpha_CEP: float = 0.06
    beta_0: float = 0.042
    alpha_AurA_215: float = 3.2
    alpha_AurA_315: float = 2.1
    kappa: float = 0.15
    tau: float = 2.5

    # Noise coefficients
    sigma_P: float = 0.08
    sigma_D: float = 0.12
    sigma_s: float = 0.06
    sigma_f: float = 0.09
    sigma_N: float = 0.45


@dataclass
class CellState:
    """Single-cell state at a given generation."""
    generation: int
    P: float = 1.0
    D: float = 0.0
    M_s: float = 1.0
    M_f: float = 1.0
    N_mat: int = 1
    S: int = 0
    A: float = 0.80  # Per-division asymmetry (unknown, parameter exploration)
    # History for delay integration
    history_N: List[float] = field(default_factory=list)
    history_sigma_N: List[float] = field(default_factory=list)


class CDATAModel:
    """
    CDATA-v2 stochastic model.
    
    Usage:
        model = CDATAModel(params=CDATAParams())
        tree = model.simulate_tree(max_generations=40, n_cells=1)
        stats = model.compute_statistics(tree)
    """

    def __init__(self, params: Optional[CDATAParams] = None, seed: Optional[int] = None):
        self.params = params or CDATAParams()
        self.rng = np.random.default_rng(seed)

    def _sigma_D(self, D: float, D_crit: float = 5.0, beta_D: float = 1.5) -> float:
        return 1.0 / (1.0 + np.exp(beta_D * (D - D_crit)))

    def _sigma_S(self, M_s: float, M_crit: float = 0.05, beta_M: float = 6.0) -> float:
        return 1.0 / (1.0 + np.exp(beta_M * (M_crit - M_s)))

    def _sigma_F(self, M_f: float, M_crit: float = 0.05, beta_M: float = 6.0) -> float:
        return 1.0 / (1.0 + np.exp(beta_M * (M_crit - M_f)))

    def _sigma_N(self, N_mat: int, gamma: float = 0.3) -> float:
        return max(0.0, 1.0 - gamma * max(0, N_mat - 1))

    def _p53_net(self, N_mat: int, M_f: float) -> float:
        """Dual-pathway p53 activity."""
        if N_mat <= 1:
            return 0.0
        excess = N_mat - 1
        atm_signal = excess * self.params.kappa
        aurora_inhibition = (1.0 + self.params.alpha_AurA_315 * excess / max(M_f, 0.01)) * \
                           (1.0 + self.params.alpha_AurA_215 * excess)
        return atm_signal / aurora_inhibition

    def _step_cell(self, state: CellState) -> CellState:
        """Advance one cell by one generation (Euler-Maruyama step)."""
        p = self.params
        g = state.generation
        dt = 1.0  # One generation

        # Age-dependent parameters
        r_g = p.r_0 + p.r_age * g
        lambda_eff = np.exp(-p.lambda_age * g)

        # PLK1 decay
        dP = -p.mu_P * state.P * dt + p.sigma_P * state.P * self.rng.normal(0, np.sqrt(dt))
        new_P = max(0.0, state.P + dP)

        # Damage accumulation
        dD = (r_g - lambda_eff * p.k_cat * state.D) * dt + \
             p.sigma_D * np.sqrt(max(0, state.D)) * self.rng.normal(0, np.sqrt(dt))
        new_D = max(0.0, state.D + dD)

        # Structural maintenance
        dM_s = (-(p.mu_s + p.eta_s * state.D) * state.M_s + p.alpha_CEP - 
                p.omega * state.D * state.M_s) * dt + \
               p.sigma_s * np.sqrt(max(0, state.M_s)) * self.rng.normal(0, np.sqrt(dt))
        new_M_s = max(0.0, state.M_s + dM_s)

        # Functional maintenance
        dM_f = (-(p.mu_f + p.eta_f * state.D) * state.M_f + 
                p.beta_0 * state.M_s * state.P) * dt + \
               p.sigma_f * np.sqrt(max(0, state.M_f)) * self.rng.normal(0, np.sqrt(dt))
        new_M_f = max(0.0, state.M_f + dM_f)

        # Centrosome number (stochastic amplification when CCP5 declines)
        dN = p.sigma_N * (1.0 - lambda_eff) * state.N_mat * dt + \
             p.sigma_N * np.sqrt(state.N_mat) * self.rng.normal(0, np.sqrt(dt))
        new_N_mat = max(1, int(round(state.N_mat + dN)))

        # Store history for delay
        history_N = state.history_N + [new_N_mat]
        history_sigma_N = state.history_sigma_N + [self._sigma_N(new_N_mat)]
        if len(history_N) > int(p.tau) + 1:
            history_N = history_N[-int(p.tau)-1:]
            history_sigma_N = history_sigma_N[-int(p.tau)-1:]

        # Senescence probability (delay feedback)
        new_S = state.S
        if state.S == 0 and len(history_N) >= int(p.tau):
            cumulative_misseg = 0.0
            for s in range(max(0, len(history_N) - int(p.tau)), len(history_N)):
                excess = max(0, history_N[s] - 1)
                cumulative_misseg += excess * (1.0 - history_sigma_N[s])
            p53_val = self._p53_net(new_N_mat, new_M_f)
            p_senescence = 1.0 - np.exp(-p.kappa * cumulative_misseg * max(0, p53_val))
            if self.rng.random() < p_senescence:
                new_S = 1

        # Asymmetry (conditional on non-senescent)
        if new_S == 0:
            A_g = state.A * self._sigma_D(new_D) * self._sigma_S(new_M_s) * \
                  self._sigma_F(new_M_f) * self._sigma_N(new_N_mat)
        else:
            A_g = 0.0

        return CellState(
            generation=g + 1, P=new_P, D=new_D, M_s=new_M_s, M_f=new_M_f,
            N_mat=new_N_mat, S=new_S, A=A_g,
            history_N=history_N, history_sigma_N=history_sigma_N
        )

    def simulate_tree(self, max_generations: int = 40, n_cells: int = 1,
                      A0: float = 0.80) -> List[List[CellState]]:
        """Simulate lineage trees."""
        trees = []
        for _ in range(n_cells):
            state = CellState(generation=0, A=A0)
            trajectory = [state]
            for _ in range(max_generations):
                if state.S == 1:
                    break
                state = self._step_cell(state)
                trajectory.append(state)
            trees.append(trajectory)
        return trees

    def compute_statistics(self, trees: List[List[CellState]]) -> dict:
        """Compute summary statistics from simulated trees."""
        exhaustion_gens = []
        for traj in trees:
            for s in traj:
                if s.S == 1 or s.A < 0.55:
                    exhaustion_gens.append(s.generation)
                    break
            else:
                exhaustion_gens.append(traj[-1].generation)

        exhaustion_gens = np.array(exhaustion_gens)
        return {
            "hayflick_median": np.median(exhaustion_gens),
            "hayflick_iqr": np.percentile(exhaustion_gens, 75) - np.percentile(exhaustion_gens, 25),
            "amplification_freq": np.mean([
                np.mean([s.N_mat > 2 for s in traj if s.generation > 30])
                for traj in trees
            ]) if trees else 0.0,
        }

    def to_dict(self) -> dict:
        """Export parameters to dict."""
        return {k: v for k, v in self.params.__dict__.items()}

    @classmethod
    def from_dict(cls, d: dict, seed: Optional[int] = None) -> "CDATAModel":
        """Create model from parameter dict."""
        params = CDATAParams(**{k: v for k, v in d.items() 
                                if k in CDATAParams.__dataclass_fields__})
        return cls(params=params, seed=seed)
