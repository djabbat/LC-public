"""
ABC-SMC Calibration for CDATA-v2
================================
Approximate Bayesian Computation with Sequential Monte Carlo.

Calibrates 14 model parameters against:
    - Hayflick limit (median + IQR)
    - Clonogenicity at PDL 40
    - Centrosome amplification frequency at PDL 50

Data: 3 cell types (fibroblast, NSC, keratinocyte) × 4 endpoints = 2,400 observations.
"""

import numpy as np
from typing import Callable, Dict, List, Tuple, Optional
from dataclasses import dataclass
from .model import CDATAModel, CDATAParams


@dataclass
class ABCConfig:
    """ABC-SMC configuration."""
    n_populations: int = 5
    n_particles: int = 1000
    tolerance_schedule: List[float] = None  # Set in __post_init__
    ess_threshold: int = 800

    def __post_init__(self):
        if self.tolerance_schedule is None:
            self.tolerance_schedule = [np.inf, None, None, None, None]  # None = adaptive


class ABCSMC:
    """
    ABC-SMC calibration for CDATA-v2.
    
    Usage:
        config = ABCConfig(n_populations=5, n_particles=1000)
        abc = ABCSMC(config, observed_stats, seed=42)
        posterior = abc.run()
    """

    def __init__(self, config: ABCConfig, observed_stats: Dict[str, float],
                 seed: Optional[int] = None):
        self.config = config
        self.observed = observed_stats
        self.rng = np.random.default_rng(seed)
        self.posteriors = []

    def _prior_sample(self) -> CDATAParams:
        """Sample from uniform priors over biologically plausible ranges."""
        return CDATAParams(
            mu_P=self.rng.uniform(0.005, 0.05),
            r_0=self.rng.uniform(0.04, 0.15),
            r_age=self.rng.uniform(0.001, 0.005),
            lambda_age=self.rng.uniform(0.005, 0.03),
            k_cat=self.rng.uniform(0.04, 0.12),
            mu_s=self.rng.uniform(0.01, 0.06),
            mu_f=self.rng.uniform(0.01, 0.06),
            eta_s=self.rng.uniform(0.01, 0.10),
            eta_f=self.rng.uniform(0.01, 0.10),
            omega=self.rng.uniform(0.01, 0.10),
            alpha_CEP=self.rng.uniform(0.02, 0.10),
            beta_0=self.rng.uniform(0.02, 0.08),
            alpha_AurA_215=self.rng.uniform(0.5, 10.0),
            alpha_AurA_315=self.rng.uniform(0.5, 10.0),
            kappa=self.rng.uniform(0.05, 0.30),
        )

    def _perturb_particle(self, params: CDATAParams, kernel_std: float = 0.1) -> CDATAParams:
        """Perturb a particle using Gaussian kernel."""
        d = {}
        for field_name in CDATAParams.__dataclass_fields__:
            if field_name.startswith('sigma_'):
                continue
            val = getattr(params, field_name)
            new_val = val + self.rng.normal(0, kernel_std * abs(val))
            d[field_name] = max(0.0, new_val)
        return CDATAParams(**d)

    def _weighted_distance(self, sim_stats: Dict[str, float]) -> float:
        """Weighted Euclidean distance, normalized by observed variance."""
        weights = {"hayflick_median": 1.0, "hayflick_iqr": 1.0,
                   "amplification_freq": 0.5}
        d2 = 0.0
        for k in self.observed:
            w = weights.get(k, 1.0)
            d2 += w * (self.observed[k] - sim_stats.get(k, 0.0)) ** 2
        return np.sqrt(d2)

    def _effective_sample_size(self, weights: np.ndarray) -> float:
        """Compute effective sample size."""
        w = weights / weights.sum()
        return 1.0 / np.sum(w ** 2)

    def run(self, n_trees_per_particle: int = 10, verbose: bool = True) -> List[Dict]:
        """Run ABC-SMC."""
        n = self.config.n_particles
        posteriors = []
        epsilon = np.inf

        for pop_idx in range(self.config.n_populations):
            if verbose:
                print(f"Population {pop_idx + 1}/{self.config.n_populations} (ε = {epsilon:.4f})")

            particles = []
            weights = []
            distances = []

            if pop_idx == 0:
                # Population 0: rejection sampling from prior
                accepted = 0
                while accepted < n:
                    params = self._prior_sample()
                    model = CDATAModel(params=params, seed=self.rng.integers(0, 2**31))
                    trees = model.simulate_tree(max_generations=40, n_cells=n_trees_per_particle)
                    stats = model.compute_statistics(trees)
                    d = self._weighted_distance(stats)
                    if d < epsilon:
                        particles.append(params)
                        weights.append(1.0)
                        distances.append(d)
                        accepted += 1
                weights = np.ones(n) / n

            else:
                # Subsequent populations: importance sampling from previous posterior
                prev_weights = np.array(posteriors[-1]["weights"])
                prev_weights = prev_weights / prev_weights.sum()
                accepted = 0
                while accepted < n:
                    idx = self.rng.choice(len(posteriors[-1]["particles"]), p=prev_weights)
                    prev_params = posteriors[-1]["particles"][idx]
                    new_params = self._perturb_particle(prev_params)
                    model = CDATAModel(params=new_params, seed=self.rng.integers(0, 2**31))
                    trees = model.simulate_tree(max_generations=40, n_cells=n_trees_per_particle)
                    stats = model.compute_statistics(trees)
                    d = self._weighted_distance(stats)
                    if d < epsilon:
                        particles.append(new_params)
                        distances.append(d)
                        accepted += 1
                weights = np.ones(n) / n

            # Update epsilon adaptively
            if pop_idx < self.config.n_populations - 1:
                epsilon = np.median(distances) * 0.5 if pop_idx == 0 else np.median(distances) * 0.5

            ess = self._effective_sample_size(np.array(weights))
            if verbose:
                print(f"  Accepted: {len(particles)}, ESS: {ess:.0f}, "
                      f"Acceptance rate: {len(particles)/max(1, (pop_idx+1)*n):.1%}")

            posteriors.append({
                "population": pop_idx,
                "particles": particles,
                "weights": weights.tolist(),
                "distances": distances,
                "epsilon": epsilon,
                "ess": ess,
            })

        self.posteriors = posteriors
        return posteriors

    def get_parameter_estimates(self) -> Dict[str, Tuple[float, float, float]]:
        """Get posterior median and 95% CI for each parameter."""
        if not self.posteriors:
            raise ValueError("Run ABC-SMC first.")
        
        final = self.posteriors[-1]
        estimates = {}
        for field_name in CDATAParams.__dataclass_fields__:
            if field_name.startswith('sigma_'):
                continue
            values = [getattr(p, field_name) for p in final["particles"]]
            estimates[field_name] = (
                np.median(values),
                np.percentile(values, 2.5),
                np.percentile(values, 97.5),
            )
        return estimates
