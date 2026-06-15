# Proteostasis — Project Map

## Structure

```
Proteostasis/
├── _pi.md          — PI rules
├── CONCEPT.md      — Concept
├── TODO.md         — Tasks
├── PARAMETERS.md   — Parameters
├── MAP.md          — This map
├── STATE.md        — Current state
├── MEMORY.md       — Decision history
└── README.md       — Introduction
```

## Dependencies
- Python 3.11+ with NumPy, SciPy, pandas, matplotlib
- BioPython for sequence analysis
- STRING database API for protein-protein interactions
- UniProt REST API for protein annotations
- Local HPC cluster (SLURM) for large-scale simulations

## Outputs
- `models/` — trained machine learning models for proteostasis network prediction
- `reports/` — quarterly progress reports (PDF)
- `data/` — processed datasets (CSV, HDF5)
- `figures/` — publication-ready figures (SVG, PNG)
- `docs/` — final documentation and user guide

## Goals & Tasks

### Goals
1. **Model the proteostasis network** — build a computational model integrating protein folding, degradation, and stress response pathways.
2. **Identify key regulatory nodes** — use sensitivity analysis to pinpoint proteins that most influence network stability.
3. **Validate predictions experimentally** — collaborate with wet-lab to test top candidates.
4. **Publish results** — prepare two manuscripts by end of next phase.

### Tasks (with priorities and deadlines)

| ID | Task | Priority | Deadline | Status |
|----|------|----------|----------|--------|
| T1 | Collect and curate proteostasis-related datasets from public databases | P0 | upon approval | Not started |
| T2 | Implement baseline ODE model of chaperone-mediated folding | P0 | after previous milestone | Not started |
| T3 | Integrate ubiquitin-proteasome system into the model | P1 | after previous milestone | Not started |
| T4 | Perform global sensitivity analysis (Morris method) | P1 | after previous milestone | Not started |
| T5 | Identify top 10 candidate regulators | P1 | next phase | Not started |
| T6 | Design validation experiments (siRNA knockdown, stress assays) | P2 | next phase | Not started |
| T7 | Analyze experimental results and refine model | P2 | next phase | Not started |
| T8 | Write first manuscript (model description) | P1 | next phase | Not started |
| T9 | Write second manuscript (experimental validation) | P2 | next phase | Not started |

**Priority definitions:**  
- P0: Critical path, must be completed on time.  
- P1: Important, but can slip by one quarter if needed.  
- P2: Desirable, but not blocking other tasks.