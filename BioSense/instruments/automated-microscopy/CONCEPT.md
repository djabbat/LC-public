# CONCEPT — Automated Microscopy (BioSense)

**Version:** 1.0

## Purpose
Automated pipeline for microscopic image analysis of centrioles.

## Context
Part of BioSense — a federative system for clinical training. Microscopy produces images of dividing cells; automated-microscopy processes them.

## Components
1. **Image Capture** — microscope control (OpenSPIM, NanoJ-Fluidics)
2. **Processing** — centriole segmentation, mitotic spindle tracking
3. **Analysis** — centriole counting, determination of "age" by GFP label
4. **Export** — data to CEDAR/CellLineageTree for lineage reconstruction

## Technologies
- Python/OpenCV for image processing
- Connection to BioSense API (Rust backend)
- Input data: .tiff, .czi (Zeiss), .nd2 (Nikon)

## Connections
- **Parent:** LC/BioSense/instruments
- **Consumer:** CEDAR/CellLineageTree, PhD/microscope
- **Standard:** OpenSPIM (Pitrone et al., 2013)

## Status
🟡 Concept. Equipment not acquired. Access to microscope required.

## Consumables (annual)

| **Cloud/API services** | **$600** |
| **Office consumables** | **$200** |


## Hypothesis

*To be specified — see CONCEPT.md §1 for project rationale.*


## References

*See project MEMORY.md for reference history.*
