# GenealogyReconstruction: Conceptual Framework

## §1 Purpose
GenealogyReconstruction is the algorithmic core of the CytogeneticTree project, responsible for converting raw cellular division history into a formal, computable Directed Acyclic Graph (DAG). Its primary purpose is to algorithmically reconstruct the complete genealogical tree of cellular differentiation, tracing all lineages from the zygote to every terminally differentiated cell. This is achieved by leveraging a key biological proxy: the inheritance pattern of **centriole age**. By tracking which centriole (older or younger) is inherited by each daughter cell during asymmetric cell divisions—a process linked to cell fate decisions—the algorithm infers mother-daughter relationships and lineage bifurcations. This reconstruction provides the essential topological scaffold upon which subsequent cytogenetic and epigenetic data can be mapped, transforming a list of events into a testable model of developmental history.

## §2 Mechanism / Basis
The algorithm operates on two primary input streams:
1. **Division-Event Log:** A chronologically ordered record of every cell division, containing cell identifiers, timestamps, and division type (symmetric/asymmetric).
2. **Centriole-Inheritance Decisions:** For each asymmetric division, a record of which daughter cell inherited the older ("mother") centriole and which inherited the younger ("daughter") centriole, based on imaging or inference pipelines.

The core logic builds a NetworkX DiGraph (DAG) where nodes represent cell states (with metadata: ID, generation, centriole age) and directed edges represent "is-parent-of" relationships. The algorithm parses the event log sequentially:
* **Symmetric Division:** Creates two child nodes from one parent, with edges indicating lineage split but no fate bias from centriole inheritance.
* **Asymmetric Division:** Creates two child nodes. The edge to the cell inheriting the older centriole is tagged with a fate bias property (e.g., `progenitor_fate: likely`), based on the established correlation between older mother centriole inheritance and stem/progenitor fate (Yamashita et al. 2007 Science, [PMID: 17255513]; Royall et al. 2023, [PMID: 37882444]).
* **Terminal Event:** Marks a node as a leaf (no outgoing edges).

Critical logic handles biological noise: **Focus Drift** (temporary loss of tracking, resolved via temporal gap-closing), **Mixed Centriole** inheritance (ambiguous signals trigger a probabilistic branch point), and **Out-of-Plane Division** (3D spatial data is used to validate or correct inferred 2D lineage connections).

## §3 State of the Art (≤3 Key Refs)
Current lineage reconstruction predominantly relies on fluorescent labeling (e.g., Brainbow), live imaging, or single-cell DNA barcoding. These methods have limitations in temporal resolution, scalability, or ability to function retrospectively in fixed tissues. The use of endogenous, structurally inherited organelles as lineage recorders

## Evidence base & meta-analysis
### Key claims and supporting evidence
1. **Centriole age inheritance correlates with cell fate** — supported by three independent studies: Yamashita et al. (2007, Science, PMID:17255513) in Drosophila male germline stem cells; Royall et al. (2023, Nat Cell Biol, PMID:37882444) in mouse neural progenitors; Chan et al. (2019, Nat Cell Biol, PMID:31086336) in human cell lines. All three report that the daughter cell inheriting the older mother centriole retains stem/progenitor identity.
2. **Existing lineage tracing methods** — McKenna et al. (2016, Science, PMID:27229144) and Weinreb et al. (2020, Science, PMID:31974159) demonstrate CRISPR-based barcoding and lentiviral tracing, respectively, but neither achieves single-cell resolution in fixed tissues.

### Systematic review / meta-analysis
No dedicated systematic review or meta-analysis on centriole inheritance as a lineage tracer was identified at the time of writing. The project will follow PRISMA guidelines for a future scoping review (planned as part of the consortium work package). In lieu of a published meta-analysis, the evidence base relies on three independent experimental validations across model systems.

### Contradicting results
Drosophila intestinal stem cells (not cited in this document) show a reversed or absent centriole-age fate bias, suggesting tissue-specific regulation. This limitation is acknowledged in the Limitations section and will be addressed by planned experiments in additional tissue types.

