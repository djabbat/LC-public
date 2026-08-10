# MCARA — Project Map

**Date:** 2026-07-08

## Structure

MCARA/
├── _pi.md              — pi Rules
├── CONCEPT.md          — Concept (Counter Race v4.4)
├── TODO.md             — Tasks
├── PARAMETERS.md       — Parameters
├── MAP.md              — This Map
├── STATE.md            — Current State
├── MEMORY.md           — Decision History
├── README.md           — Introduction
├── DESIGN.md           — Experimental Design
├── THEORY.md           — Theoretical Basis
├── EVIDENCE.md         — Evidence Base
│
├── CEDAR/              — Counter #1: Centriolar Theory + Simulations + Code
│   ├── Aubrey/         —   ARGUS: Scientific Core (Hardware Architecture)
│   ├── CellLineageTree/—   Cell Lineage Simulation
│   ├── simulator/      —   CEDAR Simulator
│   └── ...
│
├── EpigeneticDrift/    — Counter #2: Epigenetic Clock
├── MitoROS/            — Counter #3: Mitochondrial
├── Telomere/           — Counter #4: Telomeric
│
├── _archive/ARGUS-LP_v3_2026-06/  — Outdated ARGUS-LP version (June 2026). Current: Marketing/ARGUS-LP_OS v49.
│   ├── CONCEPT.md      —   6 Microscope Versions (V1–V6), AI Agent
│   ├── docs/           —   Correspondence, BOM, Audits
│   └── ...
│
├── Aubrey/             — 🆕 Scientific Project (Moved from Marketing 2026-07-08)
│   ├── CONCEPT.md      —   First Human Centriolar Atlas
│   ├── Phase-0/        —   Critical Check Experiment
│   ├── Phase-A/        —   ARGUS-LP Platform
│   ├── Phase-B/        —   Counter Race
│   └── ...
│
├── crates/             — Rust Code (mcara_core, mcara_api, mcara_cli, mcara_compare, mcara_simulation, mcara_tests)
├── backend/            — Python Backend
├── frontend/           — Web Interface
├── docs/               — Documentation
├── audits/             — Peer Review
├── letters/            — Letters to Partners
├── refs/               — References
└── data/               — Data

## Subprojects

| Subproject | Type | Role |
|-----------|-----|------|
| **CEDAR** | Theory + Code | Counter #1 — Centriolar |
| **EpigeneticDrift** | Theory | Counter #2 — Epigenetic |
| **MitoROS** | Theory + Code | Counter #3 — Mitochondrial |
| **Telomere** | Theory + Code | Counter #4 — Telomeric |
| **ARGUS-LP_OS** | Tools + Grants | AI Robot for Lineage Tracking | → Marketing/ARGUS-LP_OS |
| **Aubrey** | Scientific Project | Centriolar Atlas, BOLD PILOT, EIC Consortium |

## Dependencies
- [LC] — Parent Project

## External Links

## Outputs
- EIC Pathfinder 2026 (28 Oct)
- MCARA Article (Biogerontology, ID 7cc6de62)
- CEDAR Article (npj Aging)
- ARGUS-LP_OS Platform → Marketing/ARGUS-LP_OS v49

**LERR — Ladder, Eliminate, Reprogram, Rebuild.**

**Step 1 (Ladder).** Cut the damage load first: slow the counter, push old centrioles into differentiating daughters, remove only the mother centriole, keep spare young ones.

**Step 2 (Eliminate).** Take out the old centriole. Restore telomeres. Wipe the epigenome. Rescue mitochondria.

**Step 3 (Reprogram).** Push to totipotency with DUX4 + KDM4D + DPPA3.

**Step 4 (Rebuild).** Grow fresh centrioles de novo. Derive clean, young adult stem cells.
**Step 1 (Ladder).** Де-риск перед элиминацией по текущим данным: замедлить счётчик (NAC-антиоксидант; обратимые PTM: TTL-ре-тирозинирование, CCP5/6-деглутамилирование); сегрегировать повреждения асимметричным наследованием материнской центриоли в дифференцирующееся потомство (Yamashita, 2007; Royall, 2023 — человеческие NPC); геми-элиминировать только материнскую центриоль (лазер/PROTAC), сохраняя контроль дупликации и избегая p53-зависимого G1-ареста (Meitinger, 2016); кондиционировать клетку (запасные PLK4-центриоли, синхронизация G1/S, протеостаз); отобрать наименее повреждённый пул (FACS по низкому Δ2/полиGlu).
**Step 2 (Eliminate).** Убрать старую повреждённую центриоль; восстановить теломеры (теломераза/ZSCAN4 через H3K14ac/H3K18ac; Meltzer, 2024); стереть эпигенетические метки (OSK/TET1-TET2-TDG; Lu, 2020 — частично, остаётся линейная память); отобрать здоровые митохондрии (PINK1-зависимая митофагия; Vázquez-Martín, 2016).
**Step 3 (Reprogram).** Индуцировать тотипотентность: DUX4 + KDM4D + DPPA3 — DUX4 открывает cleavage-стадийные гены (Hendrickson, 2017), KDM4D снимает H3K9me3-барьер репрограммирования, DPPA3 (Stella) стабилизирует тотипотентное (2C-подобное) состояние.
**Step 4 (Rebuild).** Пересобрать молодые центриоли de novo (PLK4 → SAS-6 → STIL → CPAP; Nigg & Holland, 2018; Gönczy, 2012) после полной элиминации (Khodjakov, 2002; Uetake, 2007); контроль геометрии (9-кратная симметрия, триплеты, длина); получить безопасные молодые взрослые стволовые клетки (проверка кариотипа, восстановление p53).
**Step 1 (Ladder).** Де-риск перед элиминацией: замедлить счётчик, сегрегировать повреждения, геми-элиминировать материнскую центриоль, кондиционировать клетку, отобрать наименее повреждённый пул.
**Step 2 (Eliminate).** Убрать старую центриоль; восстановить теломеры; стереть эпигенетические метки; отобрать здоровые митохондрии.
**Step 3 (Reprogram).** Индуцировать тотипотентность: DUX4 + KDM4D + DPPA3.
**Step 4 (Rebuild).** Пересобрать молодые центриоли de novo; получить безопасные молодые взрослые стволовые клетки.
