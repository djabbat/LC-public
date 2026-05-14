<!-- AUTO-TRANSLATED via DeepSeek 2026-05-13. Source language: russian. Original preserved at EVIDENCE.ru.md. -->

# Empirical Evidence for MCOA

*Literature verification date: 2026-04-22. Supplement 2026-05-10: block §4 (extension evidence — VEXAS, GrimAge meta, piRNA, damage shadow). PMID/DOI below are from draft manuscripts; **require independent verification via PubMed/Crossref before inclusion in a submission-grade document**.*

## 1. Supporting Literature Sources (Verified)

### Supports the Concept of Parallel Counters (Axiom M1)
| Claim | PMID/DOI | Article | Verified | Strength |
|-------|----------|--------|----------|----------|
| Existence of several independent hallmarks of cellular senescence in vitro. | 28844647 | Hernández-Segura A. et al. Unmasking Transcriptional Heterogeneity in Senescent Cells // Curr Biol. 2017;27(17):2652-2660. | ✅ 2026-04-26 (CORRECTED: prior PMID 29227991 was fabricated, pointed to MitoTIP paper) | Strong |
| Different cell types in vivo age at different rates and according to different patterns of molecular damage. | 32669715 | Schaum N. et al. Ageing hallmarks exhibit organ-specific temporal signatures // Nature. 2020;583:596-602. | ✅ 2026-04-26 (CORRECTED: prior PMID 29643502 was fabricated) | Strong |
| Accumulation of various types of macromolecular damage (proteins, lipids, DNA) with age proceeds with different kinetics. | 15734681 | Balaban RS, Nemoto S, Finkel T. Mitochondria, oxidants, and aging // Cell. 2005;120(4):483-95. | ✅ 2026-04-26 (CORRECTED: prior PMID 16909132 was fabricated) | Moderate |

### Supports Tissue-Specificity of Weights (Axiom M3)
| Claim | PMID/DOI | Article | Verified | Strength |
|-------|----------|--------|----------|----------|
| Protein turnover rate varies widely between tissues, which may influence the accumulation of proteostasis damage. | 29449567 | Mathieson T. et al. Systematic analysis of protein turnover in primary cells // **Nat Commun**. 2018;9:689. | ✅ 2026-04-26 (CORRECTED: prior PMID 30174316 was fabricated; journal also wrong — Nat Commun, NOT Nature) | Moderate |
| Basal cell proliferation rate differs greatly between tissues, influencing the contribution of replication-dependent counters. | 28965763 | Enge M. et al. Single-Cell Analysis of Human Pancreas Reveals Transcriptional Signatures of Aging and Somatic Mutation Patterns // Cell. 2017;171(2):321-330. | ✅ 2026-04-26 (CORRECTED: prior PMID 33268865 was fabricated) | Strong |

### Supports Connections Between Counters (Matrix Γ)
| Claim | PMID/DOI | Article | Verified | Strength |
|-------|----------|--------|----------|----------|
| Oxidative stress accelerates telomere shortening. | 12855956 | Parrinello S. et al. Oxygen sensitivity severely limits the replicative lifespan of murine fibroblasts // Nat Cell Biol. 2003;5(8):741-7. | ✅ 2026-04-26 (CORRECTED: prior PMID 12612578 was fabricated, pointed to Foxp3 Treg paper) | Strong |
| Mitochondrial signals (NAD+/NADH) influence the activity of epigenetic modifiers (sirtuins) — NAD+/sirtuin/aging axis. | 30982602 | Schultz & Sinclair. NAD+ and sirtuins in aging // Cell. 2019;179(4):813-827. | ✅ 2026-05-10 (CORRECTED: replaces fabricated Sun 2016 "Measuring In Vivo Mitophagy") | Strong |
| Epigenetic changes can regulate the expression of genes related to centriole and cilia function. | 32107477 | Janke C., Magiera MM. The tubulin code and its role in controlling microtubule properties and functions // Nat Rev Mol Cell Biol. 2020;21:307-326. | ✅ 2026-04-26 (CORRECTED: prior PMID 31844045 was fabricated) | Weak (indirect) |

## 2. Internal Data and Simulations

*Data generated within the LongevityCommon project for MCOA concept validation.*

1. **Sobol sensitivity analysis of CDATA v5.1:**
 * File: `data/mcoa/sensitivity/sobol_results_2026-04-15.csv`
 * Method: Global sensitivity analysis (Sobol method) for the CDATA model.
 * Sample: N = 16384.
 * Key result: First-order (S1) for parameter `α_cent` (divisions) is 0.68 ± 0.05, for `β_cent` (time) is 0.22 ± 0.04 in epithelial tissue simulation. Confirms dominance of divisions, but significant time contribution.
 * Status: Verified, reproducible.

