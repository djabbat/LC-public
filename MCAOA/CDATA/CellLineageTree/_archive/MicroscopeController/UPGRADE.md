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
