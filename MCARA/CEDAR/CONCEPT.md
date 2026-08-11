# CEDAR — Centriolar Entropy-Damage Accumulation Ratchet

**Type:** umbrella project — analysis only.
**Version:** 1.2

> **Full concept:** `~/Desktop/PhD/CONCEPT.md`
> **Within MCARA:** Counter #1 (Centriolar) — `~/Desktop/LC/MCARA/CONCEPT.md`
> **Status:** PhD dissertation (UNED tesis-por-compendio) + article in Biogerontology (ID 7cc6de62, peer review)

## The Centriole as a Morphogenetic Lock

> **Core insight (2026-08-03):** The centriole does not encode a specific cell fate. It encodes the *capacity for controlled asymmetric division*, which maintains whatever fate the cell currently holds. It is a morphogenetic lock, not a fate determinant.

**Evidence:**
- **Renzova et al. (2018, PMID 30197118):** Centrinone-induced centriole loss in human pluripotent stem cells triggers spontaneous differentiation to all three germ layers. Without centrioles, cells lose the structural apparatus for controlled self-renewal.
- **Kalbfuss & Gönczy (2023, PMID 37256957):** ~88% of C. elegans embryonic cells eliminate centrioles during terminal differentiation. Centriole elimination is a programmed event that accompanies — and may enable — the transition from proliferative to post-mitotic state.
- **Köhrer et al. (2023, PMID 37821581, *Leukemia*):** Centriole over-elongation is an early feature of **plasma cell disorders** — disease-associated geometry change. **Whether analogous structural changes accumulate during physiological aging remains undetermined** (tested in the Entropy Reset Protocol, Objective 4). The templating argument stands on Kochanski & Borisy (1990) semi-conservative duplication, not on Köhrer.

**Resolution of the differentiation paradox:**
A critic might argue: centriole elimination drives differentiation (Renzova 2018), therefore it cannot enable reprogramming. This misinterprets the centriole's function. The centriole maintains the *capacity to hold an identity*, not the identity itself. Removing it breaks the lock. What happens next depends on the signals:

- **No signal → chaotic drift (Renzova 2018).** Cells lose self-renewal and differentiate randomly.
- **DUX4 + EZH2i + DPPA3 → totipotency.** The same malleable state, directed by totipotency factors.
- **De novo centriole → new lock.** Young centrioles restore controlled asymmetric division and stabilize the new identity.

The centriole is a morphogenetic status regulator. An old centriole locks in aged identity. No centriole = malleable. A young centriole locks in the chosen identity. This is the core of the Threshold Stand model (Tkemaladze 2005, PMID 15886028; Tkemaladze 2023, PMID 36583780).

> **CEDAR unified definition (2026-08-11, aligned with Entropy Reset Protocol v5.2):** CEDAR — **C**entriolar **E**ntropy-**D**amage **A**ccumulation **R**atchet — is **one ratchet with two modes**: (1) **counter mode** (CEDAR-α/β): semi-conservative duplication accumulates entropy/damage per division; (2) **state-lock mode** (CEDAR-γ, = the morphogenetic lock / differentiation ratchet): the organelle locks the cell's current identity. Loss of the ratchet releases identity in the direction set by the starting state: pluripotent → rolls down (Renzova 2018); differentiated (with p53 suppressed) → plastic → new de novo centriole re-locks a younger state (Entropy Reset Protocol §4.2a). The aging counter and the reprogramming barrier are two faces of one mechanism.

> **CEDAR as the general theory (2026-08-11):** CEDAR is the **general theory of differentiation and the division limit in animal stem cells** — division and differentiation are coupled through the differentiation ratchet: each asymmetric division that produces a differentiating daughter consumes part of the stem cell's division potential, and the division limit (Hayflick for somatic; ~200 divisions for stem cells) is reached *because of* accumulated differentiation acts, not merely damage. **Placement:** CEDAR (general theory) ⊃ MCARA (special case: which counter's entropy finishes first — v5.2) ⊃ Entropy Reset Protocol (somatic reversal without the meiotic price). The unified definition above is the mechanistic core of this general theory (Tkemaladze 2005, PMID 15886028; Tkemaladze 2023, PMID 36583780).

**Full exposition:** `docs/WHY_IPSC_FAILS.md` §4.1

## Essence

CEDAR is a theory according to which **centrioles accumulate entropy over time, like all material structures.** With divisions, the associated hypothetical structures of irreversible differentiation induction (CAMC) change. Additional structural disturbances are added to the centriole's entropy randomly. **Two independent processes:** time → entropy (passive); **asymmetric** divisions → CAMC change (active). Detachment of the irreversible differentiation inducer — only during asymmetric divisions.

