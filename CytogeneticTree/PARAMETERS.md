# PARAMETERS — CytogeneticTree

## Experimental parameters

| Parameter | Value | Units | Source | Status |
|-----------|-------|-------|--------|--------|
| Pulse duration (tamoxifen for Cre-ERT2) | 24-48 | hours | standard Cre-ERT2 protocol | planned |
| Recombination target efficiency | ≥90 | % switched within 48 h | benchmark from Royall 2023 | target |
| Imaging interval | 20 | minutes | balance time-res vs photo-damage | planned |
| Ablation laser wavelength (primary) | 405 | nm | Cobolt 06-MLD CW | bought |
| Ablation laser wavelength (fallback) | 804 | nm | fs-IR if Month-2 photo-tox calibration fails | contingency |
| Objective | Plan-Apochromat 100×/1.4 NA oil | - | required for 300 nm centriole-pair resolution | bought |
| Camera pixel resolution | 2448×2048 mono | pixels | FLIR BFS-U3-63S4M-C or equivalent | bought |
| CO₂ concentration in chamber | 5.0 ± 0.2 | % | SenseAir S8 0-10% sensor | planned |
| O₂ concentration in chamber | 3.0 ± 0.3 | % | physiological hypoxia (Parrinello 2003) | planned |
| Temperature in chamber | 37.0 ± 0.5 | °C | standard mammalian culture | planned |
| Experiment duration | 6 | months | until arrest or end | planned |
| Target population-doubling range | 50-300 | PD | bounded by arrest detection + cost | planned |
| Biological replicates | ≥3 | clonal founder lines | per arm, per Impetus power calc | required |
| Experimental arms | 6 | (Arm 0/1/2/3/4/4b + Rescue) | per Impetus LOI v25.1 | required |

## Budget line items (from Impetus LOI v25.1 Phase A $92,000)

| Line | Amount | Notes |
|------|--------|-------|
| AutomatedMicroscopy hardware + laser | $14,500 | incl. 100× oil objective + Cobolt 405 nm |
| RITE-Centriolin de-novo cloning | $7,000 | Twist Bio synth + HEK293T packaging |
| CCP1 / PACT-CCP1 / TTLL6 constructs | $9,000 | lentiviral, Addgene |
| Cell culture | $15,000 | BJ-hTERT, media, hypoxia rental, consumables |
| Antibodies + IF (GT335, Ninein, ARL13B, secondaries) | $8,000 | - |
| Technician 50% FTE × 6 mo | $18,000 | daily culture, sample processing |
| Lab space / partnership fee | $10,000 | - |
| General consumables | $7,500 | plastics, cryo, pipettes |
| AI/software subscription | $120 | Claude Code Team, 6 mo |
| NGO admin (5%) | $4,880 | - |

## Computational parameters

| Parameter | Value | Notes |
|-----------|-------|-------|
| Segmentation model | CellPose 3.0 | Stringer 2021 |
| GPU for real-time inference | RTX 3080 Ti or better | local compute for live analysis |
| Storage per month | ~2 TB | 2448×2048 mono @ 20-min intervals × 6 channels × 6 mo |
| Archive storage total | ~12 TB | full experiment raw image + processed data |
| DAG algorithm | custom (GenealogyReconstruction subproject) | Python + NetworkX |

## Statistical parameters

| Parameter | Value | Source |
|-----------|-------|--------|
| α (significance) | 0.05 | standard |
| β (power) | 0.20 | standard |
| Test | log-rank (Kaplan-Meier survival) | primary endpoint |
| MCMC chains | 4 × 2000 iterations | Bayesian calibration |
| Gelman-Rubin target | R̂ < 1.05 | convergence |

## Open: dependent on agent-generated KNOWLEDGE.md

Parameters above are design targets. Once `KNOWLEDGE.md` literature search completes, cross-check against published values and update this file.
