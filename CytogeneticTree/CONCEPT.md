# CytogeneticTree — Cytogenetic Tree of Differentiation

**Version:** v1.0 (initial scaffolding)
**Date:** 2026-04-21
**Status:** 🟡 Active (new LongevityCommon subproject)
**Parent umbrella:** `~/Desktop/LongevityCommon/` (coordinator of CDATA, FCLC, Ze, BioSense, MCOA, HAP, Ontogenesis, AutomatedMicroscopy, and now CytogeneticTree)

---

## §1 Central Vision

**Reconstruct the complete genealogical tree of cellular differentiation — from zygote to terminally-differentiated cells — by tracking centriole age across every cell division.**

The centriole is the one cellular organelle that is **structurally heritable, asymmetric, and long-lived**:

- **Heritable:** each daughter cell inherits centrioles from the mother; never synthesized *de novo* in somatic cells (except rare *de novo* biogenesis in early embryos)
- **Asymmetric:** mother and daughter centriole differ morphologically (sub-distal appendages, distal appendages). In many stem-cell divisions the OLDER mother centriole goes to the self-renewing daughter (Yamashita 2007, Wang 2009, Royall 2023)
- **Long-lived:** centriole proteins are not continuously exchanged; polyglutamylation and other post-translational marks accumulate monotonically across divisions

These three properties make the centriole the **ideal physical marker of cell age in a lineage**. If we can mark centriole age (e.g., via RITE pulse-chase fluorescent tagging) and follow every asymmetric division in a population, we can reconstruct the **complete genealogical DAG** of cellular differentiation.

Such a tree would reveal:

1. **Which lineage branches accumulate centriole damage** (aging trajectories)
2. **Which asymmetric divisions conserve youth** vs squander it (regenerative vs senescent fates)
3. **Where differentiation commitment happens** in terms of centriole inheritance
4. **The full DAG from zygote to terminally-differentiated somatic cell**
5. **Empirical validation of CDATA** — do "old-centriole lineages" proliferate less, as predicted?

---

## §2 Scientific Framing

### 2.1 Position within the LongevityCommon ecosystem

| Subproject | Role vs Cytogenetic Tree |
|------------|---------------------------|
| **MCOA** (parent theoretical framework) | Cytogenetic Tree instantiates MCOA Counter #1 (centriolar) at single-cell resolution across lineage |
| **CDATA** (centriolar damage theory) | Cytogenetic Tree is the **empirical test bed** for CDATA — if tree shows old-centriole-lineages arrest, CDATA validated |
| **AutomatedMicroscopy** | Physical hardware platform; Cytogenetic Tree is the experimental + analytical layer on top |
| **Ze / BioSense** | Orthogonal biomarker work at organism level; Cytogenetic Tree operates at single-cell level |
| **FCLC** (federated learning) | Future: Cytogenetic Tree data could feed FCLC aging biomarker training |

### 2.2 Connection to Impetus LOI 2026-04-25

The Impetus LOI proposes a FOCUSED experiment (AI-directed purification of pure old-centriole fibroblast lineage) that is a **minimum-viable demonstration** of the Cytogenetic Tree concept. The LOI tests **one lineage purification** (red-only inheritance → arrest?) as a binary Go/No-Go; Cytogenetic Tree aims for the **complete lineage DAG** as a multi-year research programme.

- **Impetus Phase A = Minimum Viable Cytogenetic Tree (MVCT):** single purification arm demonstrating the method in BJ-hTERT fibroblasts over 6 months
- **Cytogenetic Tree Phase 1 (post-Impetus-Go):** extend to full tree tracking in BJ-hTERT — not just arm 1 but all daughters, all divisions, no ablation
- **Cytogenetic Tree Phase 2:** mouse HSCs (serial transplantation) + embryonic stem cells (zygote → early blastomere)
- **Cytogenetic Tree Phase 3:** vertebrate embryo tracking (zebrafish or mouse), full zygote-to-differentiated DAG

---

## §3 Methodological Core

### 3.1 The centriole-age tag

**RITE (Recombination-Induced Tag Exchange):**

