# Four Counters, One Limit: Centriole, Telomere, Mitochondrion, and Epigenome in the Architecture of Replicative Aging

**Jaba Tqemaladze, MD**\
Georgia Longevity Alliance, Tbilisi, Georgia\
jaba@longevity.ge | ORCID: 0000-0001-8651-7243

**Version 9 — 2026-07-17** | Corrections: planarian data (Azimzadeh 2012), C. elegans mechanism acknowledged as unknown, prediction range 1.5–10×, totipotency control added.

---

## Abstract

Cells have brakes. Remove two of them — short telomeres and mitochondrial damage — and human fibroblasts still stop dividing. Something else counts.

Here I propose that the centriole is that counter. The centriole and the primary cilium are the same organelle in two states. In its cilium state the centriole is short, sits at the membrane, and receives developmental signals. In its centrosome state the cilium is gone, the centriole elongates, and the cell divides. With every division cycle the centriole spends time in each state, but the balance may tip. Over many divisions the centriole dominates more and the cilium less. The cumulative ratio is written into the centriole as polyglutamylation, a post-translational mark that accumulates during the centrosome phase and not during the ciliary phase.

Four lines of evidence support this. In C. elegans, 551 of 558 cells eliminate their centrioles upon differentiation — a one-way exit from the centrosome-cilium cycle, though the molecular mechanism of elimination remains unknown. In Drosophila oocytes, forced retention of the old centrosome breaks totipotency. In planarians, centrosomes (the pericentriolar material) have been evolutionarily lost while centrioles persist in mitotic stem cells — a unique strategy correlated with maximal regenerative capacity. In mammals, the older mother centriole segregates asymmetrically at division and helps determine what a daughter becomes — an effector cell or a memory cell, a neuron or a progenitor. A microprotein at the centriole's subdistal appendages, miP-FERMT3, directly triggers p53-independent senescence and increases with age.

I define an experiment to test the hypothesis: remove centrioles from a fibroblast, add Yamanaka factors, and count the colonies. If the centriole anchors differentiation, removing it should lift the lock. The predicted magnitude is 1.5–10-fold, depending on cell type, elimination method, and stress-pathway suppression. A totipotency arm — centriole elimination followed by DUX4 and TPRX1 — tests whether the barrier extends to the earliest embryonic state.

**Four counters — centriole, telomere, mitochondrion, and epigenome — run in parallel. When three are protected (hTERT for telomeres, low oxygen for mitochondria, partial reprogramming for the epigenome), the centriole continues to count. MCARA explains why cells still arrest.**

**Keywords:** centriole, cilium, polyglutamylation, telomere, epigenome, mitochondria, replicative senescence, MCARA, aging, reprogramming

---

## 1. A Puzzle

Two discoveries frame the problem. Bodnar and colleagues showed that hTERT — the catalytic subunit of telomerase — extends the lifespan of human fibroblasts but does not make them immortal [1]. Parrinello and colleagues showed that growing the same cells in 2% oxygen, which shields mitochondria from oxidative damage, extends lifespan further [2]. Take both precautions together — protect telomeres, shield mitochondria — and BJ fibroblasts still arrest. Add partial epigenetic reprogramming, and the cells still stop. Three counters are slowed. A fourth continues.

Passanisi and Spencer confirmed this at single-cell resolution in 2026: telomere length, DNA damage foci, and oxidation state do not distinguish cycling cells from arrested ones [3]. Something else marks the limit.

This paper proposes that the centriole is the counter that marks it.

The centriole is a small cylinder of microtubules that every dividing cell inherits from its mother. It has been studied for over a century as a spindle-organizer — the structure that ensures chromosomes separate cleanly at mitosis. But a spindle-organizer does not explain why 551 of 558 cells in a C. elegans embryo eliminate theirs upon differentiation [4], or why planarians have evolutionarily lost their centrosomes while retaining their centrioles [5], or why the mother centriole is not inherited randomly but handed preferentially to one daughter at division [6,7].

The proposal here is that the centriole is more than a spindle pole. It is a structural memory device whose history — how many divisions it has survived, how many times it has cycled between its two states — determines whether a cell can still change identity.

