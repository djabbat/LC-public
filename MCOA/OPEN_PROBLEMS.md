<!-- AUTO-TRANSLATED via DeepSeek 2026-05-13. Source language: russian. Original preserved at OPEN_PROBLEMS.ru.md. -->

# Open Problems and Falsifiable Tests for MCOA

*Version: 2026-04-22. Each problem contains a specific falsifiable test with clear outcomes and priority.*

## Problem 1: Defining *A Priori* Tissue Weights `w_i(tissue)`

**Power analysis for Test 1A:** Placeholder — effect size δ = 0.3, α = 0.05, power = 0.80, required N = 1875 (to be specified in pre-registration). For Tests 2A and 3A: similar placeholder values TBD. For Test 4A: N_replicates = 1000.

**Description:** Axiom M3 requires that the weight of each counter in a tissue be determined prior to model fitting, based on independent biological knowledge. Currently, there is no generally accepted, quantitative method for predicting `w_i`. The use of heuristics or placeholder values undermines falsifiability.

**Priority:** **High** (P0). Blocks full experimental validation of MCOA.

### Falsifiable Test 1A: Prediction of Tissue Hierarchy of Counters

**Hypothesis:** Based on a combination of RNA-seq data (expression of genes related to counters), in vivo cell division rate measurements, and metabolomics, a predictive model for `w_i` can be constructed that will correlate with the experimentally measured importance of each counter for the tissue's aging phenotype.

**Experiment:**
1.  **Sample:** 5-7 different mouse tissues (e.g., liver, skin, intestine, brain, skeletal muscle, spleen, fat).
2.  **Prediction:** For each tissue, compute `w_i_pred` based on:
    *   Expression levels of key genes (e.g., TERT for telomeres, TTLL/CCP for CP, oxidative stress markers for mito).
    *   In vivo proliferation rate estimates (e.g., via EdU).
    *   Metabolomics data (NAD+/NADH, ATP/ADP).
3.  **Measurement:** Conduct a longitudinal study (3 age points) for the same tissues. Quantitatively measure the age-related change for each counter (telomere shortening via qFISH, CP level via GT335, 8-OHdG, epigenetic age via clock CpGs).
4.  **Criterion:** Calculate the correlation between the predicted weight `w_i_pred` and the measured proportion of age-related phenotypic variance explained by that counter in that tissue.

**Possible Outcomes:**
1.  **✅ Strong correlation (R² > 0.7):** Confirms the possibility of *a priori* weight prediction. MCOA passes this test.
2.  **⚠️ Moderate correlation (0.3 < R² < 0.7):** Indicates partial predictive power of the method. Refinement of the weight prediction model is required (e.g., adding new parameters).
3.  **❌ Weak/absent correlation (R² < 0.3):** Falsifies the specific method for predicting `w_i`. Calls into question the practical feasibility of Axiom M3 in its current formulation. Requires searching for fundamentally different ways of *a priori* fixing weights.
4.  **🔀 Contradictory result:** Different prediction methods yield widely varying `w_i_pred`. Indicates fundamental uncertainty in the selection of *a priori* parameters, which weakens the entire MCOA framework.

## Problem 2: Resolving the ABL-2 Paradox and Positioning the CP Counter

**Description:** The high correlation of ABL-2 protein levels with epigenetic age in CDATA data questions the causal relationship. Is centriolar polyglutamylation (CP) an upstream driver of aging, a downstream effect, or a parallel process?

**Priority:** **High** (P1). Concerns the validity of selecting CP as counter #1.

### Falsifiable Test 2A: Causal Intervention in the CP Pathway

**Hypothesis:** If CP is an upstream driver, then experimental inhibition of polyglutamylation enzymes (TTLL) or activation of deglutamylating enzymes (CCP) in young animals should slow the accumulation of damage in other counters (telomeres, epigenetic drift) and delay age-related phenotypes.

**Experiment:**
1.  **Model:** Mice with conditional knockout/inhibition of a key TTLL enzyme in intestinal epithelium (high proliferation) or hepatocytes (low proliferation). Control — wild-type mice.
2.  **Intervention:** Activation of knockout/start of inhibitor administration at 6 months of age.
3.  **Measurements (at 12 and 18 months):**
    *   **CP counter:** Tubulin polyglutamylation level (mass spectrometry).
    *   **Other counters:** Telomere length (qFISH), DNA methylation (clock CpGs), mitochondrial function markers.
    *   **Phenotype:** Histology for fibrosis/inflammation, tissue functional tests.