- Cells express `Centrin1-loxP-mCherry-loxP-GFP` construct
- Before recombination: all centrioles **RED (mCherry)**
- Pulse of tamoxifen + Cre-ERT2 → recombination permanently excises mCherry cassette
- After pulse: new centrioles synthesized from activated locus are **GREEN (GFP)**
- Pre-existing centrioles remain RED (Cre can't recombine protein already synthesized)

This creates a **permanent, heritable color tag** distinguishing centrioles present BEFORE vs AFTER the pulse event.

### 3.2 Division watcher

AI-operated live-cell imaging:

- Zeiss IM 35 retrofit + 100×/1.4 NA oil + piezo Z focus correction + FLIR Blackfly S mono camera
- Every cell observed every 10-30 minutes across 6-month experiment
- CellPose segmentation (Stringer 2021) identifies each cell in each frame
- Algorithm detects division events
- Per daughter cell: classify as Red-only, Green-only, or Mixed based on centriole fluorescence

### 3.3 Lineage recording

Every division event is recorded as a **node** in a directed acyclic graph:

```
Mother cell ─┬─ Daughter 1 (inherited: R1+R2+G0)
             └─ Daughter 2 (inherited: R0+G2+G1)
```

Where `Rn / Gn` denote number of centrioles of each type inherited. Over time, the tree grows.

### 3.4 Differentiation annotation

Each lineage node can be annotated with:

- Morphology (cell shape, size from microscopy)
- Single-cell RNA-seq signature (if destructively sampled at endpoint)
- Functional markers (immunofluorescence for differentiation markers)
- PolyGlu signal at mother centriole (proxy for CDATA damage counter)

---

## §4 Technology Stack (Subprojects)

The full methodology requires integration of ~12 technologies. Each is a sub-subproject under CytogeneticTree with its own CONCEPT + core files. See MAP.md for the full dependency graph.

| # | Subfolder | Technology |
|---|-----------|------------|
| 1 | `RITE_Centriole/` | Recombination-induced centriole tag; de novo cloning of `Centrin1-loxP-mCherry-loxP-GFP` |
| 2 | `CellPose_Segmentation/` | AI live-cell segmentation (Stringer 2021 Nat Methods) |
| 3 | `LaserAblation_405/` | 405 nm / fs-IR microablation for lineage purification |
| 4 | `LiveCellMicroscopy/` | Zeiss IM 35 retrofit (100×/1.4 NA oil, piezo Z, env chamber) |
| 5 | `FluorescentCameras/` | FLIR Blackfly S / Hikrobot mono scientific CMOS, global shutter |
| 6 | `MicroscopeController/` | Micro-Manager 2.0 + PyMMCore-Plus automation |
| 7 | `AICoordinator/` | Claude Code `/overnight` agent orchestration + PROMPT.md |
| 8 | `LentiviralTools/` | Plasmid design + HEK293T packaging + BJ-hTERT transduction |
| 9 | `ImageAnalysis/` | Fiji / ImageJ / CellProfiler for polyGlu, Ninein, ARL13B |
| 10 | `StatisticalAnalysis/` | log-rank + Kaplan-Meier + Bayesian MCMC + Sobol |
| 11 | `GenealogyReconstruction/` | DAG algorithm from division-event log → lineage tree |
| 12 | `DifferentiationAnnotation/` | Lineage node → cell-state mapping (scRNA-seq + morph) |

---

## §5 Key Scientific Hypotheses (will be tested via the Tree)

### H1 — Pure old-centriole lineage arrests

Predicted by CDATA; tested in Impetus Phase A; first proof point for the Tree.

### H2 — Differentiation commitment correlates with centriole asymmetry

Stem vs non-stem daughter distinguishable by which centriole inherited (older → self-renewing?).

### H3 — Terminal differentiation = centriole accumulated damage threshold

Terminally-differentiated cells have oldest centrioles; threshold triggers cell-cycle exit.

### H4 — The Tree has a canonical topology

Across replicate embryos / tissue systems, the cytogenetic tree has conserved structure → differentiation is deterministic at centriolar level.

---

## §6 Current Status (2026-04-21)

- **Umbrella CONCEPT + core files** — generated 2026-04-21 (this document)
- **Technology subfolders** — being scaffolded in parallel via agent
- **Literature landscape + technology ecosystem** — KNOWLEDGE.md being populated via PubMed search agent
- **Impetus LOI 2026-04-25** — scheduled submission, minimum-viable demonstration of Tree concept
- **Connection to Ilia Zheleznov HSC simulator** — planned integration; his simulator could run predicted lineage trajectories to pre-validate expected Tree topology before wet-lab experiment

---

## §7 Outputs (Future)

### Near-term (post-Impetus, conditional)

- Full 6-month time-lapse of BJ-hTERT lineage tree with RITE tags
- Public dataset release (raw images + division event log + DAG topology)
- Companion paper — "Cytogenetic Tree of Fibroblast Differentiation: A Lineage-Resolved Test of the Centriolar Counter Hypothesis"

### Long-term

- Mouse HSC Cytogenetic Tree (serial transplantation)
- Embryonic Cytogenetic Tree (zygote → differentiated somatic; vertebrate model)
- Cross-species Cytogenetic Tree comparison (conserved topology?)

---

## §8 Integrations Required

- **CDATA** — theoretical model must be updated to include lineage-level predictions
- **MCOA** — Counter #1 (centriolar) formalism needs per-lineage version
- **Impetus grant** — Phase A provides the first MVCT data
- **Ilia Zheleznov HSC simulator** — parallel simulation of expected Tree topology
- **AIM** — not directly (this is research, not clinical)
- **DrJaba** — not directly (clinic, not research)

---

## §9 Open Questions (2026-04-21)

1. Does RITE-Centriolin construct exist? (As of Impetus LOI audit: **NO** — has to be de novo synthesized)
2. Can Cre-ERT2 achieve high enough recombination efficiency for clean pulse-chase? (Target: >90% in 48h)
3. Is 405 nm laser ablation centriole-specific enough? (Key phototoxicity control needed — Month 2 calibration)
4. Can AI segmentation reliably classify mixed-centriole daughters? (Edge case: 1R+1G)
5. How to handle divisions where we can't see both centrioles clearly? (Focus drift, out-of-plane events)

---

*Generated 2026-04-21. Part of LongevityCommon ecosystem. Connected to MCOA (theoretical), CDATA (mechanistic), AutomatedMicroscopy (hardware), Impetus LOI (funding).*
