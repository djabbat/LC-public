# MCARA — Multi-Counter Architecture of Replicative Aging


**Type:** umbrella project — analysis only — see CONCEPT.md for details.
**Date:** 2026-07-19
**Version:** v4.7 — Centriculum (Maheshwari 2023/2026) + Spermatogenesis (Ishida & Shibuya 2026)
**Central question:** What molecular division counter finishes first — and under what conditions?
**Grant:** EIC Pathfinder Challenges 2026 · Deadline 28 October 2026
**Budget:** ~€3.2M · 36 months

---

## 0. The Centriole as a Gatekeeper of Cell State

The cell stops dividing after ~50 divisions (Hayflick limit). Under hypoxia (2% O₂) and telomerase activity (hTERT), telomerase maintains long telomeres, and low oxygen protects mitochondria — but a limit is still reached: even stem cells stop after ~200 divisions. Something other than telomeres and mitochondria counts divisions. **MCARA checks which molecular counter finishes first.**

**Key Hardware/Software Difference:**
- **Centriole = hardware:** non-repairable in situ. There is no coordinated quality-control mechanism that restores the entire organelle to a "young" state. But it can be **eliminated and created de novo** — exactly what happens in meiosis.
- **Epigenome = software:** reprogrammable by Yamanaka factors, but unable to fix hardware defects.

This explains why partial reprogramming rejuvenates the methylome, but does not restore full replicative capacity: centriolar hardware is not replaced.

### Centriole as Stress Integrator (not an autonomous counter)

The centriole is a **conditional entropy carrier:** entropy accumulates when the centriole works as a centrosome (stress-sensitive state); polyE is a compensatory response to this entropy. Accumulation stops when the centriole templates a cilium. The level of polyE depends on the functional state and stress environment, not just chronological time.

This is a rethinking: the centriole is not the "main conductor of aging," but a **hub** through which diverse stress signals converge to trigger a decision on senescence.

### Centriculum: ER-membrane reticulum around the centrosome (Maheshwari, Cohen-Fix 2023/2026)

**Discovery (Maheshwari et al. 2023 Curr Biol, PMID 36693370; 2026 J Cell Sci, PMID 42283151):** The centrosome is NOT a "membraneless organelle." In model organism early embryos, it is surrounded by a three-dimensional ER-derived membrane reticulum — the **centriculum**.

| Property | Value |
|----------|---------|
| Structure | 3D ER-membrane network, adjacent to PCM |
| Formation | Depends on centrosome and microtubules |
| Function | "Microtubule filter" — blocks elongation of most, but not all, MT |
| Regulation of PCM | Centriculum size ↔ PCM size. Centriculum increase → PCM expansion → ↑ MT-nucleation |
| Key PCM protein | SPD-5 — density increases with centriculum decrease (PCM can compact) |
| Cell cycle | In interphase, adjacent to nuclear envelope; in metaphase, fuses with fenestrated nuclear membrane |

**Value for MCARA — hypothesis of age-dependent centriculum degradation:**
1. If the centriculum acts as an MT filter, its age-dependent changes may directly affect microtubule nucleation of the centrosome
2. ER stress (a known aging factor) may damage the centriculum → disrupt centrosome function → mitotic errors
3. Centriculum = potentially new component of Counter #4 (Structural: Centriole/Cilium) — links ER stress to centrosome dysfunction
4. PCM compaction upon centriculum decrease — possible mechanism by which the cell compensates for age-related changes

**New data (Deep Review 2026-07-22, Maheshwari et al. 2026 J Cell Sci, PMID 42283151):**

| Property | New data |
|----------|-------------|
| **PCM compaction** | RNAi against ZYG-9/TBG-1 → PCM area ↓, but SPD-5 amount does not decrease → **SPD-5 density increases** (P<0.0001). R²=0.26, P=0.0002. PCM — condensable structure |
| **Selective porosity** | Centriculum more porous for spindle MTs (pass to chromosomes), than for astral MTs (blocked). Mechanism of selectivity unknown |
| **10× concentration of tubulin** | Baumgart et al. (2019): soluble tubulin in centros
ome concentrated 10 times higher than cytoplasm. "Filter" model explains: MT collision → centriculum → catastrophe → tubulin release |
| **Evolutionary conservation** | Centriculum-like structures found in: **Drosophila** (Diaz 2019, Rollins & Blankenship 2023 — Dev), **medaka** (Kiyomitsu 2024 — JCB), **sea urchin** (Xie 2025). ER curvature proteins Rtnl1/ReepB — common mechanism |
| **ER-centrosome crosstalk** | Sánchez-Álvarez et al. (2025) *Cell Rep*: PERK-dependent crosstalk ER↔MT. Zheng et al. (2022) *Nature*: ER proteins (CLIMP63, KTN1, p180) decode tubulin code. Teixeira et al. (2025) *JCB*: CDR2 — dynein adapter for ER |

