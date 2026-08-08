# A Proposed Experimental Protocol for Testing Centriole-Mediated Somatic Totipotency Induction

Jaba Tqemaladze — Georgia Longevity Alliance
Correspondence: jaba@longevity.ge | ORCID: 0000-0001-8651-7243

---

## 1. Background and Rationale

This protocol operationalizes the hypothesis that the centriole functions as a physical ratchet locking somatic cells in a differentiated state (Tqemaladze, 2026a). The companion Hypothesis article proposes that centriole elimination followed by totipotency factor expression and de novo centriole assembly — Eliminate, Reprogram, Rebuild — may produce sustained totipotency from somatic cells. The central experiment — combining centriole elimination with DUX4/TPRX1 expression in fibroblasts — has not been reported (P. Gönczy, EPFL, pers. comm., July 2026).

**Natural precedent.** Oocytes eliminate centrioles during oogenesis in all metazoans examined (Manandhar et al., 2005). Sperm contributes centriole seeds for de novo assembly in the zygote (Avidor-Reiss et al., 2019; Fishman et al., 2018). Planarians, among the most plastic animals, lost centrosomes from dividing cells during evolution; centrioles appear only de novo in terminally differentiated cells (Azimzadeh et al., 2012).

**Key motivating observations.** 8CLCs and 2CLCs are transient, reverting within one to two divisions (Hendrickson et al., 2017). Recent work identifies *intermediate* 8CLC states expressing only partial ZGA markers (e.g., TPRX1 without LEUTX), distinct from mature 8CLCs with the full program — a distinction relevant to protocol endpoints. Centriole elimination in hPSCs triggers p53-dependent and p53-independent differentiation (Renzova et al., 2018) — but this outcome may be context-dependent.

**Critical caveats.** Centrinone causes reversible p53-dependent G1 arrest in normal diploid fibroblasts but not in cancer cells (Wong et al., 2015). p53 protects against genome instability following centriole duplication failure (Lambrus et al., 2015) — p53 blockade during Phase 1-2 therefore creates a window of vulnerability that must be monitored by karyotyping at each passage. Separately, centriolar satellite protein CEP131 participates in mitochondrial apoptosis independently of p53 (Renaud et al., 2023). Mammalian cells lack checkpoints for aberrant centrosome number (Wong & Stearns, 2005), meaning acentriolar cells may bypass G1 arrest under certain conditions — a double-edged sword that could either enable the protocol or permit genomically unstable cells to proliferate.

---

## 2. Experimental Design

### 2.1 Cell System and Phase 0 Characterization

**Primary cells.** Human dermal fibroblasts (HDF, neonatal or adult, passages 3–8). Mouse dermal fibroblasts for tetraploid complementation (Phase 2, conditional on Phase 1 success).

**Phase 0 — Centriole characterization (mandatory preliminary work).** Before any elimination experiment, the fibroblast centriole proteome must be characterized:
- Mass spectrometry of isolated centrosome fractions to quantify polyglutamylation (GT335 immunoblot), carbonylation (anti-DNPH), and the full CAMC composition
- Ultrastructure expansion microscopy (U-ExM; Gambarotto et al., 2019) for baseline centriole morphology and centriolar protein copy numbers
- TRIM37 expression level by qPCR and western blot — critical for predicting PLK4 PROTAC and inhibitor sensitivity (Sun et al., 2023)
- Baseline karyotype (SKY/FISH) to establish chromosomal stability before interventions

### 2.2 Experimental Groups (n = 15 biological replicates per group for centriole elimination arms; n = 10 for control arms)

*Rationale for increased n: centriole elimination groups are expected to show higher within-group variability due to heterogeneous elimination efficiency and variable p53/p38 stress responses. n = 15 provides 80% power for Cohen's d = 0.7 at α = 0.004.*

