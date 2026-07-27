# MAP.md — CEDAR

**Parent:** LC/MCARA
**Updated:** 2026-07-26

## Structure

```
CEDAR/
├── _pi.md              — pi rules
├── CONCEPT.md          — CEDAR concept (centriolar damage accumulation theory)
├── TODO.md             — Tasks
├── PARAMETERS.md       — Model parameters (centriole aging)
├── MAP.md              — This map
├── STATE.md            — Current state
├── MEMORY.md           — Decision history
├── README.md           — Introduction
├── DESIGN.md           — Architecture
├── THEORY.md           — Theory (M1-M9, axioms, equations)
├── EVIDENCE.md         — Evidence base (empirical support)
├── Cargo.toml          — Rust workspace
├── Cargo.lock
├── LICENSE             — Apache 2.0
├── .gitignore
│
├── crates/             — Rust crates (cedar-core, cedar-sim, etc.)
├── backend/            — Server side (API, database)
├── frontend/           — Web interface (Streamlit)
├── scripts/            — Utility scripts (run.sh, cedar_autofix.sh)
├── docs/               — Documentation, verification reports
├── refs/               — References (PMID-named markdown files)
│
├── simulator/          — Python simulator CEDAR-v2
│   └── _pi.md
├── Aubrey-Platform/    — ARGUS hardware + Aubrey platform integration
│   ├── _pi.md
│   └── ARGUS-Hardware/
│       └── _pi.md
├── CellLineageTree/    — Cell lineage tree modeling
│   └── _pi.md
├── articles/           — Article manuscripts and submissions
│   └── _pi.md
├── submissions/        — Journal submission records
├── audits/             — Audit reports
├── _archive/           — Archived/obsolete files
└── target/             — Rust build output
```

## Key relationships
- **Parent:** LC/MCARA — Counter #1 (Centriolar) of MCARA
- **Sibling counters:** EpigeneticDrift, MitoROS, Proteostasis, Telomere
- **Subprojects:** simulator, Aubrey-Platform, CellLineageTree, articles
- **External:** PhD/ (dissertation), Services/ (scripts), Marketing/Aubrey/ (grant applications)

## Data flow
```
Literature (PubMed) → EVIDENCE.md → THEORY.md → CONCEPT.md
                                        ↓
                                simulator/cedar_sim
                                        ↓
                                PARAMETERS.md (calibrated)
                                        ↓
                                STATE.md (results, status)
```
