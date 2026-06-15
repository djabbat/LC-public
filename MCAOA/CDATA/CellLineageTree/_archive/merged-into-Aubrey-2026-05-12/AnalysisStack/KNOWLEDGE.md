# KNOWLEDGE — AnalysisStack

This document summarises the existing knowledge base that informs the project.

## 1. Cell segmentation and tracking

- **Deep learning for segmentation:** U‑Net variants (Ronneberger et al. 2015, MICCAI) and flow‑based methods (CellPose) are the current standard for cellular instance segmentation.
- **Tracking:** The CellTrackingChallenge (Ullman et al. 2017) provides objective benchmarks. Top methods combine detection with temporal linking (e.g., TrackMate, Jaqaman et al. 2008).
- **Centriole detection:** Diffraction‑limited spot localization using radial symmetry (Parthasarathy 2012) or neural networks (spotiflow, Dohmen et al. 2024).

## 2. Centriole biology

- **Centriole structure and function:** Centrioles are microtubule‑based organelles that form centrosomes and cilia. Their duplication is tightly regulated in S phase.
- **Asymmetric inheritance:** In many cell types, the older (“mother”) centriole is preferentially inherited by the cell that remains in the stem‑cell niche (Yamashita et al. 2007, PMID: 17928863).
- **Centriole loss and senescence:** Cuartero et al. (2022) showed that centriole loss triggers p53‑dependent senescence, linking centriole dysfunction to aging.
- **RITE‑centriole system:** Jones et al. (2022) developed a dual‑colour fluorescent tagging system that allows discrimination of mother and daughter centrioles in live cells.

## 3. Statistical modelling of lineage data

- **Survival analysis:** Kaplan‑Meier estimators and Cox proportional‑hazards models are standard for time‑to‑event data (here, centriole loss or senescence onset).
- **Mixed‑effects models:** Account for correlation within lineages (Pinheiro & Bates 2000).
- **Bayesian inference:** Provides full posterior distributions for model parameters, useful when sample sizes are limited (Gelman et al. 2013, ISBN 978‑1‑4398‑4095‑5).

## 4. Lineage reconstruction

- **Phylogenetic methods adapted to cell lineages:** Maximum‑parsimony and maximum‑likelihood approaches can be applied when branch lengths are known (frames) and topology is informed by centriole age.
- **Graph‑based algorithms:** NetworkX (Hagberg et al. 2008, SciPy) is used to build and analyse directed acyclic graphs of cell divisions.

## 5. Single‑cell RNA‑seq for cell‑type annotation

- **Reference atlases:** Tabula Sapiens (PMID: 35549404) and Human Cell Landscape provide reference signatures.
- **Classification methods:** Elastic‑net multinomial regression (Zou & Hastie 2005) or support vector machines are commonly used for supervised annotation.
- **Integration with imaging:** Spatial transcriptomics or single‑cell RNA‑seq after imaging can link molecular profiles to lineage trees.

All references listed in EVIDENCE.md are incorporated here.