### 1.1. Two States, One Organelle

The centriole oscillates between two states [8].

In the **cilium state**, it is a basal body — short, anchored at the plasma membrane, supporting a primary cilium that receives Hedgehog, Wnt, and other developmental signals. The cell listens. In the **centrosome state**, the cilium is resorbed, the centriole elongates and matures, and the cell divides. The two states are mutually exclusive: a single centriole cannot be both a basal body and a spindle pole at the same time.

With each division cycle the centriole cycles between these states. But the balance may shift. Over many divisions the centriole may spend progressively more time in the centrosome state and less in the ciliary state. The cumulative ratio — time spent dividing vs. time spent listening — is a measure of how far the cell has travelled from plasticity.

### 1.2. How the Ratio Is Recorded

Polyglutamylation (polyE) is a post-translational modification in which glutamate side-chains are added to tubulin. On centriolar microtubules, polyE accumulates during the centrosome phase — when the centriole is long and the cilium absent — and does not accumulate during the ciliary phase [9]. The level of polyE on a centriole therefore reflects the cumulative time that centriole has dominated over its cilium.

PolyE is not a simple clock. It is a structural index of the centriole-to-cilium ratio — higher when the centriole has a long history of centrosomal dominance, lower when the cilium has been maintained. Critically, polyE is also a compensatory response to structural entropy: as the centriole accumulates damage over time, TTLL enzymes add polyE to stabilise its microtubules. The balance between TTLL-mediated glutamylation and CCP-mediated deglutamylation reflects the centriole's ongoing struggle against thermodynamic decay.

```
YOUNG CELL                              OLD CELL
(plastic)                               (committed)

    ═══ long cilium                       ─── short/absent cilium
    ─── short centriole                   ═══ long centriole
    ↓ polyE                               ↑ polyE
    centriole/cilium ratio ↓              centriole/cilium ratio ↑
    Wnt/Hh active                         Wnt/Hh suppressed
    can divide + dedifferentiate          cannot dedifferentiate
```

This hypothesis is testable. Expansion microscopy (U-ExM) can measure centriole length, cilium length, and GT335 (anti-polyE) fluorescence intensity in the same cell across passages. The prediction: polyE per unit centriole length increases monotonically with the centriole-to-cilium ratio.

### 1.3. What Follows

Section 2 reviews the evidence from five phyla. Section 3 describes the central experiment: centriole elimination followed by OSKM reprogramming, plus a totipotency arm. Section 4 presents the multi-counter architecture (MCARA) — how the centriole clock integrates with epigenetic and mitochondrial clocks. Section 5 discusses implications for aging. Section 6 lists limitations.

---

## 2. Evidence from Five Phyla

### 2.1. C. elegans: Elimination Locks the Post-Mitotic State

The C. elegans embryo starts with 671 cells. By the L1 larval stage, 551 of 558 have eliminated their centrioles [4]. Seven cells retain them: four rectal epithelial cells (B, F, U, Y) and two germline precursors (Z2, Z3). These seven divide throughout the animal's life. The rest never divide again.

The elimination follows a rigid schedule — same cell type, same developmental time, every time. When Kalbfuss and Gönczy experimentally altered a cell's fate, its centriole's fate followed [10]. Three stages have been described — maintenance, priming, and execution — but the molecular executors of elimination remain unknown [11].

Without a centriole there is no centrosome state and no cilium state. The cell is locked in permanent post-mitotic identity. In the framework proposed here, centriole elimination is a one-way exit from the centriole-cilium cycle — a structural commitment to the differentiated state that cannot be undone.

### 2.2. Drosophila: Forced Retention Breaks Totipotency

In Drosophila oocytes, Polo kinase triggers the degradation of pericentriolar material, which is followed by centriole elimination [12]. The oocyte must clear its centrosomes entirely before it can become totipotent. When Pimenta-Marques and colleagues blocked this elimination, centrioles were retained and the flies were sterile.

The oocyte inherits not just DNA but a history of divisions. If that history — recorded in the centriole — is not erased, totipotency fails. The same principle operates in mammals: oocyte centrioles are eliminated during oogenesis, and sperm centrioles are restructured into seeds for de novo assembly in the early embryo [13].

