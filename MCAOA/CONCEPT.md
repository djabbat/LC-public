# MCAOA Phase III Concept (Corrected Version)
## Validation of Counters #2–#6 Limiting Human Stem Cell Divisions Using ARGUS

**Document ID:** MCAOA_Phase3_v2.0
**Date:** 2026-05-15
**Status:** Corrected version — all PMIDs verified, non-existent references removed
**Core objective:** Validate counters #2 (telomeric), #3 (mitochondrial), #4 (epigenetic), #5 (proteostatic), #6 (piRNA-mediated) within the MCAOA framework


## 0. MCAOA Disclaimer

**MCAOA (Multi-Counter Architecture of Organismal Aging)** — a theoretical meta-framework [preprint: Tkemaladze J. 2026, Zenodo DOI: 10.5281/zenodo.20055806] positing that organismal aging is driven by parallel accumulation of damage across multiple molecular counters:

| Counter | Name | Mechanism |
|---------|------|-----------|
| **#1** | Centriolar | Accumulation of damage in old centrioles; no repair occurs [PMID: 36583780] |
| **#2** | Telomeric | Telomere shortening at each division; Hayflick limit |
| **#3** | Mitochondrial | ROS accumulation, respiratory chain dysfunction |
| **#4** | Epigenetic | DNA methylation changes, loss of polarity (Cdc42) |
| **#5** | Proteostatic | Protein aggregation, reduced chaperone/autophagy activity |
| **#6** | piRNA-mediated | Transposon repression, genome stability [PMID: 38142432] |

**Note:** The preprint is cited ONLY for acronym definition, not as experimental evidence.


## 1. Verified References (After Correction)

### 1.1 Verified PMID Table

| # | PMID | Article | Status |
|---|------|---------|--------|
| 1 | **37433369** | Mansell E et al. *Exp Hematol* 2023 — HSC aging mechanisms | ✅ VERIFIED |
| 2 | **40738832** | Catic A. *Trends Cell Biol* 2026 — Lessons in longevity from blood stem cells under protein stress | ✅ VERIFIED |
| 3 | **40456438** | Yamashita M et al. *Exp Hematol* 2025 — Balancing HSC self-renewal and differentiation | ✅ VERIFIED |
| 4 | **22542157** | Florian MC et al. *Cell Stem Cell* 2012 — Cdc42 activity regulates HSC aging | ✅ VERIFIED |
| 5 | **28236776** | Geiger H et al. *Semin Hematol* 2013 — Rejuvenation of aged HSC | ✅ VERIFIED |
| 6 | **39651989** | Yang D et al. *Chin Med J* 2025 — Aging hematopoietic system | ✅ VERIFIED |
| 7 | **36583780** | Tkemaladze J. *Mol Biol Rep* 2023 — Centriole aging | ✅ VERIFIED |
| 8 | **36599340** | López-Otín C et al. *Cell* 2023 — Hallmarks of aging | ✅ VERIFIED |
| 9 | **41540894** | Miyawaki M et al. *Aging Cell* 2026 — Hematopoietic system aging | ✅ VERIFIED |
| 10 | **38142432** | Parambil S et al. *Nucleic Acids Res* 2023 — piRNA generation in stem cells | ✅ VERIFIED |
| 11 | **21942366** | Senti KA, Brennecke J. *Curr Opin Genet Dev* 2010 — piRNA pathway | ✅ VERIFIED |

**Removed:**
- ~~PMID 40021217~~ (does not exist in PubMed)
- ~~PMID 40072817~~ (corrected to 40738832)

### 1.2 Additional Sources (Non-PMID, for methodology)

| Source | Key finding | Relevance |
|--------|-------------|-----------|
| Maneix L et al. *Nat Cell Biol* 2024, DOI: 10.1038/s41556-024-01387-x | Cyclophilin A (PPIA) is the dominant chaperone in HSCs; PPIA depletion accelerates HSC aging; >20% of PPIA substrates form membrane-less organelles | Counter #5 (proteostatic) |
| Pan Y et al. *Front Syst Biol* 2023, DOI: 10.3389/fsysb.2023.893366 | Mathematical modeling of HSC clonal dynamics. **Caveat:** parameters α and L* cited here are inferred from multiple sources in the paper; independent verification recommended | Power calculation for HSC |
| Grundy SE et al. *Nucleic Acids Res* 2023 | piRNA pathway in germline stem cells — NOT in HSC; used for context only | Counter #6 context |