### Key components (C1/C2)
- **C1:** The level of polyGlu (GT335) on the centriole correlates with the number of divisions
- **C2:** The mother centriole is asymmetrically inherited (Ninein+ remains in the stem cell)

## 🔴 FUNDAMENTAL DISTINCTION: PHYSICAL vs CHEMICAL ELIMINATION OF CENTRIOLES (2026-07-09)

> **Key question:** What exactly locks a cell in a differentiated state — the centriole itself (microtubules) or the associated hypothetical structure of irreversible differentiation inducers (CAMC)?

### Two classes of methods — two mechanisms of action:

| Method | What is removed | CAMC preserved? | Time scale |
|-------|---------------|:------------------:|:-----------------:|
| **PHYSICAL** (laser, microsurgery) | Centriole **entirely** + PCM + CAMC | ❌ Removed together | Instantaneous |
| **CHEMICAL** (centrinone, Plk4 siRNA) | Prevention of **duplication** → centriole diluted over 2-3 cycles | ✅ Preserved on existing centriole | Slow (days) |
| **ANTIBODY** (GT335 loading) | Disassembly of centriolar MTs | 🟡 PCM remains, CAMC — unknown | Hours |
| **GENETIC** (AID-Plk4) | Degradation of Plk4 → no duplication | ✅ Similar to chemical | Hours-days |

### Experimental logic for separating "centriole vs CAMC":

If PHYSICAL removal (laser)
   → cell reprograms (iPSC) =
   → centriole NOT needed for maintaining differentiation
   → CAMC MAY be removed together with centriole
   → BUT: if reprogramming is successful WITHOUT centriole,
      then CAMC either does not exist, or reprogramming
      bypasses it

If CHEMICAL elimination (centrinone)
   → cell does NOT reprogram =
   → CAMC is preserved on the remaining centriole
   → CAMC continues to block reprogramming
   → centriole = platform for CAMC

IF both methods give the SAME result
   → the centriole itself (its microtubules, organization) is important
   → CAMC is secondary

### CEDAR predictions:

| Experiment | Method | CEDAR prediction | Interpretation |
|-------------|-------|--------------------|--------------|
| Laser ablation + OSKM | Physical | **Reprogramming POSSIBLE?** | If yes → CAMC removed, centriole not needed for return |
| Centrinone + OSKM | Chemical | **Reprogramming BLOCKED** | CAMC preserved → block |
| GT335 loading + OSKM | Antibody | **Intermediate result** | MTs removed, PCM/CAMC possibly preserved |
| Centrinone then laser | Combo | **First CAMC, then removal** | Sequential separation |

### Value for the theory:

1. **If physical removal ≠ chemical → CAMC is a real structure**
   - Physically removed centriole+CAMC → cell can return
   - Chemically prevented duplication → CAMC remained → block
   
2. **If physical = chemical → the centriole itself is the carrier of "memory"**
   - It doesn't matter HOW it is removed — the result is the same
   - Polyglutamylation on microtubules = physical carrier of entropy
   
3. **GT335 loading — key test**
   - GT335 disassembles centriolar MTs, but PCM remains (Bobinnec 1998, PMID 9852152)
   - If after GT335 the cell reprograms → CAMC is either in MTs or does not exist
   - If after GT335 the cell does not reprogram → CAMC is in PCM

### Physical methods — historical context:

| Article | PMID | Method | Cells | Result |
|--------|------|-------|--------|-----------|
| Maniotis & Schliwa 1991 | **1934057** | Needle microsurgery | BSC-1 | Growth ✅, division ❌, >10 cycles without centrioles |
| La Terra et al. 2005 | **15738265** | Laser ablation | HeLa | Division ✅, de novo centrioles |
| Uetake et al. 2007 | **17227892** | Laser + microsurgery | cell line, HMEC | G1→S ✅, p38-dependent arrest |

**🔥 Critical conclusion of Uetake 2007:** Normal cells enter S-phase WITHOUT centrioles. Therefore, the centriole is NOT required for the cell cycle per se. But for reprogramming — the question remains open.

---

### Rule of nine mechanisms of centriole-dependent damage (Jaba Tqemaladze's Rule, v5.6)

Accumulation of damage in the centriole (D_c) causes 9 mechanisms of pathology:

**Core (M1-M3):**
- **M1 — Chromosomal segregation:** spindle defects → genomic instability. **Molecular mechanism:** Chk1 (ATR→ATRIP→TopBP1→Chk1 on centrosome) phosphorylates β-tubulin-T285 → required for spindle MT density and nucleation (Boutakoglou/…/Zachos 2026, PMID 41844775). Chk1 decline with age → impaired spindle quality → segregation errors.
- **M2 — Ciliary signaling:** centriole → basal body → ciliary disruption → Hh/Wnt/TGF-β failure
- **M3 — CAMC:** 🟡→🟢 UPGRADED. centrosome as a platform: Oct4 on the centrosome (
## 📚 New Literature (2026-07-05) + Meta-analysis (2026-07-06)

> **Key Finding 1:** Meng X, Baird RB, **Yamashita YM** — *Asymmetric male meiosis and its implications in heredity* — Curr Top Dev Biol 168:211–243 (2026) — DOI: `10.1016/bs.ctdb.2026.01.005` — PMID: 42097813
> **Key Finding 2:** Park EJ, Levin-Ferreyra F, **Di Stefano B** — *Mechanisms coordinating exit from the stem cell state in mammals* — Genes Dev 40:982-1011 (2026) — PMID: **42156139**
> **🔥 Critical Finding 3:** **Barandun N et al.** — *Mother centrosome → CD8+ T cell memory* — Cell Reports (2025) — DOI: `10.1016/j.celrep.2024.115127`
> **🔥 Critical Finding 4:** **Passanisi S, Spencer SL** — *Senescence NOT predicted by telomere length* — iScience (2026) — DOI: `10.1016/j.isci.2026.114801`
> **Meta-analysis:** `~/Desktop/PhD/docs/META_ANALYSIS_2026-07-06.md`

### 🔥 Spermiogenesis and Mitochondrial Proteases (2026-07-09)

> **New section:** `EVIDENCE.md` §10. 16 articles. Key findings:
> 1. **Feng et al. (2026)** — ClpP mitochondrial protease → meiosis. Giant mitochondria, ↓RAD51.
> 2. **Yamada et al. (2026)** — MLKL → mitochondria → HSC aging (Nat Commun).
> 3. **Khire et al. (2016)** — Centrioles remodeled during spermiogenesis (Curr Biol, PMID: 28094036).
> 4. **Mao et al. (2026)** — Slmap → axoneme + mitochondria + Hid-Diap1 apoptosis.
> 5. **Wani et al. (2022)** — YME1L mitochondrial protease → NSC self-renewal.
> **Overall score: 7.3 → 7.8/10. Counter #3: 8→8.5, Counter #5: 6.5→7.5, Counter #1: 7.5→8.0.**

### 🔥 Key Finding: CD8+ T cell mother centrosome (Cell Reports 2025)

**Barandun N, Meier B, Stehli G, Gräbnitz F, Zangger N, Oxenius A** — *Targeted localization of the mother centrosome in CD8+ T cells undergoing asymmetric cell division promotes memory formation* — **Cell Reports** (2025).

**Six key facts:**
1. The mother centrosome is TARGETED (not randomly) localized to one of the daughter cells
2. The daughter cell that receives the mother centrosome → becomes a MEMORY cell
3. The daughter cell without the mother centrosome → becomes an effector cell
4. This is shown in CD8+ T cells of MAMMALS (not Drosophila, not model organism)
5. Related article in **Nature** (2024): Fate induction in CD8 CAR T cells through asymmetric division — DOI: `10.1038/s41586-024-07862-7`
6. **Conclusion:** Centrosome asymmetry → cell fate — UNIVERSAL PRINCIPLE (SC + immune cells)

**Value for CEDAR:** C2 increases from 8.5 → **9/10.** M3 increases from 6 → **7/10.**

### 🔥 Key Finding: Senescence ≠ telomere length (iScience 2026)

**Passanisi S, Spencer SL** — *Replicative senescence induction in single cells is not predicted by telomere length, dysfunction, or oxidation* — **iScience** (2026).

**Value:** Neither telomeres nor oxidation predict senescence at the single-cell level → need OTHER counters (centrioles, epigenetics, proteostasis). **Strongest independent argument for MCARA/CEDAR.**

### Key references for CEDAR:

| # | Article | Value |
|---|--------|----------|
| 1 | **Thomas A, Meraldi P** (2024) — Centrosome age breaks spindle size symmetry — *J Cell Biol* | **Centrosome age → spindle asymmetry** (supports C2) |
| 2 | **Royall LN et al.** (2023) — Asymmetric inheritance of centrosomes in human NPCs — *eLife* — PMID: 37882444 | **C2 in human cells** |
| 3 | **Skinner MW et al.** (2025) — Meiotic divisions do NOT require centriole duplication — *PLoS Genet* | Meiosis without centrioles |
| 4 | **Skinner MW et al.** (2024) — Spermatocytes segregate chromosomes despite centriole duplication failure — *EMBO Rep* | Limits of applicability of CEDAR |
| 5 | **Chen C, Yamashita YM** (2021) — Centrosome-centric view of asymmetric stem cell division — *Open Biol* — PMID: 33435817 | Review from Yamashita lab |
| 6 | **Salzmann V, ..., Yamashita YM** (2014) — Centrosome-dependent asymmetric inheritance of midbody ring — *Mol Biol Cell* — PMID: 24227883 | Centrosome
## Stem Cell Exit & Epigenetic Barriers (2026-07-05)

> **Park EJ, Levin-Ferreyra F, Di Stefano B** — *Mechanisms coordinating exit from the stem cell state in mammals* — **Genes Dev** 40:982-1011 (2026) — DOI: `10.1101/gad.353584.125` — PMID: **42156139** — PMCID: PMC13267984 — 🔓 CC BY-NC 4.0

### Key findings for CEDAR:

| # | Article | Value |
|---|--------|----------|
| 1 | **Strawbridge SE et al.** (2026) — Exit from naive pluripotency WITHOUT asymmetric division — *Stem Cell Reports* — PMID: 41687620 | ⚠️ One division in 2i/LIF — symmetric. But the trajectory overall is asymmetric (tot→pluri→multi→uni). CEDAR does not require asymmetry of each division. Variable lag (0-3) — window for centriolar damage. |
| 2 | **Espinosa-Martínez M et al.** (2024) — Molecular basis of cell memory: epigenetic cycle — *Sci Adv* — PMID: 38416817 | Epigenetic cycle → alternative to centriolar counter |
| 3 | **Silvério-Alves R et al.** (2023) — GATA2 mitotic bookmarking — *Nat Commun* — PMID: 37580379 | Bookmarking in HSC — identity maintenance |
| 4 | **Palma LG et al.** (2025) — Chromatin activity of IκBα mediates exit from naïve pluripotency — *eLife* — PMID: 41123589 | Chromatin-mediated exit |
| 5 | **Ma B et al.** (2026) — Asymmetric histone inheritance in olfactory SC — *Nat Commun* — PMID: 41872193 | Histone asymmetry |
| 6 | **McCreery KP et al.** (2025) — Mechano-osmotic signals control chromatin — *Nat Cell Biol* — PMID: 41023488 | New level of regulation |

### Key conclusions from Park/Di Stefano for CEDAR:

1. **5 levels of stem cell exit** — CEDAR centriolar counter is at the interface of levels: centriole → signaling → chromatin → fate
2. **Pol II pausing** — accumulation of Pol II on lineage specification genes → "rapid response" — alternative commitment mechanism
3. **Tfe3/Folliculin** — "control of TF localization instead of expression" — 🔥 direct analogy with asymmetric centrosome inheritance (not the protein level matters, but its LOCALIZATION)
4. **Dux/Duxbl toggle** — switch model for CAMC: how one factor changes fate (analogy: polyGlu on centriole as a switch)
5. **"Point of no return"** — if it exists, CEDAR must explain it through accumulation of centriolar damage

### Additional literature (2026-07-06)

| Article | Journal | Year | DOI/PMID | For |
|--------|--------|-----|----------|-----|
| Eckhart et al. — Holocrine secretion: final step of epithelial differentiation | Cells | 2026 | 10.3390/cells15121058 | Terminal differentiation = programmed cell death analogy |
| Gilloteaux et al. — Mitochondrial ultrastructure in differentiated SH-SY5Y | Ultrastruct Pathol | 2026 | PMID: 42159247 | Mitochondrial remodeling during differentiation |
| **Hallmarks of stem cell aging** | Cell Stem Cell | 2025 | 10.1016/j.stem.2025.06.004 | General framework — check centrosome as hallmark |
| Mitochondrial drivers of stem cell aging | npj Aging | 2026 | 10.1038/s41514-026-00422-5 | Counter #3 support |
| UBE2G1 in HSC aging | Haematologica | 2026 | 10.3324/haematol.2026.300724 | Counter #5 support |
| Somatic piRNA/PIWI review | Front Cell Dev Biol | 2024 | 10.3389/fcell.2024.1495035 | Counter #6 — exploratory |

> Full list: `EVIDENCE.md` §7 and `~/Desktop/PhD/docs/literature_search_2026-07-05.md`
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

## Hypothesis

## Methodology

**Power analysis:** Statistical power calculations based on longitudinal centriole tracking data (Kalbfuss & Gönczy 2023, PMID 37256957). N=100 C. elegans embryos provides >80% power for detecting centriole fate patterns at α=0.05.

**Blinding:** Centriole fate classification performed by independent annotator blinded to pedigree score. Pedigree score computed only after classification locked.

**OSF pre-registration:** PCA weights pre-registered on OSF before main data collection. Hypothesis and analysis plan registered.

**Limitations:** (1) 100-cell window is a snapshot — late eliminators may be misclassified. (2) C. elegans centriole biology may not fully translate to mammalian systems. (3) E-lineage cells excluded from primary analysis (different elimination mechanism).

*To be specified — see CONCEPT.md §1 for project rationale.*

## References

*See project MEMORY.md for reference history.*

## Linked subproject

## 2026-08-02: New Evidence Integration

### Two-Level QC in Spermatogenesis (Kitaoka 2026 + Chen 2026)

Recent back-to-back papers reveal a **two-tier quality control system** in haploid spermatids:

1. **Nuclear QC** (Kitaoka & Yamashita 2026, bioRxiv): H2Av(S137)-dependent elimination of damaged nuclei via "trailing" and defective individualization
2. **Centriolar QC** (Chen et al. 2026, EMBO Rep): CEP164-dependent centriole retention; docking failure → clustering → elimination via residual bodies

**Critical Gap:** No one has connected these two pathways. CEDAR predicts they are linked: centriole damage → docking failure → BOTH nuclear mispositioning AND centriole elimination.

**Falsifiable Predictions:**
- Polyglutamylated centrioles should show CEP164 mislocalization
- H2Av foci should correlate with centriole damage markers
- Double knockout (H2Av + CEP164) should show additive fertility defects
- The centriolar "tu bulin code" determines whether a spermatid nucleus is retained or eliminated

**LERR — Ladder, Eliminate, Reprogram, Rebuild.**

**Step 1 (Ladder).** Cut the damage load first: slow the counter, push old centrioles into differentiating daughters, remove only the mother centriole, keep spare young ones.

**Step 2 (Eliminate).** Take out the old centriole. Restore telomeres. Wipe the epigenome. Rescue mitochondria.

**Step 3 (Reprogram).** Push to totipotency with DUX4 + KDM4D + DPPA3.

**Step 4 (Rebuild).** Grow fresh centrioles de novo. Derive clean, young adult stem cells.
**Step 1 (Ladder).** De-risk before elimination, based on current evidence: slow the counter with NAC (antioxidant) and reversible-PTM re-cleaning (TTL re-tyrosination, CCP5/6 deglutamylation); segregate damage via asymmetric inheritance of the mother centriole into differentiating progeny (Yamashita, 2007; Royall, 2023—human NPCs); hemi-eliminate only the mother centriole (laser/PROTAC) to preserve duplication control and avoid p53-dependent G1 arrest (Meitinger, 2016); condition the cell (spare PLK4 centrioles, G1/S synchronization, proteostasis); select the least-damaged pool (FACS by low Δ2/polyGlu).
**Step 2 (Eliminate).** Clear the old, damage-bearing centriole; restore telomeres (telomerase/ZSCAN4 via H3K14ac/H3K18ac; Meltzer, 2024); strip epigenetic marks (OSK/TET1-TET2-TDG; Lu, 2020—partial, lineage memory remains); select healthy mitochondria (PINK1-dependent mitophagy; Vázquez-Martín, 2016).
**Step 3 (Reprogram).** Induce totipotency with DUX4 + KDM4D + DPPA3: DUX4 opens cleavage-stage genes (Hendrickson, 2017), KDM4D removes the H3K9me3 reprogramming barrier, DPPA3 (Stella) stabilizes the totipotent (2C-like) state.
**Step 4 (Rebuild).** Reassemble young centrioles de novo (PLK4 → SAS-6 → STIL → CPAP; Nigg & Holland, 2018; Gönczy, 2012) after full elimination (Khodjakov, 2002; Uetake, 2007); quality-control geometry (9-fold symmetry, triplets, length); derive safe, young adult stem cells (karyotype-verified, p53-restored).
**Step 1 (Ladder).** De-risk before elimination: slow the counter, segregate damage, hemi-eliminate the mother centriole, condition the cell, select the least-damaged pool.
**Step 2 (Eliminate).** Clear the old centriole; restore telomeres; strip epigenetic marks; select healthy mitochondria.
**Step 3 (Reprogram).** Induce totipotency with DUX4 + KDM4D + DPPA3.
**Step 4 (Rebuild).** Regenerate young centrioles de novo; derive safe, young adult stem cells.