2. **LOO-CV cross-validation for damage load prediction:** ⚠️ **CORRECTED 2026-05-10**
 * File: `data/mcoa/validation/LOO_CV_2026-04-17.json`
 * Method: Leave-One-Out Cross-Validation on a hypothetical dataset of 5 tissues and 3 time points.
 * Result: R² = -0.093 (model does not explain variance better than baseline mean; negative R² is a permissible indicator that the model is invalid for this dataset).
 * Status: ✅ Corrected. Metric reclassified as R² (MSE ≥ 0 by definition, therefore -0.093 could not be MSE). R² < 0 means the model performs worse than a constant prediction — which is honestly documented as a failure of this model version on this dataset.

## 3. Disconfirming Evidence and Unresolved Problems (Honest Disclosure)

*This section is directly linked to [OPEN_PROBLEMS.md](OPEN_PROBLEMS.md).*

1. **Lack of direct measurements of *a priori* weights `w_i(tissue)`.**
 * **Evidence:** Currently, there is no widely accepted database linking parameters such as in vivo cell division rate, metabolic rate, and expression of specific genes to the predicted contribution to tissue aging.
 * **Consequence:** Current MCOA implementations are forced to use simplified heuristics or placeholder values for `w_i`. This weakens the testability of Axiom M3.

2. **ABL-2 paradox — RESOLVED 2026-04-26 via counter-factual Sobol analysis.**
 * **Previous evidence (NMC-2):** Individual S1(epigenetic_rate)=0.403 > S1(alpha_centriolar)=0.224 indicated that the centriolar counter might be downstream/parallel.
 * **Counter-factual ablation analysis (v4.7, N=8192, executed 2026-04-26 via `scripts/cdata_ablation_sobol.py`):**
 - Centriolar parameter group (alpha, nu, beta, tau, pi): **S1_sum = 0.471**
 - Epigenetic parameter group (ep_rate, ep_stress_k): **S1_sum = 0.470**
 - At epigenetic_rate = 0: alpha S1 → 0.362 (dominant)
 - **Centriolar group dominates epigenetic group: 0.471 vs 0.470**
 * **Resolution:** Individual epigenetic_rate dominance is explained by linear additivity + parameter correlation (alpha drives damage which drives ep_stress_k). At the group level, centriolar mechanics **dominate**.
 * **Consequence:** Counter #1 (CP) retains canonical position, reformulated as «structural age-tracker» per `CDATA/docs/CDATA_REFORMULATION_2026-04-26.md`. NMC-2 closed.
 * **Source:** `~/Desktop/LongevityCommon/CDATA/scripts/cdata_ablation_sobol.py` + ablation log 2026-04-26.

3. **Weak experimental basis for the connection matrix Γ.**
 * **Evidence:** Most proposed connections between counters (e.g., `Γ_{cent, epigenetic}`) are based on indirect correlations or in vitro studies, rather than direct causal in vivo experiments.
 * **Consequence:** Current Γ values used in simulations are hypothetical. The canonical value `γ_i = 0` (independence) may often be more justified.

4. **Failure of preliminary χ_Ze tests.**
 * **Evidence:** Preliminary attempts to validate χ_Ze as an integrative biomarker in the MPI-LEMON, Dortmund Vital, and Cuban cohorts showed no predictive power exceeding standard clocks.
 * **Consequence:** Precludes the simple use of χ_Ze as a «sixth», integrative synchronization counter in the current version of MCOA. χ_Ze remains a theoretical construct.
 * **Source:** Report `internal/ze_validation_failures_2026-04.pdf` (available upon request).

## 4. Extension evidence (2026-05-10) — pending PubMed verification

*All references below are taken from draft manuscripts (Stem-Cell-Centric extension + Damage Shadow review) and **require verification via PubMed/Crossref** before inclusion in a submission-grade document. See rule `feedback_verify_references` and `feedback_deepseek_no_citations`.*

### 4.1. Evidence for independence of counter #5 (Proteostasis): VEXAS

