# LaserAblation_405 — TODO (Phase A)

## A1. Hardware sourcing
- [ ] Quote 405 nm 100 mW diode (Coherent, Oxxius, Cobolt)
- [ ] Quote galvo head (Thorlabs GVS012 or alternative)
- [ ] Quote DAQ (NI USB-6009 or LabJack T7)
- [ ] Confirm fit with existing Zeiss IM 35 retrofit optical train
- [ ] Purchase + receive

## A2. Integration
- [ ] Mount galvo head + align to back aperture of 100× objective
- [ ] Install dichroic combiner (405 nm reflect / imaging path transmit)
- [ ] Enclosure interlock wiring
- [ ] Safety SOP sign-off (institutional)

## A3. Calibration
- [ ] Power meter calibration at sample plane (x, y, z)
- [ ] Beam-profiler check at focus (Rayleigh range)
- [ ] Dose-response curve in fixed fluorescent beads
- [ ] Dose-response in live BJ-hTERT (viability at 24 h)

## A4. Automation
- [ ] Python API: `ablate(mask: ndarray, dose_mW: float, dwell_ms: float)`
- [ ] Integration test: CellPose mask → ablation → post-image check
- [ ] End-to-end demo: track one cell, ablate one daughter at division, continue tracking sister
- [ ] Log format: OME-NGFF ROI table with ablation events

## Gate to Phase B
- ≥ 90 % single-cell kill rate within 5 s
- ≤ 5 % off-target (neighbor) damage
- Full Python automation demonstrated