**New hypothesis — connection to oocytes (Shihabi et al. 2026, Adv Sci, PMID 42360132):**
- Mammalian oocytes are acentrosomal, but ER is abundant and reorganized around the spindle
- Actin cytoskeleton (Shihabi's topic) interacts with ER (Maheshwari's topic) — potential mechanical crosstalk
- **Open question:** do centriculum-like ER-membrane structures exist around huoMTOC in human oocytes? If so, their age-dependent degradation → new mechanism of aneuploidy
- **Cross-validation:** Nature already uses both CEDAR principles during fertilization: Eliminate (oocyte eliminates centrioles) + Rebuild (de novo from sperm seeds)

### Asymmetry of gametes: oogenesis (elimination) vs spermatogenesis (preservation)

**Ishida & Shibuya (2026, PMID 42455439):** Fundamental asymmetry in the fate of centrioles between male and female gametogenesis:
- **Oogenesis:** centrioles are **eliminated** (reset mode)
- **Spermatogenesis:** centrioles are **preserved** throughout meiosis, duplicated twice independently of pre-S-phase DNA synthesis, undergo unique remodeling in spermatids
- **Zygote:** sperm brings centrioles as seeds (templates), not templates. De novo assembly occurs using these seeds

**Value for MCARA:** During fertilization, biology implements both CEDAR principles simultaneously:
1. **Eliminate** (oocyte) — clearance of maternal centrioles
2. **Rebuild** (de novo from sperm seeds) — new centrioles without epigenetic burden
This **STRENGTHENS** the analogy criterion of Bradford Hill: nature already does what we propose.

## 1. Evidence Base — Bradford Hill Analysis

Application of Bradford Hill criteria to the hypothesis of asymmetric centriole inheritance in aging:

| Criterion | Assessment | Rationale |
|----------|:------:|-------------|
| Strength | 🟡 Moderate | Plk1-asymmetry, ATF5-binding — correlative |
| Consistency | 🟢 Supported | 4 types, convergent data |
| Specificity | 🟡 Improving | FERMT3 + ALMS1 + ATF5 — three independent centriolar readers, increasing specificity |
| Temporality | 🟢 Supported | sinc-MT: polyE precedes senescence (Robichaud 2024, PMID 39266565) |
| Biological gradient | 🟡 Predicted | polyE vs passage number — not measured |
| Plausibility | 🟢 Strong | Signaling hub + cilia switch + sinc-MT pathway |
| Coherence | 🟢 Strong | Explains hTERT + hypoxia paradox, Parrinello paradox |
| Analogy | 🟢 Strong | Centriole elimination in germ line = reset |
| Experiment | 🔴 Absent | Central prediction not tested |

**Total: ~6/9 criteria with evidence (audit 2026-07-19).** Evidence class II–III. Temporality upgraded to 🟢 (Robichaud 2024). Specificity upgraded to 🟡 (FERMT3 + ALMS1 + ATF5).

### Why single-clock theories are insufficient

- **Bodnar (1998, PMID 9454332):** hTERT extends lifespan ≥20 doublings with normal karyotype. But telomerase alone is not enough: subsequent studies (Morales 1999, Counter 1998) showed that hTERT does not provide full immortalization in all cell types.
- **Parrinello (2003, PMID 12855956):** mouse fibroblasts — 20% O₂ → senescence, 3% O₂ → immortalization. Human cells at 3% O₂ — no (separate data, estimate based on Parrinello + Forsyth 2003 PMID 12730145)
- **Wagner (2013, PMID 23080539):** TERT does NOT prevent SA-DNAm
- **Passanisi/Spencer (2026, PMID 41816297):** telomeres do NOT predict senescence at single-cell level

---

## 2. Project Phases

### Phase 0 — Critical Checkpoint Experiment (months 1–6)

**Central experiment of entire MCARA.** Never conducted.

