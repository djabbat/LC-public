# ImagingControl / UPGRADE.md (merged 2026-05-09)


---
## === MicroscopeController / UPGRADE.md ===

# MicroscopeController — UPGRADE

## U1. Multi-position high-content mode
- Schedule N × well plates with independent adaptive policies
- Required once throughput outpaces single-colony tracking

## U2. On-the-fly denoising
- Integrate CARE / Noise2Void / CSBDeep inline
- Reduces photon dose per frame → less phototoxicity over 72 h

## U3. Digital twin / simulator
- Offline useq-schema simulator to test acquisition policies without scope time
- Couples to `GenealogyReconstruction` synthetic data generator

## U4. Cloud sync
- Live mirror zarr store to S3 / MinIO during acquisition
- Enables remote monitoring + FCLC federated contribution

## U5. WebGUI dashboard
- FastAPI + React panel for remote monitoring
- View live frames + event log from phone during overnight runs

## U6. ROS2 bridge
- Expose hardware as ROS2 nodes → enables future robotic fluidic / well-plate handler

## U7. Hardware-in-the-loop unit tests
- CI pipeline that runs smoke tests on the actual rig nightly
- Detects drift in device drivers / OS updates

## U8. Multi-rig orchestration
- Coordinate multiple microscope workstations from one AIC controller
- Step toward FCLC-scale federated imaging experiments

---
## === FluorescentCameras / UPGRADE.md ===

# FluorescentCameras — UPGRADE

## U1. Back-thinned sCMOS
- Photometrics Prime BSI Express or Hamamatsu Orca-Fusion BT
- Boost QE to 95 % at green/red; worth ~ €15–25 k per unit when grant-funded

## U2. Third camera for far-red / BFP
- Support post-upgrade triple-tag RITE (mCherry → GFP → BFP)
- 405 nm excitation + 450 nm emission on third stream

## U3. High-speed burst mode
- At mitotic onset, switch to 500 fps burst (ROI cropped) for sub-frame anaphase tracking
- Useful for ciliary beat / spindle dynamics

## U4. Deep cooling
- -10 °C cooled head (LN₂ or two-stage TEC) → sub-e⁻ effective read noise on averages
- Overkill for centriole counting; valuable if extending to single-molecule imaging

## U5. Event cameras (Prophesee / SiliconSoftware)
- DVS / event-based sensors for high-temporal-resolution motion detection
- Speculative — could detect mitosis onset with zero latency

## U6. Lensless / in-line holography adjunct
- Cheap add-on for wide-field survey of colonies before zooming to tracked cells

## U7. On-camera GPU inference
- Smart cameras (JAI Go, Allied Vision Alvium) with onboard FPGA for real-time thresholding
- Reduces data transfer burden during 72 h runs

## U8. Full-frame integration with OME-NGFF
- Direct streaming to zarr store during acquisition (avoid disk-bottleneck between cameras and segmentation)

---
## === AICoordinator / UPGRADE.md ===

# AICoordinator — UPGRADE

## U1. Multi-rig coordination
- One AIC instance coordinates N microscopes in parallel
- Essential step toward FCLC federated lineage atlas

## U2. Self-updating policy
- AIC reads outcomes of past runs → proposes PROMPT.md diffs for human review
- Version-controlled policy evolution

## U3. Fine-tuned local LLM fallback
- Distill PROMPT.md-driven behaviour into a local 7B–14B model
- Eliminates API latency + cloud dependency for decision loop

## U4. RLHF from expert annotations
- Dr. Tqemaladze labels "good" vs "bad" decisions on replayed runs
- Use DPO to refine the decision policy

## U5. Integration with CDATA
- AIC queries CDATA's biological-age model in real time to tag cells as "old-centriole equivalents"
- Closes loop between theory (CDATA) and experimental observation (tree)

## U6. Natural-language interface for lab staff
- Staff can type "next experiment: knock out CEP152, track for 48 h, abort if division rate < 0.3/day"
- AIC generates full PROMPT + useq-schema YAML

## U7. Ethical / safety board
- Formal policy-approval workflow for risky experiments
- PROMPT.md changes require sign-off from PI + safety officer

## U8. Scientific-reasoning agent
- AIC drafts results sections + figures directly from event logs
- Auto-composes methods paragraph with correct parameters from PARAMETERS.md chain
