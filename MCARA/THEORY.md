<!-- AUTO-TRANSLATED via DeepSeek 2026-05-13. Source language: russian. Original preserved at THEORY.ru.md. -->

# Theoretical Foundation of MCARA

**Дата:** 2026-07-08 (v4.5 — Hardware/Software + Stress Integrator)

## 0. Hardware/Software Distinction (Jaba Tqemaladze, 2026-07-08)

**Центриоль = hardware.** Не ремонтируется in situ. UPS деградирует отдельные повреждённые центросомные белки, но нет координированного механизма quality-control, восстанавливающего всю органеллу до «молодого» состояния. Однако центриоль можно **элиминировать и создать de novo** — именно это происходит в мейозе: ооцитные центриоли элиминируются, сперматозоидные центриоли реструктурируются в seeds (не темплаты) для de novo сборки в бластомерах нового организма.

**Эпигеном = software.** Перепрограммируем факторами Яманаки, но неспособен исправить hardware-дефекты. Это объясняет, почему частичное репрограммирование омолаживает метилом, но не восстанавливает полную репликативную способность: центриолярный hardware не заменён.

## 0.1. Centriole as Conditional Entropy Carrier (Stress Integrator)

Центриоль — не автономный счётчик делений, а **стресс-интегратор.** polyE накапливается, когда центриоль работает как центросома-организатор (стресс-чувствительное состояние), и прекращает накопление, когда центриоль темплатирует цилию. Накопление зависит от функционального состояния и стресс-среды, а не только от хронологического времени.

The sinc-MT/KIFC3 pathway (Robichaud 2024, PMID 39266565) — это стресс-ответ, ведущий к cell cycle arrest. Убери стресс — polyE-массивы могут не формироваться, и сенесценс может не наступить несмотря на центриолярный «возраст».

**Это не ослабляет CEDAR:** интегратор, прогрессивно отказывающий при хроническом низкоуровневом стрессе (hTERT + гипоксия), функционально неотличим от автономного счётчика, и экспериментальное предсказание — удаление центриоли повышает эффективность репрограммирования — остаётся идентичным.

## 0.2. Centriole-Specific Kinetic Equations

### Линейное приближение
```
S_centriole(t) = S₀ + β·t + η(t) − δ·CCP(t)
CAASM(N) = CAASM₀ − λ·N_asym
```
где:
- `β·t` — время-зависимое накопление энтропии (термодинамическое)
- `η(t)` — окислительные повреждения (ROS → центриоль)
- `δ·CCP(t)` — активность деглутамилаз (снижается с возрастом). Rogowski et al. (2010, PMID 21074048): CCP1–CCP6 удаляют polyGlu. CCP1 KO → impaired osteogenesis (PMID 40349688).
- `λ·N_asym` — ремоделирование CAASM при асимметричных делениях

### Сигмоидальная модель (биологически реалистичнее)
```
S(t) = S₀ + S_max / (1 + e^(−k(t − t½))) − δ·CCP(t)
```
Критический порог `t½` — точка ускорения накопления polyE, потенциально соответствующая onset репликативного сенесценса.

### Динамика polyE баланса
```
d[E]/dt = k_E · [Stress] − k_CCP · [CCP] · [E]
```
polyE ([E]) растёт, когда стресс (k_E) превышает ёмкость CCP (k_CCP).

**Гипотеза:** Активность TTLL (глутамилаз) растёт с репликативным стрессом; активность CCP (деглутамилаз) падает с возрастом. Параметры β, λ, η(t), δ не откалиброваны — уравнения представляют тестируемые предсказания.

## 0.3. Bradford Hill Analysis (Causal Inference)