### 2.3. Planarians: Centrosome Loss, Centriole Retention

The planarian *Schmidtea mediterranea* regenerates any body part from a population of pluripotent stem cells called neoblasts. Azimzadeh and colleagues made a striking discovery: planarians have evolutionarily lost centrosomes — the pericentriolar material that organises microtubules — but retain centrioles in their mitotic neoblasts [5]. Immunofluorescence for the centriolar marker SMED-CEP135 and transmission electron microscopy both confirmed that centrioles are present during neoblast mitosis.

This is a unique strategy among animals. Planarians have decoupled the centriole from the centrosome. They retain the centriole cylinder — required for cilium formation and duplication — but have dispensed with the PCM scaffold that, in other animals, anchors signalling complexes at the centriole's appendages.

When a neoblast commits to mucociliary differentiation, it massively amplifies its centrioles via the canonical Plk4/Sas6/Cep152 pathway to form basal bodies for multiple cilia [14]. Here, centriole amplification — not de novo appearance — marks the transition. The pattern is distinct from C. elegans and Drosophila, but the principle is shared: the centriole/centrosome system changes state when cell fate changes. In planarians, the loss of PCM — possibly releasing centrioles from signals that enforce commitment — may contribute to the remarkable plasticity of neoblasts.

### 2.4. Mammals: The Mother Centriole Chooses Fates

A mammalian cell about to divide has two centrioles of unequal age. The mother centriole was built at least two cycles ago and carries fully mature appendages. The daughter was built in the previous cycle. They do not segregate randomly.

Yamashita and colleagues first showed this in Drosophila germline stem cells: the mother centrosome stays with the stem cell [6]. Wang and colleagues extended the finding to the mammalian neocortex, showing that the older mother centriole is retained in radial glia progenitors while the newer mother centriole segregates to differentiating cells; removal of ninein disrupted this asymmetry and prematurely depleted progenitors [29]. The pattern is not limited to neural progenitors. Izumi and Kaneko demonstrated asymmetric centrosome inheritance in human neuroblastoma cells, with the older mother centriole migrating to one daughter and the younger to the other [30]. Barandun and colleagues found the same principle in mammalian CD8⁺ T-cells: the daughter that inherits the mother centrosome becomes an effector cell; the other becomes a memory cell [7]. Zhao and colleagues showed that in zebrafish radial glia, the mother centrosome coordinates polarized endosome trafficking to determine whether a division produces two progenitors or a progenitor and a neuron [15].

In parallel, asymmetric segregation of epigenetic information during germline stem cell division has been demonstrated for histones in Drosophila [31] and for the centromeric histone variant CENP-A, whose age-dependent loss in germline stem cells — regulated by CENP-C — provides a direct molecular link between asymmetric inheritance and epigenetic age [32]. The 'mitotic drive' model provides a framework for how epigenetically distinct sister chromatids are differentially segregated [33]. While recent work using photoconvertible reporters has questioned whether histones H3.1 and H3.3A are asymmetrically segregated in all stem cell contexts [34], centrosome asymmetry is independently validated across multiple species using diverse methodologies.

The direction is tissue-specific — mother centrosome to stem cell in some contexts, to differentiated daughter in others — but the principle is conserved: centrosome asymmetry carries fate-relevant information.

### 2.5. Human Cells: A Centriole Protein That Triggers Senescence

In 2026, Raheja and colleagues reported that the microprotein miP-FERMT3 localizes to centriole subdistal appendages — the same structures that anchor ninein. It triggers p53-independent senescence by promoting proteasomal degradation of p21 [16]. FERMT3 expression increases with age in both mouse and human endothelium. Separately, Ozaki and Tsou showed that the intrinsically disordered protein ALMS1 seeds centriole cartwheel assembly and may confer a form of structural memory on the organelle [17].