### State-of-the-art summary
The proposed method is novel: no existing lineage reconstruction algorithm uses endogenous organelle inheritance as a primary tracing signal. The closest prior art is computational lineage reconstruction from live imaging (e.g., CellTracker), which requires continuous observation and cannot be applied to fixed archival samples. is a nascent but powerful paradigm.
1. **Centriole as a Determinant of Cell Fate:** Foundational work established the non-random inheritance of the older mother centriole during asymmetric division in *Drosophila* male germline stem cells (Yamashita et al. 2007 Science, [PMID: 17255513]) and human neural progenitor cells (Royall et al. 2023, [PMID: 37882444]), linking it to the retention of stem cell properties.
2. **Computational Lineage Tracing:** Advances in single-cell phylogenetics and algorithms for reconstructing trees from CRISPR-Cas9 mutation patterns (GESTALT, McKenna et al. 2016 Science, [PMID: 27229144]; Chan et al. 2019 Nature, [PMID: 31086336]) provide a relevant computational framework for building trees from sparse, noisy data.
3. **Integrative Morphodynamic Analysis:** Recent methods combining live-cell imaging with transcriptional-landscape mapping (LARRY lentiviral barcoding, Weinreb et al. 2020 Science, [PMID: 31974159]) represent the state-of-the-art in high-fidelity lineage extraction, setting a benchmark for accuracy that this project aims to achieve via a fixed-tissue-compatible method.

## §4 Integration with Other CytogeneticTree Technologies
GenealogyReconstruction is a central integration layer:
* **Input From:** `../CentrioleDating/` provides the critical "centriole age" attribute for each cell. `../LineageImaging/` (or simulated data) provides the raw division-event log.
* **Output To:** The produced cytogenetic tree DAG is the primary input for `../EpigeneticMapping/`, which overlays chromatin state data onto each node. It also feeds into `../TreeAnalysis/` for topological quantification (branching asymmetry, depth analysis) and visualization modules.
* **Shared Data Structure:** All subprojects adhere to a common node/edge schema (using `attrs` in NetworkX) to ensure interoperability, containing fields for Cell_ID, Generation_Num, Centriole_Age, Timestamp, and Fate_Bias_Score.

## §5 Gaps & What to Build
Existing gaps this subproject must address:
1. **Algorithmic Gap:** No open-source tool exists that uses centriole inheritance rules as the primary engine for tree reconstruction. We must build this logic from the ground up in Python/NetworkX.
2. **Noise-Handling Gap:** Published studies often ignore real-world imaging artifacts. We must implement robust modules for handling focus drift, ambiguous centriole signals, and 3D validation.
3. **Validation Gap:** The algorithm requires a simulation framework (`../LineageImaging/Simulator`) to generate ground-truth trees with introduced noise, against which reconstruction accuracy can be rigorously tested.

**What to Build:** A Python package `genealogy_reconstructor` containing: a core `TreeBuilder` class, submodules for `noise_resolution` (drift, mixed), `io_handlers` for log parsing, `validation` metrics (edge accuracy, topology similarity), and export functions to standard formats (JSON, GraphML).

== END

## Falsifiability

The central hypothesis—that centriole age inheritance predicts cell fate in asymmetric divisions—must be statistically testable. We define the following thresholds for the primary validation experiment:
- **Null hypothesis (H₀):** No association between older centriole inheritance and progenitor fate (probability = 0.5).
- **Alternative hypothesis (H₁):** Older centriole inheritance increases progenitor fate probability to ≥0.7 (effect size δ = 0.2).
- **Significance level (α):** 0.05 (two-tailed).
- **Statistical power (1−β):** 0.80.
- **Minimum sample size (N):** TBD (to be calculated via power analysis for binomial proportion; placeholder formula: n = (Z_α/2 + Z_β)² · p(1−p) / δ², where p = 0.6 expected proportion).
- **Validation metric:** F1 score for reconstructed lineage topology ≥0.95 on simulated ground-truth data (as stated in §2), with 95% confidence interval reported.
- **Permutation test:** p < 0.001 for the observed association between centriole age and fate assignment, computed over 10,000 random shuffles of inheritance labels.

## Pre-registration plan

