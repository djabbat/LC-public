# Organismal Aging

**A Self-Learning 4D Simulator of the Organism (3D + Time).**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-63%20passed-brightgreen)](https://github.com/djabbat/LC/actions)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)
[![Status](https://img.shields.io/badge/status-Pre--Alpha-yellow)](https://github.com/djabbat/LC)

Models organismal development from zygote through ontogenesis to aging and death.  
Integrates the centriolar damage theory, 5 molecular aging counters (MCAOA), and inter-tissue Ze-conflicts into a unified computational platform.

---

## 🧬 Architecture (Three Levels)

```
LEVEL #3: 8 TISSUES + Ze-CONFLICTS
├── Different renewal periods (5 days → ∞)
├── Inter-tissue conflicts: Z_conflict(i,j) = |τᵢ·dLᵢ/dt − τⱼ·dLⱼ/dt|·C_ij
└── Disease at L_tissue > L_crit

LEVEL #2: 5 MCAOA COUNTERS
├── #2 Telomere — shortening, Hayflick limit
├── #3 Mitochondrial — ROS, dysfunction
├── #4 Epigenetic — methylation, Cdc42
├── #5 Proteostatic — protein aggregation
└── L_tissue = Σ wᵢ·fᵢ(Dᵢ)

LEVEL #1: PRIMARY CAUSE (PLUG-AND-PLAY)
└── Swappable module: centriole | telomeres | mitochondria | ...
```

---

## ⚡ Quick Start

```bash
# Clone
git clone https://github.com/djabbat/LC.git
cd LC/sim_core

# Run tests (63 tests)
cargo test

# Full human lifespan simulation (120 years)
cargo run --example life_simulation --release

# CLI: compare diets
cargo run --bin oa -- compare

# CLI: provenance audit
cargo run --bin oa -- audit
```

**Simulation output (human, baseline):**

| Metric | Value |
|---|---|
| Death | ~101 years (L_max → 0.99) |
| S_centriole | 0.01 → ~0.95 |
| Frailty Index | 0.00 → 0.69 |
| Diseases | 8 (neurons ~56 yr, heart ~53, HSC ~50...) |

---

## 📊 Project Status

| Component | Status |
|---|---|
| **sim_core** (Rust, engine) | ✅ v0.1.1 — 63 tests |
| **CLI** (`oa`) | ✅ simulate, audit, compare, species |
| **Python visualization** | ✅ plot_simulation.py |
| **Bayesian learning loop** | 🟡 Basic impl (Phase 3 target) |
| **INFOGEST (diets)** | ✅ 4 diets, digestion |
| **Interventions** | 🟡 7 types (CR, rapamycin...) |
| **ARGUS-LP bridge** | 🔴 In design |
| **Phoenix LiveView** | 🔴 In design |
| **GTEx/UKB calibration** | 🔴 Phase 4 |

---

## 📁 Structure

```
LC/
├── CONCEPT.md              ← Concept (458 lines)
├── THEORY.md               ← Mathematical formalism
├── EVIDENCE.md             ← 30+ verified PMIDs
├── OPEN_PROBLEMS.md        ← Open problems
│
├── sim_core/               ← Core engine (Rust)
│   ├── src/
│   │   ├── centriole/      ← Level #1
│   │   ├── counters/       ← Level #2 (5 counters)
│   │   ├── tissue/         ← Level #3 (8 tissues)
│   │   ├── organism/       ← Integration
│   │   ├── species/        ← Species (human, mouse...)
│   │   ├── learning/       ← Bayesian loop
│   │   ├── microbiome/     ← Gut, skin, oral
│   │   ├── macrobiome/     ← INFOGEST, diets
│   │   └── spatial/        ← 3D model
│   ├── tests/              ← Integration tests
│   └── examples/           ← Examples
│
├── Organismal_Aging/       ← Detailed documentation
└── docs/                   ← EIC Pathfinder, papers
```

---

## 🔬 Evidence Base

30+ PMIDs verified via PubMed API (2026-06-21):

| Key Publication | PMID |
|---|---|
| Argentieri 2025 — exposome vs genome (n=492,567) | 39972219 |
| Goeminne 2024 — organ-specific aging (>53,000 UKB) | 39488213 |
| Argentieri 2024 — proteomic clocks & multimorbidity | 39117878 |
| Tqemaladze 2023 — centriolar hypothesis | 36583780 |
| Horvath 2013 — epigenetic clock | 24138928 |
| Rockwood 2005 — Frailty Index | 16129869 |

Full list: [EVIDENCE.md](Organismal_Aging/EVIDENCE.md)

---

## 📖 Documentation

- [CONCEPT.md](CONCEPT.md) — full project concept
- [THEORY.md](Organismal_Aging/THEORY.md) — mathematical formalism
- [EVIDENCE.md](Organismal_Aging/EVIDENCE.md) — evidence base
- [OPEN_PROBLEMS.md]() — open problems
- [AUDIT_2026-06-21.md]() — code & document audit
- [EIC_PATHFINDER.md](Organismal_Aging/docs/EIC_PATHFINDER.md) — grant proposal

---

## 🏗️ Development

```bash
# Build
cargo build --release

# Test
cargo test

# Benchmark
cargo bench --bench aging_simulation

# Docs
cargo doc --open
```

---

## 📜 License

Apache 2.0 © 2026 Jaba Tqemaladze, MD — Georgia Longevity Alliance

---

## 🔗 Links

- **PubMed:** [Tqemaladze J (ORCID: 0000-0001-8651-7243)](https://pubmed.ncbi.nlm.nih.gov/?term=Tkemaladze+J)
- **MCAOA Preprint:** [Zenodo 10.5281/zenodo.20055806](https://doi.org/10.5281/zenodo.20055806)
- **ARGUS-LP:** [github.com/djabbat/Aubrey](https://github.com/djabbat/Aubrey)