These are the first molecular chains traced from specific centriole substructures to specific cell-fate outcomes. They run through the same appendages that control asymmetric centrosome inheritance. A third protein, ATF5, forms a physical bridge between the pericentriolar material and the proximal end of the mother centriole, its localisation dependent on centriole age and polyE status [18]. The ciliary membrane attached to the mother centriole is itself asymmetrically inherited during neural stem cell division, and the daughter that inherits it re-establishes a signalling-competent primary cilium earlier than its sibling [35] — a direct demonstration that centrosome-associated structures carry information that biases daughter cell behaviour. A centrosome-centric review of asymmetric stem cell division summarises the current state of the field [36]. Together, ATF5, FERMT3, ALMS1, and asymmetrically inherited ciliary membrane are candidate components of a Centrosome-Associated Memory Complex (CAMC) — a hypothetical assembly that translates centriole history into cell-fate decisions.

---

## 3. The Central Experiment

### 3.1. Rationale

Renzova and colleagues provided the closest precedent [19]. They treated human embryonic stem cells and iPSCs with centrinone, a PLK4 inhibitor. The cells lost their centrioles and, within days, their pluripotency. OCT4 and NANOG were degraded. The cells differentiated into all three germ layers. The mechanism was partly p53-dependent and partly proteasome-mediated.

That experiment showed that the centriole is required to *maintain* pluripotency in cells that already possess it. It says nothing about whether the centriole is required to *maintain the differentiated state* in cells that possess that.

If the centriole anchors whatever identity the cell currently holds — pluripotency in a stem cell, commitment in a fibroblast — then removing it from a fibroblast should make the cell more responsive to reprogramming signals, not less. The logic is symmetric: the same anchor that holds a stem cell in its state holds a fibroblast in its state. Remove the anchor and the cell becomes more malleable.

This prediction has never been tested. A systematic PubMed search for "(centriole OR centrosome) AND (iPSC OR reprogramming) AND (OSKM OR Yamanaka)" returns no results. Pierre Gönczy, whose laboratory at EPFL leads the field of programmed centriole elimination, confirmed in a personal communication (July 9, 2026) that no such experiment exists in the literature.

### 3.2. Design

The basic experiment uses p53-knockout BJ human fibroblasts with SB203580 to block p38 stress kinase. Wild-type cells arrest after centriole loss; the double blockade lets them continue dividing through the reprogramming window. Sendai virus delivers OCT4, SOX2, KLF4, and c-MYC. The primary endpoint is the number of TRA-1-60-positive colonies at day 21.

Fourteen groups are required to distinguish five competing hypotheses (Table 1). Groups 1–3 test whether centriole loss affects reprogramming and whether the effect depends on p53/p38 stress. Group 4 controls for Aurora A off-target effects. Group 5 tests the nucleus-to-cilium senescence pathway (KIFC3). Group 6 tests the centriolar fate-switch protein ODF2. Group 7 tests cilium loss without centriole loss (IFT88). Group 8 tests proteostasis (MG132). Group 9 provides genetic confirmation (SAS6-KO). Groups 10–11 use laser ablation to distinguish centriole removal from CAMC removal. Group 12 tests reversibility (centrinone washout). Group 13 discriminates centriole effects from pericentriolar material effects (CEP192-KD). Group 14 — added in v9 — tests totipotency: centriole elimination followed by DUX4 and TPRX1, with MERVL and Zscan4 as readouts.

**Table 1. The Fourteen Groups**

| Group | Treatment | What it tests |
|:---:|---|------|
| 1 | DMSO → OSKM | Baseline reprogramming |
| 2 | Centrinone 500 nM × 3d → OSKM | Centriole loss effect |
| 3 | Centrinone + p53-KO + p38i → OSKM | p53/p38 independence |
| 4 | MLN8237 10 nM → OSKM | Aurora A off-target control |
| 5 | KIFC3-KD → OSKM | Senescence pathway |
| 6 | Odf2-KO → OSKM | Fate-switch protein |
| 7 | IFT88 siRNA → OSKM | Cilium without centriole loss |
| 8 | Centrinone + MG132 → OSKM | OCT4/NANOG proteostasis |
| 9 | SAS6 CRISPR-KO → OSKM | Genetic confirmation |
| 10 | Laser ablation, both centrioles → OSKM | Physical removal |
| 11 | Laser ablation, mother only → OSKM | Asymmetric inheritance test |
| 12 | Centrinone washout → OSKM | Reversibility |
| 13 | Centrinone + CEP192-KD → OSKM | PCM vs centriole |
| 14 | Centrinone + DUX4 + TPRX1 | Totipotency (MERVL/Zscan4) |

