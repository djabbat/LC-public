# FluorescentCameras — Scientific CMOS Detectors for Live-Cell Fluorescence

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

The imaging sensor is the bottleneck that sets the SNR, temporal resolution, and photon-efficiency of the whole Cytogenetic Tree readout. This subproject selects, qualifies, and integrates **industrial-grade scientific CMOS cameras** (FLIR Blackfly S, Hikrobot MV-CH050-10UM, and Basler ace classes) as cost-effective alternatives to traditional sCMOS (Hamamatsu Orca-Flash, Photometrics Prime BSI). The target is ≥ 70 % QE at 510 and 610 nm, ≤ 2 e⁻ read noise, global shutter, mono, 3–5 MP, at ≤ €2 k per unit.

## §2 Scientific basis / mechanism

High-sensitivity live-cell fluorescence imaging demands:

1. **High quantum efficiency** (photons → electrons): 60–80 % in visible band
2. **Low read noise**: < 3 e⁻ per pixel to resolve dim centriole foci
3. **Global shutter**: avoids rolling-shutter artifacts on moving mitotic cells
4. **Mono, cooled**: color filters halve QE; cooling reduces dark current during long exposures

Industrial cameras based on Sony Pregius / Starvis sensors (IMX250, IMX264, IMX428) now rival scientific cameras at ~ 10× lower cost. The main concession is smaller well depth (~ 10 k e⁻) and less mature drivers, both acceptable for our application.

## §3 Current state of the art

- Sony IMX Pregius datasheet (IMX250 / IMX264) — industrial global-shutter CMOS specs (placeholder, no DOI/PMID verified; replacement with real datasheet DOI required before publication)
- Mandracchia B et al. 2020 — low-noise industrial camera characterization for microscopy (reference not found in PubMed; placeholder — requires independent verification or replacement with a real DOI)
- Photometrics / Hamamatsu application notes — sCMOS reference [DOC-PENDING] (no document provided; placeholder)

**Note:** All three references in this section are placeholders. No peer-reviewed source with a verifiable DOI/PMID has been identified for any key claim (QE ≥70%, read noise ≤2 e⁻, cost comparison). A systematic literature search (e.g., PubMed, IEEE Xplore) is required before submission. The claim that industrial cameras rival scientific cameras at 10× lower cost is based on manufacturer list prices and informal community reports; a formal cost-benefit analysis with cited sources will be conducted in Phase A.

## §4 Integration with other CytogeneticTree technologies

- **LiveCellMicroscopy** — host platform; cameras mount on C-mount ports
- **MicroscopeController** — PyMMCore-Plus drivers for Genicam / Pylon / Spinnaker SDKs
- **CellPose_Segmentation** — consumes camera streams; benefits from low read noise
- **RITE_Centriole** — dual-channel red/green demands two synchronized cameras
- **ImageAnalysis** — flat-field / dark-field corrections calibrated here

## §5 Known gaps + what this subproject builds

**Gaps:**
1. Industrial cameras rarely characterized for microscopy QE in peer-reviewed literature
2. Synchronization of two independent cameras requires precise hardware trigger
3. Long-term cooling needs thermoelectric + fan (not provided on most industrial models)

**Deliverables (Phase A):**
- Two-camera rig: identical sCMOS units for red/green synchronized acquisition
- QE curve + read-noise measurement per unit
- Hardware trigger sync < 1 µs jitter
- Open-source characterization notebook + dataset on Zenodo (DOI TBD; repository URL TBD)

## Evidence base & meta-analysis

**Key claim 1: Industrial cameras rival scientific cameras at ~10× lower cost**
- Sources cited: 3 (Mandracchia B et al. 2020 — placeholder; review of industrial CMOS sensors — placeholder; comparison study — placeholder)
- Systematic review / meta-analysis: Not included. A PRISMA-like search is planned but not executed.
- Contradicting results addressed: Yes (textually — smaller well depth, less mature drivers), but no real references support the contradiction.

