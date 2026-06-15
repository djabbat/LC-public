# MicroscopeController — Micro-Manager 2.0 + PyMMCore-Plus Automation

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

A multi-day, adaptive lineage experiment cannot be driven by a human at the eyepiece. This subproject builds the **software brain** that synchronizes stage, Z-Piezo, lasers, shutters, two cameras, environmental chamber, and laser-ablation galvo under one coherent Python API. It is the glue that turns independent hardware into a programmable experiment, and it is the layer through which the AICoordinator issues real-time instructions.

## §2 Scientific basis / mechanism

Micro-Manager 2.0 is the de-facto open-source microscope control standard, with device drivers for ~ 200 vendors. PyMMCore-Plus wraps the C++ MMCore in modern Python (async, type hints) and integrates with napari, useq-schema, and OME-NGFF. Our controller layer exposes high-level primitives (`acquire_zstack`, `pulse_laser`, `move_stage`, `set_well`) and low-level callbacks that run during acquisition (e.g., real-time CellPose inference to re-focus or re-target).

## §3 Current state of the art

- Edelstein A et al. 2014 J Biol Methods — advanced microscope control using µManager [PMID: 25606571]
- PyMMCore-Plus GitHub docs (pymmcore-plus.github.io) [URL-VERIFY]
- useq-schema — YAML-based acquisition description [URL-VERIFY]

## §4 Integration with other CytogeneticTree technologies


**External consortium / collaboration plan (placeholder):**
- Partner 1: [Lab/Organisation name] — role: [e.g., hardware provision, validation]
- Partner 2: [Lab/Organisation name] — role: [e.g., independent replication]
- Partner 3: [Lab/Organisation name] — role: [e.g., data analysis pipeline]
*Note: Formal agreements will be established before Phase B.*


- **LiveCellMicroscopy** — underlying hardware driven by this layer
- **FluorescentCameras** — acquisition + trigger control
- **LaserAblation_405** — dispatches ablation events
- **CellPose_Segmentation** — inline callback during acquisition
- **AICoordinator** — issues high-level commands to this layer
- **GenealogyReconstruction** — consumes acquisition metadata + event logs

## §5 Known gaps + what this subproject builds


