# AICoordinator — TODO (Phase A)

## A1. Prompt engineering
- [ ] Draft `PROMPT.md` with policy sections (tree shape, age bias, focus, mitosis, phototoxicity, abort)
- [ ] Peer-review PROMPT.md with Dr. Tkemaladze + one imaging specialist
- [ ] Define JSON command schema (machine-readable)
- [ ] Safety / dry-run rules documented

## A2. Dry-run harness
- [ ] Synthetic lineage generator in `GenealogyReconstruction` produces mock frames
- [ ] Orchestrator reads mock data, emits commands → validator script checks safety
- [ ] Run 100 virtual experiments → compile failure modes → patch PROMPT.md

## A3. Integration
- [ ] Zarr reader: subscribe to new-frame events from MicroscopeController
- [ ] Command dispatcher: write to named pipe / ZeroMQ → controller
- [ ] Human override dashboard (FastAPI minimal)
- [ ] Slack / Telegram notification channel

## A4. Co-driven run
- [ ] 24 h supervised run (human approves every ablation)
- [ ] 48 h semi-autonomous (human approves every 10th ablation)
- [ ] 72 h fully autonomous with daily human review
- [ ] Publish decision log + post-mortem

## A5. Packaging
- [ ] Open-source `cytotree-aic` on GitHub (MIT)
- [ ] Zenodo DOI for PROMPT.md v1.0
- [ ] Methods note / preprint

## Gate to Phase B
- 72 h autonomous run with ≤ 3 human interventions
- Decision accuracy ≥ 90 % vs human expert on replay
