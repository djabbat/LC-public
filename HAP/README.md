# HAP — Hepato-Affective Primacy Theory

**Evolutionary necessary condition for affective states in Bilateria.**

HAP asserts: no bilaterian animal can possess affective states (emotions, feelings) without a functional hepatic organ — liver or its homolog (fat body + nephrocytes in insects, hepatopancreas in mollusks).

## Publication
Tqemaladze, J. (2026). The Hepato-Affective Primacy (HAP) Theory. *Longevity Horizon*, 2(4). DOI: [10.65649/d76f6c48](https://doi.org/10.65649/d76f6c48)

## Second article (in development)
In collaboration with Afaf Elfet: nonlinear dynamics model of HAP/NHAM — formalization of steroid-permissive feedback loops.

## Simulation
Python + SciPy, 6-variable ODE system:
- L — Hepatic steroid output
- B — Brain steroid sensitivity
- A — Affective circuit integrity
- I — Inflammatory state
- S — HPA / stress activity
- M — Metabolic state

bash
cd src/
python3 main.py all   # run simulation + experiments + plots


## Status
- ✅ First article published
- ✅ Simulation prototype works (HAP Predictions reproduced)
- ⏳ Waiting for Afaf's response
- ⏳ Bifurcation analysis
- 📅 Search for empirical data — after model completion