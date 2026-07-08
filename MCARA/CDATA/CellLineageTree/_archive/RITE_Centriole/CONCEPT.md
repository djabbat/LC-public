# RITE_Centriole — Recombination-Induced Tag Exchange for Centriole Age Tracking

**Parent project:** [CytogeneticTree](../CONCEPT.md) (LC umbrella)

## §1 Purpose

RITE_Centriole provides the **molecular clock** that makes the Cytogenetic Tree observable. By genetically encoding an inducible fluorescent tag swap (mCherry → GFP) on a centriolar scaffold protein (e.g., Centrin-1, SAS-6, or CEP152), we create a system in which every centriole synthesized *before* a tamoxifen pulse remains red, and every centriole synthesized *after* the pulse becomes green. This permits direct optical read-out of centriole age across multiple cell divisions — the foundational measurement required to reconstruct the lineage tree from zygote to terminally-differentiated cell.

## §2 Scientific basis / mechanism

RITE (Verzijlbergen et al. 2010 [PMID: 20018668]) uses Cre/loxP-mediated excision to irreversibly swap the C-terminal tag of a target protein. A Cre-ER^T2 driver activates only when 4-hydroxytamoxifen (4-OHT) is added, giving temporal control within minutes. Applied to centriolar proteins, the system exploits the semi-conservative mode of centriole duplication: mother centrioles retain their original (pre-pulse, red) coat, while procentrioles assembled de novo after the pulse incorporate newly-translated (green) protein. Because centriolar proteins such as Centrin are extremely stable (turnover > 24 h), the tag-swap cleanly distinguishes "old" from "new" centrioles over multiple divisions.

## §3 Current state of the art

- Verzijlbergen KF et al. 2010 — original RITE method, yeast histones [PMID: 20018668]
- Yamashita YM et al. 2007 Science — asymmetric centrosome inheritance in *Drosophila* male germline stem cells [PMID: 17255513]
- Nigg EA, Holland AJ 2018 Nat Rev Mol Cell Biol — centrosome biogenesis review [PMID: 29363672]

No published study has yet combined RITE with centriolar scaffolds in mammalian cells for lineage tracing. This gap is the core opportunity.

### Contradictory evidence
- Symmetric centriole inheritance has been observed in mouse neural progenitors (PMID: 25955889). In human iPSCs, the evidence is mixed: PMID 33725406 reports both asymmetric and symmetric inheritance patterns, not exclusively symmetric. This nuance must be considered when interpreting lineage data from RITE_Centriole.
- Centrin-1 turnover may exceed 24 h in some cell types (PMID: 21795662), but shorter turnover (~12 h) has been reported in cycling cells (PMID: 24218623), potentially reducing tag-swap contrast.

## §4 Integration with other CytogeneticTree technologies

- **LentiviralTools** — delivers the RITE cassette into BJ-hTERT and other cell lines
- **LiveCellMicroscopy** + **FluorescentCameras** — image two-channel (red/green) centriole signals over days
- **CellPose_Segmentation** — segments cells and centriole foci across frames
- **LaserAblation_405** — selectively kills daughter lineages to simplify tree reconstruction
- **ImageAnalysis** — quantifies red:green ratio per centriole per frame
- **GenealogyReconstruction** — consumes per-division red/green inheritance calls to assemble the tree
- **StatisticalAnalysis** — MCMC inference of centriole-age inheritance bias

## §5 Known gaps + what this subproject builds

**Gaps in literature:**
1. No validated RITE construct for mammalian centrioles exists
2. Centriolar protein choice (Centrin-1 vs SAS-6 vs CEP152) unresolved for optimal signal/noise
3. Tamoxifen pulse duration vs Cre efficiency not mapped for this application
4. Tag maturation times (mCherry ~40 min; GFP ~30 min) must be factored into timing calibration

**Deliverables (Phase A):**
- Design 3 candidate RITE-centriole plasmids (Centrin-1, SAS-6, CEP152)
- Validate tag-swap kinetics in HEK293T after 4-OHT
- Establish single-cell clonal BJ-hTERT-RITE lines
- Publish construct maps + validation data on Addgene + Zenodo

Budget line-items, pulse protocols, and imaging parameters are in PARAMETERS.md.

## Consortium / partners

**Proposed external collaborators (to be confirmed):**
- Corey Nislow (University of British Columbia) — lentiviral library design and delivery
- Takashi Akera (National Institutes of Health) — centrosome biology and live-cell imaging expertise
- [Additional partner TBD] — bioinformatics for lineage tree reconstruction

## Evidence base & meta-analysis

