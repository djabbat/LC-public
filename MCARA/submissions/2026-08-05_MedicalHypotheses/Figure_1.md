# Figure 1: The Four-Counter Architecture of Replicative Aging

## Caption

**Figure 1. Four counters, one limit.** Four independent mechanisms track replicative history in dividing cells. **(A)** The centriole (C1) alternates between a basal body state (cilium, signalling-competent) and a centrosome state (spindle pole, division-competent). The cumulative centriole-to-cilium ratio is recorded by polyglutamylation (polyE) on centriolar microtubules. The centriole is the only counter without a known protective intervention. **(B)** Telomeres (C2) shorten with each division and are restored by telomerase (hTERT). **(C)** Mitochondria (C3) accumulate mtDNA damage from reactive oxygen species and are partially protected by low oxygen (2% O₂). **(D)** The epigenome (C4) accumulates CpG methylation changes (Horvath clock) and is partially reset by OSKM reprogramming. When C2, C3, and C4 are simultaneously protected (hTERT + 2% O₂ + partial reprogramming), cells still arrest — implicating C1 as the limiting counter.

---

## Schematic (ASCII — for conversion to vector graphic)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    FOUR COUNTERS OF REPLICATIVE AGING                     │
│                                                                           │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐   ┌──────────┐│
│  │  C1 CENTRIOLE│    │ C2 TELOMERE  │    │ C3 MITOCHON. │   │C4 EPIGEN.││
│  │              │    │              │    │              │   │          ││
│  │  Centriole   │    │  Telomere    │    │  mtDNA       │   │  CpG     ││
│  │  ──▶ Cilium  │    │  shortening  │    │  mutations   │   │  methyl. ││
│  │              │    │  ──▶ DDR     │    │  ──▶ ROS     │   │  ──▶ Age││
│  │              │    │              │    │              │   │          ││
│  │  polyE ↑     │    │  hTERT ✅    │    │  2% O₂ ✅    │   │  OSKM ⚠️ ││
│  │  = age       │    │  = protect   │    │  = protect   │   │  = ~40%  ││
│  │              │    │              │    │              │   │          ││
│  │  ❌ NO       │    │              │    │              │   │          ││
│  │  PROTECTION  │    │              │    │              │   │          ││
│  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘   └────┬─────┘│
│         │                   │                   │                │      │
│         └───────────────────┴───────────────────┴────────────────┘      │
│                                    │                                      │
│                                    ▼                                      │
│                         ┌────────────────────┐                           │
│                         │  REPLICATIVE ARREST │                           │
│                         │  (Hayflick limit)   │                           │
│                         └────────────────────┘                           │
│                                                                           │
│  ┌──────────────────────────────────────────────────────────────────┐    │
│  │  CENTRIOLE–CILIUM CYCLE (C1 — the rate-limiting counter)         │    │
│  │                                                                    │    │
│  │    CILIUM STATE              CENTROSOME STATE                     │    │
│  │    ┌──────────┐              ┌──────────┐                         │    │
│  │    │  ●━━━━━━ │              │    ━━━━● │                         │    │
│  │    │  ╱       │   division   │         ╲│                         │    │
│  │    │ ╱ short  │ ◀─────────▶ │ long     ╲│                        │    │
│  │    │  9+0 axon │              │  MTOC     │                        │    │
│  │    │ polyE ↓  │              │ polyE ↑   │                        │    │
│  │    │ Sig: ON  │              │ Sig: OFF  │                        │    │
│  │    └──────────┘              └──────────┘                         │    │
│  │   «listening»               «dividing»                            │    │
│  │                                                                    │    │
│  │  Young cell:  cilium long,  centriole short, polyE LOW            │    │
│  │  Old cell:    cilium short, centriole long,  polyE HIGH           │    │
│  │                                                                    │    │
│  │  polyE = structural index of cumulative centriole/cilium ratio    │    │
│  └──────────────────────────────────────────────────────────────────┘    │
│                                                                           │
│  ┌──────────────────────────────────────────────────────────────────┐    │
│  │  CRITICAL EXPERIMENT (untested)                                    │    │
│  │                                                                    │    │
│  │  p53-KO hFibro  ──▶ Centrinone  ──▶ OSKM  ──▶ iPSC colonies     │    │
│  │                    (centriole                                     │    │
│  │                     elimination)                                   │    │
│  │                                                                    │    │
│  │  Prediction: 1.5–10× increase if centriole = barrier             │    │
│  │  Falsification: ≤ control → centriole ≠ barrier                  │    │
│  └──────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘
```

## Notes for graphic designer

- Use a four-panel layout (A–D) corresponding to the four counters
- Panel A (centriole) should be the largest — it's the main argument
- Colour palette: muted scientific (blue for centriole, red for telomere, green for mitochondria, orange for epigenome)
- The centriole should be visually depicted as a cylinder with graded polyE density (darker = older)
- The centriole–cilium cycle should show the two states with a double-headed arrow
- Bottom panel: experimental design with predicted outcome (bar chart: control vs centrinone+OSKM)