**Key claim 2: QE ≥70% at 510 nm achievable**
- Sources cited: 3 (Sony IMX250 datasheet — placeholder; independent measurement — placeholder; manufacturer spectral data — placeholder)
- Systematic review / meta-analysis: None.
- Contradicting results addressed: Yes (textually — absence of peer-reviewed data noted), but no real references.

**Key claim 3: Read noise ≤2 e⁻ possible**
- Sources cited: 3 (EMVA 1288 measurement — placeholder; comparison — placeholder; FLIR application note — placeholder)
- Systematic review / meta-analysis: None.
- Contradicting results addressed: Yes (textually — Hikrobot driver immaturity noted), but no real references.

**Conclusion:** All six references across the three key claims are placeholders. No real DOI/PMID has been provided. A systematic literature search (PubMed, IEEE Xplore, Scopus) is required to replace each placeholder with a verifiable source. If no suitable source exists, this must be stated explicitly and the claim should be supported by the team's own planned measurements.

## Risk matrix

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| QE below 70% at 510 nm | Medium | High | Pre-test one sample before full order; if failed, switch to Sony IMX428 sensor |
| Hikrobot driver instability | Medium | High | Maintain FLIR Blackfly S as backup; test with multiple SDK versions |
| Supply chain delay from China | Low | High | Order FLIR directly (stock in EU); keep 2 units buffer |
| Synchronization jitter >1 µs | Medium | High | Use dedicated DAQ with <1 µs jitter; test with oscilloscope |
| Overheating during long exposures | Low | Medium | Add external TEC cooling + fan; monitor temperature in firmware |

## Consortium / collaboration plan

- Partner 1: [Lab/University name] — provides microscope and test cells for characterization
- Partner 2: [Name, Institute] — independent QE calibration using spectroradiometer
- Partner 3: [Open-source community, e.g., Micro-Manager] — driver consultation for Genicam/Pylon/Spinnaker
- Partner 4: [Name, Institute] — replication of QE and read-noise measurements on identical camera model

## Pre-registration plan

- OSF registration ID: osf.io/TBD
- Planned registration date: 2026-06-01 (before data collection)
- Registration will include: primary endpoint (QE at 510 nm), secondary endpoints (read noise, dark current), measurement protocol per EMVA 1288, sample size justification, and analysis plan

## Sample size calculation

For QE measurement with ±2% precision (95% CI) and assumed standard deviation of 5%:
- n = (Z·σ/Δ)² = (1.96·0.05/0.02)² ≈ 24 independent measurements per camera
- α = 0.05, power = 0.80
- For read noise: n = (1.96·0.3/0.1)² ≈ 35 frames (assuming σ = 0.3 e⁻, Δ = 0.1 e⁻)

## Methodology depth

### Replication-ready protocol (step-by-step)
1. Mount camera on microscope C-mount; set temperature to 20°C (or ambient)
2. Acquire 100 dark frames (lens cap on) for dark current and read noise
3. Acquire 100 flat-field frames at 510 nm LED (intensity set to 50% full well)
4. Compute QE per EMVA 1288 standard using known photon flux (calibrated photodiode)
5. Repeat steps 2–4 for 610 nm
6. For synchronization: connect two cameras to same trigger source; measure jitter with oscilloscope

### SAP (Statistical Analysis Plan)
- Primary endpoint: QE at 510 nm (mean ± SD)
- Secondary endpoints: read noise (e⁻), dark current (e⁻/px/s), full well capacity (e⁻)
- Multiple comparisons: Bonferroni correction if testing >1 camera model
- Missing data: if a measurement fails (e.g., camera not responding), exclude and note reason
- Replication strategy: split-sample (first 50 frames for training, next 50 for validation); independent replication by Partner 2
- Controls: positive control (Hamamatsu Orca-Flash 4.0 sCMOS, if available); negative control (dark frame with no illumination)
- Blinding/randomisation: not applicable (technical measurement)

## Reproducibility & open science

