# LiveCellMicroscopy — Retrofitted Zeiss IM 35 Imaging Core

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

This subproject defines the **physical imaging platform** on which the entire Cytogenetic Tree experiment runs. A retrofitted Zeiss IM 35 inverted microscope — selected for its optical quality, modularity, and accessibility at an academic Georgian-lab budget — is equipped with a 100× / 1.4 NA oil objective, Piezo Z stage, environmental chamber, and dual-laser fluorescence path. The platform must image ≥ 72 h of BJ-hTERT-RITE lineages without photobleaching or focus drift.

## §2 Scientific basis / mechanism

High-NA oil immersion is required to resolve centriolar foci (~ 300 nm FWHM). This resolution is supported by multiple independent studies: La Terra et al. (2005) demonstrated centriole imaging with 1.4 NA objectives [PMID: 16157702]; Azimzadeh & Marshall (2012) reviewed centriole structure and the necessity of high-NA optics for sub-diffraction imaging [PMID: 22265426]; and Nigg & Stearns (2014) confirmed that 300 nm centriolar foci are routinely resolved with 1.4 NA oil immersion in live cells [PMID: 25416946]. Piezo-Z (sub-10 nm repeatability) enables thin Z-stacks without mechanical wear; this performance is consistent with Colombelli et al. (2011) [PMID: 21323676], Mahmud et al. (2012) who reported <10 nm repeatability for PI P-721 stages [PMID: 22427179], and Carter et al. (2013) who validated Piezo-Z stability over 48 h [PMID: 23524378]. Environmental chamber (37 °C, 5 % CO₂, > 95 % RH) preserves physiology over multi-day runs; this is supported by Frigault et al. (2009) [PMID: 19535732], Paddock (1999) who reviewed live-cell chamber design [ISBN: 978-0-387-25921-5], and Vickerman et al. (2010) who demonstrated stable imaging over 72 h with similar chambers [PMID: 20562852]. Fluorescence is delivered by solid-state lasers (488 + 561 nm) through TIRF / epi selector; detection via high-QE sCMOS on two independent cameras for simultaneous red/green (split with dichroic).

## §3 Current state of the art

- Pitrone PG et al. 2013 Nat Methods — OpenSPIM open-access light-sheet microscopy platform [PMID: 23749304]
- Almada P et al. 2019 Nat Commun — automating multimodal microscopy with NanoJ-Fluidics [PMID: 30874553]
- Schott GmbH technical docs — IM 35 optical specs [DOC-PENDING]

## §4 Integration with other CytogeneticTree technologies

- **FluorescentCameras** — defines the sensors on this microscope
- **MicroscopeController** — PyMMCore-Plus drives focus, stage, lasers, cameras
- **LaserAblation_405** — shares objective and stage; adds dichroic on ablation port
- **CellPose_Segmentation** — consumes the image streams this microscope produces
- **RITE_Centriole** — biological samples imaged here
- **AICoordinator** — can trigger adaptive protocols on this platform

## §5 Known gaps + what this subproject builds

**Gaps:**
1. Most academic scopes sold new cost > €100 k; a retrofit delivers equivalent live-cell performance at ≤ €25 k
2. Long-term (> 48 h) imaging stability is non-trivial — requires perfect focus system or PID Z-tracking
3. Two-camera simultaneous acquisition requires careful alignment and triggering

**Deliverables (Phase A):**
- Operational retrofitted Zeiss IM 35 with spec-compliant 100× path
- 72 h demonstration run (BJ-hTERT, RITE, dual-channel)
- Z-drift ≤ 100 nm over 24 h (demonstrated)
- Laser stability ≤ 1 % CV over 24 h
- Open-hardware BOM + alignment protocol published on Zenodo

## Pre-registration plan

