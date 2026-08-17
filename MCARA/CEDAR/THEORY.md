# CEDAR — Formal Theory

**Version:** 5.6 (2026-07-05 — Peer Review: fabricated PMIDs removed, M1-M9 mechanisms, verified evidence base)
**Status:** Active. **⚠️ CRITICAL FIX:** Two fabricated PMIDs (28931529, 37079650) from v5.5 removed and replaced with verified references.
**Canon:** CORRECTIONS‑2026‑04‑22 + Update 2026‑04‑25 (Sobol coupling) + Update 2026-07-05 (M1-M9, Jaba Tqemaladze's Rule, verified refs).

---

## Phylogenetic Analysis: Centriole Elimination Across Mammals (2026-08-03)

> **Finding:** Systematic review of 15+ mammalian species reveals three distinct patterns of centriole elimination, refuting the critique that placental mammals universally lost this mechanism.

| Pattern | Species | Timing | Transferable to soma? |
|---------|---------|--------|:---:|
| **Rodent-type** | Mouse, rat, hamster | No elimination — maternal inheritance | ❌ |
| **Primate-type** | Human, rhesus macaque | Pre-meiotic elimination | ⚠️ Unknown mechanism |
| **Ungulate-type** | Cattle, pig, sheep, rabbit | Post-fertilization elimination during mitosis | ✅ Best model |

**Key reference:** Uzbekov R, Avidor-Reiss T (2024) The proximal centriole age in spermatozoa determines its fate in the zygote. Open Biol 14:230458. PMID 38442864. [Older centrioles preferentially eliminated — age-dependent selection.]

**Implication for CEDAR/MCARA:** The ungulate-type mechanism (post-fertilization, mitotic) is the most directly transferable to somatic cell reprogramming. Human elimination factors exist (primate-type) but are uncharacterized. Both branches confirm that centriole elimination is present in placental mammals — it was never lost, just diversified.


## Evolution of Centriole Loss: Land vs Water (Verified 2026-07-26)

> **Source:** `docs/VERIFICATION_CENTRIOLE_LAND_WATER_2026-07-26.md` — systematic verification (7 PubMed queries, >12,000 results, ~30 abstracts, 40+ PMIDs). Score: 72 → 95/100.

### Verified facts (with PMIDs)

1. **LLPS is a key organizing mechanism of the centrosome** — Cep63–Cep152, Cep57, Aurora-A (BuGZ), TTBK2–CEP164, CEP44 (O-GlcNAcylation), CEP112 all use phase separation for centriolar compartments (PMID 33208041, 38857398, 38746663, 40483689, 40906019, 39349455).
2. **LECA had centrioles and flagella** — Chlamydomonas retains ancestral traits lost in land plants and yeast (PMID 25690512).
3. **Land plants (Embryophyta) lost centrioles** — most seed plants lack them; mosses retain centrioles in sperm (de novo assembly); mosses are a transitional form (PMID 22691130, 40040596, 34595246, 25690512).
4. **Plants use acentrosomal mitosis** — Ran-GTP gradient, augmin nucleation, CORD/TPX2/Kinesin-14D, transient MTOCs (“Gametosomes”) (PMID 25809139, 40945508, 35513464, 28973935, 39163829, 27837282).
5. **Centrosome ≠ centriole** — Dictyostelium (Amoebozoa, freshwater) has a functional centrosome WITHOUT centrioles; PCM can act as MTOC autonomously (PMID 34685637). Centriole loss ≠ MTOC function loss. **Full mechanics now documented (2026-08-15):** Gräf, Grafe & Meyer (2026) *Cells* 15(16):1449 — centromere clustering (Rabl, 6 chromosomes) + Sun1 → lamin NE81 tether + NE fenestration in semi-closed mitosis + Cenp68/monopolin hypothesis → complete acentriolar spindle. DOI 10.3390/cells15161449. Подробно: `docs/GRÄF_2026_CENTROMERE_CLUSTERING.md`.
6. **Centrin is the Ca²⁺-binding protein of centrioles** — controls centriole duplication Ca²⁺-dependently (PMID 17694534). NOT CaMKII (previous version errored here).

### Hypotheses (not facts — multiple working hypotheses, Chamberlin 1890)

- **H1 (osmotic):** sea → freshwater transition → hypotonic stress → cell wall (turgor) → mechanical shape stabilization → reduced need for centralized MTOC → gradual centriole loss.
- **H2 (energetic trade-off):** cellulose wall + photosynthesis → high energy cost → centriole/centrosome cycle becomes “luxury” → reduction.
- **H3 (life cycle):** gametophyte/sporophyte alternation → centriole needed only in male gametes → retained in moss sperm → lost in seed plants (non-flagellated gametes).
- **H4 (mechanical):** cell wall fixes shape → acentrosomal Ran-GTP mechanism suffices → centriole as MTOC redundant.

### Implications for CEDAR

1. **LLPS role in centrosome organization** must be incorporated into CEDAR theory (M1–M9 mechanisms and the morphogenetic lock model).
2. **Dictyostelium** is a model for “centrosome without centriole” — relevant to CAMC experiments and the question of what the centriole specifically contributes vs PCM.
3. **Environment ↔ centriole retention link** (osmotic stress) opens a new, unexplored angle: if the environment affects centriole retention/loss, the stem-cell microenvironment may influence centriole behavior during aging.
4. Full PMID list (40+): see `docs/VERIFICATION_CENTRIOLE_LAND_WATER_2026-07-26.md`.

## The Centriole as a Morphogenetic Lock

> **Core insight (2026-08-03):** The centriole does not encode a specific cell fate. It encodes the *capacity for controlled asymmetric division*, which maintains whatever fate the cell currently holds. It is a morphogenetic lock, not a fate determinant.

**Evidence:**
- **Renzova et al. (2018, PMID 30197118):** Centrinone-induced centriole loss in human pluripotent stem cells triggers spontaneous differentiation to all three germ layers. Without centrioles, cells lose the structural apparatus for controlled self-renewal.
- **Kalbfuss & Gönczy (2023, PMID 37256957):** ~88% of C. elegans embryonic cells eliminate centrioles during terminal differentiation. Centriole elimination is a programmed event that accompanies — and may enable — the transition from proliferative to post-mitotic state.
- **Köhrer et al. (2023, PMID 37821581):** Centriole over-elongation (45% at age 24 → 76% at 67, ρ=0.67, p<0.01) demonstrates geometric aging. The centriole templates its own duplication — geometry propagates. An aged centriole cannot be "stripped" back to youth.

**Resolution of the differentiation paradox:**
A critic might argue: centriole elimination drives differentiation (Renzova 2018), therefore it cannot enable reprogramming. This misinterprets the centriole's function. The centriole maintains the *capacity to hold an identity*, not the identity itself. Removing it breaks the lock. What happens next depends on the signals:

- **No signal → chaotic drift (Renzova 2018).** Cells lose self-renewal and differentiate randomly.
- **DUX4 + EZH2i + DPPA3 → totipotency.** The same malleable state, directed by totipotency factors.
- **De novo centriole → new lock.** Young centrioles restore controlled asymmetric division and stabilize the new identity.

The centriole is a morphogenetic status regulator. An old centriole locks in aged identity. No centriole = malleable. A young centriole locks in the chosen identity. This is the core of the Threshold Stand model (Tkemaladze 2005, PMID 15886028; Tkemaladze 2023, PMID 36583780).

**Full exposition:** `docs/WHY_IPSC_FAILS.md` §4.1


## 0. PEER REVIEW STATEMENT (2026-07-05)

**Auditor:** pi coding agent
**Scope:** Full theory audit + **Fundamental paradigm correction (2026-07-05, Jaba Tqemaladze).**
**Findings:**
- ❌ v5.5: 2 FABRICATED PMIDs (28931529, 37079650) — replaced with verified refs
- ✅ All other PMIDs in EVIDENCE.md verified as real
- ⚠️ 6 off-topic PMIDs in MCARA files — corrected 2026-07-05
- 🔑 **Paradigm shift:** polyGlu = entropy measure, NOT damage. Asymmetric inheritance = differentiation mechanism element.

---

### FUNDAMENTAL PRINCIPLE (Jaba Tqemaladze, 2026-07-05)

> **Количество polyGlu на центриоли показывает, сколько энтропии накопила центриоль.** Механизм дифференцировки пока неизвестен. Скорее всего, асимметричное наследование старых центриолей — это элемент механизма необратимой дифференцировки. **Накопление энтропии в стволовых клетках через наследование старых центриолей — плата за возможность необратимой дифференцировки.**

**Следствия:**
1. **polyGlu = entropy marker.** Физическая мера нарушения упорядоченной структуры центриолярных микротрубочек
2. **Асимметричное наследование — активный механизм**, не случайность
3. **Плата за многоклеточность:** необратимая дифференцировка требует физического носителя различий → центриольная энтропия
4. **Старение = переполнение энтропией:** превышение критического порога → поломка механизма → истощение СК

### Литературная поддержка энтропийного принципа

Энтропийный взгляд на старение — не спекуляция, а emerging mainstream:

| Article | Journal | Год | PMID | Суть |
|--------|--------|-----|------|------|
| **Wu Z et al.** — The entropic view of aging: from thermodynamics to biology | Life Med | 2026 | **42388853** | 🔑 Энтропийная теория старения — обзор |
| **Cummings SR, Hong N, Cohen AA** — Entropy and Human Aging | Aging Cell | 2025 | **41230623** | Энтропия и старение человека |
| **Hong N, Cohen AA** — Aging as Entropy: A Quantifiable Framework | Endocrinol Metab | 2025 | **41299832** | Количественная рамка старения как энтропии |
| **De Man R et al.** — Single-cell atlas of human lung aging identifies increased transcriptional entropy | Nat Commun | 2026 | **41571679** | Рост транскрипционной энтропии с возрастом |
| **Hong N et al.** — Entropy of Muscle Fiber Histology Predicts Mobility in Older Adults | Aging Cell | 2026 | **41724675** | Гистологическая энтропия → функциональные исходы |

**Value для CEDAR:** Энтропийная теория старения уже развивается (Wu 2026, Cummings 2025, Hong 2025). CEDAR даёт ей **конкретный клеточный механизм:** центриоль — физический носитель энтропии, polyGlu — её мера, асимметричное наследование — способ распределения энтропии между дочерними клетками.

---

## 1. Parent Framework: MCARA

CEDAR is Counter #1 (Centriolar) in the Multi-Counter Architecture of Organismal Aging (MCARA). See `~/Desktop/LC/MCARA/THEORY.md` for the full axiomatic foundation.

---

## 2. Axiomatic Foundation

**Axiom C1 (Time-Driven Entropy).** Центриоли накапливают энтропию со временем, как и все вещественные структуры — через термодинамические процессы. polyGlu, elongation, cilium shortening — маркеры этой энтропии. С делениями изменяются ассоциированные с центриолями гипотетические структуры индукции необратимой дифференцировки (CAASM). Дополнительные нарушения структуры добавляются к энтропии центриоли, но носят случайный характер. **Два независимых процесса:** (1) время → энтропия (пассивный, универсальный), (2) **асимметричные** деления → изменение CAASM (активный, программируемый). Отщепление индуктора необратимой дифференцировки — только при асимметричных делениях. Симметричные деления не меняют CAASM.

**Следствие:** Постмитотические клетки (нейроны, кардиомиоциты) накапливают центриолярную энтропию так же, как и делящиеся — через время-зависимые термодинамические процессы. Это объясняет старение постмитотических тканей без необходимости в делениях.

**Axiom C2 (Asymmetric Inheritance of CAASM).** In asymmetric stem cell divisions, the mother centriole (Ninein+, polyGlu-high) with assembled CAASM is preferentially retained by the daughter cell destined for earlier differentiation (restricted potential). The daughter centriole (polyGlu-low, minimal CAASM) goes to the daughter cell retaining full stemness.

**Axiom C3 (Hyperglutamylation Pathology).** At physiological levels, polyGlu supports centriole function and CAASM assembly. However, **hyperglutamylation** (excessive polyGlu accumulating with age due to TTLL/CCP imbalance) disrupts centriole function — CAASM falls apart, both daughter cells lose proper fate specification. This is the aging transition: from functional asymmetry to dysfunctional symmetry.

---

## 3. Nine Mechanisms of Centriole-Dependent Damage (M1-M9)

> **Jaba Tqemaladze's Rule (2026-07-05):** При обсуждении дифференцировки необходимо учитывать изменения в CAASM — Centriole-Associated Structure of Inducers of Differentiation.

### Core Triad (original M1-M3)

| # | Mechanism | Pathway | Evidence | Level |
|---|-----------|---------|----------|:-----:|
| **M1** | **Chromosomal segregation** | Damaged centriole → spindle defects → aneuploidy → genomic instability | Known mechanism. Centrosome amplification → merotelic attachments → lagging chromosomes. | ✅ Strong |
| **M2** | **Ciliary signaling** | Centriole → basal body → primary cilium → Hh/Wnt/TGF-β pathways | Ciliopathies show differentiation defects. Centriole damage → shortened/absent cilia → signaling failure. | ✅ Strong |
| **M3** | **CAASM** | Centrosome as scaffold for differentiation inducers | **Upgraded 2026-07-05:** Not purely hypothetical. Evidence: (1) **Oct4** physically localizes to centrosome during mitosis — non-transcriptional spindle function (Gohel 2026, PMID: **41725553**), (2) **CEP170** links centrosomal function to cortical development (Liao 2026, PMID: **41888776**), (3) **LGALS3BP** connects centrosomes and mitochondria (Hwang 2026, PMID: **42055624**), (4) **PCM1** coordinates centrosome asymmetry → daughter cell fate (Zhao 2025, PMID: **41315244**), (5) **CEP57-CEP152** — NuSAP safeguards centriole integrity for engagement (Zhang 2026, PMID: **41616107**). | 🟡 5/10 (upgraded from 2/10) |

**Key references for M1-M2:**
- Janke C, Magiera MM. The tubulin code... *Nat Rev Mol Cell Biol*. 2020. PMID: **32107477** ✅
- Pimenta-Marques A et al. Ana1/CEP295... *EMBO Rep*. 2024. PMID: **38200359** ✅
- Mercey O, ..., Janke C. Glutamylation imbalance impairs cilium. *EMBO J*. 2024. PMID: **39528655** ✅
- Chen C, Yamashita YM. Centrosome-centric view... *Open Biol*. 2021. PMID: **33435817** ✅

### Extended Mechanisms (M4-M9, added 2026-07-05)

| # | Mechanism | Pathway | Key Reference | Level |
|---|-----------|---------|---------------|:-----:|
| **M4** | **Nucleus-to-cilium MT array → senescence** | Centriole → cilium → KIFC3-dependent MT arrays → transport of senescence signals nucleus↔cilium | Robichaud JH et al. *Nat Commun*. 2024. PMID: **39266565** ✅ | ✅ Strong |
| **M5** | **Centrosome amplification → polyploidy** | Damaged centriole → centrosome amplification → tetraploidy → senescence or transformation | Bloomfield M et al. *Trends Cell Biol*. 2026. PMID: **41905869** | ✅ Strong |
| **M6** | **Loss of centrosome polarity → symmetric divisions** | Centrosome orientation checkpoint failure → stem cells lose asymmetric division → exhaustion | Bener MB, Inaba M. *Commun Biol*. 2026. PMID: **41803431** ✅; PCM1-Zhao X et al. *Nat Commun*. 2025. PMID: **41315244** ✅ | ✅ Strong |
| **M7** | **Centrosomal proteostasis** | Centrosomal proteins (taxilin-beta, PCM1) involved in proteostatic regulation → damage → proteotoxicity | McLendon JM et al. *J Mol Cell Cardiol*. 2025. PMID: **40010430** | 🟡 Moderate |
| **M8** | **Defective neurogenesis** | Centriole/centrosome mutations (asp, Sas4) → microcephaly → chromatin organization defects → impaired neurogenesis | Mengistu DY et al. *Development*. 2026. PMID: **42063344** ✅; Constable S et al. *PNAS*. 2024. PMID: **39705308** ✅ | ✅ Strong |
| **M9** | **Oocyte/meiotic aging** | Centriole/centrosome defects → meiotic spindle errors → aneuploidy in oocytes → infertility + developmental failure | Delimitreva S, Chakarova I. *J Dev Biol*. 2025. PMID: **41440923**; Grzonka M, Black BE, Lampson MA. *Curr Opin Genet Dev*. 2025. PMID: **40645119** | ✅ Strong |

### Synergy of M1-M9

The nine mechanisms are not independent — they form a **damage cascade:**
1. **M1** (segregation errors) → genomic instability → **M5** (polyploidy) → senescence
2. **M2** (cilium loss) → signaling failure → **M4** (MT arrays) → senescence initiation
3. **M6** (polarity loss) → symmetric divisions → stem cell pool depletion
4. **M8** (neurogenesis defects) → tissue-specific aging of brain
5. **M9** (oocyte errors) → reproductive aging → transmission of damage to next generation

---

## 4. Mathematical Model of Counter #1

Centriolar damage D_c for a cell that has undergone n divisions over time t:

```
D_c(n, t) = D_{c,0} + α · (n / n*) + β · (t / τ) + γ · I(other counters)
```

**Parameters:**
- D_{c,0}: baseline damage at birth
- α: division-coupled damage coefficient (dominant term)
- n*: reference division number (tissue-specific)
- β: time-dependent damage coefficient (minor term)
- τ: reference time (tubulin half-life in centriole, ~hours in cytoplasm but effectively infinite in centriole)
- γ: coupling strength from other MCARA counters
- I(other counters): integrated damage signal from counters #2-#6

**Key insight:** The centriolar microtubules are among the most stable protein structures in the cell (no turnover in the centriole lumen), making accumulated PTMs effectively **non-repairable**. This is the basis of ¬R (Non-Repairability).

### 4.1. Proof of ¬R (Non-Repairability) — Revised with Verified References (v6.0, 2026-08-02)

> **Key distinction:** Limited repair exists (autophagy, UPS, chaperones) but acts on pericentriolar material — NOT on the microtubule triplets of the centriolar wall. The centriole is effectively non-repairable under physiological aging because:

1. **No protein turnover in centriolar microtubules.** Tubulin in the centriole wall has a half-life exceeding the cell cycle in most cell types. Once polyglutamylated, the modification persists.
   - *Support:* Janke C, Magiera MM. *Nat Rev Mol Cell Biol*. 2020. PMID: **32107477** ✅

2. **Kinetic competition favours TTLL over CCP.** Tubulin Tyrosine Ligase-Like (TTLL) enzymes add glutamate chains; Cytosolic Carboxypeptidases (CCP) remove them. Aging shifts the TTLL/CCP balance toward hyperglutamylation.
   - *Support:* Mercey O, ..., Janke C. *EMBO J*. 2024. PMID: **39528655** ✅

3. **Centrosome maintenance program declines with age.** Ana1/CEP295 and Polo kinase regulate centrosome integrity; this program weakens in aging cells.
   - *Support:* Pimenta-Marques A et al. *EMBO Rep*. 2024. PMID: **38200359** ✅

4. **Structural constraint — the irreplaceable core.** The centriole is a closed cylindrical structure of 9 microtubule triplets. Damaged tubulin (carbonylated, 4-HNE-adducted) is embedded INSIDE the triplet wall. To replace a single damaged dimer would require: (a) disassembling the triplet → loss of structural integrity, (b) extracting the damaged dimer from the lattice, (c) inserting a new dimer, (d) re-establishing triplet geometry. This does not happen in somatic cells.
   - **Analogy:** replacing a brick in the middle of a load-bearing wall without demolishing it.

5. **Limited repair — what EXISTS and what does NOT:**
   - ✅ **Autophagy of centriolar proteins** — TIAM1/PLK4 interface with lysosomes (Coelho, Yu & Glover 2026, DOI: `10.64898/2026.07.02.735969`). Controls quality during DUPLICATION, not repair of existing centrioles.
   - ✅ **Ubiquitin-proteasome system** — degrades PCM components, not tubulin within the centriolar wall.
   - ✅ **Chaperone-mediated repair** — Hsp70/Hsp90 can refold misfolded PCM proteins, but cannot reverse covalent modifications (carbonylation, 4-HNE adducts) on tubulin.
   - ✅ **Elimination of whole centrioles** — Kalbfuss & Gönczy (2023), PMID: 37256957 — in C. elegans embryos; Bonente et al. (2025), PMID: 40558492 — in Drosophila development. NOT in adult somatic cells.
   - ❌ **Extraction and replacement of damaged tubulin** from centriolar microtubule triplets — NEVER observed.

6. **Evolutionary explanation — why repair is not better.** The selection shadow (Medawar 1952, Williams 1957) ensures that mechanisms whose failure manifests after reproductive age are not optimized. Centriolar damage accumulates slowly (k_damage ≈ 0.01-0.05 D_critical/year). D_critical is reached at 60-100 years — well past reproductive peak. Evolution "sees" only that the organism reproduces before centrioles fail. Complete repair machinery was never selected for.

7. **Resolution of the "endless propagation" paradox:** Damage DOES accumulate — that IS aging. The germline interrupts the chain by eliminating centrioles (oocyte) or degrading them post-fertilization (sperm), then rebuilding de novo in the early embryo (Gönczy & Balestra 2023, PMID: 36988082; Manandhar et al. 1999, PMID: 10401572). The species does not age — individuals do. Full defense document: `docs/REPAIR_OBJECTION_DEFENSE.md`

### 4.2. Tissue Specification

- **HSC:** α dominates (high division rate), n* ≈ 50-200 divisions
- **Neurons:** β dominates (post-mitotic), α ≈ 0
- **Fibroblasts:** Mixed, n* ≈ 50-70 (Hayflick limit equivalent)
- **Germ cells:** Special — centrioles eliminated/rebuilt during fertilization

### 4.3. Epigenetic Coupling (v5.3)

```
ep_age(t) = ep_rate_base × t + k_ep × ∫₀ᵗ D_c(τ) dτ
```

Centriolar damage accelerates epigenetic drift via coupling constant k_ep.

---

## 5. Key Predictions (P1-P10)

| # | Prediction | Test | Status |
|---|-----------|------|:------:|
| P1 | polyGlu (GT335) signal on mother centriole correlates with division number in HSC | FT1.1 — Spearman ρ ≥ 0.6 | Pre-registered |
| P2 | Mother centriole (Ninein+) preferentially retained by stem daughter in HSC division | FT1.2 — Binomial test p > 0.5 | Pre-registered |
| P3 | CCP1 KO → accelerated centriolar polyglutamylation → premature HSC exhaustion | FT6.1 — mouse model | Planned |
| P4 | Aging tissues show tissue-specific centriolar PTM patterns matching division history | Comparative GT335 across tissues | Theoretical |
| P5 | Centriole damage precedes epigenetic clock acceleration (temporal order) | Time-series in HSC | Theoretical |
| P6 | CAASM proteins identified by centrosomal IP-MS | Proteomics | Hypothetical |
| P7 | Cells with amplified centrosomes enter senescence via M4/M5 pathways | Test in culture | Supported |
| P8 | PCM1 mislocalization → symmetric stem cell divisions → pool depletion | Test in PCM1-KD | Supported |
| P9 | Oocyte centriole quality declines with maternal age → meiotic errors | Test in aged oocytes | Supported |
| P10 | Centriole damage load predicts remaining division capacity (biomarker) | Prospective cohort | Theoretical |

---

## 6. Open Problems & Weaknesses (Honest Assessment)

1. **CAASM (M3) is hypothetical — UPGRADED to plausible (5/10).** Five centrosome-associated proteins with differentiation functions identified: Oct4 (PMID: 41725553), CEP170 (PMID: 41888776), LGALS3BP (PMID: 42055624), PCM1 (PMID: 41315244), CEP57-CEP152 (PMID: 41616107). Direct centrosomal IP-MS during HSC differentiation still needed for definitive proof.

2. **Causality vs. correlation.** Does centriole damage CAUSE aging, or is it a passive marker? The CCP1-KO experiment (P3) is designed to establish causality.

7.5. **Limited repair — addressed (2026-08-02).** See §4.1 for full treatment. Repair mechanisms exist (autophagy, UPS, chaperones) but are structurally incapable of replacing damaged tubulin within centriolar microtubule triplets. The germline solves this by elimination + de novo synthesis. Somatic cells accumulate damage — this is aging. Full defense: `docs/REPAIR_OBJECTION_DEFENSE.md`

3. **Tissue-specificity underdeveloped.** The theory handles HSC and neurons but lacks quantitative predictions for 200+ human cell types.

4. **Interaction with other counters.** The coupling constant γ and matrix Γ are placeholder values. Real measurements needed.

5. **Strawbridge et al. (2026) challenge.** ES cells exit pluripotency without asymmetric division. CEDAR limits its scope to adult stem cells — but the boundary between "adult" and "embryonic" stem cell programs is blurry.

6. **Centriole elimination in some species.** Planarians, Naegleria, and early mouse embryos eliminate centrioles. How do these organisms age? Tqemaladze J (2025) preprint addresses this.
   - **Gene loss vs organelle loss (2026-08-06):** Martín-Durán et al. (2017, PMID 28400424) recovered centrosome-related genes as hidden orthologs in *S. mediterranea*, weakening Azimzadeh's genomic claim. However, this does NOT refute the TEM evidence that neoblasts lack centrioles. Presence of a gene does not imply presence of the organelle — hidden orthologs may have alternative functions. CEDAR must distinguish: (a) genomic argument (centrosome genes «lost») — weakened; (b) structural argument (centrioles absent by TEM) — stands. Both should be cited together, with qualification.

7. **Reversibility question.** If centriole damage is truly non-repairable (¬R), then cellular reprogramming (iPSC) should NOT reset centriolar damage. This is testable.

---

## 7. Connection to Other MCARA Counters

| Counter | Interaction with Counter #1 |
|---------|----------------------------|
| #2 Telomeric | Centriolar damage → defective mitosis → telomere crisis (M1) |
| #3 Mitochondrial | Centriole → MT → mitochondrial transport (M2 cilium pathway intersects) |
| #4 Epigenetic | Coupling via k_ep (§4.3); M4 MT arrays → chromatin remodeling |
| #5 Proteostatic | M7: centrosomal proteins involved in proteostasis |
| #6 piRNA | Germline-specific; M9: oocyte centriole quality affects piRNA pathway? |

---

## References (All Verified 2026-07-05)

1. Janke C, Magiera MM. The tubulin code and its role in controlling microtubule properties and functions. *Nat Rev Mol Cell Biol*. 2020;21(6):307-326. PMID: **32107477** ✅
2. Pimenta-Marques A et al. Ana1/CEP295 is an essential player in the centrosome maintenance program... *EMBO Rep*. 2024;25(1):102-127. PMID: **38200359** ✅
3. Mercey O, ..., Janke C. Glutamylation imbalance impairs the molecular architecture of the photoreceptor cilium. *EMBO J*. 2024;43(24):6414-6441. PMID: **39528655** ✅
4. Chen C, Yamashita YM. Centrosome-centric view of asymmetric stem cell division. *Open Biol*. 2021;11(2):200314. PMID: **33435817** ✅
5. Tqemaladze J. Reduction, proliferation, and differentiation defects of stem cells... *Mol Biol Rep*. 2023;50(3):2681-2691. PMID: **36583780** ✅
6. Robichaud JH et al. Transiently formed nucleus-to-cilium microtubule arrays mediate senescence initiation... *Nat Commun*. 2024;15:7906. PMID: **39266565** ✅
7. Bener MB, Inaba M. Stem cells resume asymmetric division upon niche re-entry... *Commun Biol*. 2026. PMID: **41803431** ✅
8. Zhao X et al. PCM1 coordinates centrosome asymmetry... *Nat Commun*. 2025;16:5230. PMID: **41315244** ✅
9. Mengistu DY et al. Microcephaly-associated genes asp and Sas4 influence chromatin organization... *Development*. 2026. PMID: **42063344** ✅
10. Constable S et al. Permanent cilia loss during cerebellar granule cell neurogenesis... *PNAS*. 2024;121(52):e2408083121. PMID: **39705308** ✅
11. Delimitreva S, Chakarova I. How Cytoskeletal Disorders Contribute to Errors in Chromosomal Segregation of Oocytes... *J Dev Biol*. 2025;13(4):43. PMID: **41440923** ✅
12. Grzonka M, Black BE, Lampson MA. Centromere regulation in the germline and early embryo. *Curr Opin Genet Dev*. 2025;91:102379. PMID: **40645119** ✅
13. McLendon JM et al. Gain and loss of the centrosomal protein taxilin-beta influences cardiac proteostasis... *J Mol Cell Cardiol*. 2025. PMID: **40010430** ✅
14. Royall LN et al. Asymmetric inheritance of centrosomes maintains stem cell properties in human NPCs. *eLife*. 2023;12:e83157. PMID: **37882444** ✅
15. Thomas A, Meraldi P. Centrosome age breaks spindle size symmetry... *J Cell Biol*. 2024;223(12). DOI: `10.1083/jcb.202311153` ✅
16. Gohel P et al. A non-transcriptional mitotic function of POU/Oct factors ensures spindle assembly... *eLife*. 2026. PMID: **41725553** ✅
17. Liao YC et al. CEP170 as a novel molecular link between centrosomal function and cerebral cortical development. *Development*. 2026. PMID: **41888776** ✅
18. Hwang JE et al. LGALS3BP Links Centrosomes and Mitochondria... *Cell Rep*. 2026. PMID: **42055624** ✅
19. Zhang S et al. NuSAP Safeguards Centriole Integrity... *Adv Sci*. 2026. PMID: **41616107** ✅
20. Herrera-Cid C et al. The primary cilium at the helm: gatekeeper of TGF-β superfamily signaling... *Biochem Soc Trans*. 2026. PMID: **42233350** ✅
21. Li B et al. Primary cilia function as hubs for signal transduction. *Cells*. 2025. PMID: **41310849** ✅


### Две линии: Дифференциация и Старение (Tqemaladze, 2026-08-13)

**Линия 1 — Дифференциация (программируемая, геометрический счётчик).**
Репликативный счёт ведётся ИЗМЕНЕНИЕМ ГЕОМЕТРИИ центриоли: каждая шаблон-зависимая дупликация на персистирующей материнской центриоли копирует слегка изменённую геометрию, поэтому число прошедших делений физически закодировано в форме скаффолда (отклонения колец/pitch, геометрия триплетов). Дифференциация — программируемый процесс, зависящий от систем асимметричных делений стволовых клеток — и поэтому зависящий от стохастического накопления ошибок СТАРЕЙШИМИ центриолями: чем старше удерживаемая центриоль, тем шумнее геометрический шаблон, тем больше ошибок входит в программу дифференциации.

**Линия 2 — Старение (побочный эффект дифференциации).**
Энтропия дисфункции центросомы и цилии накапливается с ХРОНОЛОГИЧЕСКИМ ВОЗРАСТОМ центриоли: даже без дальнейших делений стареющая центриоль прогрессивно теряет (а) центросомную функцию — понижение темпа делений в системах асимметричных делений стволовых клеток; (б) цилиарную функцию — понижение восприятия сигналинга. Старение — побочный эффект дифференциации, а не независимый процесс: это накопленная цена работы геометрического счётчика.

**Избирательное накопление старейших центриолей** в системах асимметричных делений стволовых клеток ведёт к: (1) понижению темпа делений (центросомная функция), (2) понижению восприятия сигналинга (цилиарная функция).

**Экспериментально разделяются:** Линия 1 (геометрический счётчик → ошибки дифференциации) — метрики M1/M5 + системы асимметрии стволовых; Линия 2 (хронологическая энтропия → дисфункция центросомы/цилии → шум часов) — постмитотические клетки + read-out дисфункции (темп делений, цилиарный сигналинг).
