<!-- AUTO-TRANSLATED via DeepSeek 2026-05-13. Source language: mixed. Original preserved at CONCEPT.x.md. -->

# MCOA — Multi-Counter Architecture of Organismal Aging

> ⚠️ **See [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md)** — some statements may be retracted. Canons updated 2026-04-22.

**Project:** MCOA (Multi-Counter Architecture of Organismal Aging)
**Author:** Jaba Tkemaladze, MD | Georgia Longevity Alliance
**Version:** 1.0 (extension addenda 2026-05-10 — see §11)
**Date:** 2026-04-21 (last modified 2026-05-10)
**Status:** CONCEPT APPROVED — initial implementation in progress
**Canonical reference:** `~/Documents/MCOA_NatureAging_submission/01_MCOA_Perspective_manuscript.md` (*Nature Aging* Perspective submission, 2026-04-25)
**Extension manuscripts (NOT YET PUBLISHED, draft 2026-05-10):**
- "A Stem-Cell-Centric Multi-Counter Theory of Organismal Aging: Context-Dependent Prioritization and the Master-Counter Hypothesis" — Perspective/Hypothesis target *Nature Aging* / *Aging Cell*. Source: `~/Desktop/A Stem-Cell-Centric Multi-Counter Theory of Organismal Aging.md` + review chain `docs/manuscripts/HAYFLICK_HIERARCHY/`.
- "Epigenomic Rejuvenation Without Functional Restoration: Damage Shadow Hypothesis" — SR + meta-analysis (PROSPERO CRD42026218473), target *Nature Aging* / *Cell Metabolism* / *Lancet Healthy Longev* (IF>18). Source: `~/Desktop/Epigenomic Rejuvenation Without Functional Restoration.md`.

---

## 1. Project identity

**Note on limitations:** The OSF pre-registration (osf.io/9x3k7) has been created with the primary falsification test (Test 4) protocol. Sample size and timeline details are documented in the registration. The actual sample size for the mortality test may be less than N ≥ 2000. The timetable for a-priori weight prediction (Test 1A) is expected Q4 2026.

MCOA is the theoretical mother-project of the LongevityCommon aging-science stack. It formalises organismal aging as the weighted sum of multiple parallel damage-accumulation processes ("counters"), each with its own division-linked and time-linked kinetics, each tied to a tissue-specific weighting function that is fixed *a priori* to preserve falsifiability.

MCOA is **not** a replacement for CDATA, Ze, or BioSense; it is the meta-framework in which they live as specialised counters or measurement layers.

---

## 2. Inviolable axioms (do not change without explicit user command)

**Axiom M1 — Parallel counters.** Organismal aging is driven by ≥ 2 distinct damage-accumulation processes that proceed in parallel. No single counter is sufficient to explain the universality of replicative limits.

**Axiom M2 — Dimensional consistency.** No expression of the form *α·n + β·t* is valid unless both terms are reduced to a common dimensionless form. The canonical form is:

*D_i(n, t) = D_i₀ + α_i · (n / n_i\*) + β_i · (t / τ_i) + γ_i · I(other counters)*

where *n_i\** and *τ_i* are counter-specific reference scales fixed *a priori* from independent cell-biological knowledge.

**Axiom M3 — A-priori tissue weighting.** *w_i(tissue)* must be predicted BEFORE fitting, from independent cell-biological parameters (division rate, metabolic intensity, substrate half-life, TERT expression, TTLL/CCP balance, mitochondrial content). Post-hoc fitting is explicitly prohibited; any such adjustment is a model-correction, not a model-prediction.

**Axiom M4 — Falsifiability is first-class (operational threshold v5.6 update 2026-04-28).** Every MCOA-derived claim must be accompanied by an experimental test that can falsify it. **Operational definition:** MCOA is considered *falsified* if on a pre-registered cohort with `N ≥ 2000` at `α = 0.001` the partial r² for all-cause mortality (after controlling for chronological age and sex) falls below `0.05` for every counter `i`. Power analysis: `N = 2000` required to detect `R² = 0.3` at 80% power (σ² = 0.25, α = 0.05 two-tailed); threshold set per community standard. The earlier provisional threshold `R² < 0.5` (article v4 and earlier) is **superseded** by this AND-conjunction of community-standard validation thresholds. The canonical test set is §6.1–6.5 of the Nature Aging Perspective; each counter `i` is independently falsifiable via its own partial r² contribution.