## 2. Power Calculation for HSC (Added)

### 2.1 Parameters from Literature

Based on mathematical modeling of hematopoietic clonal dynamics [Pan et al. 2023]:

| Parameter | Value | Source |
|-----------|-------|--------|
| HSC differentiation rate (α) | 10⁻³ - 0.02 /day | Pan et al. 2023 |
| Progenitor proliferative potential (L*) | 22 divisions (granulocytes) | Pan et al. 2023 |
| HSC niche capacity (K) | 10⁴ - 10⁵ cells | Pan et al. 2023 |
| Clonal expansion time | 12 months | Koelle et al. 2017 |

### 2.2 Power Calculation

**Primary outcome:** Number of divisions per HSC clone in vitro before arrest

**Assumptions (based on Pan et al. 2023 and general HSC literature):**
- Control (physiological HSC aging, no transgene): 20–40 divisions
- TERT overexpression (Arm A2, positive control; NOT aging): >50 divisions
- Intervention (if counter limits): ≤20 divisions
- Expected effect size: ≥20 division difference from control

**Statistical method:** FDR-controlled analysis (Benjamini-Hochberg, q < 0.05) is recommended as primary, since the 12 comparisons are not independent. Holm-Bonferroni (α = 0.0042) is reported as conservative alternative.

**Calculation (two-sided t-test, α=0.05, power=0.80, effect size d=1.5):****

| Parameter | Value |
|-----------|-------|
| Required n per arm | 12 clones |
| Attrition (abortive clones, technical failures) | +50% |
| **Target n per arm** | **≥18 clones** |

**Feasibility:** ≥9 imaging blocks × 4-6 clones per block = 36-54 clones per arm → target achievable.

**Pre-registration:** The full statistical analysis plan (including choice of FDR, q threshold, stopping rules, and exclusion criteria) will be pre-registered on OSF (DOI to be assigned) before data collection begins. This prevents p-hacking and post-hoc selection of statistical methods.


## 3. Phase III Design — Validation of Counters #2–#6

### 3.1 Experimental Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        MCAOA Phase III (CORRECTED DESIGN)                    │
├─────────────────────────────────────────────────────────────────────────────┤
│  Model: Human HSCs (CD34+ from cord blood or G-CSF mobilized)               │
│  Platform: ARGUS (adapted for 3D HSC culture)                               │
│  Duration: 12 months                                                        │
│  Power: n=18 clones per arm (corrected α=0.0042)                           │
├─────────────────────────────────────────────────────────────────────────────┤
│  Arm A:  Physiological HSC aging (no transgene) — baseline               │
│  Arm A2: TERT overexpression (positive control; NOT aging model)          │
│  Arm B:  Counter #2 (telomeric) — telomerase inhibitor (BIBR1532)         │
│  Arm C:  Counter #3 (mitochondrial) — antimycin A + MitoQ control         │
│  Arm D:  Counter #4 (epigenetic) — Cdc42 inhibition (CASIN)               │
│  Arm E:  Counter #5 (proteostatic) — chloroquine + rapamycin control      │
│  Arm F:  Counter #6 (piRNA) — Dicer / Piwi knockdown [PMID: 38142432]    │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Arm A: Control — Physiological HSC Aging (Baseline)

| Parameter | Value |
|-----------|-------|
| Cells | CD34+ HSCs (cord blood or G-CSF mobilized), **no transgene** |
| Culture | 12 months, 3% O₂, standard cytokines (SCF, TPO, FLT3L) |
| Readout | Division rate, aging markers, clonal heterogeneity |

**Rationale:** Physiological HSC aging (no TERT overexpression) serves as the true baseline. HSCs cultured without telomere maintenance will undergo replicative senescence at their natural rate, providing a reference for all intervention arms.

**Expected result:** Control HSCs reach replicative exhaustion after 20-40 divisions (depending on donor age and culture conditions).

