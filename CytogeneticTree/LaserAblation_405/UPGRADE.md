# LaserAblation_405 — UPGRADE

## U1. fs-IR path for organelle-scale ablation
- Add 800 nm fs laser + beam combiner
- Enables single-centriole ablation: direct causality test for centriole-age-dependent fate
- Priority Phase B upgrade

## U2. Adaptive dosing
- Close-loop: re-image within 100 ms post-pulse; increase dose if cell still alive
- Use Kalman filter on fluorescence loss as kill indicator

## U3. Multi-spot parallel ablation
- Spatial light modulator (SLM) in Fourier plane → ablate multiple targets in one shot
- Useful at mitotic onset (many simultaneous decisions)

## U4. UV confinement variants
- Test 355 nm pulsed for cleaner DNA damage with less ROS
- Test 488 nm KillerRed / miniSOG for chromophore-assisted light inactivation (CALI) — organelle-selective without hardware changes

## U5. Computational dosing model
- Monte-Carlo photon transport simulation per cell geometry
- Predicts required dose from segmentation mask thickness + depth

## U6. Remote ablation service
- API endpoint allowing the AICoordinator (or even an off-site scientist) to schedule ablation events during overnight runs

## U7. Retraction / mark mode
- Sub-lethal dose creates permanent auto-fluorescent "tattoo" on cell membrane
- Alternative lineage labelling when RITE is unavailable
