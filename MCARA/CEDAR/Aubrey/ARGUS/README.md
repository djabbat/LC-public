# ARGUS-LP — Autonomous Centriole Tracking Platform

**Status:** Hardware specification complete. Engineering subcontract (G. Tsomaia) pending.

## What is ARGUS-LP?

ARGUS-LP (Autonomous Robotic Genealogical Ultra-surveillance for Lineage Purification) is a **purpose-built**, COTS-assembled, fully autonomous live-cell imaging and centriole tracking station. It is **not** a Zeiss retrofit — the entire platform is designed from scratch for 24/7 unattended operation.

## Architecture

```
┌──────────────────────────────────────────────┐
│          Isolated Environmental Enclosure     │
│  ┌─────┐   ┌──────────────┐   ┌───────────┐ │
│  │ XY  │   │ sCMOS + 100× │   │  Fluidics │ │
│  │Stage│   │ Plan Apo obj │   │  (RITE    │ │
│  │     │   │ 488/561/405  │   │   stains) │ │
│  └─────┘   └──────────────┘   └───────────┘ │
│       O₂/CO₂ control · Brightfield · Galvo   │
└──────────────────────────────────────────────┘
         │
         ▼
┌──────────────────────┐
│  AI Station (RTX 4090)│
│  CellPose v3          │
│  spotiflow            │
│  DeepSeek-V3 local    │
└──────────────────────┘
```

## Key Decisions

| Aspect | ARGUS-LP (NEW) | Old ARGUS (deleted) |
|--------|----------------|---------------------|
| Microscope | Purpose-built, COTS | Zeiss IM 35 retrofit |
| AI | Local RTX 4090, zero API cost | Claude Code API (€8K recurring) |
| Biology | BJ-hTERT live cells, 6 mo | Simulator-only (beads) |
| Ablation | Autonomous (no operator confirm) | Manual |
| Engineering | G. Tsomaia subcontract (€3,900) | DIY |
| Language | English | Russian + English mixed |

## Links

| File | Location |
|------|----------|
| Biology CONCEPT | `../CONCEPT.md` (Aubrey) |
| Hardware spec | `~/Desktop/ARGUS-LP_hardware_spec.md` |
| Schematic (drawio) | `~/Desktop/ARGUS-LP_schematic.drawio` |
| Budget v5 | `~/Desktop/ARGUS-LP_budget_v5.md` |
| Meta-review | `~/Desktop/Marketing/Aubrey/Aubrey_META-REVIEW_v5.md` |
| Engineering refs | `refs/` (PMID summaries) |