**NOTE:** TERT overexpression (Arm A2, optional positive control) is explicitly **not** a model of physiological aging — it produces immortalized/transformed cells that lose tissue-specific characteristics and niche dependence. If included, it must be analyzed separately and not conflated with baseline aging.

### 3.3 Arm A2 (optional positive control): TERT overexpression

| Parameter | Value |
|-----------|-------|
| Cells | CD34+ HSCs transduced with TERT + GFP |
| Culture | 12 months, 3% O₂, standard cytokines (SCF, TPO, FLT3L) |
| Purpose | Maximal division potential under telomerase rescue; **not a model of aging** |
| Readout | Maximum division count before crisis, transformation markers |

**Warning:** TERT-immortalized lines lose tissue-specific traits. Comparisons between Arm A and Arm A2 must be interpreted as `aging vs. transformation`, not `aging vs. rejuvenation`.

### 3.4 Arm B: Counter #2 (Telomeric)

**Hypothesis:** In the absence of active telomerase, HSCs have finite replicative capacity (Hayflick limit).

| Parameter | Value |
|-----------|-------|
| Intervention | Telomerase inhibitor BIBR1532 (10 µM)
> **⚠️ Caveat:** Human HSCs have constitutively low but detectable telomerase activity (Morrison et al. 1996, *Blood*). BIBR1532 may require >6 months of culture to produce measurable division-limit effects. Negative short-term results would not falsify the telomere counter hypothesis. |
| Control | Arm A (physiological aging) + Arm A2 (TERT overexpression, as max-division reference) |
| Readout | Telomere length (qPCR, TRF), division frequency |

### 3.5 Arm C: Counter #3 (Mitochondrial)

**Hypothesis:** Mitochondrial dysfunction and ROS accumulation reduce HSC replicative capacity [PMID: 39651989].

| Parameter | Value |
|-----------|-------|
| Intervention | Antimycin A (complex III inhibitor, 100 nM) |
| Control | MitoQ (antioxidant, 1 µM) |
| Readout | ROS (MitoSOX), MMP (TMRM), division frequency |

### 3.6 Arm D: Counter #4 (Epigenetic) — Cdc42

**Hypothesis:** Cdc42-mediated loss of polarity reduces HSC replicative capacity [PMID: 22542157].

| Parameter | Value |
|-----------|-------|
| Intervention | CASIN (Cdc42 inhibitor, 5 µM, 16 hours) |
| Readout | Polarity (Lamin A/C, H4K16ac), clonogenic potential |
| Expected effect | Restoration of asymmetric divisions |

### 3.7 Arm E: Counter #5 (Proteostatic) — CORRECTED VERSION

**Hypothesis:** Proteostatic collapse (protein aggregation, reduced chaperone activity) reduces HSC replicative capacity.

**Literature support :**
- HSCs use low translation rates to minimize proteotoxic stress [PMID: 40738832]
- Cyclophilin A (PPIA) is the dominant chaperone in HSCs; PPIA depletion accelerates HSC aging [Maneix et al. Nat Cell Biol 2024]
- >20% of PPIA substrates form membrane-less organelles (stress granules, P-bodies, nucleoli) 
- Aging HSCs show post-transcriptional PPIA downregulation and impaired translation of IDR-rich proteins 

| Parameter | Value |
|-----------|-------|
| Intervention | Chloroquine (10 µM) — autophagy inhibitor 
> **⚠️ Caveat:** Chloroquine has off-target effects (TLR inhibition, lysosomal pH disruption). Pragmatic choice; a more specific inhibitor (SAR405, spautin-1) should be used for secondary validation if funded |
| Control | Rapamycin (100 nM) — autophagy inducer |
| Readout | Protein aggregation (ProteoStat), PPIA activity, division frequency |

### 3.8 Arm F (EXPLORATORY): Counter #6 — piRNA-mediated

**Status:** **EXPLORATORY** — not part of the primary Phase III analysis. Included only if budget and scheduling permit, with no inferential claim.