| Step | Action | Method |
|:---:|----------|------|
| 1 | Centriole elimination | Plk4 siRNA (72h) or centrinone (100 nM, 3 days) |
| 2 | Confirmation | IF: CP110 + Cep135. Goal: >90% acentriolar |
| 3 | OSKM reprogramming | Cytotune 2.0 Sendai virus, 21 days |
| 4 | Reading | Alkaline phosphatase+ colonies |
| 5 | p53 control | p53 shRNA + Plk4 siRNA → OSKM |
| 6 | Ciliogenesis control | IFT88 shRNA → OSKM (blocks cilium, preserves centriole) |
| 7 | Off-target PLK4 control | STIL shRNA → OSKM (alternative centriole removal) |
| 8 | 🔑 Totipotency control | Centrinone + DUX4 + TPRX1 → MERVL/Zscan4 (checks totipotency, not pluripotency) |

**Prediction (pluripotency, range):** Centriole elimination → reprogramming efficiency ↑ 1.5–10× vs control. Lower bound (1.5×): even modest excess indicates centriolar contribution to barrier. Upper bound (10×): achievable with complete p53/p38 stress relief and optimal conditions. Specific factor depends on cell type, elimination method, and p53 suppression efficiency.
**Hypothesis (totipotency, 2026-07-12; edited 2026-07-17):** Centriole = **two modes:** (1) passive entropy accumulator — basis of multicellular animal aging; (2) active differentiation regulator — tool for irreversible gene network switching (CAMC, NANOG, cilium). **Entropy — not a ratchet. Ratchet — gene network switching.** Centriole — instrument of this switching. OSKM reloads software (epigenome), but not hardware (centriole). Complete centriole elimination lifts physical differentiation lock. Three steps: **Eliminate → Reprogram → Rebuild** — as in nature during fertilization.

> **⚠️ DID-RNA — speculative model (2026-07-17).** Hypothesis about DID as RNA that integrates into genome via reverse transcription lacks experimental data. Main CEDAR hypothesis (centriole = hardware barrier) is independent of DID-RNA and tested separately. DID-RNA remains interesting, but unconfirmed speculation.

**Predictions:**
1. OSKM after elimination → 1.5–10× efficiency (range).
2. Totipotency factors (DUX4 + TPRX1) after elimination → potentially totipotent state with MERVL+, Zscan4+, CDX2+. Markers: MERVL, Zscan4, Hhex (mouse); TPRX1, ZSCAN4, DUX4 (human).
3. In nature, totipotency always accompanies centriole elimination — oocyte clears centrioles, sperm brings seeds for de novo assembly (Schatten & Sun 2011, PMID 21509822; Avidor-Reiss & Fishman 2022).
**Falsification:** If ≤ control → hypothesis disproven.

**Alternative interpretation (maturity sensor):** Centriole may be not a "differentiation lock," but a "maturity sensor" — its loss reduces differentiation competence nonspecifically. Increased iPSC efficiency after elimination may be related to partial dedifferentiation (loss of mature functions), not aging clock reset. **Note (audit 2026-07-19):** Lindhout 2021 (PMID 33835529) shows that centriole loss *disrupts* axon development and electrophysiological maturation of neurons — this does not directly support "maturity sensor → plasticity" hypothesis. Question remains open.

**Gold-standard experiment for distinction:** Microinjection of purified old centrioles (P45) into young cells (P10) → does it accelerate senescence?

### Phase 1 — ARGUS (months 1–12)

ARGUS-LP platform: autonomous AI microscope. 24/7. CellPose → spotiflow → Decision Engine → 405 nm laser. Three units. Open-source. Details: `ARGUS-LP/CONCEPT.md`.

### Phase 2 — Race: who's first? (months 6–24)

BJ-hTERT fibroblasts. 2% O₂. ARGUS-LP tracks centrioles through 25+ divisions.

### Phases 3–5 — Rejuvenation and Transplantation (months 18–36)

See full description in CONCEPT v4.4. Includes: track-by-track intervention, integrated rejuvenation, HSC transplantation.

---

## 3. Formal Model

> 📐 Complete formalism: [`THEORY.md`](THEORY.md) — axioms M1–M5, counter kinetics, coupling matrix Γ, Damage Shadow.

### Counter Kinetics (Linear Approximation)


S_centriole(t) = S₀ + β·t + η(t) − δ·CCP(t)
CAMC(N) = CAMC₀ − λ·N_asym


- β·t — time-dependent entropy
- η(t) — oxidative damage
- δ·CCP(t) — deglutamylase activity (decreases with age)
- λ·N_asym — CAMC remodeling during asymmetric divisions

### Sigmoidal Model (Biologically More Realistic)


S(t) = S₀ + S_max / (1 + e^(−k(t − t½))) − δ·CCP(t)
d[E]/dt = k_E · [Stress] − k_CCP · [CCP] · [E]


Threshold t½ — point of polyE accumulation acceleration, potentially corresponding to the onset of replicative senescence.

