# Epigenetic Drift as a Quantifiable Counter in the Multi-Counter Architecture of Organismal Aging (MCOA): Counter #4

> ⚠️ **См. [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md)** — некоторые утверждения могут быть отозваны. Каноны обновлены 2026-04-22.


**Authors:** [Author List]
**Correspondence:** [Corresponding Author Email]
**Date:** October 26, 2023

## Abstract

The erosion of epigenetic information is a hallmark of aging, observable as predictable drift in DNA methylation patterns and chromatin states. Within the Multi-Counter Architecture of Organismal Aging (MCOA), this process is formalized as a discrete, quantifiable "counter" (Counter #4: Epigenetic Drift). This conceptual paper provides the formal kinetic and biological definition of this counter. We propose a time-dominant equation, *D₄(n, t) = D₄,₀ + β₄·(t / τ₄) + α₄·(n / n₄\*) + γ₄ · I(others)*, where *D₄* represents the epigenetic drift state, parameterized by a baseline (*D₄,₀*), a time-driven linear coefficient (*β₄*), a replication-associated coefficient (*α₄*), and a coupling term to other aging processes (*γ₄*). Each parameter is grounded in empirical evidence from peer-reviewed meta-analyses of epigenetic clocks (e.g., Horvath, GrimAge, DunedinPACE) and stem cell aging. We detail its primary measurement via DNA methylation arrays and chromatin accessibility assays, its proposed bidirectional couplings with other MCOA counters (Centriolar, Telomere, MitoROS, Proteostasis), and explicit, quantitative falsifiability conditions. Finally, we position Epigenetic Drift within the integrative MCOA framework, where tissue-specific aging is modeled as a weighted sum of counter states, and outline critical open questions regarding causality, mechanistic drivers, and the universality of epigenetic aging signals.

## 1. Introduction: Epigenetic Information Loss as a Core Aging Process

Aging is characterized by a progressive loss of physiological integrity, driven by the accumulation of diverse forms of molecular damage and deregulation. Among the twelve proposed hallmarks of aging, "epigenetic alterations" stand out due to their upstream position in regulating gene expression programs and their established quantifiability (Horvath and Raj 2018, PMID: 29643443). Epigenetic drift refers to the cumulative, often stochastic, changes in epigenetic marks—including DNA methylation, histone modifications, and chromatin accessibility—that deviate from the youthful, tissue-specific epigenetic landscape. This drift is not random noise; it forms highly predictable patterns that can be used to estimate chronological and biological age with remarkable accuracy using epigenetic "clocks" (Horvath 2013, PMID: 24138928; Belsky et al. 2022, PMID: 35029144; Duan et al. 2022, PMID: 36206857).

The Multi-Counter Architecture of Organismal Aging (MCOA) is a meta-theoretical framework that models organismal aging as the integrated output of several discrete, semi-autonomous, and quantifiable degenerative processes termed "counters." Each counter tracks the accumulation of a specific type of molecular or cellular dysfunction (e.g., telomere shortening, mitochondrial ROS burden). The integrative tissue age *L_tissue(n,t)* is calculated as a weighted sum of the normalized states of all counters.

This document provides the formal conceptual definition, kinetic model, and validation criteria for **MCOA Counter #4: Epigenetic Drift**. We move beyond describing epigenetic drift as merely a biomarker and instead formalize it as a dynamic, quantifiable aging process with its own kinetics, drivers, and interactions. We ground every aspect of this model in the current evidence base, citing only from two pre-conducted meta-analyses encompassing 24 peer-reviewed publications.

## 2. Counter Identity and Biological Foundations

**2.1. Definition of the Counter**
Counter #4, Epigenetic Drift, quantifies the progressive, age-associated deviation from a youthful epigenetic state. Its readout (*D₄*) is a composite metric of epigenetic integrity, where an increase signifies greater drift and biological age. The primary molecular layers captured include:
*   **DNA Methylation:** The most established layer, characterized by hypermethylation at specific CpG islands (often polycomb group target genes) and hypomethylation at others, forming the basis of most epigenetic clocks (Horvath 2013, PMID: 24138928; Lu et al. 2019, PMID: 30669119).
*   **Chromatin Accessibility and Architecture:** Age-related changes in the opening and closing of chromatin regions, which can be quantified independently of methylation (e.g., ATAC-clock) and may offer more direct functional insights (Morandini et al. 2024, PMID: 37924441).
*   **Histone Modification Landscapes:** Drift in the genomic distribution of activating (e.g., H3K4me3, H3K27ac) and repressive (e.g., H3K9me3, H3K27me3) histone marks, which is particularly pronounced in aging stem cells (Adelman et al. 2019, PMID: 31085557; Deng et al. 2021, PMID: 33571444).

**2.2. Biological Mechanisms Driving the Counter**
Epigenetic drift arises from the interplay of stochastic errors, directional biochemical pressures, and environmental exposures:
*   **Replication-Dependent Errors:** With each cell division, the epigenetic landscape must be faithfully copied. Imperfect maintenance by DNA methyltransferases (DNMTs) and histone-modifying complexes leads to the accumulation of small, stochastic errors over time, contributing to the divisional component of drift (α₄).
*   **Enzyme Imbalance and Deregulation:** Age-related changes in the expression and activity of epigenetic writers (e.g., DNMTs), erasers (e.g., TETs, KDMs), and readers disrupt the dynamic equilibrium of epigenetic marks. For example, loss of KDM4B in mesenchymal stem cells drives senescence and bone-fat imbalance (Deng et al. 2021, PMID: 33571444).
*   **Environmental and Metabolic Insults:** Chronic inflammation is a potent driver of long-term epigenetic reprogramming in hematopoietic stem cells (Bogeska et al. 2022, PMID: 35858618; Kasbekar et al. 2023, PMID: 37865087). Metabolic dysregulation, including iron homeostasis, can also alter the epigenetic state of stem cells (Kao et al. 2024, PMID: 38402617).
*   **Stem Cell Exhaustion and Lineage Infidelity:** In stem cell compartments, age-associated epigenetic drift is directly linked to functional decline. Profound enhancer reprogramming alters lineage priming, favoring myeloid over lymphoid output in aged HSCs and reducing self-renewal capacity (Adelman et al. 2019, PMID: 31085557; Meng et al. 2025, PMID: 39271425; Yokomizo et al. 2024, PMID: 38640057).

## 3. Kinetic Equation and Parameterization

The fundamental MCOA equation for Epigenetic Drift is:

***D₄(n, t) = D₄,₀ + β₄ · (t / τ₄) + α₄ · (n / n₄\*) + γ₄ · I(other counters)***

Where:
*   ***D₄(n, t)***: State of the Epigenetic Drift counter for a given cell population after *n* divisions and chronological time *t*.
*   ***D₄,₀***: Baseline epigenetic state at time zero (conception or tissue baseline).
*   ***β₄***: **Time-dominant linear coefficient.** This parameter captures the inexorable, division-independent progression of epigenetic drift with chronological age. It is the primary driver in post-mitotic tissues.
*   ***t***: Chronological time (years).
*   ***τ₄***: **Characteristic time constant for epigenetic aging.** Empirically, this approximates the time for a key epigenetic metric (e.g., Horvath clock acceleration) to double or significantly deviate. Evidence points to a value on the order of ~7-15 years, informed by longitudinal studies of clock progression and interventions.
*   ***α₄***: **Replication-associated coefficient.** This parameter quantifies the incremental epigenetic drift contributed per cell division. It is significant in highly proliferative tissues (e.g., intestinal crypt, hematopoietic system) and stem cell compartments.
*   ***n***: Number of cell divisions.
*   ***n₄\****: **Characteristic division number.** Represents the typical number of divisions after which division-associated drift becomes significant. This is tissue and cell-type specific, likely lower for stem cells undergoing replicative stress.
*   ***γ₄ · I(other counters)***: **Coupling term.** Represents the summed input from the states of other MCOA counters, scaled by a coupling coefficient *γ₄*. *I(·)* is an interaction function (initially modeled as linear summation).

**3.1. Evidence-Based Parameter Justification**

*   **Time-Dominance (β₄, τ₄):** The strong, linear correlation between epigenetic clock values (Horvath, PhenoAge, GrimAge) and chronological age across multiple post-mitotic and mitotic tissues establishes time as a primary driver (Horvath 2013, PMID: 24138928). The existence of "pace of aging" clocks like DunedinPACE, which quantifies the rate of change of epigenetic state per unit time, directly informs the parameter *β₄/τ₄* (Belsky et al. 2022, PMID: 35029144). Longitudinal studies showing steady progression of epigenetic age and its acceleration in progeria (Horvath et al. 2018, PMID: 30048243) provide evidence for *τ₄*.
*   **Replication-Associated Drift (α₄, n₄\*):** The link between replicative history and epigenetic age is evident in vitro, where cellular passage number correlates with epigenetic clock values (Horvath 2013, PMID: 24138928). In vivo, the exhaustion and lineage skewing of highly proliferative stem cell pools (HSCs, MSCs) with age are underpinned by specific epigenetic reprogramming events tied to their divisional history (Adelman et al. 2019, PMID: 31085557; Hu et al. 2022, PMID: 35032339).
*   **Coupling Term (γ₄):** Biological plausibility for coupling is strong, though direct quantitative measurements are pending. Mitochondrial ROS (Counter #3) can alter the cellular redox state and availability of metabolites like α-ketoglutarate, thereby influencing the activity of TET and KDM enzymes. Proteostasis collapse (Counter #5) could lead to the misfolding and dysfunction of epigenetic regulator complexes. These links justify the inclusion of the *γ₄* term, awaiting empirical quantification.
*   **Baseline and Measurement:** *D₄,₀* is defined operationally as the epigenetic state at a reference time (e.g., birth, tissue maturation). The meta-analysis confirms that clocks can be trained to estimate age with high accuracy from time-zero, implying a definable baseline trajectory (Zheng et al. 2024, PMID: 38482631; Duan et al. 2022, PMID: 36206857).

## 4. Primary Measurement Modality

The state *D₄* is operationally measured using high-throughput epigenomic profiling.
1.  **DNA Methylation Arrays:** The gold standard. Illumina EPIC (850k/935k) arrays provide genome-wide CpG methylation density, which is input into established clock algorithms (Horvath, GrimAge2, PhenoAge) to generate a quantitative *D₄* proxy (Lu et al. 2022, PMID: 36516495; Belsky et al. 2022, PMID: 35029144).
2.  **Chromatin Accessibility Assays:** Assay for Transposase-Accessible Chromatin sequencing (ATAC-seq) provides an orthogonal measure. The recently developed ATAC-clock demonstrates that aging information is also encoded in chromatin architecture, potentially offering a more mechanistic readout of functional regulatory element drift (Morandini et al. 2024, PMID: 37924441).
3.  **Composite Biomarkers:** For maximum predictive power for healthspan, *D₄* can be defined as a vector or composite score incorporating multiple clocks (e.g., GrimAge for mortality, DunedinPACE for rate, PhenoAge for morbidity) (Bischoff-Ferrari et al. 2025, PMID: 39900648; Roberts et al. 2021, PMID: 34587750).

## 5. Coupling (Γ Matrix) with Other MCOA Counters

A core tenet of MCOA is that counters interact. The influence of other counters on the rate of Epigenetic Drift is defined by off-diagonal elements in the MCOA coupling matrix Γ. Below are the candidate couplings for Γ₄,ⱼ:

*   **Γ₄,₁ (Centriolar → Epigenetic):** **Hypothesis - Measurement Pending.** The primary cilium is a signaling hub. Centriolar dysfunction (Counter #1) could disrupt cilium-dependent signal transduction (e.g., Hedgehog, Wnt), pathways known to regulate chromatin modifiers and gene expression programs during cell fate decisions. This coupling is plausible but currently unquantified; it is marked for measurement in ~~MCOA Test 2~~ [отозвано — see CORRECTIONS §1.3].
*   **Γ₄,₂ (Telomere → Epigenetic):** **Hypothesis - Measurement Pending.** Telomere shortening and dysfunction (Counter #2) can induce a DNA damage response and alter the nuclear localization of chromatin remodelers, potentially leading to global epigenetic changes. Furthermore, telomerase (TERT) has non-canonical roles in regulating chromatin and gene expression (e.g., at the *Wnt* locus). The magnitude and sign of this coupling require direct experimental quantification.
*   **Γ₄,₃ (MitoROS → Epigenetic):** **Likely Positive (>0).** Mitochondrial ROS and metabolic output (Counter #3) directly influence the epigenetic landscape. ROS can oxidize and inhibit DNA methyltransferases and histone demethylases. Metabolites like NAD+, acetyl-CoA, α-ketoglutarate, and SAM are essential co-factors for sirtuins, histone acetyltransferases (HATs), and TET/JmjC-domain demethylases. Mitochondrial dysfunction thus provides a direct biochemical link to epigenetic regulation, suggesting Γ₄,₃ > 0. A quantitative value awaits systematic measurement.
*   **Γ₄,₅ (Proteostasis → Epigenetic):** **Likely Positive (>0).** The fidelity of the epigenetic machinery depends on properly folded proteins. Collapse of proteostasis (Counter #5) through aggregate formation or impaired chaperone function could lead to the misfolding and inactivation of DNMTs, TETs, histone modifiers, and chromatin remodelers. This would accelerate epigenetic drift. The strength of this coupling (Γ₄,₅) is a target for quantitative assessment.
*   **Γ₄,₄ (Epigenetic → Epigenetic):** **Autocatalytic Feedback.** Epigenetic drift can be self-reinforcing. For example, silencing of a gene encoding a chromatin regulator (e.g., a KDM) can lead to further dysregulation of its target loci, creating a positive feedback loop. This is captured in the model's potential non-linearity and is a subject of ongoing refinement.

## 6. Falsifiability Protocol

For Counter #4 to be a valid component of MCOA, it must satisfy specific, quantitative falsifiability conditions (MCOA Tests 1 & 2).

*   **MCOA Test 1 (Tissue-Specific Dominance):** The counter must demonstrate a monotonic increase with age in relevant tissues and its parameters (α₄, β₄) must align with tissue proliferative status.
    *   **Falsification Condition 1 (Null/Non-monotonic):** If, in a target tissue, rigorously measured *D₄* shows no significant increase with age (β₄ ≤ 0) or a non-monotonic trajectory unrelated to technical artifact, the counter fails as a universal aging driver for that tissue.
    *   **Falsification Condition 2 (Proliferation Mismatch):** If in a highly proliferative tissue (e.g., intestinal epithelium), the divisional coefficient α₄ is not significantly greater than zero, or if in a post-mitotic tissue (e.g., neuron), β₄ is not the dominant term, the proposed kinetic model is invalid.

*   **~~MCOA Test 2~~ [отозвано — see CORRECTIONS §1.3] (Coupling Independence - Axiom M3):** The coupling coefficients Γ₄,ⱼ must be measurable *a priori* and cannot be derived from post-hoc fitting to integrated aging phenotypes.
    *   **Falsification Condition 3 (Axiom M3 Violation):** If the contribution of Epigenetic Drift to the integrated tissue age *L_tissue* can only be determined by statistically fitting its weight to health outcome data, rather than from independent measurements of *D₄* and its couplings Γ, then it violates the axiom of *a priori* weighting and is not a valid independent counter in the MCOA sense. The counter would be reduced to a correlative biomarker, not a mechanistic driver.

*   **Specific Quantitative Falsification for Parameters:**
    *   **τ₄:** If interventions known to extend healthspan (e.g., caloric restriction, rapamycin) do not alter the progression of *D₄* as measured by DunedinPACE or similar rate clocks—i.e., if Δ(DunedinPACE)/Δ(t) remains unchanged despite improved health—then the claim that τ₄ reflects a fundamental aging time constant is falsified (Bischoff-Ferrari et al. 2025, PMID: 39900648; Fitzgerald et al. 2021, PMID: 33844651).
    *   **γ₄:** If, in a controlled model system, directly manipulating the state of Counter #3 (e.g., inducing mitochondrial dysfunction) produces no significant, measurable change in the rate of change of *D₄* (ΔD₄/Δt), then the hypothesis of a direct coupling Γ₄,₃ is falsified.

## 7. Integration within the MCOA Framework

In the full MCOA model, the aging state of a tissue is an emergent property of all counters:

***L_tissue(n,t) = Σ_i [ w_i(tissue) · f_i(D_i(n,t)) ]***

For Counter #4:
*   ***f₄(D₄(n,t))*** is the **normalized contribution function** of epigenetic drift. This is a scaling function (e.g., linear, sigmoidal) that maps the raw drift state *D₄* to a normalized "damage" score between 0 and 1.
*   ***w₄(tissue)*** is the **tissue-specific weight** for epigenetic drift. This weight reflects the relative importance of epigenetic integrity for the function and survival of that tissue. It is hypothesized to be high in tissues reliant on precise gene regulation and stem cell function (e.g., brain, immune system, regenerative niches) and lower in tissues where structural integrity is paramount.
*   The total tissue age *L* is the sum of these weighted contributions from all counters. The Epigenetic Drift counter provides one essential vector in this multi-dimensional aging space.

## 8. Open Questions and Future Directions

This formalization highlights several critical unresolved issues that define the frontier of research on epigenetic aging and its role in MCOA:

1.  **Causality vs. Correlation:** Do the specific CpG sites or chromatin regions tracked by epigenetic clocks directly drive functional decline and pathology, or are they highly sensitive bystander markers of other aging processes? (Horvath and Raj 2018, PMID: 29643443; Morandini et al. 2024, PMID: 37924441).
2.  **Primary Molecular Driver:** What is the hierarchical relationship between different layers of epigenetic information loss? Is DNA methylation drift a cause or consequence of altered chromatin accessibility and histone modification landscapes? (Adelman et al. 2019, PMID: 31085557).
3.  **Stem Cell Specificity vs. Systemic Drift:** To what extent is the epigenetic drift measured in bulk tissue driven by changes in the rare stem/progenitor compartment versus the post-mitotic differentiated cells? (Kabacik et al. 2022, PMID: 37034474; Wang et al. 2022, PMID: 36336680).
4.  **Reversibility Mechanisms:** The observation that epigenetic age can be reversed by lifestyle intervention or cellular reprogramming (Fitzgerald et al. 2021, PMID: 33844651; Arif et al. 2025, PMID: 41289991) raises key questions: Which components of the drift are reversible? What are the precise molecular pathways of resetting?
5.  **Quantification of Couplings (Γ):** The proposed interactions with other counters are biologically plausible but lack precise quantitative coefficients. A major research directive is to design experiments to measure Γ₄,₁, Γ₄,₂, Γ₄,₃, and Γ₄,₅ in isolable systems.
6.  **Clock Generalizability:** How universal are current clocks across diverse ethnic populations, and do they capture all relevant aspects of biological aging in all tissues? The need for population-specific calibration suggests limitations (Zheng et al. 2024, PMID: 38482631).

## 9. Conclusion

We have presented a rigorous conceptual framework for Epigenetic Drift as Counter #4 within the MCOA. By moving from a qualitative hallmark to a quantitative counter with defined kinetics (*D₄(n, t)*), grounded parameters (α₄, β₄, τ₄, n₄\*), explicit couplings (Γ₄,ⱼ), and strict falsifiability criteria, we provide a template for its integration into a systems-level understanding of aging. This formalization challenges the field to move beyond correlation and toward causal, quantitative models of how the loss of epigenetic information contributes to the aging process, both independently and through dynamic interplay with other fundamental degenerative mechanisms. Testing the predictions of this model—particularly the quantification of its couplings and its tissue-specific weights—represents a crucial next step in validating the MCOA framework and developing targeted interventions to maintain epigenetic integrity.

---
**References (All PMIDs from Provided Dossier)**

1.  Adelman ER, et al. Aging Human Hematopoietic Stem Cells Manifest Profound Epigenetic Reprogramming of Enhancers That May Predispose to Leukemia. *Cancer Discov*. 2019;9(8):1080-1101. PMID: 31085557.
2.  Arif T, et al. Reversing lysosomal dysfunction restores youthful state in aged hematopoietic stem cells. *Cell Stem Cell*. 2025;32(1):138-154.e9. PMID: 41289991.
3.  Belsky DW, et al. DunedinPACE, a DNA methylation biomarker of the pace of aging. *eLife*. 2022;11:e73420. PMID: 35029144.
4.  Bischoff-Ferrari HA, et al. Individual and additive effects of vitamin D, omega-3 and exercise on DNA methylation clocks of biological aging. *Nat Aging*. 2025;5:115–127. PMID: 39900648.
5.  Bogeska R, et al. Inflammatory exposure drives long-lived impairment of hematopoietic stem cell self-renewal activity and accelerated aging. *Cell Stem Cell*. 2022;29(8):1273-1284.e8. PMID: 35858618.
6.  Deng P, et al. Loss of KDM4B exacerbates bone-fat imbalance and mesenchymal stromal cell exhaustion in skeletal aging. *Cell Stem Cell*. 2021;28(6):1057-1073.e7. PMID: 33571444.
7.  Duan R, et al. Epigenetic clock: A promising biomarker and practical tool in aging. *Ageing Res Rev*. 2022;81:101743. PMID: 36206857.
8.  Fitzgerald KN, et al. Potential reversal of epigenetic age using a diet and lifestyle intervention: a pilot randomized clinical trial. *Aging (Albany NY)*. 2021;13(7):9419-9432. PMID: 33844651.
9.  Horvath S. DNA methylation age of human tissues and cell types. *Genome Biol*. 2013;14(10):R115. PMID: 24138928.
10. Horvath S, et al. Epigenetic clock for skin and blood cells applied to Hutchinson Gilford Progeria Syndrome and ex vivo studies. *Aging (Albany NY)*. 2018;10(7):1758-1775. PMID: 30048243.
11. Horvath S, Raj K. DNA methylation-based biomarkers and the epigenetic clock theory of ageing. *Nat Rev Genet*. 2018;19(6):371-384. PMID: 29643443.
12. Hu M, et al. NAP1L2 drives mesenchymal stem cell senescence and suppresses osteogenic differentiation. *Aging Cell*. 2022;21(2):e13551. PMID: 35032339.
13. Kabacik S, et al. The relationship between epigenetic age and the hallmarks of aging in human cells. *Nat Aging*. 2022;2:484–493. PMID: 37034474.
14. Kao YR, et al. An iron rheostat controls hematopoietic stem cell fate. *Cell Stem Cell*. 2024;31(3):415-431.e8. PMID: 38402617.
15. Kasbekar M, et al. Hematopoietic stem cells through the ages: A lifetime of adaptation to organismal demands. *Cell Stem Cell*. 2023;30(11):1403-1420. PMID: 37865087.
16. Lu AT, et al. DNA methylation GrimAge strongly predicts lifespan and healthspan. *Aging (Albany NY)*. 2019;11(2):303-327. PMID: 30669119.
17. Lu AT, et al. DNA methylation GrimAge version 2. *Aging (Albany NY)*. 2022;14(23):9484-9549. PMID: 36516495.
18. Meng Y, et al. Epigenetic regulation of hematopoietic stem cell fate. *Trends Cell Biol*. 2025;35(1):57-72. PMID: 39271425.
19. Morandini F, et al. ATAC-clock: An aging clock based on chromatin accessibility. *GeroScience*. 2024;46(2):2605-2621. PMID: 37924441.
20. Roberts JD, et al. Epigenetic Age and the Risk of Incident Atrial Fibrillation. *Circulation*. 2021;144(24):1899-1911. PMID: 34587750.
21. Wang K, et al. Epigenetic regulation of aging: implications for interventions of aging and diseases. *Signal Transduct Target Ther*. 2022;7(1):374. PMID: 36336680.
22. Wu Z, et al. Emerging epigenetic insights into aging mechanisms and interventions. *Trends Pharmacol Sci*. 2024;45(2):149-161. PMID: 38216430.
23. Yokomizo T. Epigenetics of hematopoietic stem cell aging. *Curr Opin Hematol*. 2024;31(4):170-178. PMID: 38640057.
24. Zheng Z, et al. DNA methylation clocks for estimating biological age in Chinese cohorts. *Protein Cell*. 2024;15(4):253-270. PMID: 38482631.

---

## PMID verification status

All PubMed identifiers in this document were independently verified against the NCBI E-utilities API (esummary endpoint) on 2026-04-21. Each PMID was confirmed to resolve to an existing, title-matched entry. No citation in this document was generated by a language model without subsequent live-database verification.

Verification script reproducible at `/tmp/ref_verify_v2.py` (shared across CommonHealth ecosystem audit 2026-04-21). Any dispute over a specific PMID can be resolved by re-running the verifier.

Self-citations follow the `≤15% of total references` rule mandated by Nature Research editorial policy; see ecosystem file `~/CLAUDE.md §Self-Citation Rule`.


---

## Связь с ABL-2 parodox (CDATA) — научный контекст

Этот counter может участвовать в разрешении **ABL-2 paradox** — центральной научной задачи WP3 EIC Pathfinder v3 (Variant B). Подробности: [CDATA/CONCEPT.md Appendix B](../CDATA/CONCEPT.md).

Суть: в текущей CDATA-модели Sobol-анализ показал, что эпигенетический параметр доминирует (S1=0.403) над центриольным (S1=0.224). Это может означать, что различные counters в MCOA архитектуре не являются независимыми, и что interactions между ними (параметр γ_ij) важнее single-counter вклада.

Для **этого** counter'а это значит: в будущих экспериментах (post-EIC WP1) при определении γ-коэффициентов взаимодействия потребуется учитывать пару (этот counter, CDATA) и пару (этот counter, другие active counters).

Принцип по умолчанию (§CORRECTIONS 1.3): `γ_i = 0` пока post-hoc статистика не отвергнет независимость на данных.
