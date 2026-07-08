# DESIGN — Aubrey

## Architecture

Integrated control plane:
- **Micro-Manager / PyMMCore** drives microscope, stage, lasers, cameras
- **AICoordinator (local RTX 4090, DeepSeek-V3)** receives per-frame ROI from CellPose v3, decides ablation targets autonomously
- **GenealogyReconstruction** ingests image streams + ablation log → tree across inter-block gaps

## Build phases

| Phase | Tasks | ETA |
|---|---|---|
| P0 — engineering | ARGUS-LP design + build + calibration (Tsomaia G.) | ~5 mo |
| P1 — molecular | RITE cassette design, lentivirus packaging, clonal isolation | ~3 mo (parallel) |
| P2 — imaging | ARGUS-LP commissioning: beads → transient → stable clones, environmental validation | ~2 mo (overlaps P1) |
| P3 — intervention | NAC treatment (Phase III), forced asymmetry construct generation | ~2 mo (overlaps P2) |
| P4 — integration | End-to-end validation: 72-h run with interventions, n≥130 divisions | ~2 mo |

## Replication strategy

- Internal: 2× identical ARGUS-LP units (Unit #1 at GLA Abastumani, Unit #2 at Zheleznov site)
- Independent validation: 2 OA publications (primary + replication results)
- All raw data deposited on Zenodo, analysis code on GitHub

## Go/no-go decision points

1. **After P0 (engineering):** ≥5× SNR on TetraSpeck beads, stage repeatability ≤1 µm RMS
2. **After P1 (molecular):** tag-swap efficiency ≥70% by flow cytometry
3. **After P2 (imaging pilot):** phototoxicity within 90% of control division rate
4. **After P4 (integration):** >70% of tracked cells complete ≥8 divisions with resolvable RITE signal
