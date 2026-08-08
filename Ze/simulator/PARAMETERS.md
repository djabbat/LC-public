# PARAMETERS — Ze Simulator

**Version:** 1.0

## Simulation Parameters
| Parameter | Value | Description |
|-----------|-------|-------------|
| **N_qubits** | 2 (CHSH) | Number of qubits in the simulation |
| **N_trials** | 10^6 | Number of trials |
| **v_star** | from bootstrap_vstar_results.json | Critical velocity v* |
| **theta_range** | [0, π] | Measurement angle range |

## Physical Constants
Transferred from LC/Ze/PARAMETERS.md.

## Metrics
- CHSH S-value (target: ≤ 2√2 ≈ 2.828)
- Born probabilities (target: match cos²(θ/2))
- Z₂ invariants (target: conservation)