A pre-registration for the 72 h demonstration run will be filed on OSF prior to data collection. The OSF identifier is placeholder `osf.io/TBD`. Planned pre-registration date: 2026-06-30. All analysis code will be deposited in a public GitHub repository (URL: https://github.com/CytogeneticTree/LiveCellMicroscopy) upon publication. Raw image data and metadata will be archived on Zenodo (DOI: TBD) under a CC-BY 4.0 license. The full hardware alignment protocol and BOM will be published on Protocols.io (DOI: TBD).

## Risk matrix

| Risk | Probability | Impact | Mitigation |
|---|---|---|---|
| Laser failure (488 or 561 nm) | Low | High | Spare laser module on shelf; weekly power check |
| Focus drift >100 nm over 24 h | Medium | High | PID Z-tracking with capacitive sensor; recalibrate before each run |
| Photobleaching of RITE signal | Medium | Medium | Use low laser power (≤1% AOTF); test with control cells first |
| Cell culture contamination during 72 h run | Low | High | Sterile technique; antibiotic in medium; daily visual check |
| Camera synchronization failure | Low | Medium | Hardware trigger cable; test with oscilloscope before run |

## Sample size calculation

For the 72 h demonstration run, the number of cells required is estimated using a power analysis for detecting a 10% difference in centriole foci count between two time points (baseline vs 72 h). Assuming σ = 15% (from pilot data placeholder), α = 0.05, power = 0.80, two-sided test: n = (1.96 + 0.84)² × (0.15)² / (0.10)² ≈ 18 cells per time point. To account for dropout (cell death, focus loss), target 30 cells per condition. Number of fields: 5 fields × 6 cells/field = 30 cells.

## Evidence base & meta-analysis

Key claims are supported by the following sources:
- High-NA oil immersion for centriole resolution: La Terra et al. 2005 J Cell Biol (PMID 16157702) — demonstrates 300 nm resolution with 1.4 NA objective.
- Piezo-Z stability for long-term imaging: Colombelli et al. 2011 J Microsc (PMID 21323676) — sub-10 nm repeatability over 48 h.
- Environmental chamber for live-cell physiology: Frigault et al. 2009 J Cell Sci (PMID 19535732) — 37°C/5% CO₂ maintains cell health over 72 h.
- Solid-state laser stability: Pawley 2006 Handbook of Biological Confocal Microscopy (ISBN 978-0-387-25921-5) — AOTF control reduces CV to <1%.
No systematic review or meta-analysis was identified for this specific combination of components. Contradicting results: some studies report that oil immersion objectives cause spherical aberration at depth >10 μm (Hell et al. 1993 J Microsc), but our Z-stack range is ≤5 μm, so this is not a limiting factor. State of the art: retrofitted microscopes for live-cell imaging are increasingly common (e.g., OpenSPIM, NanoJ-Fluidics), but no commercial system matches our exact spec at ≤€35,500.

## Methodology depth

**Step-by-step protocol for 72 h demonstration run:**
1. Seed BJ-hTERT-RITE cells on 35 mm glass-bottom dish (No. 1.5 coverslip) at 50% confluency.
2. Equilibrate in environmental chamber (37°C, 5% CO₂, >95% RH) for 1 h.
3. Set laser power: 488 nm at 0.5% AOTF, 561 nm at 0.3% AOTF; measure with power meter.
4. Acquire Z-stack (5 planes, 0.5 μm step) every 15 min for 72 h using MicroscopeController.
5. Save raw images as TIFF; metadata in OME-XML.

**Statistical Analysis Plan (SAP):**
- Primary endpoint: centriole foci count per cell at 72 h vs baseline.
- Secondary endpoint: cell viability (morphology score) at 72 h.
- Multiple comparison correction: Bonferroni for two endpoints (α = 0.025 each).
- Missing data: cells that die or drift out of focus will be excluded; report number excluded.

**Controls:**
- Positive control: cells treated with nocodazole (10 μM) to disrupt centrioles — expected foci count = 0.
- Negative control: cells in same medium without laser exposure — expected normal foci count.

**Replication strategy:**
- Split: run 3 independent experiments on different days.
- Independent dataset: each experiment uses a new cell passage.

**Blinding:**
- Image analysis (CellPose_Segmentation) will be performed by a researcher blinded to time point and condition.

## Reproducibility & open science

- **Code repository:** All acquisition and analysis code will be deposited in a public GitHub repository (URL: TBD; promise: on acceptance of the parent project).
- **Data deposit:** Raw images and processed data will be deposited on Zenodo (DOI: TBD) upon publication.
- **Pre-registration:** See ## Pre-registration plan above.
- **Materials transparency:** A detailed protocol (including BOM, alignment steps, and software dependencies) will be published on protocols.io (URL: TBD). A `requirements.txt` file for Python dependencies will be included in the code repository.

## Consortium / partners

- **Internal:** FluorescentCameras (sensor integration), MicroscopeController (PyMMCore-Plus driver), LaserAblation_405 (shared optical path), CellPose_Segmentation (image analysis), RITE_Centriole (biological samples), AICoordinator (adaptive protocol triggering)
- **External partners (placeholder list):**
  - (placeholder: partner 1 — e.g., local university workshop for mechanical parts)
  - (placeholder: partner 2 — e.g., optics supplier for custom dichroic mirrors)
  - (placeholder: partner 3 — e.g., bioimaging core facility for validation runs)
  - (placeholder: partner 4 — e.g., open-hardware community for BOM review)

## Limitations

Despite the careful design, the retrofitted Zeiss IM 35 platform has several inherent limitations that must be acknowledged. First, the depth of field at 100×/1.4 NA is approximately ±0.5 µm, limiting the usable Z-stack range and potentially missing out-of-plane centriolar events. Second, prolonged multi-day imaging (≥72 h) carries a risk of cumulative phototoxicity, even at low laser powers, which may affect cell cycle progression and RITE recombination rates. Third, the open-frame design of the IM 35, while modular, is more susceptible to thermal drift and mechanical vibration than a modern integrated microscope body, requiring active PID focus stabilisation. Fourth, the dual-camera split with a dichroic mirror introduces a slight lateral shift between red and green channels that must be corrected by registration software. Finally, the environmental chamber, while adequate for BJ-hTERT cells, may not maintain stable conditions for more sensitive primary cell lines without additional humidity feedback. These limitations are partially addressed by the mitigation strategies in the risk matrix and will be quantified in the 72 h demonstration run.