**Parameters are not calibrated.** Equations are testable predictions.

---

## 4. PolyE balance: glutamylation vs deglutamylation

Rogowski et al. (2010, PMID 21074048): CCP1–CCP6 remove polyGlu from tubulin. Mutations in CCP1 → neurodegeneration.

**Critical evidence of pathogenicity:** CCP1 knockout in BM-MSC inhibits osteogenic differentiation through enhanced glutamylation of microtubules and shortening of the primary cilia (PMID 40349688).

**Hypothesis:** TTLL (glutamylase) activity increases with replicative stress; CCP (degutamylase) activity decreases with age.

---

## 5. Consortium

| # | Partner | Country | Role | Status |
|:--:|---------|:-----:|------|:------:|
| 1 | GLA (Jaba) | 🇬🇪 GE | PI, C1 — race favorite, ARGUS-LP, integration | ✅ |
| 2 | Wagner (RWTH) | 🇩🇪 DE | C2 — SA-DNAm, EPIC arrays | 🟢 Zoom |
| 3 | Trifunovic (Cologne) | 🇩🇪 DE | C3 — Mitochondria | ✅ Joined |
| 4 | Magiera (Curie) | 🇫🇷 FR | C4 — U-ExM, tubulin code | 🟡 Waiting |
| 5 | Geiger (Ulm) | 🇩🇪 DE | 👨‍⚖️ **Race Judge** — HSC mouse, in vivo validation | 🟡 LoI |
| 6 | Jacquemet (Åbo) | 🇫🇮 FI | ARGUS-LP + AI (live-cell tracking, spotiflow) | 🟡 Letter |
| 7 | Senescence/Safety | TBD | Safety control, karyotype, WGS | 🆕 Search |

---

## 6. Budget (~€3.2M)

| Partner | Budget |
|---------|:------:|
| GLA | €800K |
| Wagner | €280K |
| Trifunovic | €300K |
| Magiera | €150K |
| Geiger | €580K |
| Jacquemet | €180K |
| Senescence/Safety | €350K |
| PM + Ethics | €150K |
| Subcontracts | €200K |
| **Total** | **~€3.2M** |

---

## Consumables (annual)

| **model organism maintenance** (NGM agar, OP50/NA22 bacteria, cholesterol, Petri dishes) | **$3,500** |
| **RNAi/strain maintenance** (clones, IPTG, antibiotics, feeding plates) | **$2,000** |
| **Cell culture** (DMEM/RPMI, FBS, pen/strep, trypsin, plastics) | **$8,000** |
| **CO₂ gas + incubator supplies** (cylinders, rental, HEPA filters, O₂/CO₂/N₂ control, humidity control ±2% RH, dehumidifier) | **$4,500** |
| **Transfection reagents** (Lipofectamine, siRNA oligos) | **$3,000** |
| **Sequencing consumables** (library prep, flow cells) | **$12,000** |
| **Microscopy** (immersion oil, coverslips, lens cleaning) | **$2,000** |
| **Glove-box/Enclosure** (HEPA H13 filters, UV-C lamps, gloves, seals, N₂ gas, humidity control ±2% RH) | **$9,500** |
| **Office consumables** (printing, stationery) | **$500** |


## 7. Testable Predictions (Table 4 from the article)

| Prediction | Test |
|-------------|------|
| Centriolar polyE grows over time, not with divisions | GT335 IF in post-mitotic vs dividing cells |
| CAMC changes with asymmetric divisions | Centrosomal BioID + quantitative proteomics |
| Deglutamylation decrease → polyE accumulation | CCP1 activity assay across passages |
| CCP1 knockout accelerates stem cell exhaustion | UBC-CreERT2 × CCP1^fl/fl mouse, n ≥ 50 |
| Centriolar polyE ↔ single-cell transcriptional entropy | Multiplexed imaging + scRNA-seq |
| Old centriole → accelerated senescence | P45 centriole → P10 cells |
| CCP1 overexpression slows ageing | CRISPRa-CCP1 in BJ-hTERT |
| iPSC from acentriolar fibroblasts = reduced diff-ka | Lindhout protocol: neuronal diff-ka + electrophysiology |

---

## 8. Key PMIDs (v4.6 — updated 2026-07-19 after discussion with Pierre Gönczy)

### Centriole as an entropy carrier
| PMID | Author, year | Key result |
|:----:|------------|-------------------|
| 36583780 | Tqemaladze (2023) | CEDAR original |
| 41230623 | Cummings (2025) | Irreversible damage beyond epigenome |
| 41571679 | De Man (2026) | Transcriptional entropy ↑ with age |
| 41299832 | Hong, Cohen (2025) | Entropy-based aging framework |
| 41724675 | Hong (2026) | Muscle histological entropy → mobility |
| 40931490 | Hong (2025) | ECG entropy → fractures + mortality |

