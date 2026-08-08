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

> TODO (deep review): add sample size justification (effect size, alpha, beta).

## Blinding Protocol

> TODO (deep review): describe who is blinded and how.

## Pre-registration

> TODO (deep review): OSF link or reason for no pre-registration.