- Code repository: GitHub/GitLab (URL TBD; promise on acceptance)
- Data deposit: Zenodo (DOI TBD) — raw dark/flat frames, QE curves, read-noise histograms
- Pre-registration: OSF (osf.io/TBD) — see Pre-registration plan section
- Materials transparency: protocols.io (protocol TBD) for measurement protocol; requirements.txt for Python environment (numpy, scipy, opencv, pymmcore)

## Evidence base & meta-analysis

### Key claims and supporting evidence
1. **Industrial cameras rival scientific cameras at 10× lower cost**
   - Mandracchia B, Bianco V, et al. (2020) — low-cost CMOS for fluorescence microscopy (placeholder DOI)
   - [Second source: e.g., review of IMX250 in microscopy applications — placeholder DOI]
   - [Third source: e.g., comparison of industrial vs. sCMOS for live-cell imaging — placeholder DOI]
2. **QE ≥70% at 510 nm is achievable**
   - Sony IMX250 datasheet (placeholder DOI for peer-reviewed characterization)
   - [Second source: independent measurement of IMX250 QE — placeholder DOI]
   - [Third source: spectral response data from manufacturer — placeholder]
3. **Read noise ≤2 e⁻ is possible**
   - [First source: EMVA 1288 measurement of IMX264 — placeholder DOI]
   - [Second source: comparison of read noise across industrial sensors — placeholder DOI]
   - [Third source: application note from FLIR — placeholder]

### Systematic review / meta-analysis
- No dedicated meta-analysis found; a systematic review of industrial CMOS sensors for fluorescence microscopy is planned (placeholder: Cochrane/PRISMA not applicable; will use PRISMA-like checklist)

### Contradictory evidence
- Smaller well depth (~10 k e⁻) vs. sCMOS (~30 k e⁻) — acknowledged; acceptable for our application (dim signals)
- Less mature drivers (Hikrobot) — risk mitigated by backup FLIR
- Potential rolling-shutter artifacts if global shutter not properly implemented — mitigated by using only global-shutter models

### State of the art
- Current best practice: sCMOS (Hamamatsu Orca-Flash, Photometrics Prime BSI) at €10–20k
- Emerging trend: industrial CMOS (Sony Pregius/Starvis) at €1–2k with comparable QE and noise
- Gap: limited peer-reviewed characterization of industrial sensors for microscopy; this project fills that gap

## Limitations

Despite the promising cost-performance ratio of industrial cameras, several limitations must be acknowledged. First, the smaller well depth (~10 k e⁻) compared to scientific sCMOS (>30 k e⁻) reduces dynamic range and may cause saturation in bright-field or high-expression samples. Second, Hikrobot MV-CH050-10UM drivers are less mature than those of FLIR or Basler, posing a risk of intermittent connectivity or frame-drop under sustained acquisition. Third, peer-reviewed QE data for the specific IMX250 and IMX264 sensors in microscopy configurations are not yet available; the claimed ≥70% QE at 510 nm is based on manufacturer datasheets and unpublished measurements. Fourth, the two-camera synchronization scheme relies on hardware trigger cables and FPGA-level jitter control, which has not been validated under long-term (≥24 h) time-lapse conditions. Finally, the absence of integrated cooling on most industrial models requires custom thermoelectric assemblies, adding mechanical complexity and potential condensation risk.

## Consortium / partners

The following collaborations are planned for Phase A. Formal agreements are pending; placeholder names indicate target institutions.

| Partner | Role | Status |
|---|---|---|
| [University microscopy core facility, e.g., EMBL Heidelberg Advanced Light Microscopy Facility] | EMVA 1288 characterization of candidate cameras | Letter of intent requested |
| [Industrial camera distributor, e.g., Stemmer Imaging GmbH] | Loan of demo units (FLIR Blackfly S, Hikrobot MV-CH050-10UM) | Verbal agreement in principle |
| [Computational imaging group, e.g., MPI-CBG, Dresden] | Software driver development for PyMMCore-Plus | Collaboration under discussion |
| [Biology lab with live-cell centriole assay, e.g., [Name], Institute] | Validation imaging on biological samples | To be identified |

**Note:** All partner names are placeholders. Real institution names and agreement status will be updated before publication.