| Критерий | Оценка | Обоснование |
|----------|:------:|-------------|
| Strength | 🟡 Moderate | Plk1-асимметрия, ATF5-связывание — корреляционные |
| Consistency | 🟢 Supported | 4 типа, конвергентные данные |
| Specificity | 🔴 Weak | Множественные механизмы |
| Temporality | 🟡 Partial | sinc-MT: polyE предшествует сенесценсу |
| Biological gradient | 🟡 Predicted | polyE vs пассаж — не измерено |
| Plausibility | 🟢 Strong | Сигнальный хаб + переключатель цилия |
| Coherence | 🟢 Strong | Объясняет hTERT + гипоксия парадокс |
| Analogy | 🟢 Strong | Элиминация центриолей в germline = сброс |
| Experiment | 🔴 Absent | Центральное предсказание не тестировано |

**Итого:** ~5/9 критериев с доказательствами. Класс доказательности II–III.

---

## 1. Philosophical and Methodological Premises

MCARA is built upon the principles of **mechanism pluralism** and **strict falsifiability**. It rejects the reductionist search for a single "root cause" of aging, recognizing that multiple, partially independent damage accumulation processes can reach critical thresholds in different tissues at different times. The key methodological tenet is the prohibition of post-hoc adjustment of counter weights (`w_i`). All weights and reference scales (`n_i*`, `τ_i`) must be fixed *a priori*, based on independent biological knowledge, prior to model validation on data. This transforms MCARA from a curve-fitting tool into a set of testable predictions.

## 2. Axiomatic Foundation

**Axiom M1 (Parallel Counters).** Organismal aging is governed by at least two (`k ≥ 2`) distinct damage accumulation processes that proceed in parallel. No single counter `i` is sufficient to explain the universality of replicative limits and the diversity of tissue aging patterns. Formally: `∃ i, j (i ≠ j)`, such that for some tissues the contributions `w_i·f_i(D_i)` and `w_j·f_j(D_j)` are comparable, and the absence of either renders the model inadequate.

**Axiom M2 (Dimensional Consistency).** In the kinetic equation of a counter, direct summation of terms dependent on the number of divisions (`n`) and chronological time (`t`) is inadmissible without conversion to a common dimensionless form. The canonical form is:
`D_i(n, t) = D_i₀ + α_i · (n / n_i*) + β_i · (t / τ_i) + γ_i · I(other counters)`.
Here `n_i*` (reference number of divisions) and `τ_i` (reference time) are constants, fixed *a priori* for each counter based on cell biology (e.g., `n_i*` = Hayflick limit for the telomere counter in fibroblasts; `τ_i` = tubulin half-life for CEDAR). This ensures that `α_i` and `β_i` become dimensionless *intensities* of damage per unit of normalized scale.

**Axiom M3 (A Priori Tissue Weighting).** The weight `w_i(tissue)`, defining the contribution of counter `i` to the total tissue burden, must be predicted BEFORE the model fitting procedure to experimental aging data. The prediction is based on independent cellular-tissue parameters: basal division rate, metabolic intensity, half-life of the counter's primary substrate, expression of relevant genes (e.g., TERT for telomeres, TTLL/CCP for CEDAR), mitochondrial content. Any post-hoc adjustment of `w_i` to improve agreement with data is considered a model adjustment, not a prediction, and must be explicitly declared as a hypothesis for the next verification cycle.

**Axiom M4 (Falsifiability as a First-Order Principle).** Any statement deductively derived from MCARA must be accompanied by a description of a practically feasible experimental test whose outcome could refute that statement. The existence of such tests is a mandatory attribute of a complete theoretical construct within MCARA.

**Axiom M5 (Time-Driven Entropy + Asymmetric CAASM — Jaba Tqemaladze, 2026-07-05).** Centrioles accumulate entropy with **time**, like all material structures. **Asymmetric** divisions remodel the Centriolar Aging-Associated Signaling Module (CAASM). One inducer of irreversible differentiation detaches per asymmetric division; symmetric divisions do not alter CAASM. Two independent processes: (1) time → entropy (passive, universal), (2) asymmetric division → CAASM (active, programmatic). Full specification: CEDAR/THEORY.md.

