# FirstWave Cuban Human Normative EEG Project — CONCEPT

**Date:** 2026-06-11

**Version:** 1.0

## Description

The Cuban normative EEG dataset (FirstWave) — a repository of EEG data from healthy subjects of the Cuban population.

## Purpose

- Validation of Ze-predictions on EEG data
- Normative data for BioSense
- Original repository: oldgandalf/FirstWaveCubanHumanNormativeEEGProject

## Structure

- `EyesOpen/` — EEG with open eyes
- `EyesClose.zip` — EEG with closed eyes
- `avr_ref.m`, `gsf.m` — MATLAB processing scripts
- `data_description.xls` — data description

## Status

🟢 Data uploaded. Used for Ze EEG validation.

## Consumables (annual)

| **Office consumables** (printing, stationery, toner) | **$300** |


## Hypothesis

*To be specified — see CONCEPT.md §1 for project rationale.*


## References

*See project MEMORY.md for reference history.*

## What is this

oldgandalf-CubanEEG is a data-analysis subproject of BioSense: it applies Ze-theory complexity metrics to the Cuban EEG dataset to test whether hierarchical dynamical measures discriminate age groups and neurological states better than standard spectral features.

## How it works

- **Input:** Cuban EEG dataset (BioSense registry, 12+ datasets).
- **Pipeline:** preprocessing (filtering, artifact removal) → feature extraction (Ze metrics: permutation entropy, hierarchy score, synchrony) → classification (age/state groups) with cross-validation.
- **Output:** benchmark table Ze-metrics vs spectral baselines; report for BioSense registry.
