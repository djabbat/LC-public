# RITE_Centriole — UPGRADE (post-MVP extensions)

## U1. Triple-tag version
- mCherry → GFP → BFP (two sequential pulses → three generations distinguishable)
- Requires second orthogonal recombinase (FLP/FRT)

## U2. Endogenous knock-in (CRISPR)
- Replace lentiviral over-expression with knock-in of RITE cassette at endogenous Centrin-1 locus
- Avoids over-expression artifacts; preserves native stoichiometry

## U3. Photoactivatable RITE
- Replace 4-OHT with caged tamoxifen + UV uncaging for **spatial** control (single-cell pulse within a colony)

## U4. In vivo extension
- Deliver to mouse bone marrow HSCs via lentivirus or AAV
- Pulse in vivo, sort cells, image ex vivo — validates HSC centriole asymmetry prediction from CDATA

## U5. Multi-organelle RITE
- Parallel RITE cassettes on mitochondria, basal body, cilium — multi-organelle age atlas

## U6. Non-centriolar controls
- RITE on histones (H3.3) or stable cytoplasmic marker — negative control for age bias

## U7. Single-cell RNA-seq integration
- Couple RITE readout with scRNA-seq (e.g., by FACS-sort red-dominant vs green-dominant cells)
- Connects centriole age to transcriptional state — bridge to `DifferentiationAnnotation`

## U8. Organoid scale-up
- Apply RITE-BJ-hTERT lines to 3D organoid models
- Test whether centriole-age asymmetry persists in crypt-like structures