| Group | Centriole status | Factors | Purpose |
|---|---|---|---|
| A | Intact | — | Baseline |
| B | Intact | OSKM (CytoTune 2.0) | Classical iPSC |
| C | Eliminated (Plk4 siRNA) | OSKM | Enhanced pluripotency |
| D | Eliminated (centrinone/RP-1664) | OSKM | Pharmacological replicate |
| E | Intact | DUX4 + TPRX1 | 8CLC induction, centrioles intact |
| F | Eliminated (Plk4 siRNA) | DUX4 + TPRX1 | **Primary test: sustained totipotency** |
| G | Eliminated (centrinone/RP-1664) | DUX4 + TPRX1 | Pharmacological replicate |
| H | Eliminated (Plk4 siRNA) | DUX4 + TPRX1 + ZSCAN4 | Enhanced totipotency |
| I | Eliminated (centrinone/RP-1664) | DUX4 + TPRX1 + ZSCAN4 | Pharmacological replicate |
| J | Intact | OSKM + DUX4 + TPRX1 | Does totipotency require hardware reset? |
| K | Eliminated (PCM disruption) | DUX4 + TPRX1 | Physiological elimination |
| L | Eliminated (autophagy induction) | DUX4 + TPRX1 | Alternative pathway |
| M | Eliminated + 53BP1/USP28 siRNA | DUX4 + TPRX1 | p53-independent arrest bypass (Fong et al., 2016; Meitinger et al., 2016) |
| N | Eliminated (centrinone) | OSKM (no p53 inhibitor) | Direct comparison with Renzova et al. (2018) |
| O | Eliminated | Chemical 8CLC induction | Taubenschmid-Stowers et al. (2022) protocol |
| Q | **Eliminated + PLK4 rescue (no factors)** | — | **Placebo-elimination: does PLK4 re-expression reverse the acentriolar phenotype?** |
| R | **Eliminated (no factors, long-term)** | — | **Fate of acentriolar fibroblasts without reprogramming stimuli** |
| S | **Intact + DUX4/TPRX1 + ROCK inhibitor** | DUX4 + TPRX1 | **Does ROCK inhibitor alone affect plasticity?** |

---

## 3. Methods

### 3.1 Centriole Elimination

**Method A — Plk4 siRNA.** Lipofectamine RNAiMAX, 20 nM Plk4 siRNA, 72-hour treatment refreshed at 48 hours. Confirmation by CP110 and Cep135 immunofluorescence. Target: > 90% acentriolar cells.

**Method B — PLK4 inhibitor RP-1664.** RP-1664 at 100–500 nM for 6 days with dose-response curve. **Critical:** PLK4 inhibitors show a paradoxical dose-response — low concentrations cause centriole *amplification*, not loss, and this effect is cell-type dependent (Wong et al., 2015; Soria-Bretones et al., 2025). A full titration curve (1 nM–10 μM) with centriole counting at each concentration is mandatory in Phase 0 to identify the window of true elimination versus amplification. RP-1664 is validated in TRIM37-amplified neuroblastoma; tissue-specific effects are documented — centrinone does not affect keratinocyte proliferation in certain models (Jaiswal et al., 2025). RP-1664 efficacy in normal diploid fibroblasts is unknown. Phase 0 must include direct comparison of RP-1664 versus CRISPR PLK4 knockout (Method C). A structurally distinct PLK4 inhibitor (CFI-400945) serves as an off-target control.

**Method C — CRISPR/Cas9 PLK4 knockout (genetic gold standard).** Lentiviral delivery of Cas9 plus PLK4-targeting sgRNA for complete, permanent PLK4 knockout. Used to validate pharmacological methods and distinguish on-target from off-target effects.

**Method K — PCM disruption (physiological).** siRNA against PCM components PCNT (pericentrin) or CDK5RAP2, mimicking the natural Drosophila oocyte program of PCM disassembly leading to centriole destabilization (Pimenta-Marques et al., 2016).

**Method L — Autophagy-mediated elimination.** Rapamycin (200 nM) plus siRNA against SAS-6 or STIL. Verify LC3 colocalization with CP110 by immunofluorescence.

**Method M — 53BP1/USP28 bypass.** siRNA against 53BP1 and USP28 to block the p53-dependent G1 arrest triggered by centriole loss (Fong et al., 2016; Meitinger et al., 2016). **Critical nuance:** 53BP1/USP28 also mediate p53 activation after extended mitotic duration, not solely centrosome loss. Centriole elimination prolongs mitosis; this can activate p53 through the same pathway independently of centriole status. Three strategies must be compared: (i) 53BP1+USP28 siRNA, (ii) TRIM37 siRNA (TRIM37-KO cells form ectopic centrosomal foci that suppress mitotic defects), (iii) inducible dominant-negative p53 (R175H). Only if all three yield consistent results can the effect be attributed specifically to centriole loss. **Mandatory pilot:** all three strategies must be tested in fibroblasts under centrinone treatment. If cells fail to exit G1 arrest, the protocol requires redesign.