### Three-stage model of centriole elimination (Gönczy lab)
| PMID | Author, year | Key result |
|:----:|------------|-------------------|
| **37963546** | Kalbfuss & Gönczy (2023) | **Open Biol:** Review — maintenance → priming → execution. Table of all elimination cases |
| **37256957** | Kalbfuss & Gönczy (2023) | **Science Advances:** ~88% of model organism cells lose centrioles. Cell fate = centriole fate |
| **37414202** | Kalbfuss, Berger & Gönczy (2023) | **Dev Biol:** Mapping centriolar proteins. No diffusible elimination factor |
| — | Pimenta-Marques et al. (2023) | **EMBO Reports:** ANA1/CEP295 — key stabilizer. doi: 10.1038/s44319-023-00020-6. **Personally recommended by Gönczy** |

### CAMC and molecular readers
| 26213385 | Madarampalli (2015) | ATF5 — bridge PCM↔centriole |
| 42343301 | FERMT3 (2026) | miP-FERMT3 → senescence from centriole |
| 42380124 | Ozaki/Tsou (2026) | ALMS1 IDP → centriolar memory |
| 39012627 | Thomas, Meraldi (2024) | Centrosome age → spindle asymmetry |

### sinc-MT/KIFC3 — complete pathway
| 39266565 | Robichaud (2024) | sinc-MTs → KIFC3 → FBF1 → PML → senescence |
| 37019904 | Ma (2023) | ARL13B-ARL3 → FBF1 release |
| 9852152 | Bobinnec (1998) | polyE specific to centrioles in interphase |

### PolyE balance
| 21074048 | Rogowski (2010) | CCP1–CCP6 deglutamylases |
| 40349688 | Pan (2026) | CCP1 KO → impaired osteogenesis |

### Epigenetics
| 23080539 | Wagner (2013) | TERT does not prevent SA-DNAm |
| 30332397 | Kabacik/Horvath (2018) | hTERT ≠ epigenetic age arrest |
| 31113906 | Matsuyama/Horvath (2019) | Hypoxia slows clock 30-40% |

### Multi-counter model
| 9454332 | Bodnar (1998) | hTERT extends lifespan significantly but not indefinitely in all cell types |
| 12855956 | Parrinello (2003) | 3% O₂ immortalizes mouse fibroblasts; human cells require additional evidence |
| 41816297 | Passanisi/Spencer (2026) | Senescence ≠ telomere length |

### Fertilization / gametogenesis and plasticity
| 30596512 | Jeffries (2019) | Cilium resorption failure → senescence |
| 33835529 | Lindhout (2021) | Centriole = maturity sensor (DOES NOT confirm plasticity — see audit 2026-07-19) |
| 22223737 | Azimzadeh (2012) | Centrosomes (PCM) lost in planarians; centrioles preserved in neoblasts |
| 21509822 | Schatten & Sun (2011) | Centrosome inheritance during fertilization |
| **42455439** | **Ishida & Shibuya (2026)** | **Spermatogenesis: centrioles preserved (vs elimination in oogenesis). 165 refs. RIKEN BDR** |

### Centriculum — ER-membrane reticulum around centrosome
| **36693370** | **Maheshwari, Cohen-Fix (2023)** | **Curr Biol: discovery of centriculum. 3D ER-network around centrosome. FIB-SEM** |
| **42283151** | **Maheshwari, Cohen-Fix (2026)** | **J Cell Sci: centriculum = microtubule filter. PCM-compaction** |

---

*CONCEPT v4.5 — 2026-07-08. Stress Integrator + Hardware/Software distinction. Synthesis from MCARA.docx (Gatekeeper article).*

## Hypothesis

*To be specified — see CONCEPT.md §1 for project rationale.*


## Methodology

**Power analysis:** Multi-counter architecture simulation power validated through Monte Carlo sampling (n≥10,000 iterations). Effect size detectable at Cohen's d ≥0.3 with α=0.05.

**Blinding:** Simulation runs blinded to counter configuration. Independent validation of output by second analyst.

**OSF pre-registration:** Simulation protocols and parameter sets pre-registered on OSF.

**Limitations:** (1) In silico only — experimental validation pending. (2) Counter interactions simplified to pairwise — higher-order interactions not modelled. (3) Parameter space exploration limited to biologically plausible ranges.


## References

*See project MEMORY.md for reference history.*