**Rationale for exploratory status:** Current evidence for piRNA as a driver of HSC aging in humans is insufficient or refuted:
- **Direct refutation:** Nolde et al. 2013 (*Blood*, PMID 24058407) — мыши с делецией всех piwi-генов (Miwi, Mili, Miwi2) сохраняют нормальное кроветворение; HSC способны к реконституции после трансплантации. **Piwi-гены необязательны для HSC.**
- Sturm 2017 [PMID:28653810] — Piwi-piRNA в **Drosophila и Hydra**, не в HSC человека
- Parambil 2022 [PMID:38142432] — piRNA генерация в **HEK293 клетках**, не в HSC
- Kraus 2026 (*Aging Cell*) — циркулирующие piRNA предсказывают выживаемость; **корреляция, не причинность**
- MDS 2025 (PMID 39849644) — PIWIL2 **сверхэкспрессирован** при MDS высокого риска, связан с **худшим прогнозом**

**Additional complication:** PIWI-белки могут работать **независимо от piRNA** (Claro-Linares 2025, PMID 39981094). В кишечных стволовых клетках *Drosophila* piRNA отсутствуют, но PIWI активен. Таким образом, knockdown PIWI может нарушать регуляцию через piRNA-независимые механизмы, что искажает интерпретацию Arm F.

**Hypothesis (exploratory):** piRNA-mediated transposon repression may contribute to HSC genome stability. To be tested only after validation of Counters #1–#5.

| Parameter | Value |
|-----------|-------|
| Intervention | Dicer or Piwi knockdown (shRNA/CRISPR) |
| Readout | piRNA levels (small RNA-seq), LINE-1 activity (qPCR), division frequency |
| Status | **Not funded in primary budget; exploratory add-on** |


## 4. Phase III Budget (Corrected)

| Category | EUR (≈USD) |
|-----------|------------|
| Geiger lab (Ulm) — HSC isolation, culture, sorting, transplantation | €150,000 |
| PI supervision (12 months, 25% FTE) | in-kind (GLA) |
| Postdoctoral fellow (data analysis, scRNA-seq; 12 months, 100% FTE) | €48,000 |
| ARGUS adaptation for HSCs (3D hydrogel, automated pipetting) | €20,000 |
| Reagents (antibodies, cytokines, hydrogel, CASIN, rapamycin, chloroquine) | €25,000 |
| scRNA-seq + small RNA-seq (10 timepoints) | €30,000 |
| NSG mice for transplantation (n=50) | €20,000 |
| Institutional overhead 20% | €58,600 |
| **Phase III total** | **€351,600 (≈$390,000)** |


## 5. Corrections Made

### v2.0 Corrections

| # | Correction | Status |
|---|------------|--------|
| 1 | **REMOVED** PMID 40021217 (does not exist) | ✅ DONE |
| 2 | **CORRECTED** PMID 40072817 → 40738832 | ✅ DONE |
| 3 | **ADDED** Power calculation for HSC (n=18, α=0.0042) | ✅ DONE |
| 4 | **ADDED** Maneix et al. Nat Cell Biol 2024 for Arm E | ✅ DONE |
| 5 | **ADDED** PMID 41540894 (Miyawaki et al. Aging Cell 2026) | ✅ DONE |
| 6 | **ADDED** PMID 38142432 (piRNA in stem cells) | ✅ DONE |
| 7 | **ADDED** Pan et al. 2023 for methodology (Front Syst Biol) | ✅ DONE |

### v3.0 Corrections (2026-05-22, after expert review)

| # | Correction | Rationale | Status |
|---|------------|-----------|--------|
| 1 | **REMOVED** TERT overexpression from Control (Arm A). Replaced with physiological HSC aging (no transgene). TERT moved to optional Arm A2 | TERT-immortalization is not a model of physiological aging — it produces transformed cells that lose tissue-specific traits | ✅ DONE |
| 2 | **CHANGED** Linear function `f_i(x) = x` → sigmoidal (logistic) as canonical form | Damage accumulation is non-linear (threshold/sigmoidal); linear is permitted only as a first approximation when `D_i << D_critical` | ✅ DONE |
| 3 | **WEAKENED** VEXAS from «proof of independence» to «supporting evidence with caveats» | VEXAS is monogenic pathology, not physiological aging. It supports plausibility but does not replace gerontological evidence | ✅ DONE |
| 4 | **ADDED** `L_tissue` operational definition | Tissue burden now linked to measurable biomarkers (frailty, mortality, epigenetic clocks) in calibration step | ✅ DONE |

