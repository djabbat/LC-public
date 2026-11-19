"""CEDAR-v2.1 reference implementation.

Reconstruction of the CEDAR-v2 model described in
Tqemaladze J. (2026) "A Stochastic Model of Centriole-Driven Stem Cell Exhaustion",
Longevity Horizon 2(5), doi:10.65649/wjtcf387, extended with the v2.1 modules
(PIDDosome p53 input, Aurora A transactivation gate, autophagy correction edge).

IMPORTANT: this is an independent reference implementation. The original
`cdata_sim` sources were lost (editable install pointed to a deleted directory;
the CEDAR-sim repository does not exist). Equations follow the published
architecture and parameter table; interpretive choices are documented in
MODEL_SPEC_v2.1.md.
"""

from .model import CEDARModel, ModelParams, REGIMES
from .calibration import calibrate, summary
from .sensitivity import sobol_first_total

__all__ = [
    "CEDARModel",
    "ModelParams",
    "REGIMES",
    "calibrate",
    "summary",
    "sobol_first_total",
]
__version__ = "2.1.0-ref"
