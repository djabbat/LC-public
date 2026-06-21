# MCAOA Simulator — Multi-Counter Architecture of Organismal Aging

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.85+-blue.svg)](https://www.rust-lang.org)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.20055806.svg)](https://doi.org/10.5281/zenodo.20055806)

**Reference:** Tqemaladze J. (2026) *Multi-Counter Architecture of Organismal Aging*. Submitted to eLife (ID eLife-RP-RA-2026-111885). Not published as Reviewed Preprint (no Reviewing Editor available). Appeal pending. Preprint on Zenodo: DOI [10.5281/zenodo.20055806](https://doi.org/10.5281/zenodo.20055806)

**Source code:** [github.com/djabbat/LC](https://github.com/djabbat/LC/tree/mcaoa-v3.2/MCAOA) (branch `mcaoa-v3.2`, в `MCAOA/`)

---

## Overview

MCAOA (Multi-Counter Architecture of Organismal Aging) is a formal, falsifiable framework that models aging as the parallel accumulation of damage across **5 independent molecular counters** in different tissues:

| # | Counter | Mechanism | Project |
|---|---------|-----------|---------|
| 1 | **Centriolar** (CDATA) | Polyglutamylation damage on maternal centrioles of stem cells | `LC/MCAOA/CDATA/` |
| 2 | **Telomeric** | Telomere shortening at each division (Hayflick limit) | `LC/MCAOA/Telomere/` |
| 3 | **Mitochondrial** | ROS accumulation, mtDNA damage, respiratory decline | `LC/MCAOA/MitoROS/` |
| 4 | **Epigenetic** | DNA methylation drift, Cdc42 polarity loss | `LC/MCAOA/EpigeneticDrift/` |
| 5 | **Proteostatic** | Protein aggregation, chaperone/autophagy decline | `LC/MCAOA/Proteostasis/` |

Candidate #6 (piRNA) is **EXPLORATORY** — see [OPEN_PROBLEMS.md]() for refuting evidence.

### Core Equations

Each counter accumulates damage as:

```
D_i(n, t) = D_i₀ + α_i · (n / n_i*) + β_i · (t / τ_i) + γ_i · I(others)
```

Tissue-integrated burden (calibrated against Frailty Index):

```
L_tissue = Σ_i [ w_i · f_i(D_i) ]    where f_i is sigmoidal (logistic)
L_tissue = FI / 0.7                  (FI = Frailty Index, Rockwood 2005)
L_critical = 0.60                    (FI > 0.42 → high mortality risk)
```

### Axioms (M1–M4)

| Axiom | Statement |
|-------|-----------|
| **M1** | At least 2 independent counters run in parallel |
| **M2** | Dimensional consistency: n and t are normalised by reference scales |
| **M3** | Tissue weights w_i are fixed *a priori*, not post-hoc |
| **M4** | Every claim must have a falsification test |

## Features

- **5-counter simulation** with tissue-specific drift rates and reference scales
- **6 tissue types**: fibroblast, HSC, neuron, hepatocyte, beta cell, CD8+ T memory
- **EDC (Endocrine Disrupting Chemical) module**: models thyroid disruptors (PCBs, bisphenols, PFAS) accelerating mito/proteostasis counters — designed for the Torres Ruiz group thesis proposal
- **Coupling matrix Γ**: directed influence between counters (default: independence)
- **CSV output** with per-step counter states and tissue load
- **API server** (axum) for HTTP-based simulation requests
- **L_tissue calibration** against Frailty Index (Rockwood 2005, Searle 2008)

## Installation

### Prerequisites

- Rust 1.85+ (install via [rustup](https://rustup.rs/))

### Build

```bash
cd LC/MCAOA
cargo build --release
```

### Run CLI Simulation

```bash
# Baseline HSC simulation, 100 division-equivalent steps
cargo run --release --bin mcoa-sim -- --tissue hsc --divisions 100

# With thyroid EDC exposure (0.8 = high)
cargo run --release --bin mcoa-sim -- \
    --tissue hsc --divisions 200 \
    --edc-exposure 0.8 --edc-target thyroid

# Post-mitotic neuron (slow aging)
cargo run --release --bin mcoa-sim -- \
    --tissue neuron --divisions 200 --seconds-per-division 2592000

# All options
cargo run --release --bin mcoa-sim -- --help
```

### Run Tests

```bash
cargo test
```

### Start API Server

```bash
cargo run --release --bin mcoa-api
# Server starts at http://localhost:3000
```

## Example Output

```
$ cargo run --release --bin mcoa-sim -- --tissue hsc --divisions 100
mcoa-sim v3.2: 100 steps, tissue=hsc, EDC=0/none, final L_tissue=0.1523 (below L_crit)
  Output: mcoa_run.csv
```

The CSV contains per-step values for all 5 counters + tissue load + EDC parameters.

## Project Structure

```
LC/MCAOA/
├── CONCEPT.md              # Phase III experimental design
├── THEORY.md               # Axiomatic foundation (M1–M4)
├── EVIDENCE.md             # Literature evidence with verification
├── OPEN_PROBLEMS.md        # Open problems + falsifiable tests
├── Cargo.toml              # Workspace definition
├── crates/
│   ├── mcoa_core/          # Core types: Counter, Tissue, Gamma, drift equations
│   ├── mcoa_simulation/    # Simulation engine: step(), run(), EDC module
│   ├── mcoa_cli/           # CLI binary (mcoa-sim)
│   ├── mcoa_api/           # HTTP API server (axum)
│   ├── mcoa_tests/         # Integration tests
│   └── mcoa_compare/       # Comparison with CDATA runs
├── CDATA/                  # Centriolar Damage Accumulation (Counter #1)
├── Telomere/               # Telomere counter (#2)
├── MitoROS/                # Mitochondrial counter (#3)
├── EpigeneticDrift/        # Epigenetic counter (#4)
├── Proteostasis/           # Proteostasis counter (#5)
└── scripts/                # Analysis scripts
```

## Dissertation Status

MCAOA Phase III Concept v3.2 — **Conditionally approved** for PhD defense (77.75/100) by the thesis committee (Prof. Torres Ruiz, UCLM).

See [CONCEPT.md](CONCEPT.md) for the full correction history.

## License

MIT — see [LICENSE](LICENSE)

## Citation

```bibtex
@software{mcoa_simulator,
  author = {Tqemaladze, Jaba},
  title = {MCAOA Simulator — Multi-Counter Architecture of Organismal Aging},
  year = {2026},
  publisher = {Zenodo},
  doi = {10.5281/zenodo.20055806},
  url = {https://github.com/djabbat/LC/tree/mcaoa-v3.2/MCAOA}
}
```

## Contact

Dr. Jaba Tqemaladze, MD  
President, Georgia Longevity Alliance  
Email: jaba@longevity.ge  
ORCID: [0000-0001-8651-7243](https://orcid.org/0000-0001-8651-7243)