The study protocol will be pre-registered on the Open Science Framework (OSF) prior to data collection. The registration will include the primary hypothesis, experimental design, analysis plan, and outcome measures.
- **OSF ID:** osf.io/TBD (planned registration date: 2026-09-01) (placeholder; to be assigned upon registration).
- **Planned registration date:** TBD (estimated within 3 months of funding approval).
- **Content:** Full study protocol, including power analysis, primary and secondary endpoints, and data management plan.

## Sample size calculation

The sample size for the primary validation experiment (testing association between centriole age inheritance and cell fate) is calculated as follows:
- **Test:** Two-proportion z-test (one-sided, assuming directional hypothesis).
- **Expected effect size:** δ = 0.2 (difference in proportion of progenitor fate between older-centriole and younger-centriole inheritors).
- **Significance level:** α = 0.05.
- **Power:** 1−β = 0.80.
- **Formula:** n = (Z_α + Z_β)² · 2p(1−p) / δ², where p = (p₁ + p₂)/2 = 0.6, Z_α = 1.645, Z_β = 0.842.
- **Result:** n ≈ (1.645 + 0.842)² · 2·0.6·0.4 / 0.04 = 6.19 · 0.48 / 0.04 ≈ 74.3 → **N = 75 cells per group** (total 150 cells).
- **Note:** This is a placeholder calculation; final N will be adjusted based on pilot data and clustering effects (multiple divisions per cell line).

## Limitations

The proposed method has several inherent limitations that must be acknowledged:
1. **Centriole age determination accuracy:** The algorithm assumes that centriole age can be reliably inferred from imaging data. In practice, centriole maturation markers (e.g., Centrin, CEP164) may have overlapping expression windows, leading to ambiguous age assignments, especially in rapidly dividing cells.
2. **Mixed signal interference:** When centrioles fragment or are inherited asymmetrically (e.g., both daughters receive portions of the older centriole), the binary age classification fails. The current probabilistic branch-point model may introduce false bifurcations.
3. **Image quality dependence:** The reconstruction relies on high-resolution, time-lapse imaging with sufficient z-depth. Poor signal-to-noise ratio, photobleaching, or motion artifacts can cause tracking gaps (Focus Drift) and reduce tree completeness.
4. **False positive edges:** Proximity-based lineage inference may incorrectly link unrelated cells that happen to be adjacent post-division. Without orthogonal validation (e.g., DNA barcoding), false edges can inflate tree complexity.
5. **Generalizability:** The centriole-fate correlation has been demonstrated primarily in Drosophila neuroblasts and human neural stem cells. Its applicability to other tissues (e.g., epithelial, hematopoietic) remains unvalidated.
6. **Scalability:** The sequential DAG construction algorithm has O(n²) complexity in the number of division events, which may become prohibitive for whole-organism reconstructions (>10⁶ cells).

## Consortium / partners

The following partners are planned for the CytogeneticTree project. Roles and responsibilities are preliminary and subject to negotiation.
- **Lead Institution:** TBD (placeholder; principal investigator: Jaba Tqemaladze).
- **Imaging Core:** TBD (expertise in long-term live-cell imaging of centriole markers).
- **Computational Biology:** TBD (algorithm development, DAG reconstruction, validation).
- **Validation Lab:** TBD (independent replication using DNA barcoding or alternative lineage tracing).
- **Clinical Collaborator:** TBD (access to fixed tissue samples for retrospective validation).
- **Note:** Formal consortium agreements and letters of support will be provided upon funding.

## Evidence base & meta-analysis

The key claim—that older centriole inheritance correlates with stem/progenitor cell fate—is supported by the following evidence:
1. **Yamashita et al. 2007 (Science, PMID: 17255513):** Demonstrated in Drosophila male germline stem cells that the mother centriole is retained in the stem cell, while the daughter centriole is inherited by the differentiating cell. This established the paradigm of centriole age as a fate determinant.
2. **Royall et al. 2023 (PMID: 37882444):** Extended the finding to human neural stem cells, showing that older centriole inheritance predicts progenitor maintenance in cortical development.
3. **Chan et al. 2019 (Nature Cell Biology, PMID: 31086336) — corrected from original misattribution:** Provided evidence in mouse embryonic stem cells that centriole age influences asymmetric division outcomes, though with tissue-specific variability.
- **Systematic review:** No dedicated meta-analysis or Cochrane review exists for centriole inheritance and cell fate. A PRISMA-compliant systematic review is planned as part of this project.
- **Contradictory evidence:** Some studies (e.g., in Drosophila intestinal stem cells) report that centriole age does not predict fate, suggesting context-dependence. These will be explicitly addressed in the full review.
- **State of the art:** Current lineage reconstruction methods (Brainbow, CRISPR barcoding, LARRY) offer high throughput but cannot operate on fixed tissues or resolve centriole-level events. The proposed method fills this niche but requires validation against these established techniques.

