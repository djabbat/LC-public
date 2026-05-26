# OPEN PROBLEMS — Aubrey

## Imaging platform
- Photobleaching mitigation over 72+ h still needs empirical tuning per fluorophore in BJ‑hTERT‑RITE cells.
- ARGUS-LP build timeline vs funding timeline: engineering (Tsomaia) cannot start until Phase A funded.
- Cross-calibration protocol for 2× ARGUS-LP units: same bead sample measured on both, SNR and stage accuracy comparison.

## Molecular clock / RITE
- Centrin1-Kaede photoconversion efficiency in BJ-hTERT under hypoxia (2-3% O₂) needs validation.
- Need to test Centrin-1 scaffold stability in long-term imaging (beyond 72 h).
- ~5 kb RITE cassette near lentiviral payload limit: test packaging efficiency and titer.

## Cell-line engineering
- Karyotype QC needed across ≥3 passages for each clone.
- Integration site mapping (TAIL-PCR / inverse PCR) for each clone.

## Ablation
- 405 nm collateral damage radius (need empirical characterization during commissioning).
- If 405 nm shows unacceptable collateral damage (>5% non-target cell death), fs-IR remains deferred to Phase B.

## AI agent
- Local DeepSeek-V3 inference latency on RTX 4090 vs frame interval (30-60 min → acceptable).
- spotiflow tracking robustness across 72-h runs with cell division and migration.

## Budget and risk
- Zheleznov LoS for Unit #2 still pending; contingency: single-unit-only Phase A with reduced scope.
- Tsomaia €3,900 at €20/h — verify against Georgian engineering rates.
