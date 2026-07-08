# Multi-Counter Architecture of Replicative Aging (MCARA)

## Jaba Tqemaladze · Georgia Longevity Alliance · jaba@longevity.ge

### Handout — AEON Workshop, 31 Aug – 2 Sep 2026, University of Cologne

---

## The problem

Human fibroblasts divide about fifty times and stop. Add telomerase, grow them at 2% oxygen — telomeres stay long, oxidative damage is minimal — and they still arrest [MacKenzie et al., 2000, PMID: 10964501; Franco et al., 2001, PMID: 11461114]. Something beyond telomeres and mitochondria is counting divisions.

I argue that the cell carries several independent counters, each accumulating a different kind of irreversible damage with every division. Under standard conditions they all reach their thresholds near the same time, which makes them look like a single clock. Silence one or two, and the race shifts to whoever is left. The cell stops when the first counter hits its limit.

---

## Four counters (alphabetical)

| Counter | What it measures | Silenced by hTERT + 2% O₂? |
|---------|-----------------|:---:|
| **Centriolar** | Polyglutamylation, oxidative damage, and architectural disruption of the mother centriole. Structural deterioration of the centrosome and primary cilium follows the molecular damage — they are two faces of one counter | No |
| **Epigenetic** | SA-DNAm: senescence-associated CpG methylation. The replication-coupled component of the Horvath clock | Slowed ~30–40%, not stopped |
| **Mitochondrial** | mtDNA mutations, ROS, membrane potential | Yes |
| **Telomeric** | 50–200 bp lost per division | Yes |

Under hTERT and 2% oxygen, the mitochondrial and telomeric counters are off. What remains is Centriolar versus Epigenetic.

---

## Why the centriole

The mother centriole duplicates conservatively — it serves as a template and is not diluted. Its structural proteins (δ/ε-tubulin, Sas-6, Cep135) show essentially no turnover, and no repair pathway is known. Oxidative damage, deamidation, and over-elongation accumulate without a mechanism to clear them.

**PolyE is an odometer, not the damage itself.** The real entropy lies in irreversible modifications — carbonylation, 4-HNE adducts, deamidation — measured by mass spectrometry. Polyglutamylation is a convenient proxy (GT335 antibody, FACS-sortable) but the proof requires direct measurement of structural damage. A single proteomics run yields both metrics.

Three mechanisms link centriole status to cell state. The centrosome concentrates PLK1, Aurora A, p53, and components of Wnt and Hedgehog — it is a signaling scaffold, not just a microtubule organizer. The same mother centriole that organizes the mitotic spindle can, in a different phase, template a primary cilium — the switch between these two states gates the division-versus-differentiation decision. And the tubulin code — polyglutamylation, acetylation, and other PTMs — modifies which motors and signaling complexes can bind, encoding a molecular memory of divisions that no enzyme fully erases.

A single empirical finding anchors the model in human data. Köhrer et al. examined 1,386 centrioles from eight healthy donors by electron tomography. Over-elongated centrioles rose from 45% at age 24 to 76% at 67 (Spearman ρ = 0.67, p < 0.01; *Leukemia*, 2023, PMID: 37821581).

The cross-phylum pattern strengthens the case. In *C. elegans*, 551 of 558 somatic cells dismantle their centrioles at the L1 stage — only the six cells that retain them keep dividing. In *Drosophila* oocytes, blocking centriole clearance breaks totipotency and causes sterility. Planarian neoblasts divide without centrioles; centrioles appear only when a cell commits to terminal differentiation. Knock down PLK4 in human ES cells and pluripotency markers drop while p53 drives differentiation. In every case, centriole removal accompanies a change in cell state.

---

## The key experiment

No one has tested whether removing centrioles from differentiated cells makes them easier to reprogram. The design:

| Step | Action |
|------|--------|
| 1 | Eliminate centrioles: Plk4 siRNA, 72 h, or centrinone 100 nM, 3 days |
| 2 | Confirm: >90% acentriolar by CP110/Cep135 immunofluorescence |
| 3 | Reprogram: Sendai OSKM, 21 days, **2% O₂** |
| 4 | Count alkaline phosphatase-positive colonies vs siScramble control |
| 5 | p53 control: p53 shRNA + Plk4 siRNA → OSKM |
| 6 | Stress control: Cep135 KD (partial disruption, centrioles intact) → OSKM |

I predict at least a twofold increase in iPSC colony formation. If the result is null, the epigenetic counter takes priority.

Running at 2% oxygen is essential. At 21% oxygen, mitochondrial damage adds noise — three counters run simultaneously and you cannot attribute the effect to any one. At 2% oxygen, mitochondrial and telomeric counters are silenced, leaving a clean Centriolar-versus-Epigenetic race.

---

## The Bodnar puzzle

BJ-hTERT fibroblasts are immortal at 21% oxygen [Bodnar et al., *Science* 1998] but undergo growth crisis at 2% [MacKenzie 2000; Franco 2001]. The difference is unexplained. I propose that at 21% oxygen, cells carrying old centrioles die by a p53/p16-dependent mechanism, and only clones with unusually young centrioles survive. What Bodnar called immortalization may be clonal selection. The test: single-cell lineage tracking of young-centriole versus old-centriole clones, with mass spectrometry of the actual irreversible damage — carbonylation and deamidation, not just the polyE odometer.

---

## From metaphor to measurement

MCARA translates "entropy in aging" into four concrete, measurable counters:

| Entropy type | Counter | How to measure |
|-------------|---------|----------------|
| Structural | Centriolar | Mass spectrometry (carbonylation, deamidation) + GT335 (polyE) + electron tomography (length, architecture) |
| Informational | Epigenetic | EPIC arrays (850K CpG), Epigenetic-Senescence-Signature |
| Energetic | Mitochondrial | Long-range PCR (mtDNA deletions), Seahorse (OCR/ECAR) |
| Terminal | Telomeric | Flow-FISH, qPCR (T/S ratio) |

The counters operate in parallel. The system halts when the first one reaches its threshold. Different tissues carry different active sets — postmitotic neurons only contend with mitochondrial and proteostatic damage, while skin and gut stem cells face all four — which explains tissue-specific aging rates. Partial reprogramming resets the epigenome but leaves the centriolar counter untouched. That is why it yields partial rejuvenation.

---

> jaba@longevity.ge · ORCID: 0000-0001-8651-7243
> Tqemaladze J. *Mol Biol Rep.* 2023. PMID: 36583780

*P.S.* Once the logic of irreversible entropy accumulation is worked out for dividing cells — the Hayflick limit, the replicative counters — the same framework extends to the whole organism: crystallin damage in the lens, lipofuscin in neurons, cardiomyocytes, and RPE, A2E in the retina, glucosepane crosslinks in collagen and elastin, deamidated myelin, racemized aspartate in dentin, mtDNA deletions, and nuclear pore proteins that last a neuron's lifetime. All of it, on the same thermodynamic footing, waiting to be mapped.
