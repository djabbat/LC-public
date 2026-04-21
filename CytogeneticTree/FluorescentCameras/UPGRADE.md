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