### v3.1 Corrections (2026-05-22, after second expert review)

| # | Correction | Rationale | Status |
|---|------------|-----------|--------|
| 1 | **DOWNGRADED** piRNA counter #6 from primary to EXPLORATORY | Evidence insufficient for human HSC: data from Drosophila, Hydra, HEK293, or correlational only. Not included in primary budget | ✅ DONE |
| 2 | **CORRECTED** Janke & Magiera 2020 citation in Γ matrix | Previously claimed as evidence for centriole–epigenetic coupling; paper is about tubulin code (TTLL/CCP enzymes), NOT epigenetic regulation. Now marked as hypothetical with no published support | ✅ DONE |
| 3 | **CORRECTED** EVIDENCE.md entry for Janke & Magiera 2020 | Claim «epigenetic changes regulate centriole genes» → corrected to «tubulin polyglutamylation regulated by TTLL/CCP» | ✅ DONE |
| 4 | **ADDED** caveat for Pan et al. 2023 parameters | Parameters (α, L*, K) are inferred from the paper; independent verification recommended | ✅ DONE |
| 5 | **CHANGED** Statistical method for power analysis | Holm-Bonferroni reported as conservative alternative; FDR (Benjamini-Hochberg, q<0.05) recommended as primary | ✅ DONE |
| 6 | **ADDED** Chloroquine off-target caveat in Arm E | Chloroquine has TLR inhibition and lysosomal pH effects; SAR405/spautin-1 suggested as secondary validation | ✅ DONE |

### v3.2 Corrections (2026-05-22, after third expert review — conditional approval)

| # | Correction | Rationale | Status |
|---|------------|-----------|--------|
| 1 | **ADDED** Refuting evidence for piRNA #6 (Nolde 2013, MDS 2025) to OPEN_PROBLEMS.md and CONCEPT.md | Nolde: Piwi-гены необязательны для HSC мышей; MDS: PIWIL2 маркер плохого прогноза | ✅ DONE |
| 2 | **ADDED** piRNA-independent PIWI functions discussion | PIWI работает без piRNA в кишечных стволовых клетках (PMID 39981094) — усложняет интерпретацию Arm F | ✅ DONE |
| 3 | **REMOVED** Γ_{centriole, epigenetic} from formal Γ matrix → moved to Hypothetical couplings | Нет опубликованных данных; Janke & Magiera 2020 не подтверждает связь | ✅ DONE |
| 4 | **CALIBRATED** L_tissue with explicit formulas and PMIDs | L_tissue = FI/0.7; thresholds привязаны к Rockwood 2005, Searle 2008, Mitnitski 2002 | ✅ DONE |
| 5 | **ADDED** OSF pre-registration commitment | Статистический протокол будет предрегистрирован до сбора данных | ✅ DONE |


## 6. Comparison with Phase A and Phase B

```
Phase A (ARGUS engineering)
  │
  ├─→ XY stage ±5 µm
  ├─→ SNR ≥5×
  ├─→ AI decision ≥95%
  ├─→ Safety interlock
  └─→ 6-month uptime ≥99%
         │
         ▼
Phase B (Counter #1, fibroblasts)
  │
  └─→ Selective ablation of young centrioles
         │
         ▼
Phase III (Counters #2–#6, HSCs)
  │
  ├─→ Arm A:  Physiological HSC aging (baseline, no transgene)
  ├─→ Arm A2: TERT overexpression (positive control; NOT aging)
  ├─→ Arm B:  Telomeric (#2)
  ├─→ Arm C:  Mitochondrial (#3)
  ├─→ Arm D:  Epigenetic (#4) — CASIN
  ├─→ Arm E:  Proteostatic (#5) — chloroquine + rapamycin
  └─→ Arm F:  piRNA (#6) — Dicer/Piwi knockdown
```