## 3. Formal Definitions

### 3.1. Single Counter Kinetics

Damage for the `i`-th counter is described by the equation:
`D_i(n, t) = D_i₀ + α_i · (n / n_i*) + β_i · (t / τ_i) + γ_i · I(other counters)`

**Symbol Definitions:**
* `D_i`: Accumulated damage for counter `i`. Dimensionless quantity, `D_i ≥ 0`.
* `D_i₀`: Baseline damage level at birth (or in the reference young state). `D_i₀ ≥ 0`.
* `α_i`: Damage intensity driven by cell divisions. Dimensionless quantity, represents the damage increment per one reference unit of divisions (`n / n_i* = 1`). `α_i ≥ 0`.
* `β_i`: Damage intensity driven by chronological time. Dimensionless quantity, represents the damage increment per one reference unit of time (`t / τ_i = 1`). `β_i ≥ 0`.
* `γ_i`: Coupling scalar. Defines the strength of influence from other counters on the damage accumulation rate in counter `i`. `γ_i ∈ ℝ`. **Canonical default value:** `γ_i = 0` (independence hypothesis). Deviation from zero requires statistical justification based on data.
* `I(other counters)`: Influence function. Simplest linear form: `I = Σ_{j≠i} (Γ_{ij} · D_j / D_j_crit)`, where `Γ_{ij}` is a dimensionless element of the coupling matrix, `D_j_crit` is the critical damage value for counter `j`. Non-linear forms may be proposed.
* `n_i*`: Reference number of divisions for counter `i`. Fixed *a priori* (e.g., Hayflick limit for a given cell type).
* `τ_i`: Reference time scale for counter `i`. Fixed *a priori* (e.g., tubulin half-life for CEDAR, drift constant of epigenetic clocks).

### 3.2. Integrated Tissue Burden

The total phenotypic burden due to aging in a given tissue is defined as the weighted sum of transformed damage values from all counters:
`L_tissue(n, t) = Σ_{i=1}^{k} [ w_i(tissue) · f_i( D_i(n, t) ) ]`

**Conditions:**
1. `Σ_i w_i(tissue) ≈ 1.0`. A significant deviation from 1 (e.g., > 0.05) is interpreted as an indication that an important counter for this tissue is missing from the model.
2. `f_i(x)` is a monotonically increasing transformation function that converts damage to a common scale of contribution to burden. The **canonical form** is a sigmoidal (logistic) function to account for threshold effects:  
   `f_i(x) = 1 / (1 + exp(-k_i · (x - x_0_i)))`  
   where `k_i` is the steepness and `x_0_i` the midpoint (half-maximal burden) for counter `i`.  
   The linear form `f_i(x) = x` is permitted only as a first approximation when `D_i` is far below `D_critical` and no threshold has been crossed. In all other cases, the sigmoidal form is mandatory.

**Operational definition and calibration of `L_tissue`:** The integrated tissue burden `L_tissue` is dimensionless and is **calibrated against the Frailty Index (FI)**, a validated clinical measure:

`L_tissue = FI / 0.7`

where FI is the Frailty Index (range 0.0–0.7 in populations >70 years, Rockwood et al. 2005, *CMAJ*, PMID 16129869). FI = Σ(deficits present) / Σ(deficits measured) per Searle et al. 2008 protocol (*BMC Geriatr*, PMID: **18826625**). The denominator 0.7 normalises so that the maximum observed FI maps to L_tissue ≈ 1.0.

**Calibrated thresholds:**
- `L_tissue < 0.30` → young/healthy tissue (FI < 0.21, corresponds to 20–30 yr old reference)
- `L_tissue = 0.30–0.60` → age-related decline (FI 0.21–0.42, corresponds to 60–80 yr old reference, Rockwood 2005 Fig. 2)
- `L_tissue > 0.60` → `L_critical`: disease onset / high mortality risk (FI > 0.42, hospitalisation threshold per Searle 2008)
- `L_tissue > 1.00` → `L_max`: death / organ failure (FI > 0.70, near-maximal deficit accumulation, Rockwood 2005)