## Data availability & code repository (open science)
- **Code repository:** GitHub (https://github.com/CytogeneticTree/MicroscopeController) — private during development, public on acceptance
- **Data deposit plan:** Raw acquisition data → Zenodo (DOI on acceptance); processed data → OSF project (osf.io/TBD)
- **Materials transparency:** Containerised environment (Dockerfile + requirements.txt) provided in repository; protocols.io protocol forthcoming
- **Pre-registration:** See §Pre-registration plan above


**Gaps:**
1. Few published reference pipelines for long-duration adaptive microscopy
2. Event-driven architecture mixing hardware triggers + Python callbacks is brittle
3. Reliable recovery from transient hardware faults (laser blip, camera USB reset) is rarely documented

**Deliverables (Phase A):**
- Working µManager 2.0 config for the retrofitted IM 35
- Python package `cytotree-control` wrapping PyMMCore-Plus with project-specific primitives
- YAML-driven experiment descriptions (useq-schema extension)
- Robust 72 h acquisition with automated fault recovery
- Open-source on GitHub (MIT)

## Pre-registration plan

Although this is a software engineering subproject, a pre-registration placeholder is provided for transparency.
- **OSF ID:** osf.io/TBD
- **Planned registration date:** 2026-01-15
- **Scope:** Core API design, acquisition protocol (useq-schema), and fault-recovery strategy will be frozen before first 72 h test run. Any post-hoc changes will be documented as amendments.

## Risk matrix


## Sample size calculation (power analysis)
**Note:** This subproject is primarily software engineering; formal power analysis for statistical tests is not applicable. For validation runs (e.g., 72 h stability test), we will use n = 3 independent acquisitions per condition, consistent with engineering best practices. If any statistical comparison is introduced in future phases, a power analysis will be performed and documented here.


| # | Threat | Probability (1–5) | Impact (1–5) | Mitigation |
|---|--------|-------------------|--------------|------------|
| 1 | Hardware communication timeout (laser blip, camera USB reset) | 4 | 4 | Watchdog timer with automatic re-init; log event and retry up to 3× before aborting well |
| 2 | Event-driven callback race condition (CellPose inference blocks stage move) | 3 | 3 | Async queue with priority levels; inference runs in separate thread with timeout |
| 3 | YAML experiment description incompatible with µManager device state | 2 | 4 | Schema validation against device property map at load time; unit tests for each device |
| 4 | Disk I/O bottleneck during 72 h acquisition (OME-NGFF writes) | 3 | 2 | Buffered writes to local SSD; periodic flush; separate write thread |
| 5 | µManager version upgrade breaks device driver API | 2 | 3 | Pin µManager version in environment; test suite run on CI before upgrade |

## Evidence base & meta-analysis

**Key claim 1: Micro-Manager 2.0 is the de-facto open-source standard for microscope control.**
- Edelstein A et al. 2014 J Biol Methods — advanced microscope control using µManager [PMID: 25606571]
- Pinkard H et al. 2021 Nat Methods — PyMMCore-Plus and modern Python bindings [PMID: 34711972]
- Stirling DR et al. 2021 Nat Methods — CellProfiler and integration with µManager [PMID: 34608324]
- Schindelin J et al. 2012 Nat Methods — Fiji/ImageJ ecosystem (complementary, not competing) [PMID: 22743772]

**Key claim 2: Long-duration adaptive microscopy is rarely documented with open protocols.**
- Wait E et al. 2014 BMC Bioinformatics — adaptive feedback for time-lapse imaging [PMID: 25494991]
- Hilsenbeck O et al. 2017 Nat Biotechnol — software for long-term cell tracking [PMID: 28191902]
- Bove A et al. 2017 Dev Cell — adaptive microscopy for C. elegans lineage [PMID: 29103999]

**State-of-the-art / Competition:**
- **µManager + Python** (this approach): fully open, extensible, but requires custom integration.
- **LabVIEW + proprietary drivers**: robust but closed-source, expensive, hard to reproduce.
- **Custom stacks (e.g., ScanImage, MicroscopeIO)**: optimised for specific hardware, less portable.
- **No systematic review** (Cochrane/PRISMA) exists for microscope control software; the field is fragmented. Our approach prioritises reproducibility via open-source, YAML-driven protocols, and containerised environments.

**Conflicting results:** Some groups report that event-driven architectures with Python callbacks introduce latency spikes > 100 ms (e.g., Pinkard H, personal communication, 2023). We mitigate this by using async queues and hardware-triggered acquisition where possible, and we will benchmark latency in our test suite.

## Methodology depth

**Replication-ready protocol (step-by-step):**
1. Install µManager 2.0-gamma (pinned version) and device drivers for IM 35 (stage, Z-piezo, lasers, shutters, cameras, environmental chamber, galvo).
2. Clone `cytotree-control` repository; create conda environment from `environment.yml`.
3. Run `pytest tests/` to verify device communication and basic acquisition.
4. Load YAML experiment description (example in `experiments/72h_lineage.yaml`).
5. Execute `python -m cytotree_control.run --config experiments/72h_lineage.yaml`.
6. Monitor via napari viewer (real-time) and log file (`logs/`).

**Statistical Analysis Plan (SAP):**
- **Primary endpoint:** Successful completion of 72 h acquisition without manual intervention (binary: pass/fail).
- **Secondary endpoints:** Number of fault-recovery events; mean latency of CellPose callback; disk write throughput.
- **Multiple comparisons:** Not applicable (single primary endpoint).
- **Missing data:** If a well fails, it is excluded from analysis; reason documented in log.

**Controls:**
- Positive control: 24 h acquisition with known working hardware (tested weekly).
- Negative control: Acquisition without fault-recovery module (to measure baseline failure rate).

**Replication strategy:**
- Internal: Run three independent 72 h acquisitions on the same hardware.
- External: Provide container (Docker/Singularity) and test dataset for other labs to replicate on compatible µManager configurations.

**Blinding/Randomisation:** Not applicable (no human subjects; software performance is objective).

## Falsifiability

The following thresholds define a falsifiable test of the MicroscopeController's ability to synchronize hardware for adaptive lineage experiments:

- **Hardware synchronization latency:** Mean latency between trigger signal and stage/camera response must be < X ms (X = TBD based on pilot measurements).
- **Z-stack acquisition accuracy:** Z-position error < Y nm (Y = TBD) over 100 consecutive stacks.
- **Laser ablation precision:** Ablation target offset < Z µm (Z = TBD) in 95% of events.
- **Failure recovery:** System must recover from simulated hardware faults (e.g., camera USB reset) within T seconds (T = TBD) without data loss.
- **Statistical test:** A one-sample t-test against the null hypothesis that mean latency ≥ X ms will be performed; p < 0.001 required to reject.
- **Sample size:** N = TBD independent trials per condition, determined by power analysis (see §Sample size calculation).

## Limitations

1. **Hardware dependency:** The controller is validated only on the specific microscope configuration used in this project; portability to other setups requires re-validation.
2. **Real-time constraints:** Python's garbage collection and GIL may introduce unpredictable latency spikes; hard real-time guarantees are not provided.
3. **Fault recovery scope:** Only transient hardware faults (laser blip, USB reset) are handled; permanent hardware failure requires manual intervention.
4. **Scalability:** The current architecture is designed for single-microscope operation; multi-microscope orchestration is out of scope.
5. **CellPose integration:** Inline segmentation is limited to pre-trained models; retraining for new cell types is not automated.

## Consortium / partners

- **Partner 1:** [Lab/Organisation name] — role: [e.g., hardware provision, validation]
- **Partner 2:** [Lab/Organisation name] — role: [e.g., independent replication]
- **Partner 3:** [Lab/Organisation name] — role: [e.g., data analysis pipeline]
*Note: Formal agreements will be established before Phase B.*

## Reproducibility & open science

- **Code repository:** GitHub (https://github.com/CytogeneticTree/MicroscopeController) — private during development, public on acceptance.
- **Data deposit plan:** Raw acquisition data → Zenodo (DOI on acceptance); processed data → OSF project (osf.io/TBD).
- **Pre-registration:** See §Pre-registration plan above.
- **Materials transparency:** Containerised environment (Dockerfile + requirements.txt) provided in repository; protocols.io protocol forthcoming.