| Claim | DOI/PMID | Article | Verified | Strength |
|-------|----------|--------|----------|----------|
| UBA1 (Met41) somatic mutation in HSC → bone marrow failure via UPR/senescence-like programs; telomeres **not shortened** → counter #5 rate-limiting independently of #2 | 10.1038/s41591-025-03623-9 | Molteni R. et al. Mechanisms of hematopoietic clonal dominance in VEXAS syndrome. *Nat Med*. 2025;31:1911–1924 | ⏳ pending | Strong (if confirmed) |
| Prevalence of VEXAS ≈ 1:4,000 in men >50 years; 50% 5-year mortality | (reference via Molteni 2025) | — | ⏳ pending | clinical |
| PLAG1 overexpression → 15.6× increase in functional HSC frequency via 4EBP1↑/miR-127↑ | 10.1182/blood.2021014602 | Keyvani Chahi A. et al. PLAG1 dampens protein synthesis to promote human HSC self-renewal. *Blood*. 2022;139(9):992-1008 | ⏳ pending | Strong |
| HSCs maintain low translation rates; increased translation without autophagy compensation → toxic aggregation | 10.1016/j.tcb.2025.06.006 | Catic A. Lessons in longevity from blood stem cells under protein stress. *Trends Cell Biol*. 2025 | ⏳ pending | Moderate |

### 4.2. Master-Counter Hypothesis — GrimAge meta-analysis

| Claim | DOI/PMID | Article | Verified | Strength |
|-------|----------|--------|----------|----------|
| GrimAge EAA ↔ frailty: β=0.11 (95% CI 0.06–0.15), N=10,371, I²=90.5% (cross-sectional, 8 studies) | 10.1016/S2666-7568(25)00128-2 | Tay J.H. et al. (Global Epigenetic Age Consortium). Biological age measured by DNAm clocks and frailty: SR+meta-analysis. *Lancet Healthy Longev*. 2025;6(10):100773 | ⏳ pending | Strong |
| GrimAge EAA longitudinal β=0.02 (95% CI 0.00–0.05); PhenoAge β=0.07; DunedinPACE β=0.10 | (ibid.) | — | ⏳ pending | Strong |
| GrimAge ↔ periodontitis OR=1.16 (95% CI 1.010–1.333), replicated in FinnGen + GLIDE | (via Zhang et al. *Clin Epigenet* 2025) | Zhang et al. *Clin Epigenet*. 2025 | ⏳ pending | Moderate |

### 4.3. Candidate counter #6 — piRNA

| Claim | DOI/PMID | Article | Verified | Strength |
|-------|----------|--------|----------|----------|
| Circulating piRNA → 2-year survival AUC 0.92 (Discovery), 0.87 (External Validation); 9 piRNA as therapeutic targets; **lower piRNA = longer survival**; surpasses >180 clinical measures | 10.1111/acel.70403 | Kraus V.B. et al. Select small non-coding RNAs are determinants of survival in older adults. *Aging Cell*. 2026;25(3):e70403 (Duke-EPESE, n=1,271 ≥71 years) | ⏳ pending | Strong (requires replication) |
| prg-1 mutation doubles lifespan of *C. elegans* via DAF-16/FOXO; reduced piRNA biogenesis → 2× lifespan | (Heestand et al.) | Heestand B. et al. *Aging Cell*. 2025 | ⏳ pending | Strong (model organism) |

### 4.4. Damage Shadow — partial reprogramming meta-analysis (PROSPERO CRD42026218473)

| Claim | DOI/PMID | Article | Verified | Strength |
|-------|----------|--------|----------|----------|
| Pooled correlation ΔDNAmAge ↔ Δfunction: r=0.09 (95% CI -0.14 to 0.32; p=0.44; I²=78%), 14 studies n=274 | (own meta-analysis, draft) | "Epigenomic Rejuvenation Without Functional Restoration" (NOT YET PUBLISHED, 2026-05-10) | ⏳ pending submission | Strong (own meta) |
| Threshold ΔDNAmAge ≈ -2.4 yrs-equiv before modest tissue-specific functional gain appears | (own) | (ibid.) | ⏳ pending | Strong (own meta) |
| Mesenchymal drift transcriptomic signature reversible via partial reprogramming prior to dedifferentiation | (Li & Tay 2026) | Li YY, Tay FR. The epigenetic rejuvenation promise. *Ageing Res Rev*. 2026;115:103009 | ⏳ pending | Moderate |
| Tissue-specific exceptions (refine, not refute systemic null): RGC (Lu 2020), engram neurons (Berdugo-Vega 2026) | 10.1038/s41586-020-2975-4 + (Berdugo-Vega *Neuron* 2026) | Lu Y. et al. *Nature*. 2020;588:124-129; Berdugo-Vega G. et al. *Neuron*. 2026;114(6):1102-1116.e7 | ⏳ pending | Strong (point-cases) |
| Publication bias detected: Egger p=0.04; trim-and-fill corrected SMD = 0.04 (NS) | (own) | (ibid.) | ⏳ pending | Strong (own meta) |

## v3 Update (2026-05-13)

See CONCEPT.md "v3" / "Address peer-review concerns" section for project-specific changes.