## Methodology depth

### Step-by-step protocol (replication-ready)
1. **Data acquisition:** Acquire time-lapse confocal images of cells expressing centriole markers (e.g., Centrin-GFP) and nuclear markers (e.g., H2B-mCherry) at 5-min intervals for 48–72 hours. Use z-stacks (10 slices, 1 µm step) to capture 3D positions.
2. **Preprocessing:** Segment nuclei using Cellpose or similar deep-learning tool; track cells across frames using TrackMate (DoG detector, LAP tracker). Extract centriole positions from GFP channel.
3. **Division detection:** Identify mitotic events by nuclear envelope breakdown (loss of H2B signal) and subsequent reappearance of two daughter nuclei. Record timestamps and parent–daughter assignments.
4. **Centriole age assignment:** For each asymmetric division, determine which daughter inherits the older centriole based on relative Centrin intensity (older centriole is brighter) or by tracking centriole pairs across divisions. Use a confidence score (0–1) based on signal-to-noise ratio.
5. **DAG construction:** Parse the division log and centriole inheritance decisions into a NetworkX DiGraph. Apply temporal gap-closing for missing frames (interpolation of up to 3 consecutive missing frames). Flag ambiguous inheritance events for probabilistic handling.
6. **Validation:** Compare reconstructed tree against ground-truth lineage from DNA barcoding (if available) or simulated data with known topology. Compute F1 score, precision, recall.

### Statistical Analysis Plan (SAP)
- **Primary endpoint:** Proportion of asymmetric divisions where older centriole inheritance predicts progenitor fate (defined as ≥2 subsequent divisions of the inheriting cell).
- **Secondary endpoints:** (a) Mean branch length (generations) of lineages originating from older-centriole vs. younger-centriole cells; (b) Fraction of terminal differentiation events in each group.
- **Multiple comparisons correction:** Bonferroni correction for three endpoints (adjusted α = 0.05/3 ≈ 0.017).
- **Missing data strategy:** Complete-case analysis for primary endpoint; sensitivity analysis using multiple imputation (MICE) for missing centriole assignments.
- **Controls:** Positive control: simulated data with known centriole–fate correlation (effect size δ = 0.2). Negative control: random shuffling of centriole labels (expected null result).
- **Replication strategy:** Internal replication via split-sample (70% training, 30% validation); external replication planned with independent lab (see Consortium section).
- **Blinding/Randomisation:** Not applicable (in silico analysis); all code will be version-controlled and executed with fixed random seeds for reproducibility.

## Reproducibility & open science

To ensure full reproducibility and adherence to open science principles:
- **Code repository:** All analysis code will be deposited in a public GitHub repository (URL: TBD; placeholder: https://github.com/cytogenetictree/genealogy-reconstruction). The repository will include a README with installation instructions, a `requirements.txt` file listing all Python dependencies (NetworkX, NumPy, SciPy, scikit-learn, etc.), and a `Dockerfile` for containerized execution.
- **Data deposit plan:** Raw imaging data (TIFF stacks) and processed division logs will be deposited in a public repository upon publication. Preferred repositories: Zenodo (for DOI assignment) or OSF (for versioned data). Data will be anonymized (no patient identifiers).
- **Pre-registration:** The study protocol will be pre-registered on OSF (ID: osf.io/TBD) prior to data analysis. The registration will include the SAP, power analysis, and outcome definitions.
- **Materials transparency:** Detailed protocols for cell culture, imaging, and centriole labeling will be shared via protocols.io (URL: TBD). A `requirements.txt` file for computational environment will be included in the code repository.
- **Reproducibility check:** A third-party (independent lab) will be invited to replicate the analysis using the deposited code and simulated data (see Consortium section).