### 3.3. Predictions

If the centriole is a barrier to reprogramming, centriole elimination should raise colony counts 1.5- to 10-fold, with the precise magnitude depending on cell type, elimination method, and the completeness of p53/p38 suppression. The lower bound (1.5×) reflects the scenario where the centriole is one of many barriers; the upper bound (10×) requires full stress-pathway ablation and optimal conditions.

If physical ablation (which removes the centriole's appendage proteins together with the organelle) outperforms centrinone (which leaves appendage proteins on surviving centrioles), the barrier resides in those appendage proteins — consistent with a Centrosome-Associated Memory Complex (CAMC).

If centriole loss lowers colony counts, the centriole is required for reprogramming and the hypothesis is wrong. If there is no change, the centriole is irrelevant.

If Group 14 (centriole elimination + DUX4 + TPRX1) produces MERVL-positive, Zscan4-positive cells while Group 3 (centriole elimination + OSKM) produces only pluripotent colonies, the centriole barrier extends beyond pluripotency to totipotency — consistent with the observation that the germline resets its centrioles at every generation.

Every outcome is informative.

---

## 4. The Four-Counter Architecture (MCARA)

Aging is not a single process. It is the output of four counters running in parallel, each measuring a different aspect of cellular history. When three are silenced, the fourth continues.

**Table 2. The Four Counters**

| # | Counter | What it measures | Can it be slowed? | Partner |
|:---:|---|---|---|---|
| C1 | **Centriole** | Divisions survived; centriole/cilium ratio via polyE | Unknown | GLA |
| C2 | **Telomere** | Telomere length, DDR | Yes (hTERT) | — |
| C3 | **Mitochondrion** | mtDNA mutations, ROS | Yes (2% O₂) | Suomalainen |
| C4 | **Epigenome** | CpG methylation, Horvath clock | ~30–40% (partial reprogramming) | Wagner |

A cell protected by hTERT (C2), low oxygen (C3), and partial reprogramming (C4) has silenced three counters. It still arrests. The arrest is accompanied by centriole elongation, polyE accumulation, and cilium shortening — the signature of C1. The four-counter architecture accounts for this: when three counters are slowed, the fourth continues.

---

## 5. Discussion

The centriole is absent from the Hallmarks of Aging [20] and from the Hallmarks of Stem Cell Aging [21]. This is a notable omission if it is correct that the organelle functions as a structural memory device whose accumulated history determines whether a cell can change identity.

The framework makes testable predictions. The centriole-to-cilium ratio should increase with passage number in cultured cells. PolyE should correlate with this ratio. Cells with high ratios should be harder to reprogram. Removing the centriole should make them easier.

The framework also suggests interventions. TTLL5 and TTLL6 catalyse polyglutamylation. Their inhibition should slow polyE accumulation. Robichaud and colleagues have already shown that TTLL5 and TTLL6 are required for the nucleus-to-cilium microtubule arrays that initiate senescence [22]. Whether inhibiting them delays commitment — or merely masks the counter — is itself testable. A centriole whose polyE is removed has not had its history erased; it has had its odometer rolled back. Structural age — the number of division cycles the microtubule cylinder has survived — cannot be erased by modifying a side-chain. Full reset requires centriole removal and de novo assembly, as occurs in the germline [13].

The planarian finding — centrosome loss with centriole retention — raises an important question. If planarian neoblasts retain centrioles but have lost centrosomes (PCM), and yet maintain maximal plasticity, what is the true barrier: the centriole or the PCM? The PCM anchors proteins like PCM1, CDK5RAP2, and pericentrin that organise signalling complexes independently of the centriole cylinder. Group 13 of the experiment (CEP192-KD) begins to address this, but a systematic comparison of the centrosome proteome in stem cells versus differentiated cells across species would be informative.

---

## 6. Limitations

No direct experimental data exist for the central prediction. The polyE-to-ratio hypothesis is novel and untested. Centrinone has off-target effects on PLK1 at high concentrations; dose-response curves and orthogonal approaches (STIL shRNA, SAS6-KO) mitigate but do not eliminate this concern. The p53 knockout required for the central experiment means the result is conditional on p53 status. C. elegans data are from L1 larvae only; the adult map is incomplete. FERMT3 data are from endothelium and have not been replicated in fibroblasts. The molecular mechanism of centriole elimination in C. elegans somatic cells remains unknown — only the phenomenology has been described [11]. CAMC is a working hypothesis; no direct structural evidence for a unitary memory complex exists, though three candidate components (ATF5, FERMT3, ALMS1) have been functionally characterised. The planarian evidence is based on a single ultrastructural study [5] and requires independent replication.

---

## 7. Conclusion

Cells have brakes. Telomeres. Mitochondria. When both are protected, cells still stop. The centriole may be the brake that remains. A single experiment — centriole elimination followed by reprogramming — can test whether it is. The experiment has not been done. It is technically feasible with off-the-shelf reagents. It costs less than three million euros and takes three years. It is time to do it.

---

## References

[1] Bodnar AG et al. Extension of life-span by introduction of telomerase into normal human cells. *Science* 279:349–352 (1998). PMID: 9454332.

[2] Parrinello S et al. Oxygen sensitivity severely limits the replicative lifespan of murine fibroblasts. *Nat Cell Biol* 5:741–747 (2003). PMID: 12855956.

[3] Passanisi C, Spencer SL. Single-cell analysis reveals that telomere length and DNA damage do not distinguish cycling from senescent fibroblasts. *iScience* (2026). PMID: 41816297.

[4] Croisier M, Gönczy P. Electron microscopy confirms that only seven cells retain centrioles in C. elegans L1 larvae. *MicroPubl Biol* (2025). PMID: 40475707.

[5] Azimzadeh J, Wong ML, Downhour DM, Sánchez Alvarado A, Marshall WF. Centrosome loss in the evolution of planarians. *Science* 335:461–463 (2012). PMID: 22223737.

[6] Yamashita YM et al. Asymmetric inheritance of mother versus daughter centrosome in stem cell division. *Science* 315:518–521 (2007). PMID: 17255513.

[7] Barandun N et al. Targeted localization of the mother centrosome in CD8+ T cells undergoing asymmetric cell division promotes memory formation. *Cell Rep* 44:115127 (2025). PMID: 39764850.

[8] Atmakuru S, Dhawan J. Centrosome-cilium axis: the yin and yang of organelle function. *BioEssays* 45:2300011 (2023). PMID: 37144419.

[9] Janke C, Magiera MM. The tubulin code and its role in controlling microtubule properties and functions. *Nat Rev Mol Cell Biol* 21:307–326 (2020). PMID: 32107477.

[10] Kalbfuss N, Gönczy P. Extensive programmed centriole elimination unveiled in C. elegans embryos. *Sci Adv* 9:eadg8682 (2023). PMID: 37256957.

[11] Kalbfuss N, Gönczy P. Towards understanding centriole elimination. *Open Biol* 13:230222 (2023). PMID: 37963546.

[12] Pimenta-Marques A et al. A mechanism for the elimination of the female gamete centrosome in Drosophila melanogaster. *Science* 353:aaf4866 (2016). PMID: 27229142.

[13] Schatten H, Sun QY. Centrosome inheritance during fertilization. *Dev Dyn* 240:1971–1982 (2011). PMID: 21509822.

[14] Li Y et al. Characterisation of centriole biogenesis during multiciliation in planarians. *Biol Cell* 112:398–408 (2020). PMID: 32776587.

[15] Zhao X et al. PCM1 conveys centrosome asymmetry to polarized endosome dynamics in regulating daughter cell fate. *Nat Commun* 16:10728 (2025). PMID: 41315244.

[16] Raheja R et al. A microprotein encoded by FERMT3 modulates endothelial cell protein catabolism and induces cell cycle arrest and senescence. *Nat Commun* (2026). PMID: 42343301.

[17] Ozaki K, Chang TJB, Yang WQ et al. Adaptable centriole biogenesis via the intrinsically disordered protein ALMS1. *Nat Commun* (2026). PMID: 42380124.

[18] Madarampalli B et al. ATF5 connects the pericentriolar materials to the proximal end of the mother centriole. *Cell* 162:580–592 (2015). PMID: 26213385.

[19] Renzova T et al. Inactivation of PLK4-STIL module prevents self-renewal and triggers p53-dependent differentiation in human pluripotent stem cells. *Stem Cell Reports* 11:959–972 (2018). PMID: 30197118.

[20] López-Otín C et al. Hallmarks of aging: an expanding universe. *Cell* 186:243–278 (2023). PMID: 36599349.

[21] Brunet A et al. Hallmarks of stem cell aging. *Cell Stem Cell* 32:751–774 (2025). PMID: 40562035.

[22] Robichaud JH et al. Transiently formed nucleus-to-cilium microtubule arrays mediate senescence initiation in a KIFC3-dependent manner. *Nat Commun* 15:7954 (2024). PMID: 39266565.

[23] Pierron M et al. Centriole elimination during C. elegans oogenesis initiates with loss of the central tube protein SAS-1. *EMBO J* 42:e115076 (2023). PMID: 37987153.

[24] Jeffries A et al. Cilium resorption failure is sufficient to induce cellular senescence. *Aging Cell* 18:e12984 (2019). PMID: 30596512.

[25] Camargo Ortega G et al. The centrosome protein AKNA regulates neurogenesis via microtubule organization. *Nature* 567:113–117 (2019). PMID: 30787442.

[26] Mikulenkova E et al. NANOG/NANOGP8 localizes at the centrosome and is spatiotemporally associated with centriole maturation. *Cells* 9:692 (2020). PMID: 32168958.

[27] Tkemaladze J. Reduction, proliferation, and differentiation defects of stem cells over time: a consequence of selective accumulation of old centrioles in the stem cells? *Mol Biol Rep* 50:2751–2761 (2023). PMID: 36583780.

[28] Lindhout FW et al. Centrosome-mediated microtubule remodeling during axon formation in human iPSC-derived neurons. *EMBO J* 40:e106798 (2021). PMID: 33835529.

[29] Wang X et al. Asymmetric centrosome inheritance maintains neural progenitors in the neocortex. *Nature* 461:947–955 (2009). PMID: 19829375.

[30] Izumi H, Kaneko Y. Evidence of asymmetric cell division and centrosome inheritance in human neuroblastoma cells. *Proc Natl Acad Sci USA* 109:18048–18053 (2012). PMID: 23064640.

[31] Tran V et al. Asymmetric division of Drosophila male germline stem cell shows asymmetric histone distribution. *Science* 338:679–682 (2012). PMID: 23118191.

[32] Carty BL et al. CENP-C functions in centromere assembly, the maintenance of CENP-A asymmetry and epigenetic age in Drosophila germline stem cells. *PLoS Genet* 17:e1009247 (2021). PMID: 34014920.

[33] Ranjan R, Chen X. Mitotic drive in asymmetric epigenetic inheritance. *Biochem Soc Trans* 50:675–688 (2022). PMID: 35437581.

[34] Li A et al. Reevaluation of whether histones are asymmetrically segregated during asymmetric divisions of stem cells in Drosophila. *Proc Natl Acad Sci USA* 122:e2513015122 (2025). PMID: 41166424.

[35] Paridaen JT et al. Asymmetric inheritance of centrosome-associated primary cilium membrane directs ciliogenesis after cell division. *Cell* 155:333–344 (2013). PMID: 24120134.

[36] Chen C, Yamashita YM. Centrosome-centric view of asymmetric stem cell division. *Open Biol* 11:200314 (2021). PMID: 33435817.

---

*Version 9 — 2026-07-17. Planarian data corrected (Azimzadeh 2012). C. elegans mechanism acknowledged as unknown. Prediction changed to 1.5–10× range. Totipotency arm (Group 14) added. DID-RNA removed from main text. 36 PMIDs.*