*Note on RP-1664 specificity.* RP-1664 is validated in TRIM37-amplified neuroblastoma (Soria-Bretones et al., 2025). PLK4 inhibitors show tissue-specific effects — centrinone does not affect keratinocyte proliferation in certain models (Jaiswal et al., 2025). RP-1664 efficacy in normal diploid fibroblasts is unknown. Phase 0 must include direct comparison of RP-1664 versus CRISPR PLK4 knockout (Method C). If RP-1664 fails to induce centriole loss in fibroblasts, CRISPR KO becomes the primary method. An additional control with a structurally distinct PLK4 inhibitor (CFI-400945) should assess off-target effects.

### 3.2 Totipotency Factor Delivery

Non-integrating, doxycycline-inducible lentivirus (Tet-On 3G) for DUX4, TPRX1, and ZSCAN4. Tet-On control is essential: sustained high-level DUX4 causes apoptosis through the FSHD pathological mechanism (Lemmers et al., 2010). Doxycycline at 0.1–2.0 μg/mL, titrated per factor. For Group P, spermine or spermidine (50–200 μM) is added for Z-DNA induction (Shajahan et al., 2025).

### 3.3 De Novo Centriole Assembly (Phase 3 — Rebuild)

Re-expression of core biogenesis factors: PLK4 (master kinase), STIL (scaffold recruiting SAS-6), SAS-6 (cartwheel — first structural element), and CPAP (microtubule growth from cartwheel) (Nigg & Holland, 2018). Inducible, time-limited expression systems (degron-tagged PLK4) are used to prevent supernumerary centriole formation. Dzhindzhev et al. (2019) demonstrated that STIL S428 phosphorylation is required for CPAP recruitment — this phospho-event must be confirmed during Rebuild. **Natural precedent:** Drosophila eggs can form centrioles de novo during parthenogenesis without sperm contribution (Riparbelli & Callaini, 2003), demonstrating that the cytoplasmic environment of a totipotent cell is competent for spontaneous centriole assembly.

### 3.4 Culture Conditions

Fibroblast medium (DMEM + 10% FBS) days 0–6. Transition to 2C-like medium: DMEM/F12 + 20% KSR + LIF (1000 U/mL) + 2i (PD0325901 1 μM + CHIR99021 3 μM). Oxygen tension: 5% O₂. ROCK inhibitor (Y-27632, 10 μM) maintained throughout the experiment for centriole elimination groups — acentriolar cells are mechanically fragile and require continuous ROCK inhibition for viability.

---

## 4. Timeline

| Day | Action |
|---|---|
| 0 | Seed HDFs (2 × 10⁵ cells/well, 6-well plate) |
| 1 | Plk4 siRNA transfection or RP-1664 treatment |
| 3 | siRNA refresh |
| 4 | Passage cells 1:3 (dilute centrioles) |
| 7 | Confirm centriole elimination by immunofluorescence |
| 8 | Tet-On lentivirus transduction (DUX4, TPRX1 ± ZSCAN4) |
| 9 | Medium change (remove virus) |
| 10–30 | Monitor colony formation and morphology |
| 14 | First assessment: MERVL/HERVL qPCR |
| 21 | Alkaline phosphatase staining |
| 28 | Full characterization (see §5) |

*Monitoring is intensified during the first 7 days: imaging and qPCR every 12 hours to capture the dynamic window of centriole loss and MERVL activation.*

---

## 5. Readouts

### 5.1 Primary Endpoints

| Marker | Method | Expected | Time point |
|---|---|---|---|
| MERVL/HERVL | RT-qPCR (mouse: MERVL; human: HERVL, MLT2A1) | ≥ 10-fold over Group E | Days 14, 21, 28 |
| ZSCAN4 | IF + qPCR | > 50% cells positive | Days 14–21 |
| ZGA panel (≥ 5 markers) | qPCR: TPRX1, LEUTX, DPPA3, ZFP352, DUXA | Coordinated activation | Days 14–21 |
| 2C/8C-like morphology | Live imaging | Enlarged nucleus, reduced heterochromatin | Days 10–28 |

**Discriminating stable totipotency from cyclic switching.** 8CLCs and 2CLCs exhibit cyclic activation — cells enter and exit the totipotent-like state stochastically. Stable totipotency requires: (a) > 50% of cells expressing the *full* ZGA panel (TPRX1+LEUTX+ZSCAN4+MLT2A1 for human; MERVL+ZSCAN4+DPPA3 for mouse) at any given time, (b) < 10% of cells expressing SOX2, and (c) maintenance of this profile for ≥ 10 passages without antibiotic selection. **Important caveat:** the 8-cell embryo has already lost strict totipotency — only the zygote and 2-cell embryo are truly totipotent in mouse (Posfai et al., 2021). The 8CLC state is therefore a *totipotent-like* state, not totipotency per se. The protocol's endpoint is best described as "stable 8CLC-like state from somatic cells" — a previously unachieved milestone that would constitute strong evidence for the hardware-reset model even without full totipotency.

