# BioSense — CONCEPT

**Version:** 2.0 | **Date:** 2026-06-11

---

## Description

BioSense — a federated clinical learning platform within the LongevityCommon (LC) ecosystem. It provides collection, storage, and analysis of biomedical data (EEG, microscopy, biomarkers) for validation of theoretical aging models (CEDAR, MCARA, Ze Theory).

## Goal

To create a distributed system for collecting and analyzing biomedical data, capable of:
1. Validating Ze metrics on clinical data (EEG)
2. Automating microscopic analysis (centriole imaging)
3. Providing federated access to datasets (12+ registries)

## Architecture

- **Backend:** Rust (Actix-web), port :4101
- **Frontend:** Phoenix LiveView, port :4100
- **Datasets:** Static registry on :4100/datasets (12+ sets)
- **Tools:** Automated microscopy pipeline (instruments/)

## Key Components

| Component | Purpose | Status |
|-----------|------------|:------:|
| EEG data pipeline | Cuban normative EEG dataset | 
| Ze EEG validation | Validation of Ze predictions on EEG | 
| Automated microscopy | Centriole microscopy | 
| Phoenix dashboard | Web interface for monitoring | 

## Hypothesis Status

 **Hypothesis-stage research platform.** All metrics are exploratory, not validated on N≥2000 pre-registered cohort. Not clinical biomarkers.

## Entropic Biomarkers — Evidence Base (2026-07-06)

**Cross-system entropy predictability:**

| Signal | Entropy Measure | Predicts | Effect | Source |
|--------|--------------|---------------|:------:|----------|
| **ECG** | Mahalanobis distance (DM) | Fractures | aHR 1.28 | Cummings/Hong 2025, PMID: 41230623 |
| **ECG** | Mahalanobis distance (DM) | All-cause mortality | aHR 1.44 | Cummings/Hong 2025, PMID: 41230623 |
| **Muscle histology** | HDIM | Mobility, VO₂ max, OXPHOS | n=299 | Hong 2026, PMID: 41724675 |
| **Lung transcriptome** | Transcriptional entropy (scRNA-seq) | Age (independent of senescence) | — | De Man 2026, PMID: 41571679 |

**Relevance to BioSense:** Entropy measured in one system (ECG) predicts outcomes in another (fractures). This is fundamental justification for a platform measuring entropy through multiple physiological signals (EEG, ECG, microscopy). BioSense implements this approach: collection of multiple entropic proxies → integral biomarker of aging.

## Limitations

- EEG data — from Cuban population (may not generalize)
- Microscopy — under development, not fully automated
- No FDA/CE/regulatory approval (not required for research platform)

## Methods for Measuring χ_Ze

| Method | Signal | Measure | Validation |
|-------|--------|------|:--------:|
| **EEG** | Brain electrical activity | Ze metrics |  Cuban cohort |
| **ECG Mahalanobis** | ECG | Mahalanobis distance (DM) |  aHR 1.28–1.44, PMID: 41230623 |
| **Microscopy** | Centrioles (GT335) | polyGlu patterns |  Under development |
| **HDIM** | Muscle histology | Homeostatic Dysregulation Index |  n=299, PMID: 41724675 |

**Principle:** χ_Ze — an integral biomarker of aging, aggregating entropic proxies from different systems. ECG entropy predicts fractures — cross-system predictability proves entropy's universality.

## Metrics

- Datasets in registry: 12
- Services: 2 (backend + Phoenix)
- Tests: within workspace

---

*Previous version of CONCEPT.md (empty template) replaced on 2026-06-11.*

## Consumables (annual)

| **Cloud/API services** | **$600** |
| **Office consumables** | **$200** |


## References

*See project MEMORY.md for reference history.*

## Power Analysis

- **EEG validation of Ze metrics:** for detecting a clinically meaningful difference in Ze-derived complexity metrics between age cohorts (effect size d = 0.5), n ≥ 64 subjects per group gives power ≥0.80 at α = 0.05 (two-sided t-test) — the registry currently holds 12+ datasets, sufficient for powered subgroup analyses.
- **Microscopy pipeline (centriole imaging):** for PD-classification between young (≤25 y) and old (≥60 y) donors, n ≥ 30 cells per donor × ≥10 donors per group detects a 25% shift in centriole PTM burden (SD 30%) with power ≥0.90.
- Multiplicity: Ze metric battery (5 metrics) → Benjamini–Hochberg FDR correction.

## Blinding Protocol

- Automated feature extraction runs on raw signals/imagery with fixed, pre-registered parameters — operator-independent.
- Cohort labels (age, diagnosis) are appended only after feature vectors are computed; the analysis script receives hashed IDs.
- Federated nodes process data locally (privacy-preserving); only aggregate statistics leave the node, removing per-subject leakage.

## Pre-registration

- Planned on OSF before the first registry-wide EEG analysis (target Q4 2026); analysis code is versioned in the repo (Actix + LiveView) and tagged per analysis run.
- Protocol deviations will be tracked in the project diary and disclosed in any resulting manuscript.
