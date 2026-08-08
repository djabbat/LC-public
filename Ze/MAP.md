# Ze — MAP

**Date:** 2026-06-11

---

## Project Structure


Ze/
├── _pi.md            — Pi rules
├── CONCEPT.md        — Concept (Ze Vectors Theory)
├── PARAMETERS.md     — Key parameters (v, τ, Z, χ)
├── MAP.md            — This map
├── MEMORY.md         — Decision History
├── STATE.md          — Current state
├── TODO.md           — Tasks
├── README.md         — Introduction
│
├── Ze_Model/         — Theoretical model (Found. of Physics)
├── Ze_CHSH/          — CHSH inequality (QSMF)
├── Ze_D/             — Multilayer age (Physica A)
├── Ze-Hierarchy/     — Age hierarchy in bristlebot swarms (NLnet)
│
├── simulator/        — Rust simulator (ze-core + ze-runner)
├── website/          — Interactive digital twin (Phoenix)
│   └── ze_sim/       — Simulation on the website
│
├── Articles/         — Published articles
├── docs/             — Documentation
├── grants/           — Grant applications
├── refs/             — Links and sources
└── audits/           — Audits


## Dependencies


Ze_Model ──→ Ze_CHSH ──→ Ze_D ──→ Ze-Hierarchy
    │            │
    └────────────┴──→ simulator ──→ website


## Key outputs

- 42 articles published
- 4 articles submitted to journals (June 2026)
- 1 NLNet grant accepted (Ze-Hierarchy)
- Rust simulator (open source)