**Key claims and supporting evidence:**
1. **RITE enables inducible tag-swap on stable proteins** — supported by Verzijlbergen et al. 2010 (PMID: 20018668) in yeast histones; no systematic review or meta-analysis available for mammalian centriolar proteins.
2. **Asymmetric centrosome inheritance occurs in stem cells** — supported by Yamashita et al. 2007 (PMID: 17255513) in *Drosophila*; contradictory evidence exists (e.g., symmetric inheritance in some mammalian systems) not yet discussed.
3. **Centrosome biogenesis is well-characterised** — supported by Nigg & Holland 2018 (PMID: 29363672) review.

**State of the art:** No published study has combined RITE with centriolar scaffolds in mammalian cells for lineage tracing. This gap is the core opportunity.

**Contradictory results:** Potential issues (e.g., tag instability, incomplete swap, mixing of old and new subunits) are not addressed in the current literature and will be assessed experimentally.

## Methodology depth

**Step-by-step protocol (to be detailed in separate protocol document):**
1. Clone RITE cassette (mCherry-loxP-GFP-loxP) into lentiviral backbone under constitutive promoter.
2. Transduce BJ-hTERT cells at MOI 0.3; select stable integrants with puromycin.
3. Induce Cre-ER^T2 with 500 nM 4-OHT for 24 h; wash out.
4. Image live cells every 30 min for 72 h using spinning-disk confocal (488 nm and 561 nm excitation).
5. Segment cells and centriole foci using CellPose; track lineages manually or via automated tracking.
6. Quantify red:green ratio per centriole per frame; call inheritance events.

**Statistical Analysis Plan (SAP):**
- Primary endpoint: proportion of cells with complete tag-swap (≥ 80% green centrioles) at 48 h post-induction.
- Secondary endpoint: number of divisions before tag-swap signal drops below 50%.
- Multiple comparisons: Bonferroni correction for pairwise comparisons across centriolar protein variants.
- Missing data: cells lost due to phototoxicity or detachment will be excluded; sensitivity analysis with worst-case imputation.

**Controls:**
- Positive control: RITE on histone H2B (known stable protein) to validate system.
- Negative control: no 4-OHT induction to measure leakiness.

**Replication strategy:**
- Split-sample: each condition run in triplicate on separate days.
- Independent dataset: repeat with different cell line (e.g., RPE-1) if feasible.

**Blinding/randomisation:**
- Image acquisition order randomised across conditions.
- Centriole age calls made by automated algorithm (blinded to condition).

## Reproducibility & open science

- **Code repository:** [github.com/CytogeneticTree/RITE_Centriole](https://github.com/CytogeneticTree/RITE_Centriole) — all analysis scripts (Python/ImageJ macros) will be made public upon acceptance.
- **Data deposit:** Raw TIFF stacks and segmentation masks will be deposited in Zenodo (DOI TBD) upon publication.
- **Pre-registration:** OSF ID `osf.io/TBD` (planned date 2025-12-01).
- **Materials transparency:** Full plasmid maps and primer sequences will be provided on protocols.io; `requirements.txt` for Python environment included in repository.

## Limitations

1. **Incomplete tag-switch:** Cre-mediated excision may fail in 5–15% of cells, leading to mixed red/green centrioles that complicate age assignment.
2. **Photobleaching:** Prolonged live-cell imaging (≥72 h) may cause significant photobleaching of GFP and mCherry, reducing signal-to-noise ratio over time.
3. **Cre-ER stability:** The Cre-ER^T2 fusion protein may exhibit leaky activity in the absence of 4-OHT, causing premature tag-switch.
4. **4-OHT toxicity:** 4-Hydroxytamoxifen can induce cell cycle arrest or apoptosis at high concentrations (>2 µM) or prolonged exposure (>24 h), potentially biasing lineage reconstruction.
5. **Centriolar protein turnover:** While Centrin-1 is stable, other centriolar proteins (e.g., CEP152) may have faster turnover, reducing the persistence of the tag-switch signal.
6. **Cell-type specificity:** Results in BJ-hTERT fibroblasts may not generalise to other cell types (e.g., stem cells, cancer cells) with different centriole dynamics.
7. **Limited lineage depth:** The red:green ratio will dilute with each division as new green centrioles are synthesised; beyond ~5–7 divisions, age discrimination may become unreliable.

## Sample size calculation

A priori power analysis for the primary endpoint (≥80% green centrioles at 48 h post-pulse) is not yet performed. A placeholder calculation will follow the formula: n = (Z_α/2 + Z_β)² · σ² / δ², where δ is the expected effect size (e.g., 20% difference in green centriole proportion between conditions), σ is the estimated standard deviation from pilot experiments (TBD), α = 0.05, and β = 0.20. The required number of cells per condition will be determined after initial pilot data (N = TBD cells per replicate, ≥3 independent replicates).
