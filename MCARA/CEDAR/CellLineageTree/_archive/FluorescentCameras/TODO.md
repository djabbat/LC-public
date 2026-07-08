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
