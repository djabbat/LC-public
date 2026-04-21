# LiveCellMicroscopy — TODO (Phase A)

## A1. Sourcing
- [ ] Locate Zeiss IM 35 body (eBay, LabX, Russian/German academic surplus)
- [ ] Source 100× / 1.4 NA oil objective (used OEM) — consider Taobao for Zeiss clones via brother in China
- [ ] Quote Piezo Z (PI, Thorlabs)
- [ ] Quote solid-state lasers 488 + 561 (Coherent OBIS, Cobolt, Oxxius)
- [ ] Quote environmental chamber (Okolab) + compare with DIY

## A2. Build
- [ ] Vibration-isolated optical table setup
- [ ] Mount body; clean + verify optical train
- [ ] Install Piezo Z, calibrate
- [ ] Build illumination path: fibers → combiner → AOTF → back aperture
- [ ] Install dual-camera split dichroic (shared with FluorescentCameras subproject)

## A3. Environmental + stability
- [ ] Install chamber; validate 37 °C / 5 % CO₂ / RH
- [ ] 24 h drift test (bead sample); measure XY / Z
- [ ] Laser stability test (power meter, 24 h log)

## A4. Validation run
- [ ] 24 h BJ-hTERT fluorescent imaging (no RITE) — baseline
- [ ] 72 h BJ-hTERT-RITE run (once Phase A of RITE ships)
- [ ] Demonstrate tracking ≥ 5 generations without focus loss
- [ ] Publish BOM + assembly doc on Zenodo

## Gate to Phase B
- 72 h run with < 100 nm Z-drift and clear two-channel centriole resolution