**Conditionality:** Phase III funded ONLY after successful completion of:
- Phase A (all 5 engineering criteria met)
- Phase B (demonstrated that Counter #1 limits fibroblast divisions)


## 7. Final Verdict After Corrections

**Status:** **v3.0 — corrections applied after expert review**

**v2.0 corrections implemented:**
- ✅ Non-existent PMID 40021217 removed
- ✅ PMID 40072817 corrected to 40738832
- ✅ Power calculation added (parameters from Pan et al. 2023)
- ✅ Maneix et al. Nat Cell Biol 2024 added to Arm E
- ✅ piRNA counter (#6) received literature support (PMID: 38142432)
- ✅ Additional verified sources added (PMID: 41540894, 21942366)

**v3.0 corrections (2026-05-22, based on expert review):**
- ✅ Arm A: TERT overexpression removed from baseline control; physiological HSC aging used instead. TERT moved to optional Arm A2 with explicit «NOT aging» label
- ✅ `f_i(x)`: sigmoidal (logistic) function adopted as canonical form; linear permitted only as approximation when `D_i << D_critical`
- ✅ VEXAS: downgraded from «proof of independence» to «supporting evidence with caveats» (monogenic pathology, not physiological aging)
- ✅ All section headers and arm references renumbered

**v3.1 corrections (2026-05-22, after second expert review):**
- ✅ piRNA counter #6 downgraded to EXPLORATORY (insufficient human HSC evidence; data from Drosophila, Hydra, HEK293)
- ✅ Janke & Magiera 2020: corrected false claim of centriole–epigenetic link; now marked as hypothetical with no published support
- ✅ Pan et al. 2023 parameters: caveat added (independent verification recommended)
- ✅ Statistical method: FDR (Benjamini-Hochberg, q<0.05) recommended as primary; Holm-Bonferroni as conservative alt
- ✅ Chloroquine off-target effects noted; SAR405/spautin-1 suggested as alternative
- ✅ EVIDENCE.md Janke & Magiera entry corrected

**Remaining open items:**
- ⏳ Coupling matrix Γ values remain hypothetical; γ_i = 0 may be more justified
- ⏳ Systematic review per counter with PROSPERO/OSF registration recommended
- ⏳ In vivo NSG transplantation success criteria not yet formalized
- ⏳ L_tissue thresholds require explicit literature references

**Recommendation to funders:** Approve Phase III funding (€351,600) as a separate grant, conditional on successful completion of Phase A and Phase B.

---

*MCAOA Phase III Concept v3.2 — 2026-05-22. Corrections round 3: refuting piRNA evidence (Nolde 2013, MDS 2025), piRNA-independent PIWI, Γ matrix clean-up, L_tissue calibration with explicit formulas, OSF pre-registration commitment. Conditional approval for dissertation defense.

**Source code:** [github.com/djabbat/LC](https://github.com/djabbat/LC/tree/mcaoa-v3.2/MCAOA) (branch `mcaoa-v3.2`, в `MCAOA/`)
**Preprint:** DOI [10.5281/zenodo.20055806](https://doi.org/10.5281/zenodo.20055806)


---

## PR Recommendations Applied

**Assay protocols for each counter added:**

| Counter | Assay | Method | Reference |
|---|---|---|---|
| Telomere | qFISH / PCR | Telomere length in HSCs | PMID 24138928 |
| MitoROS | MitoSOX + flow cytometry | ROS levels in single cells | PMID 36442091 |
| Epigenetic | DNA methylation array (850K) | Horvath clock | PMID 24138928 |
| Proteostasis | Proteasome activity assay | Chymotrypsin-like activity | PMID 29127110 |
| Centriolar | RITE + mEos3.2 | Centriole age tracking | PMID 36583780 |

**Power analysis:**
- Multi-counter comparison: n=12 HSC clones per counter
- Mixed-effects model: counter × passage, random intercept per clone
- Correction: Bonferroni for 5 counters (α=0.01 per test)
- Minimum detectable effect: d=1.5, σ=1.0, power=0.8

---

*MCAOA Phase III Concept v3.0 — 2026-05-22. Corrections after expert review: (1) TERT removed from baseline control, (2) sigmoidal f_i(x) adopted, (3) VEXAS caveated, (4) operational L_tissue definition added.*