### 5.2 Secondary Endpoints

| Assay | Method | Expected | Time point |
|---|---|---|---|
| Trophectoderm differentiation | BMP4 (50 ng/mL, 4 days) → CDX2+, GATA3+, KRT7+ | Positive for totipotency groups | Day 35 |
| Embryoid bodies | Suspension, 7 days → 3 germ layers + trophoblast | Totipotency groups: + trophoblast | Day 35 |
| Teratoma assay | Subcutaneous injection, NSG mice, 8 weeks | Totipotency: + trophoblast | Week 8 |
| Chimera contribution | 8-cell mouse embryo injection | Contribution to ICM + TE | Weeks 3–4 |
| Tetraploid complementation | 4n host + totipotent donor cells | Full organism = true totipotency | Weeks 4–6 |

*Note on tetraploid complementation.* Paim & FitzHarris (2019) demonstrated that tetraploidy causes chromosomal instability in acentriolar mouse embryos independently of centriole status. Furthermore, tetraploid complementation tests pluripotency, not totipotency (Posfai et al., 2021). To establish totipotency, the donor cell must contribute to trophectoderm. An intermediate chimera assay tracking both ICM and TE contribution, with karyotype verification, is therefore required before tetraploid complementation. Single-cell embryo injection with lineage tracing provides a more direct readout of totipotency.

### 5.3 Molecular Characterization

- **RNA-seq:** Days 0, 7, 14, 21, 28 with 3 technical replicates per group. Compare to published human embryo transcriptomes (GSE178379 and equivalent datasets). FDR < 0.01 with subsequent validation by qPCR for all differentially expressed genes used in downstream analysis.
- **ATAC-seq:** Days 0, 14, 28
- **DNA methylation:** EPIC arrays (850K CpG) Days 0, 28
- **Single-cell RNA-seq:** 4 time points × 2 conditions (Groups E and F), capturing donor variability through inclusion of at least 2 independent donors
- **Hi-C:** Day 28 for Groups A, E, and F to assess 3D chromatin reorganization
- **ChIP-seq:** H3K4me1, H3K27ac, H3K9me3 on Days 0, 7, 14, 28 to track enhancer activation and repressive mark erasure during the somatic-to-totipotent transition
- **MERVL-GFP reporter line:** Live FACS-based tracking of MERVL-positive cells

### 5.4 Centriole Tracking

- Centrin1-GFP lentivirus — live-cell imaging across all phases
- U-ExM — structural analysis of centriole morphology and copy number
- GT335 antibody — polyglutamylation quantification before and after elimination
- Anti-DNPH — protein carbonylation on centrosome fractions
- **Mass spectrometry proteomics** of centrosome fractions at baseline and after elimination (Phase 0 and Day 7)

---

## 6. Critical Controls