4.  **Criterion:** Comparison of the rate of change of other counters and the severity of phenotypes between the experimental and control groups.

**Possible Outcomes:**
1.  **✅ Slowing of all counters and phenotypes:** Strongly supports an upstream role for CP. MCOA with CP #1 is confirmed.
2.  **⚠️ Slowing of CP only, but not other counters:** Indicates that CP is a parallel, but not causal, process. Requires revision of its position in the hierarchy.
3.  **❌ No effect or acceleration of aging:** Falsifies the hypothesis of CP as a significant driver of aging in this tissue. May indicate compensatory mechanisms or an error in target selection.
4.  **🔀 Tissue-specific effect:** Slowing observed only in the highly proliferative intestinal epithelium, but not in the liver. Confirms a context-dependent role for CP, which is consistent with MCOA but complicates the overall theory.

## Problem 3: Measuring the Connection Matrix Γ and Testing the Hypothesis `γ_i = 0`

**Description:** By default, MCOA assumes `γ_i = 0` (counter independence hypothesis). Non-zero connections must be proven. There is no standardized in vivo protocol for directly measuring the elements `Γ_{ij}`.

**Priority:** **Medium** (P2). Critical for predicting intervention synergy.

### Falsifiable Test 3A: MCOA Test 2 Protocol (in vitro calibration)

**Hypothesis:** Under controlled cell culture conditions, by manipulating one counter (e.g., inducing oxidative stress in mitochondria), one can quantitatively measure the acceleration of damage accumulation in another counter (e.g., telomere shortening rate).

**Experiment:**
1.  **System:** Primary human fibroblasts or mesenchymal stem cells.
2.  **Intervention:** Treatment with low doses of rotenone (mitochondrial stress) or direct generation of ROS in mitochondria (optogenetics).
3.  **Measurements:** Parallel, longitudinal monitoring of:
    *   Mitochondrial ROS level (MitoSOX).
    *   Telomere length (qFISH or flow-FISH) at each passage.
    *   (Optional) CP level, epigenetic change markers.
4.  **Analysis:** Compare the telomere shortening rate (Δtelomeres/passage) under stress and control conditions. Calculate `Γ_{telomere, mito}` as the ratio of these rates.

**Possible Outcomes:**
1.  **✅ `Γ_{telomere, mito}` significantly > 1:** Confirms the existence of a connection. Allows establishing a numerical value for this matrix element.
2.  **✅ `Γ_{telomere, mito}` ≈ 1:** Does not reject the independence hypothesis (`γ_i = 0`) for this pair in this system.
3.  **❌ `Γ_{telomere, mito}` < 1 (slowing):** Falsifies the commonly assumed direction of the connection. Indicates a possible protective effect of mild stress or a system artifact.
4.  **🔀 Non-linear dependence:** The effect is observed only after a certain threshold of mitochondrial damage. Would require modification of the linear connection model in MCOA to a threshold or sigmoidal model.

## Problem 4: Operationalizing "Load" `L_tissue` and its Threshold

**Description:** The equation `L_tissue = Σ_i [ w_i · f_i(D_i) ]` and the condition `L_tissue > L_critical` are abstractions. It is unclear which specific biological quantity (cell mortality, secretory phenotype, functional decline) should be mapped onto `L_tissue` and how to measure `L_critical`.

**Priority:** **Medium** (P2). Necessary for quantitative predictions.

### Falsifiable Test 4A: Linking Computed `L_tissue` to the Replicative Limit in vitro

**Hypothesis:** In a fibroblast culture where the telomere counter dominates, the MCOA-computed load `L_tissue` will increase monotonically with passages and will reach a constant value `L_critical` at the moment of entry into senescence (Hayflick limit).

**Experiment:**
1.  **System:** Culture of human fibroblasts from a young donor.
2.  **Measurements at each passage:**
    *   Telomere length (primary counter, `w ≈ 1`).
    *   CP level, oxidative stress markers (secondary counters, `w` small).
    *   Senescence marker (SA-β-Gal).
3.  **Calculation:** Compute `L_tissue(passage)` using a simplified model (functions `f_i` — linear).
4.  **Calibration:** Determine `L_critical` as the value of `L_tissue` at the passage when >70% of cells become SA-β-Gal+.