**Validation reference:** Gompertz mortality hazard doubles with each 0.07 increment in FI (Mitnitski et al. 2001, *ScientificWorldJournal*, PMID: **12806071**). `L_critical = 0.60` corresponds to a FI of 0.42, at which 5-year mortality exceeds 50% in septuagenarians (Rockwood 2005, Fig. 3).

These thresholds are **provisional** and will be refined during the calibration phase (MCARA Test 1). They provide a falsifiable mapping: if longitudinal data show that tissue dysfunction occurs at significantly different L_tissue values, the model must be revised.

**Caveat for HSC in vitro:** The L_tissue calibration above is based on **population frailty indices (whole-organism)**, not on tissue-specific HSC markers. For HSC in vitro, an independent calibration against NSG mouse transplantation readouts (reconstitution potential, bone marrow chimerism) is required. This is planned as part of the Phase III in vivo component (budget line: NSG mice).

### 3.3. Functional Transition (Senescence/Dysfunction)

A cell or tissue niche transitions into a state of senescence, apoptosis, or pronounced dysfunction upon fulfillment of one of two conditions:
1. `L_tissue(n, t) > L_critical(tissue)`, where `L_critical` is a tissue-specific threshold for integrated burden.
2. `∃ i : D_i(n, t) > D_critical(i, tissue)`, where `D_critical` is a tissue-specific threshold for a specific counter (e.g., critical telomere shortening).

## 4. Canonical Set of MCARA Counters (v4.5 — Gatekeeper article)

> 🔄 **Нумерация обновлена 2026-07-08** для соответствия MCARA.docx (Gatekeeper article).
> Старая нумерация (v1.0): #1=CP, #2=Telomere, #3=MitoROS, #4=Epigenetic, #5=Proteostasis.
> Новая нумерация (v4.5): #1=Centriolar, #2=Epigenetic, #3=Mitochondrial, #4=Structural, #5=Telomeric.

| # | Name | Project | Nature | `n_i*` (Anchor) | `τ_i` (Anchor) | Comment |
|---|------|--------|---------|----------------|---------------|---------|
| 1 | **Centriolar Polyglutamylation (CP)** | CEDAR | Divisions + Time | ~50–80 (HSC), ~30–50 (epithelium) | Months–Years (mass spectrometry) | Stress integrator. polyE accumulation + CAASM. `α_i` significant, `β_i` driven by tubulin turnover. **Фаворит гонки** — без защиты. |
| 2 | **Epigenetic Drift (SA-DNAm)** | EpigeneticDrift | Dominantly Time + Divisions | `α_i → 0` for most cells; `α_i > 0` in proliferating compartments | Doubling time of epigenetic age (~3.6 years DunedinPACE) | SA-DNAm trajectory (Wagner 2013). TERT does NOT prevent. EPIC arrays (850K CpG). Замедлен гипоксией на ~30-40%, но тикает. |
| 3 | **Mitochondrial ROS / mtDNA Damage** | MitoROS | Dominantly Time | `α_i → 0` for postmitotic cells | Days–Weeks (mtDNA damage turnover) | ROS повреждает центриоль (η(t)), но не originates from it. **Защищён 2% O₂.** |
| 4 | **Structural (Centriole/Cilium)** | CEDAR | Divisions + Time | ~50–80 (follows C1) | Months–Years | Tubulin code (polyE, detyrosination). Цилия ↔ центриоль переключатель. **Ведомый** — следует за C1. |
| 5 | **Telomere Shortening / Telomere Stress** | Telomere | Dominantly Divisions | Hayflick limit (~50 for fibroblasts) | Weeks (telomeric repeat turnover) | Классический репликативный счётчик. **Снят с гонки** теломеразой (hTERT). `β_i ≈ 0` для большинства соматических клеток. |

