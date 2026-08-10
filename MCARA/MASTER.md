# MASTER — MCARA (Multi-Component Aging Oscillation Analysis)

**Version:** 2026-06-15 (pi audit)
**Project:** LC/MCARA

## Purpose
MCARA — an umbrella project for modeling multi-component aging oscillations. Combines 4 components (CEDAR, EpigeneticDrift, MitoROS, Telomere) into a single model.

## Structure


MCARA/
├── CONCEPT.md            ← this document (umbrella concept)
├── MASTER.md             ← this file (cross-references)
├── crates/mcara_*/        ← common crates (core, api, cli, simulation, compare, tests)
├── CEDAR/                ← Centriolar Entropy-Damage Accumulation Ratchet
│   ├── CellLineageTree/  ← cell lineage reconstruction
│   ├── Aubrey/           ← ARGUS-LP (article)
│   └── articles/         ← publications
├── EpigeneticDrift/      ← epigenetic drift
├── MitoROS/              ← mitochondrial ROS
├── Proteostasis/         ← proteostasis (protein homeostasis)
└── Telomere/             ← telomere length


## Cross-References

| Component | Depends on | Provides |
|-----------|-----------|---------------|
| **CEDAR** | — (root) | Basic centriolar aging model |
| **CellLineageTree** | CEDAR | Cell lineage tree |
| **ARGUS** | CEDAR + CellLineageTree | ML model for lineage tracing |
| **EpigeneticDrift** | CEDAR (8 links) | Epigenetic clock |
| **MitoROS** | CEDAR (8 links) | Mitochondrial dysfunction |
| **Proteostasis** | CEDAR (8 links) | Proteostasis |
| **Telomere** | CEDAR (9 links) | Telomere dynamics |

## Common Crates

| Crate | Purpose |
|-------|-----------|
| `mcara_core` | Common structures, types, constants |
| `mcara_api` | REST API for MCARA |
| `mcara_cli` | CLI interface |
| `mcara_simulation` | Oscillation simulation |
| `mcara_compare` | Model comparison |
| `mcara_tests` | Common tests |

## Weakening Links Rule

When changing CEDAR, check:
1. `EpigeneticDrift/CONCEPT.md` (8 links)
2. `MitoROS/CONCEPT.md` (8 links)
3. `Proteostasis/CONCEPT.md` (8 links)
4. `Telomere/CONCEPT.md` (9 links)

Aim to minimize direct links — use `mcara_core` as an intermediate layer.

## Subproject Status

| Subproject | CONCEPT | Code | Status |
|-----------|:-------:|:---:|--------|
| CEDAR | 2KB | ✅ | 🟡 Being prepared for submission |
| CellLineageTree | 36KB | ✅ | 🟡 Active |
| EpigeneticDrift | 44KB | ✅ | 🟢 Submitted |
| MitoROS | 71KB | ✅ | 🟢 Active |
| Proteostasis | 45KB | ✅ | 🟢 Active |
| Telomere | 49KB | ✅ | 🟢 Active |

## Next Steps
- CEDAR: complete language refinement, submit
- CellLineageTree: model validation
- Weaken direct links to CEDAR → use mcara_core