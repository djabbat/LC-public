# ImagingControl / TODO.md (merged 2026-05-09)


---
## === MicroscopeController / TODO.md ===

# MicroscopeController — TODO (Phase A)

## A1. Environment setup
- [ ] Install Ubuntu 24.04 on control workstation
- [ ] Install µManager 2.0
- [ ] Python 3.11 venv + pymmcore-plus + useq-schema + napari
- [ ] UPS + scheduled backups

## A2. Device config
- [ ] Write .cfg for all hardware (Piezo, XY, AOTF, lasers, cameras, galvo, chamber)
- [ ] Verify each device independently
- [ ] Integration test: 30-min dummy acquisition

## A3. Primitives API
- [ ] `cytotree_control.Scope` class wrapping PyMMCore-Plus
- [ ] `acquire_zstack(channels, n_planes, step_nm)`
- [ ] `ablate_mask(mask, laser='405', dose_mW, dwell_ms)`
- [ ] `adaptive_refocus(method='cellpose' | 'bead')`
- [ ] `run_useq(schema_path)` dispatcher
- [ ] Unit tests with pymmcore demo device

## A4. Event logging + recovery
- [ ] JSON-Lines event log per experiment
- [ ] Retry + exponential backoff on device errors
- [ ] State checkpoints every 10 min → resume on restart
- [ ] Slack / Telegram webhook notifications for failures

## A5. 72 h dry-run
- [ ] 72 h cycling without samples (stability + recovery test)
- [ ] 72 h BJ-hTERT test run
- [ ] Debrief + publish

## Gate to Phase B
- 72 h uninterrupted acquisition with event log intact
- API stable enough for `AICoordinator` to consume

---
## === FluorescentCameras / TODO.md ===

# FluorescentCameras — TODO (Phase A)

## A1. Procurement
- [ ] Confirm Hikrobot model availability via brother in China (Taobao / 1688)
- [ ] Order 2 × matched units + 2 × USB 3.0 active cables
- [ ] Back-up plan: 1 × FLIR BFS-U3-51S5M from European distributor
- [ ] Receive + inventory

## A2. Characterization
- [ ] Measure QE curve (integrating sphere + calibrated lamp + monochromator) — 400–700 nm
- [ ] Measure read noise (bias frames, dark condition)
- [ ] Measure dark current at 3 sensor temperatures (25, 15, 10 °C)
- [ ] Measure photon transfer curve → system gain + full well
- [ ] Linearity over full dynamic range

## A3. Integration
- [ ] C-mount adapters to microscope ports
- [ ] Dichroic split path: 561 reflect → camera 1, 488 transmit → camera 2
- [ ] Hardware trigger sync; verify < 1 µs jitter with LED pulser
- [ ] Micro-Manager driver: verify both Genicam-compatible
- [ ] PyMMCore-Plus acquisition test

## A4. Validation
- [ ] Flat-field correction (uniform illuminator)
- [ ] Dark-field calibration
- [ ] Pixel-level registration between two cameras (affine + distortion)
- [ ] Publish characterization notebook + dataset (Zenodo DOI)

## Gate to Phase B
- QE ≥ 70 %, read noise ≤ 2 e⁻
- Dual-camera sync < 1 µs
- Centriole SNR ≥ 4 in BJ-hTERT-RITE test run

---
## === AICoordinator / TODO.md ===

# AICoordinator — TODO (Phase A)

## A1. Prompt engineering
- [ ] Draft `PROMPT.md` with policy sections (tree shape, age bias, focus, mitosis, phototoxicity, abort)
- [ ] Peer-review PROMPT.md with Dr. Tqemaladze + one imaging specialist
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