| Control | Method | Purpose |
|---|---|---|
| p53/p38 inhibition | Pifithrin-α (20 μM) + SB203580 (10 μM), days 3–8 | Prevent centriole loss-induced arrest (Uetake et al., 2007; Mikule et al., 2007) |
| **53BP1/USP28 knockdown** | siRNA against 53BP1 + USP28 | **Specific bypass of centrosome-loss p53 arrest (Fong et al., 2016)** |
| **Inducible dominant-negative p53** | DOX-inducible dn-p53 (R175H or R273H) | **Cleaner alternative to pifithrin-α for long-term experiments** |
| PLK4 rescue | PLK4 cDNA re-expression (DOX-inducible) | Should abrogate totipotency — confirms causality |
| Cilium-specific | IFT88 shRNA (blocks ciliogenesis, spares centriole) | Should not enable totipotency |
| p53 rescue | p53 cDNA (shRNA-resistant) co-expression | If p53 restoration abrogates totipotency → p53-mediated |
| OSKM + p53/p38 inhibitors (no centriole elimination) | Critical: p53 effect independent of centriole loss | If MERVL+ appears → p53 is the mechanism, not centriole |
| PIDDosome inhibition | siRNA against PIDD1 or ANKRD26 | Block p53 activation specifically from centriole loss |
| DUX4 toxicity control | NOXA shRNA or MCL-1 co-expression | Separate DUX4 toxicity from totipotency induction |
| SAS-1/C2CD3 rescue | C2CD3 overexpression | Stabilize centriole fragments → should abrogate totipotency |
| Maturity sensor discrimination | Centriole elimination + neural factors (Neurogenin-2 + BDNF) | If neural differentiation also enhanced → maturity sensor favored |
| TLSC protocol control | DOT1L inhibitor + KDM5B inhibitor ± centriole elimination | Does elimination rescue TLSC failure on somatic cells? |
| **Z-DNA induction control** | Polyamines without centriole elimination | Does Z-DNA formation alone suffice? (Shajahan et al., 2025) |
| **Primary cilium analysis** | Anti-ARL13B + anti-γ-tubulin IF | Assess cilium integrity after centriole loss and Rebuild |
| **Karyotype monitoring** | SKY/FISH at each passage | Exclude chromosomal instability from acentriolar mitosis |
| **CEP131/apoptosis control** | Western blot: cytochrome c, caspase-3, cleaved PARP | CEP131 regulates mitochondrial apoptosis independently of p53 (Renaud et al., 2023); delayed apoptosis may be an artefact, not totipotency |
| **p53 rescue control** | shRNA-resistant p53 cDNA in p53-inhibited groups | If p53 restoration abrogates the effect → confirms p53-dependence |
| **Window-of-plasticity timing** | Variable delay: 0, 24, 48, 72 h between elimination and factor delivery | Determines optimal window; if effect decays with delay → plasticity is transient, not stable |
| **Restoration block control** | SAS-6 + STIL + CPAP expression immediately after elimination, before factors | If early centriole restoration blocks totipotency → centriole is the barrier, not elimination stress |
| **Stress-induced plasticity control** | Heat shock (42°C, 2 h) or MG132 (1 μM, 6 h) + DUX4/TPRX1, no centriole elimination | If generic stress mimics the elimination effect → plasticity is stress-induced, not centriole-specific |

---

## 7. Statistical Analysis

**Power calculation.** For 17 experimental groups with a primary comparison of interest (Group F vs. Group E), α = 0.05 is Bonferroni-corrected for 17 groups: α_adj ≈ 0.003. Cohen's d ≥ 0.7 is expected for centriole elimination versus control. **n = 15** biological replicates for centriole elimination groups provides 80% power; n = 10 for control arms. Pilot experiment (n = 3) precedes the full study to estimate within-group variance.

**Multiple testing correction.** With 17 groups × 5 primary markers × 3 time points = 255 comparisons for qPCR endpoints, Benjamini-Hochberg FDR is applied within each marker family. For RNA-seq data with ~20,000 genes × 30 samples, a stringent FDR < 0.01 is used, followed by qPCR validation of all differentially expressed genes that enter downstream functional analysis.

**Analysis plan.**
- Colony counts: one-way ANOVA with Tukey's HSD
- qPCR: ΔΔCt method, Kruskal-Wallis with Dunn's post hoc
- RNA-seq: DESeq2 with donor as random effect in mixed models; GSEA against embryo-stage signatures
- scRNA-seq: Seurat integration, cluster annotation against embryo references
- Repeated measures: linear mixed-effects models with subject-level random intercepts; batch (reagent lot, operator) included as random effect
- Independent biological replication: 3 biological replicates, each with 3 technical replicates, performed on separate days by independent operators

---

## 8. Budget

### Phase 1 — Human cells (marker validation)

| Item | Cost |
|---|---|
| Cell culture consumables | $12,000 |
| RP-1664 dose-response + validation | $5,000 |
| Plk4 siRNA + transfection | $3,000 |
| PCM siRNA, 53BP1/USP28 siRNA | $2,500 |
| Tet-On lentivirus (DUX4, TPRX1, ZSCAN4) | $18,000 |
| Doxycycline titration | $2,000 |
| NOXA shRNA, MCL-1 | $3,000 |
| ROCK inhibitor, pifithrin-α, SB203580 | $3,500 |
| Antibodies (CP110, Cep135, GT335, ZSCAN4, CDX2, OCT4, NANOG, ARL13B, γ-tubulin) | $9,000 |
| qPCR reagents | $4,000 |
| RNA-seq (40 samples × $400) | $16,000 |
| ATAC-seq (16 samples × $350) | $5,600 |
| EPIC arrays (24 samples × $200) | $4,800 |
| Hi-C (6 samples × $800) | $4,800 |
| Single-cell RNA-seq (4 time points × 2 conditions × 2 donors × $2,000) | $32,000 |
| Lentivirus (Centrin1-GFP, C2CD3, IFT88 shRNA, dn-p53) | $8,000 |
| MERVL-GFP reporter line | $5,000 |
| Phase 0 — centrosome proteomics (MS, 6 samples) | $12,000 |
| SKY/FISH karyotyping (20 samples × $300) | $6,000 |
| Personnel (postdoc × 15 months, 50%) | $37,500 |
| **Phase 1 total** | **~$193,000** |

