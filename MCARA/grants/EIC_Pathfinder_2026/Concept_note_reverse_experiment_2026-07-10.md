# A Direct Test: Does the Centriole Determine Differentiation Trajectory Across Generations?

**For:** Mónica Bettencourt-Dias / Lukáš Čajánek  
**From:** Jaba Tqemaladze, MD · Georgia Longevity Alliance  
**Date:** 10 July 2026  

This experiment has never been done. Pierre Gönczy confirmed as much on 9 July 2026: "I am not aware of this either."

---

## What we want to test

CEDAR (Tqemaladze, 2023) argues that the centriole is the cell's hardware — it accumulates polyglutamylation over time and cannot be repaired in situ. Unlike the epigenome, which reprogramming factors can reset, the centriole carries a directional arrow: it determines how far a cell has travelled along its differentiation trajectory, and in which direction it continues across generations.

The centriole does not simply block reprogramming. It defines the trajectory. A somatic cell has centrioles with decades of accumulated polyE and structural age. These are not neutral passengers — they actively orient the cell toward its somatic identity. Remove them, and the cell loses its directional bearing. The question is: does it then become freer to reprogram?

**Prediction: take away the centriole, and the cell loses the arrow that points it toward its somatic fate. Reprogramming should become more efficient.**

Nobody has checked. The tools exist. The logic is testable.

---

## The experiment

| Step | Action | Method |
|:---:|--------|--------|
| 1 | Eliminate centrioles | Plk4 siRNA (72 h) or centrinone (100 nM, 3 days) |
| 2 | Confirm loss | IF: CP110 + Cep135. Target: >90% acentriolar |
| 3 | Reprogram | Cytotune 2.0 Sendai virus, 21 days |
| 4 | Score | Alkaline phosphatase+ colonies |
| 5 | p53 control | p53 shRNA + Plk4 siRNA → OSKM |
| 6 | Ciliogenesis control | IFT88 shRNA → OSKM (blocks cilium, spares centriole) |
| 7 | Off-target control | STIL shRNA → OSKM (alternative centriole removal) |

### A practical detail that may matter

Uetake et al. (2007, PMID 17227892) showed that normal human cells without centrioles still enter S-phase — but then arrest through **p38**, not p53. Most people only block p53 in reprogramming experiments. If the cells arrest through p38 before they can reprogram, the experiment looks like a null result when it is not.

You need both:
- pifithrin-α (p53)
- SB203580 (p38)

This may be why no one has stumbled into this result before.

---

## A deeper question: centriole or CAMC?

There is a further puzzle here. When we say "remove the centriole," what exactly are we removing? The organelle is two things at once: a physical structure (microtubules, polyE) and a platform that may carry a Centrosome-Associated Asymmetric Segregation Memory complex. We do not know which one carries the directional information.

Three classes of elimination can tell them apart:

| Class | Method | What goes | CAMC? |
|-------|--------|-----------|:-----:|
| Physical | Laser ablation, microsurgery | Whole organelle + PCM | Gone |
| Chemical | Centrinone, Plk4 siRNA | Blocks new centrioles; old ones dilute over cycles | Stays |
| Antibody | GT335 loading (Bobinnec 1998) | Microtubules only; PCM remains | Unknown |

If laser ablation permits reprogramming but centrinone does not, CAMC carries the directional signal. If they give the same result, the microtubules are the arrow and CAMC is secondary.

This can be a follow-up paper. The core experiment — chemical elimination + OSKM — comes first.

---

## What we might find

| Outcome | vs control | What it means |
|---------|:----------:|---------------|
| **↑↑ (>10×)** | Massively more colonies | The centriole is the primary determinant of differentiation direction. Removing it frees the cell from its somatic trajectory. |
| **↑ (1.5–5×)** | Modestly more | The centriole contributes directional information — one input among several. |
| **No change** | Same | The centriole does not determine differentiation trajectory. CEDAR is wrong. |
| **↓** | Fewer colonies | Centrioles are required for pluripotency. Removing them destabilises the undifferentiated state (Renzova 2018). |

Every one of these is a paper.

---

## Why you

We are theorists. We have the framework, the logic, and the experimental design. What we do not have is a bench.

| Person | Expertise | Key paper | Why them |
|--------|-----------|-----------|----------|
| **Mónica Bettencourt-Dias** | Centriole biology, Polo kinase, oocyte centriole elimination | Science 2016, PMID 27229142 | Knows centriole elimination cold. Drosophila → mammalian transition is the obvious next step. |
| **Lukáš Čajánek** | PLK4/STIL, centrosome loss and differentiation in hPSC | Stem Cell Reports 2018, PMID 30197118 | Works in human pluripotent stem cells. Closest system to iPSC. |

Gönczy (EPFL) confirmed the reverse experiment has never been published. His lab is focused on C. elegans and declined to participate. That leaves this question open.

---

## What we bring

- The theoretical framework: CEDAR (Tqemaladze, 2023), MCARA multi-counter architecture
- A detailed protocol catalogue: 42 described experimental variations with power calculations (available on request)
- A catalogued literature base: 11 distinct centriole elimination methods across three classes
- Data analysis and manuscript writing

**What we need:** Someone who can do the bench work. Your lab has the plasmids, the cell lines, and the hands.

---

## Timeline and practicalities

- **3–4 months** for the core experiment
- **Authorship:** your lab as first author, Tqemaladze as corresponding
- **Target:** *Nature Cell Biology*, *Cell Stem Cell*, or *Developmental Cell*
- **Pre-registered:** OSF (DOI 10.17605/OSF.IO/KQBY4)
- **Funding:** We can discuss small consumables support; the protocol itself is modest (siRNA, Sendai virus, antibodies)

---

## References

| PMID | Author | Finding |
|------|--------|---------|
| 36583780 | Tqemaladze (2023) | CEDAR — centriole as determinant of differentiation trajectory |
| 17227892 | Uetake (2007) | p38-dependent arrest in acentriolar normal cells |
| 1934057 | Maniotis & Schliwa (1991) | Microsurgery: growth without centrioles |
| 9852152 | Bobinnec (1998) | GT335 disassembles centriolar MTs |
| 30197118 | Čajánek (2018) | PLK4/STIL, centrosome loss → differentiation in hPSC |
| 27229142 | Bettencourt-Dias (2016) | Centriole elimination in Drosophila oocytes |
| 37256957 | Kalbfuss & Gönczy (2023) | Programmed centriole elimination in C. elegans |
| 12855956 | Parrinello (2003) | 3% O₂ immortalises mouse, not human |

---

*Jaba Tqemaladze, MD · jaba@longevity.ge · +995 555 185161 · ORCID: 0000-0001-8651-7243*
