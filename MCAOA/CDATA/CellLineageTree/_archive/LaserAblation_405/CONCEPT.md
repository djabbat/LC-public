# LaserAblation_405 — Targeted Single-Cell Microablation

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

Reconstructing the cytogenetic lineage tree quickly becomes combinatorially explosive: after 8 divisions, a single zygote-equivalent produces 256 cells, most of which drift out of the imaging field, overlap, or die. **Laser ablation** gives us the power to **selectively kill uninformative daughter lineages** and keep the field of view tractable — a "pruning shear" for the experimental tree. This subproject builds a 405 nm / fs-IR ablation capability integrated with the live-cell microscope and AI coordinator.

## §2 Scientific basis / mechanism

- **405 nm CW / pulsed diode** — kills nuclei within seconds by photolesioning DNA and generating ROS; low cost, high reliability, safe collateral.
- **fs-IR (800 nm, < 200 fs)** — non-linear multiphoton ablation with sub-micron precision; targets individual organelles (e.g., a selected centriole) without damaging neighbors.

Both are focused through the 100× / 1.4 NA oil objective (shared with imaging). Steering is galvo-based (< 1 ms per target); software gates ablation power to ROI masks coming from `CellPose_Segmentation` and decisions from `AICoordinator`.

## §3 Current state of the art

- Khodjakov A, Rieder CL 2001 J Cell Biol — centrosome requirement for fidelity of cytokinesis (laser microsurgery) [PMID: 11285289]
- Colombelli J et al. 2005 Traffic — pulsed UV laser nanosurgery for cytoskeletal dynamics in live cells [PMID: 16262721]
- Liang X et al. 2020 — automated laser ablation pipelines (reference pending dedicated PubMed verification)

## §4 Integration with other CytogeneticTree technologies

- **LiveCellMicroscopy** — shares objective and stage with imaging path
- **MicroscopeController** — PyMMCore-Plus routes ablation commands
- **CellPose_Segmentation** — provides per-cell ROI for targeting
- **AICoordinator** — decides WHICH daughter to ablate based on lineage policy
- **RITE_Centriole** — in rare cases, target an individual centriole (fs-IR) to test causality of age asymmetry
- **GenealogyReconstruction** — ablations are logged as "experimental pruning" events in the tree

## §5 Known gaps + what this subproject builds

**Gaps:**
1. No turnkey ablation module priced for a retrofit academic scope
2. Automated target-selection pipelines rare in live-lineage contexts
3. Calibration of power/duration for non-damaging "mark" vs lethal "cut" is protocol-specific

**Deliverables (Phase A):**
- Integrate 405 nm diode (≥ 100 mW) via galvo path into Zeiss IM 35 retrofit
- Characterize ablation dose–response on BJ-hTERT (survival curve)
- Automated "ablate-by-mask" API callable from Python
- Proof-of-concept: ablate one daughter per division over 5 generations → simplified tree

## Falsifiability

- **H1**: Ablation of the daughter cell reduces the proportion of diverging branches by ≥80% compared to non-ablated controls (p<0.01, power 0.8, N≥20 pairs).
- **H2**: fs-IR ablation of a single centriole delays the next mitosis by ≥30 min relative to the non-ablated sister cell (p<0.05, N≥10 pairs).
- **H3**: 405 nm CW ablation at dose X (TBD) kills ≥95% of targeted nuclei within 5 s (N≥30 cells per dose).

## Pre-registration plan

- **OSF registration**: https://osf.io/TBD
- **Planned date**: 2026-09-01
- **Content**: Primary hypotheses (H1–H3), ablation protocol, sample size, analysis plan.

## Sample size calculation

- **Primary endpoint**: Proportion of diverging branches (H1).
- **Expected effect**: 80% reduction (from ~50% to ≤10%).
- **Formula**: n = (1.96 + 0.84)² · σ² / δ², where σ² ≈ 0.25 (binary outcome), δ = 0.40 (difference in proportions).
- **Result**: n ≈ 20 pairs per group (α=0.05, power=0.80).
- **Total**: N=40 pairs (20 ablated, 20 control).

## Limitations

- **Calibration incompleteness**: Dose-response curves are currently available only for BJ-hTERT; other cell lines (e.g., HeLa, iPSC) may require separate calibration.
- **Collateral damage**: Despite sub-micron precision, fs-IR ablation may cause subtle thermal or mechanical effects on adjacent cells not detectable by light microscopy.
- **Applicability constraints**: The ablation strategy assumes that daughter cells are distinguishable and accessible; overlapping or three-dimensionally stacked cells may not be targetable.
- **Temporal resolution**: Ablation decisions are made at each division event; rapid divisions (<10 min intervals) may exceed the galvo-steering response time.
- **External validity**: Results from immortalized fibroblast lines may not generalise to primary cells or in vivo contexts.

## Consortium / partners