### Phase 2 — Mouse cells (conditional on Phase 1)

| Item | Cost |
|---|---|
| Mouse fibroblast isolation + culture | $8,000 |
| Repeat Phase 1 protocol on mouse cells | $50,000 |
| 8-cell embryo injection + transfer | $25,000 |
| Tetraploid complementation assay | $18,000 |
| Intermediate chimera assay | $12,000 |
| NSG mouse colony (teratoma) | $12,000 |
| Personnel (postdoc × 7 months, 50%) | $21,000 |
| **Phase 2 total** | **~$146,000** |

| **Combined total** | **~$339,000** |

---

## 9. Ethical Considerations

1. **No implantation.** Totipotent human cells must never be transferred to a human or non-human primate uterus.
2. **14-day limit.** Culture limited to 14 days or appearance of the primitive streak — per ISSCR Guidelines (2021).
3. **Genetic containment.** Inducible suicide gene (HSV-TK) in all totipotent cell lines.
4. **SCRO oversight.** All experiments reviewed by institutional stem cell research oversight committee.
5. **Mouse experiments first.** Tetraploid complementation and chimera assays use mouse cells exclusively.
6. **p53 inhibition risk.** Time-limited p53 blockade creates a window of potential genome instability. Karyotype monitoring (SKY/FISH) at every passage is mandatory for all groups receiving p53 or PIDDosome inhibitors. Any line showing clonal aneuploidy is discarded.
7. **Soft agar assay.** All totipotent lines are tested for anchorage-independent growth before any in vivo experiment.
8. **Donor consent.** Fibroblast donors must provide explicit consent for reprogramming experiments including the possibility of totipotent cell generation.

---

## 10. Contingency Plans

1. **Elimination successful, cells fail to survive (viability < 10% at Day 7).** Test BCL-2 overexpression, pan-caspase inhibitor Z-VAD-FMK, or switch to neonatal fibroblasts. Consider partial elimination (50-70%) — the ratchet model predicts a dose-response relationship between centriole number and plasticity.

2. **Transient 8CLC signal, not stable (CTI peaks Day 14, decays Day 28).** Screen additional stabilizing factors (DPPA3, LEUTX); test continuous low-dose DUX4 pulses; test TLSC protocol on acentriolar fibroblasts — chromatin plus hardware reset may both be required for stability.

3. **No effect beyond factors alone (Group F CTI ≈ Group E CTI).** Verify elimination by serial-section EM; test CRISPR KO and laser ablation to rule out pharmacological off-target effects. If all methods confirm elimination without effect, the centriolar ratchet hypothesis is falsified — a valuable outcome.

4. **Factors alone succeed (Group E > Group F).** The centriole is not a barrier — DUX4/TPRX1 suffice for somatic 8CLC induction, a significant finding in its own right.

---

## References

Avidor-Reiss, T., Mazur, M., Fishman, E. L., & Sindhwani, P. (2019). The role of sperm centrioles in human reproduction — the known and the unknown. *Frontiers in Cell and Developmental Biology*, *7*, 188. https://doi.org/10.3389/fcell.2019.00188

Azimzadeh, J., Wong, M. L., Downhour, D. M., Sánchez Alvarado, A., & Marshall, W. F. (2012). Centrosome loss in the evolution of planarians. *Science*, *335*(6067), 461–463. https://doi.org/10.1126/science.1214457

Brevini, T. A. L., Pennarossa, G., Maffei, S., & Gandolfi, F. (2009). Cell lines derived from mammalian parthenogenetic embryos display abnormal centriole distribution and elevated aneuploidy. *Reproduction, Fertility and Development*, *21*(1), 171–172. https://doi.org/10.1071/RDv21n1Ab172

Dzhindzhev, N. S., Tzolovsky, G., Lipinszki, Z., Schneider, S., Lattao, R., Fu, J., Debski, J., Dadlez, M., & Glover, D. M. (2019). STIL S428 phosphorylation is required for recruitment of CPAP to de novo centrioles. *Journal of Cell Biology*, *218*(6), 1859–1873. https://doi.org/10.1083/jcb.201809109