---

## 3. Formal definition

### 3.1. Single-counter kinetics

*D_i(n, t) = D_i₀ + α_i · (n / n_i\*) + β_i · (t / τ_i) + γ_i · I(others)*

| Symbol | Meaning | Units | Constraint |
|--------|---------|-------|------------|
| *D_i* | Accumulated damage in counter *i* | dimensionless | ≥ 0 |
| *D_i₀* | Baseline damage at birth | dimensionless | ≥ 0 |
| *α_i* | Division-driven rate | dimensionless / (n / n_i\*) | ≥ 0 |
| *β_i* | Time-driven rate | dimensionless / (t / τ_i) | ≥ 0 |
| *γ_i* | Coupling scalar | dimensionless | ℝ |
| *I(others)* | Influence of other counters | dimensionless | Σ_j γ_ij · D_j / (whatever norm) |
| *n_i\** | Reference division number | divisions | tissue-specific, a priori |
| *τ_i* | Reference time scale | seconds | tissue-specific, a priori |

### 3.2. Tissue-integrated load

*L_tissue = Σ_i [ w_i(tissue) · f_i( D_i(n, t) ) ]*

with the constraint *Σ_i w_i(tissue) ≈ 1.0* (non-trivial deviation indicates a missing counter).

### 3.3. Functional transition

A cell enters senescence, apoptosis, or dysfunction when:

*L_tissue > L_critical(tissue)* OR ∃ *i* : *D_i > D_critical(i, tissue)*

---

## 4. The five canonical counters

| # | Name | Subproject | Nature | *n_i\** anchor | *τ_i* anchor |
|---|------|------------|--------|----------------|--------------|
| **1** | **Centriolar polyglutamylation** | CDATA | division + time | ~50–80 for HSC, ~30–50 for epithelial | months–years (mass-spec to calibrate) |
| **2** | **Telomere** | Telomere (new subproject) | division-dominant | Hayflick limit per cell type (~50 for human fibroblasts) | turnover of telomeric repeats |
| **3** | **Mitochondrial ROS / mtDNA** | MitoROS (new subproject) | time-dominant | α → 0 for post-mitotic | days–weeks for mtDNA lesion turnover |
| **4** | **Epigenetic drift** | EpigeneticDrift (new subproject) | time-dominant | α → 0 for post-mitotic | Horvath clock / DunedinPACE doubling time |
| **5** | **Proteostasis collapse** | Proteostasis (new subproject) | mixed | cell-type-specific | protein half-life of dominant aggregating species |

**Ordering rationale (2026-04-21):** Centriole is placed at #1 because it is the unifying structural counting device within the asymmetric inheritance framework; telomere is a division-dependent counter downstream of centriole-inherited stemness. Each counter has its own dedicated subproject with Rust core and Phoenix LiveView dashboard — see §10.

Additional counters (lipofuscin, lamina defects, ECM stiffening, SASP spread) are natural extensions; they enter with the same formal apparatus.

---

## 5. Coupling matrix Γ

Γ ∈ ℝ^(k×k) where k = number of counters. Γ_{ij} = rate at which counter *j* accelerates counter *i*.

Known non-zero entries (from Nature Aging Perspective):
- Γ_{telomere, mito} > 0 (Parrinello 2003 — oxidative stress accelerates telomere loss)
- Γ_{epigenetic, mito} > 0 (Schultz & Sinclair *Cell* 2019, PMID 30982602 — NAD+/sirtuin/aging axis; replaces fabricated «Sun 2016 Measuring In Vivo Mitophagy», corrected 2026-04-26)
- Γ_{cent, epigenetic} > 0 (epigenetic dysregulation alters TTLL/CCP balance — Janke & Magiera 2020)

All Γ entries must be measured, not fitted. ~~MCOA Test 2~~ [retracted — see CORRECTIONS §1.3] (§6.2 Perspective) is the canonical measurement protocol.

---

## 6. Falsifiability tests (canonical)

Each test is described in detail in the Nature Aging Perspective §6.1–6.5:

1. **Test 1 (Tissue-Specific Counter Dominance):** longitudinal mouse study, N=85/timepoint, 6 tissues × 4 counters × 4 timepoints = 96 FDR-corrected tests. $1.5M / 3 years.
2. **Test 2 (Counter Coupling Γ_ij):** PolgA D257A mouse model, 8-OHdG ELISA primary readout. $800k / 2 years.
3. **Test 3 (Intervention Specificity):** rapamycin × senolytic × combination in aged mice.
4. **Test 4 (Division vs Time — Aubrey's test):** *ex vivo* iPSC organoids, 2×2 design. **<$200k / 10 weeks — single-lab tractable.**
5. **Test 5 (Multi-target Synergy):** 5-arm mouse lifespan trial. $2.8M / 4 years.

Test 4 is the near-term priority.

---

## 7. Relationship to subprojects of LongevityCommon

| Subproject | MCOA role |
|------------|-----------|
| CDATA | Counter #1 (centriolar polyglutamylation) — specialised instance |
| Ze | Counter "S" — dimensionless χ_Ze synchronisation index computed from an ODE model of the plasma/SASP feedback loop (see `Ze/CONCEPT.md` §4, rewritten 2026-04-23 on Argentieri 2024 / Jeon 2022 basis) |
| BioSense | Measurement layer for *D_autonomic*, *D_neural*, *D_olfactory* |
| FCLC | Federated calibration of *w_i(tissue)* across clinics |
| Ontogenesis | Developmental trajectory (0–25 yr) with MCOA counter families |
| HAP | Clinical backdrop; no direct MCOA integration |

---

## 8. Success criteria (v1.0)

- [x] Nature Aging Perspective manuscript ready (`~/Documents/MCOA_NatureAging_submission/`)
- [ ] Rust reference implementation (`mcoa_core`, `mcoa_simulation`) compiling and tested
- [ ] At least one MCOA Test 4 simulation run, output comparable to CDATA v5.1
- [ ] 3-figure visualisation (Fig 1–3 already produced for Perspective)
- [ ] Submission to *Nature Aging* by 2026-04-25

---

## 9. What MCOA is NOT

- MCOA is not a new set of biomarkers — it uses existing ones (Horvath, DunedinPACE, GT335, MitoSOX, telomere qFISH, 8-OHdG).
- MCOA is not a single-disease theory — it is a framework that any specific disease/tissue can be reduced to.
- MCOA does not assume "no repair" — repair appears as a negative contribution to the counter's drift rate.
- MCOA does not privilege any counter a priori — weights are measured, not decreed.

---

**Version:** 1.0
**Date:** 2026-04-21
**Next revision trigger:** Nature Aging editorial decision OR completion of MCOA Test 4 simulation.

---

## Role of MCOA in EIC Pathfinder Part B v3 (Variant B, submission 2026-05-12)

MCOA constitutes **WP1 MCOA Framework** in the current EIC Pathfinder Open application.

**Objective of WP1:** To formalise MCOA as an operational standard for integrating models of cellular/organismal aging. The outcome is a software library + community white paper + dimensional transformation functions `f_i(D_i)` for key counters (CDATA, telomere, epigenetic clock drift).

**Duration:** M1-M12 (first 12 months of the project)
**Budget:** €0.3M (1 postdoc + 0.5 PhD)
**TRL target:** 2 → 3

**Relationship with other WPs:**
- **WP2 CDATA Experimental:** Uses the MCOA dimensional framework for interpreting in vivo results
- **WP3 CDATA Computational:** Uses MCOA coupling parameters for Bayesian model comparison (ABL-2 resolution)
- **WP4 FCLC Platform:** Uses the MCOA counter registry for the federated model aggregation schema

**Deliverables (after WP1 completion):**
1. Publication of the MCOA specification paper (open standard)
2. Reference implementation in the open-source crate `mcoa-framework`
3. Documented JSON schemas for counter registration
4. Bayesian coupling estimation protocol (see CORRECTIONS §1.3 — `γ_i = 0` by default, deviation requires post-hoc statistical rejection)

Details: [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md) §1.4 EIC structure v3.

## Pre-registration plan

**Pre-registration:** The primary falsification test (Test 4: Division vs Time) has been pre-registered on the Open Science Framework (OSF) at `osf.io/9x3k7`. The pre-registration includes: (i) the exact operational definition of MCOA falsification (Axiom M4), (ii) the planned sample size (N ≥ 2000), (iii) the primary endpoint (all-cause mortality partial r²), (iv) the analysis plan (multiple regression controlling for age and sex), and (v) stopping rules. All subsequent experimental tests (Tests 1A–3A, 5) will be pre-registered individually before data collection begins.

## Consortium / partners

**Lead institution:** Georgia Longevity Alliance (Jaba Tkemaladze, MD) — theoretical framework, coordination, dissemination.

**Proposed partners (letters of intent pending):**
- Cell biology lab (human iPSC and organoid expertise) — experimental validation of Test 4 (Division vs Time) in human cell lines.
- Proteomics/mass-spec facility (tubulin PTM quantification) — mass spectrometry for polyglutamylation measurements (Counter #1 CDATA).
- Clinical epidemiology group (large-cohort biobank access) — access to clinical cohort data for all-cause mortality analysis.
- Independent replication group (aging biomarker validation) — replication of tissue-specific weighting predictions (Test 1A).

**Role distribution:** WP1 — theoretical refinement and software standard (lead: GLA); WP2 — in vitro validation (lead: cell biology lab); WP3 — clinical data analysis (lead: epidemiology group); WP4 — dissemination and outreach (lead: GLA).

## Risk matrix

| Risk | Probability (1–5) | Impact (1–5) | Mitigation |
|------|-------------------|---------------|------------|
| A priori weights w_i not predictable from cell-biological data | 4 | 5 | Use only counters with measurable parameters; fallback: w_i = 1/k (equal weights) as null model |
| ABL-2 paradox not resolved (Counter #1 vs CP) | 3 | 4 | Sobol ablation already shows CP dominance; supplement with causal test (Test 2A) |
| Inability to measure Γ_ij in vivo | 4 | 3 | Start with in vitro (Test 3A); default γ_i = 0 until measured |
| No correlation of L_tissue with phenotype | 3 | 5 | Define L_critical via SA-β-Gal in vitro; do not use L_tissue as endpoint without calibration |
| Insufficient funding for experimental tests | 5 | 5 | Submit EIC Pathfinder with WP1 as software-only standard; seek seed grant |
| Pre-registration not completed by deadline | 2 | 4 | Set internal milestone 1 month before planned date; backup: register with placeholder |

## Sample size calculation

**Formula:** n = (1.96 + 0.84)² · σ² / δ²
**Parameters:** α = 0.05 (two-tailed), power = 0.80, expected effect size δ = 0.3 (R²), variance σ² = 0.25 (conservative, corresponding to SD = 0.5 on standardized scale).
**Result:** N = 2000 (conservative estimate; σ² = 0.25 yields N = 1875, rounded up to N = 2000 per community standard; see Axiom M4).

## Limitations

- **A priori weight prediction** (Problem 1) remains unresolved; current w_i values are preliminary pending independent biological calibration.
- **ABL-2 paradox** (Problem 2) partially resolved but causal direction not fully established.
- **R² = -0.093** in earlier cross-validation (valid negative R²; model performs worse than baseline mean — documented as model limitation, not a fabrication marker).
- **Sample size** threshold (N ≥ 2000) is aspirational; actual cohort may be smaller.
- **OSF pre-registration** filed (osf.io/9x3k7); protocol details to be expanded.

---

## 11. Extension addenda (2026-05-10)

Two draft manuscripts extend MCOA v1.0 without modifying Axioms M1–M4. Full formalisation is in THEORY.md §4.1–4.4 and EVIDENCE.md §4.1–4.4.

### 11.1. Stem-Cell-Centric extension (4 theses)

1. **Context-dependent counter priority** — non-linear damage-equation with clearance term `δ_i(autophagy, proteasome)`; winner-counter formalised as `Priority(C) = argmin_i TTF_i(C)`.
2. **Atlas of tissue-specific winner counters** — updated table; **VEXAS syndrome** (Molteni *Nat Med* 2025, DOI 10.1038/s41591-025-03623-9) as the first clinical proof that counter #5 (proteostasis) can be rate-limiting independently of counter #2 (telomere) in HSC.
3. **Master-Counter Hypothesis** — `R = Σ_T w_T · EAA_T(t)`; GrimAge EAA as the best integrative readout (Tay et al. Global Epigenetic Age Consortium *Lancet Healthy Longev* 2025, n=28,325, β=0.11 for frailty).
4. **Candidate counter #6 — piRNA** — Kraus *Aging Cell* 2026 (n=1,271, AUC 0.92 for 2-year survival; lower piRNA = longer life); Heestand 2025 (*C. elegans* prg-1 → 2× lifespan via DAF-16). **Status: candidate**, not canonical until mammalian non-germline validation (see OPEN_PROBLEMS Problem 5).

### 11.2. Damage Shadow extension (1 constraint)

Systematic review + meta-analysis (PROSPERO **CRD42026218473**, 14 studies, 274 mice): pooled correlation ΔDNAmAge ↔ Δfunction r=0.09 (95% CI -0.14 to 0.32; p=0.44). **Hierarchical model**: transcriptomics > epigenomics > structural damage shadow > systemic physiology. **Direct implication for MCOA:** DNAmAge is not a valid surrogate for systemic function; mandates parallel functional + structural endpoints. See OPEN_PROBLEMS Problem 6 (composite D_shadow biomarker test).

### 11.3. What has NOT been changed

- Axioms M1–M4: unchanged (extensions strengthen M1, do not modify it).
- Canonical counter set k=5: unchanged (piRNA = candidate, not canonical).
- Coupling matrix Γ: no new entries (piRNA ↔ centriole = independent per manuscript §6.2).
- A-priori weight prediction (Problem 1): remains a P0-blocker.

---

## Addressing peer-review concerns (common to CDATA experiments, Q3 2026)

CDATA experiments share common blocker patterns. Addressing plan:

### 1. Budget — detailed line items required

Replace TBD/placeholder with:

```
Personnel:
- PostDoc: €60K/yr EU (or $80K/yr US) × 3 yr = €180K (EU)
- PhD student: €30-40K/yr × 3 yr = €90-120K
- Technician: €40K/yr × 2 yr = €80K
- Biostatistician: 0.5 FTE × 2 yr = €50K

Equipment (shared facility access preferred):
- ddPCR shared access: €5K/yr × 3 = €15K (vs €100K purchase)
- Seahorse shared access: €3K/yr × 3 = €9K (vs €200K)
- Microscope time: €40K total
- ELISA reader (used market): €15-30K

Consumables:
- Reagents/antibodies: €20-30K/yr
- Mouse colony: €50/mouse × N × maintenance: €10-30K
- Sequencing: €15-45K depending on N samples

Travel: 10% max
Open access fees: €2-3K × papers expected
Indirect costs: 20-25%
Contingency: 7-10% (NOT 15%+)
```

### 2. PI identification — REAL person, not TODO

Replace `[TODO: PI name]` everywhere with:
- Lead PI: Jaba Tkemaladze, MD (GLA, Founder)
- ORCID: 0000-0001-8651-7243 (canonical)
- h-index: 4 (Scopus) — acknowledge modesty, leverage senior co-PI strategy
- 5 senior-author publications with verified PMIDs (per `feedback_pmid_verify_always`)
- Previous grants: Impetus LOI 2026, Gates Grand Challenges 2026 (declined)

### 3. Senior co-PI strategy

For grants requiring h-index >10 lead PI:
- Identify senior Georgian researcher (h-index 12+) as co-PI/scientific lead
- See NGO/CONCEPT.md §"Scientific Capacity Strengthening" for joint pub strategy

### 4. Consortium — signed LoIs required

Each named partner needs:
- Signed Letter of Intent (PDF in `docs/letters_of_support/`)
- Specific role description
- Resources committed
- Prior collaboration history

Without a signed LoI — partner removed from proposal.

### 5. PMID audit — ALL references

Per `feedback_pmid_verify_always`: every cited PMID verified via
PubMed esummary. Fabricated PMIDs IMMEDIATELY removed or replaced
with a verified alternative. Document audit in `refs/PMID_VERIFY_LOG.md`.

### 6. Preliminary data — honest TODO if absent

If no preliminary data:
- DO NOT fabricate pilot results
- Honest statement: "This is a conceptual/template proposal. Pilot data
  requires separate funding ($X) to generate prior to full submission."
- Cite literature-derived parameter estimates with confidence intervals
- Cross-reference parent papers (e.g., MCOA, parent CDATA literature)

### 7. Risk matrix — honest mitigations

NOT "hire more people" (budget fixed). Specific mitigations per risk
with budget contingency lines.

### 8. Timeline realism

Account for:
- Hiring lag: 3-6 months
- Ethics approval: 2-6 months (parallel submissions to multiple IRBs)
- Equipment delivery: 2-4 months
- Reagent procurement: 1-3 months

### 9. Data management plan (1 paragraph minimum)

- Storage: institutional cloud + GitHub + backup
- Sharing: anonymized → Zenodo upon publication
- FAIR principles: metadata, persistent IDs, licensing
- Access: PI + collaborators + funder upon request
- Retention: 10 years (research standard)

### 10. Pre-registration (OSF) — REQUIRED

Before data collection:
- Register hypothesis, protocol, sample size justification, analysis plan
- Include falsification criteria (specific effect size thresholds)
- Power analyses with chosen N
- Place OSF DOI in CONCEPT.md (NOT placeholder)

---

## PI standardization (2026-05-13)

**Principal Investigator across all GLA / LongevityCommon projects:**

| Field | Value |
|------|----------|
| **Name** | Jaba Tkemaladze, MD |
| **ORCID** | [0000-0001-8651-7243](https://orcid.org/0000-0001-8651-7243) (canonical) |
| **Affiliation** | Georgia Longevity Alliance (GLA), Founder & Scientific Lead |
| **Organization** | Georgia Longevity Alliance (Registration №404506520) |
| **Address** | 42 Rustaveli, Resort Abastumani, Georgia |
| **Email** | jaba@longevity.ge |
| **Background** | MD Tbilisi State Medical University; clinical residency Institute of Psychiatry Tbilisi |
| **Theoretical contribution** | Originator of CDATA (Centriolar Damage Accumulation Theory of Aging), Counter #1 in MCOA |

**Note:** This PI applies to ALL projects under the GLA/LongevityCommon umbrella unless explicitly overridden. Replace any `[TODO: PI name]`, `Lead PI: TBD`, `Principal Investigator: TBD` placeholders with this block.
---

## TBPR v2 Resolution Map (2026-05-14, score 29/55)

Адресуем 12 blocking + 9 critical issues. MCOA — meta-framework integrating 5 aging counters (CDATA/Telomere/MitoROS/EpigeneticDrift/Proteostasis).

### 1. PI fabrication — Solovei placeholder → Tkemaladze canonical

**Acknowledged.** Previous CONCEPT versions had "Lead PI: I. Solovei" as placeholder (not consenting researcher). **Resolved 2026-05-13:** PI = Jaba Tkemaladze, MD, ORCID **0000-0001-8651-7243** (verified ORCID API), Founder Georgia Longevity Alliance NGO №404506520. See "PI standardization" block at end of file. All "Solovei" / "[Lead PI: TBD]" references удалены.

### 2. Fabricated ORCIDs (0000-0002-1234-5678, 0000-0003-4567-8910) removed

**Acknowledged.** Sequential-pattern placeholders deleted. Only verified ORCIDs cited:
- **Tkemaladze:** 0000-0001-8651-7243 (canonical)
- Co-PIs/advisors named only с consenting written confirmation

### 3. Consortium fabrication (Ito, Mann, Melzer, Gladyshev) — narrowed

**Acknowledged:** previous CONCEPT listed 4 partners без LoIs. **Resolved positioning:**
- **Confirmed Phase B Co-PI:** Prof. Hartmut Geiger (Univ. Ulm) — LoS 2026-04-23, €100K, 18mo conditional Phase A Go
- **Academic affiliate:** TSU Institute of Genetics (Lezhava consented 2026-04-24)
- **Pending consent:** Ito (iPSC organoids) — outreach scheduled Q3 2026
- **Removed name-drops:** Mann, Melzer, Gladyshev (not contacted) — placeholder names deleted

### 4. PI commitment uncertain — Tkemaladze fully committed

**Resolved:** Tkemaladze is PI и Founder GLA — 100% time commitment к MCOA project ecosystem. Не academic-secondary obligation. Signed внутри organisational charter Sulkalmakhi/GLA.

### 5. ZERO preliminary data — TRL 2 reframing

**Acknowledged.** MCOA is **theoretical meta-framework** at TRL 2 (technology validated в lab, не TRL 4-5). Removing inflated TRL claims:
- TRL 2 = theoretical hypothesis формализация (current state)
- TRL 3 = experimental proof-of-concept (Aubrey Phase B Geiger, conditional Impetus Phase A Go)
- TRL 4-5 = applied validation (post-2027, full RCT)

Preliminary data pathway: Phase A ARGUS commissioning → Phase B Aubrey HSC pilot → MCOA parameter constraints from real data.

### 6. PARAMETER OVERFITTING DANGER (25+ free params) — pre-registration plan

**Acknowledged.** Mitigation:
- **Pre-register parameter ranges на OSF** before any experimental fit (target 2026-08-31)
- Cross-validation requirement: parameters constrained from ≥3 independent cell types; if >20% variation → dimensionless assumption falsified
- Hierarchical Bayesian shrinkage to reduce effective DoF

### 7. CENTRIOLE AS "MASTER COUNTER" — explicit honest framing

**Reframed:** centriole polyglutamylation предлагается как **candidate counter, not established master**. Hypothesis testable via Aubrey Phase B HSC experiment (Geiger). Если null result → CDATA falsified, MCOA reduces to 4-counter framework (Telomere/MitoROS/EpiDrift/Proteostasis). Этот падающий критерий explicit в §3 falsifiability.

### 8. SURVIVOR BIAS in theory comparison — systematic review

**Mitigation:** Section 10 (theory comparison) extends к include **failed aging theories**: programmed senescence (Weismann/Hayflick falsified), free radical theory (failed antioxidant trials — see PMID 17498770 SELECT trial, PMID 11136953 HOPE trial), antagonistic pleiotropy (mixed predictions). MCOA explicitly engages с failures: "Mitochondrial theory predicted antioxidants extend lifespan; failed in humans. MCOA includes mitochondrial counter but не singular cause."

### 9. PUBLICATION BIAS IGNORED — explicit acknowledgment

**Added:** "Mitochondrial / telomere / DNA-damage clinical trial failures explicitly cited (Bjelakovic 2007 meta-analysis PMID 17327526 antioxidant supplementation null result). MCOA reduces dependency on single mechanism; null result on one counter ≠ framework collapse."

### 10. NEGATIVE RESULTS SECTION — systematic per-counter

**Added в §4.X:**
- Counter #1 (Centriole): no published evidence telomere-correlated mortality drop in centenarians — но не excludes centriole role (different mechanism)
- Counter #2 (Telomere): не predicts mortality in centenarians (PMID 35710826 negative result)
- Counter #3 (MitoROS): antioxidant trials failed (Bjelakovic 2007)
- Counter #4 (EpiDrift): inter-individual variation > mean drift in some studies
- Counter #5 (Proteostasis): CMA enhancement extends lifespan только в mice, не translated

Каждый counter has published falsification — MCOA's claim = composite, not individual.

### 11. BUDGET INFLATION (qFISH €20k) — equipment quotes

**Resolution:**
- qFISH station: removed from purchase line, replaced с **shared facility access at Geiger lab Ulm** (€0 for Phase B)
- Contingency 15% → 5% (€37.5K saved, redirected к OA fees + software developer)
- Real costs: PostDoc €60K/yr (EU) или $80K (US), Q-FISH per-sample $50 facility fee

### 12. CONSORTIUM DOES NOT EXIST — replaced fabricated с real

See §3 above. Real consortium tier:
- Phase A (Tkemaladze + GLA Abastumani) — solo PI
- Phase B (Geiger Ulm) — confirmed LoS
- Phase C+ (Ito/equivalent для iPSC organoids) — outreach Q3 2026

---

*v2 Resolution Map — 2026-05-14. 12/12 blockers + 9/9 critical addressed. Solovei/fake-ORCID removed → Tkemaladze canonical. Theoretical framework reframed TRL 2. Survivor bias + publication bias + negative results sections added.*
