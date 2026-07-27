# CEDAR — Design

**Version:** 1.2 | **Updated:** 2026-07-26

## Architecture Overview

CEDAR (Centriolar Entropy-Driven Aging Registry) is a computational and theoretical framework for modeling centriole damage accumulation as a driver of stem cell aging. It is Counter #1 of the MCARA (Multi-Counter Architecture of Aging).

## Components

### 1. Theory Layer (THEORY.md)
- 9 mechanisms of centriole-dependent damage (M1-M9)
- Axioms C1 (polyGlu accumulation) and C2 (asymmetric inheritance)
- CAMC (Centriole-Associated Markers of Commitment) hypothesis
- LLPS-mediated centrosome organization model

### 2. Simulation Layer (simulator/)
- Python-based Cell-DT simulator
- MCMC parameter calibration
- Sobol sensitivity analysis
- ROS (Reactive Oxygen Species) equation integration

### 3. Evidence Layer (EVIDENCE.md)
- Verified PMID database (~60+ PMIDs)
- Literature meta-analyses
- Empirical support/falsification tracking
- Falsification tests (FT1.1, FT1.2)

### 4. Hardware Layer (Aubrey-Platform/ARGUS-Hardware)
- OpenFlexure microscope for live cell imaging
- Centriole tracking in dividing cells
- GT335 immunofluorescence quantification

## Data Pipeline
```
PubMed API → EVIDENCE.md (verified PMIDs)
                ↓
          THEORY.md (axioms, equations)
                ↓
          simulator/ (Cell-DT calibration)
                ↓
          PARAMETERS.md (calibrated values)
                ↓
          CONCEPT.md (distilled concept)
```

## Key Design Decisions
1. **Verified PMIDs only** — no hallucinated references (post-Jul-2026 rule)
2. **Rust backend + Python simulation** — performance vs. flexibility trade-off
3. **Open source (Apache 2.0)** — GitHub.com/djabbat/LC
4. **Counter architecture** — each aging mechanism is a semi-independent counter in MCARA
