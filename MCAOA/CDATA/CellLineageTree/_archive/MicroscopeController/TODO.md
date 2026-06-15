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