**Order and Rationale:** Counter CP (#1) = фаворит гонки — stress integrator без защиты. C2 (эпигенетический) бежит независимо, замедлен гипоксией. C3 (митохондриальный) защищён 2% O₂. C4 (структурный) следует за C1. C5 (теломерный) снят с гонки теломеразой. **Судья гонки:** Geiger (Ulm) — HSC трансплантация, in vivo валидация победителя.

### 4.1. Candidate counter #6 — piRNA (placeholder, NOT canonical in v1.0)

| # | Name | Project | Nature | `n_i*` (Anchor) | `τ_i` (Anchor) | Comment |
|---|------|--------|---------|----------------|---------------|---------|
| 6 | **piRNA-mediated regulation (circulating piRNA as marker/driver)** | piRNA (TBD subproject) | Dominantly Time | `α_i → 0` for most somatic cells; `α_i > 0` in active stem and germline compartments | TBD — half-life of PIWI/piRNA cluster transcripts; not formalized for mammalian soma | Candidate for counter #6. Empirical evidence: Kraus et al. *Aging Cell* 2026 (n=1,271 ≥71 years, Duke-EPESE) — 9 piRNA → 2-year survival AUC 0.92 (Discovery), 0.87 (External Validation), surpasses >180 clinical markers; **lower piRNA = longer survival** (direction opposite to classic "protective" view). Heestand et al. *Aging Cell* 2025 — prg-1 mutation doubles lifespan *C. elegans* via DAF-16/FOXO. **Barrier to inclusion in canonical set:** piRNA mechanisms outside germline in mammals are poorly characterized; requires validation in mammalian non-germline context. Until then, status is **candidate**. |

**Relationship with other counters:** piRNA functions are **independent** of centriolar mechanisms (#1) — coordination via common signaling (CCR4-NOT translation control, P-bodies/SG), but no direct interactions identified. This justifies classification as a separate counter, rather than a sub-counter of #1 or #4.

**Therapeutic relevance:** 9 piRNA identified as potential therapeutic targets (Kraus 2026); GLP-1-induced shifts in piRNA profiles and lifestyle-interventions (exercise) are actively studied. Require independent confirmation prior to operationalization in MCARA protocol.

### 4.2. Supporting Evidence for Counter #5 (Proteostasis) — VEXAS Syndrome (with caveats)

**VEXAS** (Vacuoles, E1 enzyme, X-linked, Autoinflammatory, Somatic) is an acquired syndrome caused by a somatic mutation in *UBA1* (Met41) in HSC. Prevalence ~1:4,000 in men >50 years, 50% 5-year mortality. Molteni et al. *Nature Medicine* 2025 (DOI 10.1038/s41591-025-03623-9) demonstrated:
1. Dysfunctional UBA1c isoform → loss of protein ubiquitination
2. ER stress, UPR activation (BiP↑), accumulation of misfolded proteins
3. Senescence-like programs in UBA1-mutant HSPC
4. Clonal dominance of mutant cells; depletion of wild-type
5. Progressive bone marrow failure

**Critically:** VEXAS patients have no telomerase mutations and telomeres are **not shortened**. This suggests that proteostatic disruption alone can drive HSC failure independently of telomere shortening — consistent with counter #5 operating as an independent axis.

**CRITICAL CAVEAT:** VEXAS is a **monogenic pathology** (somatic UBA1 mutation), not a model of physiological aging. Extrapolating from VEXAS to normal gerontogenesis requires caution: 
- The mechanism is acute (proteotoxic crisis driven by a single mutation), not the gradual, multi-factorial decline seen in aging.
- VEXAS involves clonal dominance and autoinflammation, which are not typical of the proteostatic counter in normal aging.
- Therefore, VEXAS is **illustrative but not definitive** proof of counter #5 independence. It supports the plausibility of the axis but does not replace direct gerontological evidence.

**Additional Support (not dependent on VEXAS):** Keyvani Chahi et al. *Blood* 2022 (DOI 10.1182/blood.2021014602) — overexpression PLAG1 → **15.6-fold** enhancement of functional HSC frequency via suppression of translation (4EBP1↑, miR-127↑) independently of c-MYC. Catic *Trends Cell Biol* 2025 — HSC maintain low translation rates; increased translation without autophagy compensation → toxic aggregation.

**Additional Support:** Keyvani Chahi et al. *Blood* 2022 (DOI 10.1182/blood.2021014602) — overexpression PLAG1 → **15.6-fold** enhancement of functional HSC frequency via suppression of translation (4EBP1↑, miR-127↑) independently of c-MYC. Catic *Trends Cell Biol* 2025 — HSC maintain low translation rates; increased translation without autophagy compensation → toxic aggregation.

**Therapeutic Implications (Atlas, Draft):**
- **HSC:** Autophagy activators, PLAG1 modulation (counters #5 ± #1).
- **VEXAS:** NLRP3 inhibitors, JAK inhibitors, allogeneic HSC transplantation (target #5-driven inflammation).
- **ISC (intestinal stem cells):** Telomerase activation (counter #2).
- **New Directions:** piRNA-targeted therapy, GLP-1 effects on piRNA profiles (counter #6 candidate).

### 4.3. Master-Counter Hypothesis (Extension Thesis, Draft)

The rate of organismal aging `R` is defined as a weighted function of stem cell failure:

`R = Σ_T w_T · EAA_T(t)`

where EAA (Epigenetic Age Acceleration) serves as a systemic readout of the master counter for each tissue T. **Empirical Basis:** Tay et al. (Global Epigenetic Age Consortium) *Lancet Healthy Longevity* 2025 (DOI 10.1016/S2666-7568(25)00128-2), meta-analysis N=28,325 — GrimAge EAA has the strongest association with frailty (β=0.11, 95% CI 0.06–0.15, I²=90.5%); PhenoAge β=0.07; DunedinPACE β=0.10.

**Consequence for MCARA:** GrimAge is the best integrative marker of the master counter; its use as a primary endpoint is justified, **but** see damage shadow rule (THEORY §4.4): epigenetic reset ≠ functional rejuvenation.

### 4.4. Damage Shadow Constraint (Extension Thesis, from Meta-Analysis 2026-05-10)

A systematic review and meta-analysis (PROSPERO **CRD42026218473**, n=14 studies, 274 mice) showed that reduction in DNAmAge upon partial reprogramming **does not translate** into systemic functional improvement in natural aging models: pooled Fisher's Z = 0.09 (95% CI -0.14 to 0.32; p=0.44). A threshold ΔDNAmAge ≈ -2.4 yrs-equiv was identified, above which modest tissue-specific gain appears (but not systemic).

**Damage Shadow** — structural/molecular damages **not corrected by epigenetic reset**:
1. ECM cross-linking (AGE, pentosidine)
2. mtDNA mutations (heteroplasmy)
3. Nuclear non-epimutation damage (DSB, telomere attrition, lipofuscin)
4. Protein aggregation (Aβ, tau, etc.)

**Hierarchical Model of Rejuvenation Potential** (supports MCARA pluralism):

| Level | Plasticity | Reversibility by Partial Reprogramming |
|-------|------------|----------------------------------------|
| Transcriptomics (mesenchymal drift, Li & Tay 2026) | High | Yes |
| Epigenomics (DNAmAge) | Moderate | Yes |
| Structural damage shadow | Low | No |
| Systemic physiology | Very Low | No (current evidence) |

**Tissue-Specific Exceptions** (refine, not refute): Lu et al. *Nature* 2020 — RGC vision restoration after optic nerve crush; Berdugo-Vega et al. *Neuron* 2026 — engram neurons cognitive restoration. Both are highly plastic cell-type specific contexts, not systemic natural aging.

**Direct Consequence for MCARA:** DNAmAge is **not a valid surrogate endpoint** for systemic functional outcomes; parallel functional validation is necessary. Confirms M1: single-counter intervention (only #4) is insufficient for rejuvenation.

See manuscripts "Stem-Cell-Centric Multi-Counter Theory of Organismal Aging" and "Epigenomic Rejuvenation Without Functional Restoration" (both NOT YET PUBLISHED, draft 2026-05-10).

## 5. Coupling Matrix Between Counters (Γ)

The matrix `Γ ∈ ℝ^{k×k}` defines directed influence: element `Γ_{ij}` is the rate at which accumulated damage in counter `j` accelerates damage accumulation in counter `i`.

**Known (from literature) Proposed Non-Zero Couplings:**
* `Γ_{telomere, mito} > 0`: Oxidative stress (mitochondria) accelerates telomere shortening (Parrinello et al., 2003).
* `Γ_{epigenetic, mito} > 0`: Mitochondrial signals (NAD+/NADH) influence the activity of epigenetic modifiers (Rajman L, Chwalek K, Sinclair DA. *Cell Metab*. 2018, PMID: **29514064** — NAD-boosting molecules review).

**Hypothetical couplings (NO published evidence; included only as testable predictions; default = 0):**
* `Γ_{centriole, epigenetic} > 0` (hypothetical): The author hypothesizes that epigenetic dysregulation could alter TTLL/CCP enzyme balance affecting polyglutamylation. **No published data support this.** Janke & Magiera (2020) describe the tubulin code but do **not** provide evidence for epigenetic regulation of TTLL/CCP. Default value: 0 (independence).
* `Γ_{proteostasis, mito} > 0` (hypothetical): Mitochondrial ROS may accelerate protein aggregation; conversely, proteostatic dysfunction may impair mitophagy. Plausible but not quantitatively established. Default value: 0.

**Key Rule:** Elements `Γ_{ij}` (and consequently `γ_i` in simplified form) must be **measured** in controlled experiments (see MCARA Test 2), not be free parameters for fitting. This separates causal inference from correlational analysis.

## 6. Predictions of MCARA Theory

1. **Heterogeneity of Dominant Counters:** In different tissues, the same counter will have a different weight `w_i`. For example, in the liver (`low division rate`), the weight of mitochondrial and epigenetic counters will be higher than that of the telomere counter.
2. **Non-Linear Response to Interventions:** The effect of an intervention targeting a specific counter (e.g., a telomerase activator) will be maximal in tissues where `w_telomere` is large, and minimal where it is small.
3. **Synergy of Targeted Interventions:** Combined intervention on several counters with high `w_i` in a given tissue will yield a super-additive effect on healthspan extension, whereas intervention on irrelevant counters will not.
4. **Existence of "Uncoupled" Tissues:** Tissues can be identified where the total burden `L_tissue` remains low, despite high values of one counter (`D_i`), due to compensatorily low weights of other counters.
5. **Prediction of Aging Trajectories:** Given known *a priori* `w_i(tissue)`, `n_i*`, `τ_i` and initial `D_i₀`, the model predicts the trajectory of burden accumulation `L_tissue(t)` for each tissue, which can be verified in longitudinal studies.

## MCARA Phase III Update (2026-05-15)

CONCEPT.md replaced with MCARA Phase III v2.0 corrected concept.
- All PMIDs verified (11 PMIDs, see CONCEPT.md §1.1)
- Power calculation for HSC clones added (n=18, α=0.0042)
- Phase III: 6 arms validating Counters #2–#6 using ARGUS (see CONCEPT.md §3)
- Budget: €351,600 (conditional on Phase A + Phase B)

---

**Source code:** [github.com/djabbat/LC](https://github.com/djabbat/LC/tree/mcaoa-v3.2/MCARA) (branch `mcaoa-v3.2`, в `MCARA/`)  
**Preprint:** DOI [10.5281/zenodo.20055806](https://doi.org/10.5281/zenodo.20055806)