**Possible Outcomes:**
1.  **✅ Clear threshold:** `L_tissue` increases smoothly and stabilizes around `L_critical` at senescence. The `L_critical` value is consistent across replicates.
2.  **⚠️ Threshold with large scatter:** `L_critical` varies between lines or experiments. Indicates additional hidden variables or noise.
3.  **❌ Absence of threshold/correlation:** `L_tissue` does not correlate with entry into senescence. Falsifies the simple linear additive model for `L_tissue` in this system.
4.  **🔀 `L_tissue` reaches `L_critical` before senescence:** Predicts senescence earlier than it occurs. May indicate that the threshold is stochastic or that additional events are required.

## Problem 5: Validation of Candidate Counter #6 (piRNA) in Mammalian Non-Germline Tissue

*Added 2026-05-10 in connection with the Stem-Cell-Centric extension manuscript.*

**Description:** Kraus et al. *Aging Cell* 2026 showed the predictive value of 9 circulating piRNAs for 2-year survival (AUC 0.92, n=1,271 ≥71 years, Duke-EPESE). Heestand et al. 2025 demonstrated in *C. elegans* a lifespan doubling via prg-1/DAF-16. However, **mammalian non-germline piRNA biology is poorly characterized**, which blocks the inclusion of #6 in the canonical counter set.

**Priority:** **Medium** (P2). Blocking issue for the canonization of #6.

### Falsifiable Test 5A: Knockdown of 9 Identified piRNAs in an Aged Mouse Model

**Hypothesis:** Antisense oligonucleotides against the 9 piRNAs identified by Kraus et al., upon systemic administration to old mice (≥18 mo), will increase:
1.  survival (primary endpoint, log-rank vs scrambled-AON)
2.  functional measures (frailty index, GTT, grip strength)
3.  reduce GrimAge EAA (mouse-equivalent clock)

**Possible Outcomes:**
1.  **✅ ↑survival + ↓EAA + ↑function:** piRNA counter #6 transitions to the canonical set; added to PARAMETERS.md.
2.  **⚠️ ↑survival, but without ↓EAA:** counter #6 acts through a damage-shadow-independent pathway; revision of the hierarchical model (THEORY §4.4).
3.  **❌ No effect:** Kraus 2026 — biomarker, not driver; counter #6 remains a placeholder without promotion.
4.  **🔀 Tissue-dependent:** requires tissue-specific weighting `w_pi(tissue)` — extends MCOA scope.

## Problem 6: Operationalization of Damage Shadow and Quantitative Dissociation of Epigenetic vs. Structural Rejuvenation

*Added 2026-05-10 in connection with the Damage Shadow systematic review (PROSPERO CRD42026218473).*

**Description:** Meta-analysis of 14 studies n=274 mice showed a disconnect between ΔDNAmAge and Δfunction (r=0.09, p=0.44). The hierarchical model (THEORY §4.4) postulates a division into 4 levels: transcriptomics > epigenomics > structural damage shadow > systemic physiology. However, the **operational measurement** of the structural damage shadow is not standardized.

**Priority:** **High** (P1). Blocks the valid use of DNAmAge as a surrogate endpoint in MCOA-driven trials.

### Falsifiable Test 6A: Composite Damage-Shadow Biomarker

**Hypothesis:** A composite marker D_shadow = w_AGE · [pentosidine] + w_mtDNA · heteroplasmy_fraction + w_aggreg · [insoluble_protein] + w_lipo · lipofuscin_AUF — upon partial reprogramming **does not decrease** statistically significantly, in contrast to ΔDNAmAge.

**Possible Outcomes:**
1.  **✅ ΔD_shadow ≈ 0 with ΔDNAmAge < 0:** confirms damage shadow as an epigenetically-independent counter group; mandates parallel functional + structural endpoints in MCOA trials.
2.  **⚠️ Partial decrease in ΔD_shadow:** some components (e.g., mtDNA heteroplasmy) are reset via clonal selection; refine model.
3.  **❌ ΔD_shadow correlates with ΔDNAmAge:** refutes the hierarchical model; DNAmAge may be a valid surrogate for D_shadow.
4.  **🔀 Tissue-specific:** in highly plastic populations (RGC, engram neurons) ΔD_shadow < 0; in systemic — ≈ 0; confirms the tissue-specific formulation (Lu 2020 / Berdugo-Vega 2026 reconciliation).

### Connection to Axiom M3 (a-priori weights)

D_shadow operationalization requires *a priori* weights `w_AGE`, `w_mtDNA`, `w_aggreg`, `w_lipo` for each tissue. This extends Test 1A (Problem 1) to the structural component and must be pre-registered simultaneously.