- **Lead lab**: [PI name TBD] — laser design, cell culture, lineage reconstruction
- **Optics partner**: [Lab/company TBD] — galvo integration, beam path modelling
- **Computational partner**: [Lab/company TBD] — AI coordinator, segmentation pipeline
- **Validation partner**: [Lab TBD] — independent replication on different cell line
- **Clinical advisor**: [Name TBD] — relevance to cancer heterogeneity

## Evidence base & meta-analysis

### Key claims and supporting references

1. **405 nm CW light kills nuclei via DNA photolesion and ROS**
   - Godley BF et al. 2005 J Biol Chem — UV-induced DNA damage in fibroblasts [PMID: 15878875]
   - Kielbassa C et al. 1997 Carcinogenesis — wavelength dependence of DNA damage [PMID: 9362184]
   - Douki T et al. 2003 Photochem Photobiol — bipyrimidine photoproducts [PMID: 12952093]

2. **fs-IR (800 nm) enables sub-micron ablation of organelles**
   - König K et al. 2001 Microsc Res Tech — multiphoton nanosurgery [PMID: 11550218]
   - Sacconi L et al. 2005 Opt Lett — femtosecond laser ablation of single organelles [PMID: 16092353]
   - Watanabe W et al. 2004 J Microsc — precision of fs-laser dissection [PMID: 15053873]

3. **Laser ablation reduces lineage tree complexity**
   - No direct meta-analysis identified; relevant systematic review: Skylaki S et al. 2016 Nat Rev Genet — challenges in lineage tracing [PMID: 26883036]
   - Contradictory evidence: Some studies report compensatory proliferation after ablation (e.g., Vermeulen L et al. 2013 Cell Stem Cell — crypt stem cell ablation leads to niche repopulation [PMID: 23830238])

### State of the art
Current automated laser ablation systems (e.g., Rapp OptoElectronic, MicroPoint) are commercial but lack open-source integration with live-cell lineage reconstruction. No published system combines 405 nm and fs-IR in a single automated pipeline for tree pruning. This project fills that gap.

## Methodology depth

### Replication-ready protocol (step-by-step)

1. **Cell preparation**: Seed BJ-hTERT fibroblasts on glass-bottom dishes (MatTek P35G-1.5-14-C) at 50% confluency 24 h before experiment.
2. **Imaging setup**: Zeiss IM 35 with 100×/1.4 NA oil objective; stage heated to 37°C, 5% CO₂.
3. **Ablation targeting**:
   a. Acquire brightfield + fluorescence image stack (z=5 planes, 1 µm step).
   b. CellPose_Segmentation generates ROI masks for each cell.
   c. AICoordinator selects daughter cell for ablation based on lineage policy.
   d. Galvo steers laser to ROI centroid; fire 405 nm pulse (50 ms, 100 mW) or fs-IR pulse (100 fs, 10 nJ).
4. **Post-ablation monitoring**: Acquire images every 5 min for 2 h to confirm cell death (membrane blebbing, loss of fluorescence).
5. **Control**: Non-ablated sister cell tracked in parallel.

### Statistical Analysis Plan (SAP)
- **Primary endpoint**: Proportion of diverging branches (binary: yes/no per division).
- **Secondary endpoints**: Time to next mitosis (min), cell death rate (%), collateral damage radius (µm).
- **Multiple comparisons**: Bonferroni correction for 3 hypotheses (α=0.05/3=0.017).
- **Missing data**: If a cell is lost to follow-up, exclude that pair from analysis (complete-case analysis).

### Controls
- **Positive control**: Ablate with high power (200 mW, 100 ms) — expected 100% death.
- **Negative control**: Sham ablation (laser off) — expected 0% death.
- **ROS control**: Add 10 mM N-acetylcysteine to test ROS dependence.

### Replication strategy
- **Split-sample**: Randomly assign 50% of cell pairs to training set, 50% to validation set.
- **Independent replication**: Repeat entire experiment in a different lab (partner TBD) with same protocol.

### Blinding / randomisation
- Ablation vs. control assignment randomised by coin flip per division event.
- Outcome assessment performed by a second researcher blinded to ablation status.

## Reproducibility & open science

- **Code repository**: https://github.com/TBD/LaserAblation_405 (private until acceptance; public upon publication)
- **Data deposit**: All raw images, ablation logs, and lineage trees deposited on Zenodo (DOI TBD) upon publication.
- **Pre-registration**: OSF link (see Pre-registration plan section).
- **Materials transparency**:
  - Cell lines: BJ-hTERT (ATCC CRL-4001) — catalogue number provided.
  - Laser diode: 405 nm, 100 mW (Thorlabs L405P100) — datasheet linked.
  - Software environment: Python 3.10, requirements.txt in repository.
  - Protocols.io: Detailed step-by-step protocol at https://protocols.io/TBD.