Fishman, E. L., Jo, K., Nguyen, Q. P. H., Kong, D., Royfman, R., Cekic, A. R., Khanal, S., Miller, A. L., Simerly, C., Schatten, G., Loncarek, J., Mennella, V., & Avidor-Reiss, T. (2018). A novel atypical sperm centriole is functional during human fertilization. *Nature Communications*, *9*(1), 2210. https://doi.org/10.1038/s41467-018-04678-8

Fong, C. S., Mazo, G., Das, T., Goodman, J., Kim, M., O'Rourke, B. P., Izquierdo, D., & Tsou, M. F. B. (2016). 53BP1 and USP28 mediate p53-dependent cell cycle arrest in response to centrosome loss and prolonged mitosis. *eLife*, *5*, e16270. https://doi.org/10.7554/eLife.16270

Gambarotto, D., Zwettler, F. U., Le Guennec, M., Schmidt, M., Fortun, D., Kunz, L., Boyden, E. S., Sauer, M., & Hamel, V. (2019). Imaging cellular ultrastructures using expansion microscopy (U-ExM). *Nature Methods*, *16*(1), 71–74. https://doi.org/10.1038/s41592-018-0238-1

Hendrickson, P. G., Doráis, J. A., Grow, E. J., Whiddon, J. L., Lim, J. W., Wike, C. L., Weaver, B. D., Pflueger, C., Emery, B. R., Wilcox, A. L., Nix, D. A., Peterson, C. M., Tapscott, S. J., Carrell, D. T., & Cairns, B. R. (2017). Conserved roles of mouse DUX and human DUX4 in activating cleavage-stage genes and MERVL/HERVL retrotransposons. *Nature Genetics*, *49*(6), 925–934. https://doi.org/10.1038/ng.3844

Huang, Y., Tan, J., & Wu, J. (2014). SAS-6 assembly templated by the lumen of cartwheel-less centrioles precedes centriole duplication. *Developmental Cell*, *30*(2), 238–245. https://doi.org/10.1016/j.devcel.2014.06.006

Jaiswal, T., Muntaqua, D., & Ahmad, N. (2025). Polo-like kinase 4: A molecular culprit in skin cancer pathogenesis. *Cells*, *14*(17), 1381. https://doi.org/10.3390/cells14171381

Lambrus, B. G., Uetake, Y., Clutario, K. M., Daggubati, V., Snyder, M., Sluder, G., & Holland, A. J. (2015). p53 protects against genome instability following centriole duplication failure. *Journal of Cell Biology*, *210*(1), 63–77. https://doi.org/10.1083/jcb.201502089

Lemmers, R. J. L. F., van der Vliet, P. J., Klooster, R., Sacconi, S., Camaño, P., Dauwerse, J. G., Snider, L., Straasheijm, K. R., van Ommen, G. J. B., Padberg, G. W., Miller, D. G., Tapscott, S. J., Tawil, R., Frants, R. R., & van der Maarel, S. M. (2010). A unifying genetic model for facioscapulohumeral muscular dystrophy. *Science*, *329*(5999), 1650–1653. https://doi.org/10.1126/science.1189044

Manandhar, G., Schatten, H., & Sutovsky, P. (2005). Centrosome reduction during gametogenesis and its significance. *Biology of Reproduction*, *72*(1), 2–13. https://doi.org/10.1095/biolreprod.104.031245

Meitinger, F., Anzola, J. V., Kaulich, M., Richardson, A., Stender, J. D., Benner, C., Glass, C. K., Dowdy, S. F., Desai, A., Shiau, A. K., & Oegema, K. (2016). 53BP1 and USP28 mediate p53 activation and G1 arrest after centrosome loss or extended mitotic duration. *Journal of Cell Biology*, *214*(2), 155–166. https://doi.org/10.1083/jcb.201604054

Mikule, K., Delaval, B., Kaldis, P., Jurcyzk, A., Hergert, P., & Doxsey, S. (2007). Loss of centrosome integrity induces p38-p53-p21-dependent G1-S arrest. *Nature Cell Biology*, *9*(2), 160–170. https://doi.org/10.1038/ncb1529

Nigg, E. A., & Holland, A. J. (2018). Once and only once: Mechanisms of centriole duplication and their deregulation in disease. *Nature Reviews Molecular Cell Biology*, *19*(5), 297–312. https://doi.org/10.1038/nrm.2017.127

Paim, L. M. G., & FitzHarris, G. (2019). Tetraploidy causes chromosomal instability in acentriolar mouse embryos. *Nature Communications*, *10*(1), 4834. https://doi.org/10.1038/s41467-019-12772-8

Pimenta-Marques, A., Bento, I., Lopes, C. A. M., Duarte, P., Jana, S. C., & Bettencourt-Dias, M. (2016). A mechanism for the elimination of the female gamete centrosome in Drosophila melanogaster. *Science*, *353*(6294), aaf4866. https://doi.org/10.1126/science.aaf4866

Renaud, C. C. N., Trillet, K., Jardine, J., Le Borgne, R., Pinson, X., & Basto, R. (2023). The centrosomal protein 131 participates in the regulation of mitochondrial apoptosis. *Communications Biology*, *6*, 1271. https://doi.org/10.1038/s42003-023-05676-3

Renzova, T., Bohaciakova, D., Esner, M., Pospisilova, V., Barta, T., Hampl, A., & Cajanek, L. (2018). Inactivation of PLK4-STIL module prevents self-renewal and triggers p53-dependent differentiation in human pluripotent stem cells. *Stem Cell Reports*, *11*(4), 929–944. https://doi.org/10.1016/j.stemcr.2018.08.008

Riparbelli, M. G., & Callaini, G. (2003). Drosophila parthenogenesis: A model for de novo centrosome assembly. *Developmental Biology*, *260*(2), 298–313. https://doi.org/10.1016/S0012-1606(03)00243-4

Shajahan, S., et al. (2025). Z-DNA formation induces the totipotent-like state and primes Zscan4-dependent chromatin compartmentalization. *bioRxiv*. https://doi.org/10.1101/2025.03.18.643869

Soria-Bretones, I., Thu, K. L., Silvester, J., Cruickshank, J., El Ghamrasni, S., Ba-Alawi, W., Fletcher, G. C., Kutasovic, J. R., Lee, J., Zhang, T., Saleeb, R., Lupien, M., Smalley, M. J., Haibe-Kains, B., Lupien, M., Hakem, R., & Cescon, D. W. (2025). The PLK4 inhibitor RP-1664 demonstrates potent single-agent efficacy in neuroblastoma preclinical models through a dual mechanism of sensitivity. *Nature Communications*, *16*, 4012. https://doi.org/10.1038/s41467-025-59384-x

Sun, Y., Xue, Y., Sun, P., Mu, S., Liu, H., Sun, Y., Wang, L., Wang, J., Wu, T., Yin, W., Qin, Q., Sun, Y., Liu, N., Wang, H., Yang, H., Zhao, D., & Cheng, M. (2023). Discovery of the first potent, selective, and in vivo efficacious Polo-like kinase 4 proteolysis targeting chimera degrader for the treatment of TRIM37-amplified breast cancer. *Journal of Medicinal Chemistry*, *66*(12), 8200–8221. https://doi.org/10.1021/acs.jmedchem.3c00505

Taubenschmid-Stowers, J., Rostovskaya, M., Santos, F., Ljung, S., Argelaguet, R., Krueger, F., Nichols, J., & Reik, W. (2022). 8C-like cells capture the human zygotic genome activation program in vitro. *Cell Stem Cell*, *29*(3), 449–459.e6. https://doi.org/10.1016/j.stem.2022.01.014

Tkemaladze, J. V., & Chichinadze, K. N. (2005). Centriolar mechanisms of differentiation and replicative aging of higher animal cells. *Biochemistry (Moscow)*, *70*(11), 1288–1303. https://doi.org/10.1007/s10541-005-0260-7

Tqemaladze, J. (2026). Eliminate, reprogram, and rebuild: Centriole removal followed by meiotic factors and de novo assembly for somatic totipotency: A hypothesis. *Manuscript in preparation*.

Uetake, Y., Loncarek, J., Nordberg, J. J., English, C. N., La Terra, S., Khodjakov, A., & Sluder, G. (2007). Cell cycle progression and de novo centriole assembly after centrosomal removal in untransformed human cells. *Journal of Cell Biology*, *176*(2), 173–182. https://doi.org/10.1083/jcb.200607073

Wong, C., & Stearns, T. (2005). Mammalian cells lack checkpoints for tetraploidy, aberrant centrosome number, and cytokinesis failure. *BMC Cell Biology*, *6*, 6. https://doi.org/10.1186/1471-2121-6-6

Wong, Y. L., Anzola, J. V., Davis, R. L., Yoon, M., Motamedi, A., Kroll, A., Seo, C. P., Hsia, J. E., Kim, S. K., Mitchell, J. W., Mitchell, B. J., Desai, A., Gahman, T. C., Shiau, A. K., & Oegema, K. (2015). Reversible centriole depletion with an inhibitor of Polo-like kinase 4. *Science*, *348*(6239), 1155–1160. https://doi.org/10.1126/science.aaa5111
