# Ontogenesis — LINKS

---

## Self-Citation (for publications from this project)

| Resource | URL / ID |
|----------|---------|
| PMID 36583780 — Tkemaladze J. *Mol Biol Rep* 2023 | https://pubmed.ncbi.nlm.nih.gov/36583780/ |
| PMID 20480236 — Lezhava T. et al. *Biogerontology* 2011 | https://pubmed.ncbi.nlm.nih.gov/20480236/ |
| Zenodo CDATA | https://doi.org/10.5281/zenodo.19174506 |
| Zenodo Ze | https://doi.org/10.5281/zenodo.19174630 |

---

## Data Sources

| Resource | URL |
|----------|-----|
| WHO Growth Charts (0–19 yr, height/weight) | https://www.who.int/tools/child-growth-standards |
| NHANES (US anthropometry, health survey) | https://www.cdc.gov/nchs/nhanes/ |
| CAESAR dataset (3D body scans) | https://www.humanics-es.com/caesar.htm |
| CBCL/ASEBA (psychological T-scores 0–18) | https://aseba.org/ |
| WISC-V / WAIS-IV (cognitive indices) | https://www.pearsonassessments.com/ |
| GSS — General Social Survey | https://gss.norc.org/ |
| ESS — European Social Survey | https://www.europeansocialsurvey.org/ |

---

## Key Literature (Developmental Endocrinology)

| Resource | URL / ID |
|----------|---------|
| GH/IGF-1 reference ranges review | https://pubmed.ncbi.nlm.nih.gov/25518808/ |
| Tanner stages (puberty staging) | https://pubmed.ncbi.nlm.nih.gov/1554967/ |
| Cortisol developmental norms | https://pubmed.ncbi.nlm.nih.gov/21705192/ |
| Testosterone/estradiol reference (pediatric) | https://pubmed.ncbi.nlm.nih.gov/20107158/ |
| Brain volume development MRI meta-analysis | https://pubmed.ncbi.nlm.nih.gov/22306083/ |
| CHIP aging (relevant for >25 boundary with CDATA) | https://pubmed.ncbi.nlm.nih.gov/28792876/ |
| Horvath epigenetic clock (boundary with CDATA) | https://pubmed.ncbi.nlm.nih.gov/24138928/ |

---

## Technology Stack

| Resource | URL |
|----------|-----|
| Rust language | https://www.rust-lang.org/ |
| Three.js (3D visualization) | https://threejs.org/ |
| WebGL 2.0 spec | https://registry.khronos.org/webgl/specs/latest/2.0/ |
| wasm-pack (Rust → WebAssembly) | https://rustwasm.github.io/wasm-pack/ |
| calamine crate (Excel import, planned) | https://crates.io/crates/calamine |

---

## Planned Publications (NEEDTOWRITE)

| Resource | Notes |
|----------|-------|
| Methods journal (empirical transition detection) | "Empirical Transition Detection in Human Ontogenesis 0–25 Years" — CV/Range algorithm |
| Computer graphics / sci-viz conference | "Ontogenesis v4.1: Rust + Three.js hybrid architecture" |
| Biomedical informatics journal | "Integrating ontogenetic trajectories in AIM clinical decision support" |

---

## Related Projects in Ecosystem

| Project | Path |
|---------|------|
| AIM (integrative medicine assistant) | ~/Desktop/AIM/ |
| CDATA (aging simulation, 25+ yr boundary) | ~/Desktop/CDATA/ |
| **Tree** (sub-project: mtDNA lineage tracing → differentiation tree) | ~/Desktop/Ontogenesis/Tree/ |
| lab_reference.py (uses Ontogenesis norms) | ~/Desktop/AIM/lab_reference.py |
| treatment_recommender.py (uses regen coefficients) | ~/Desktop/AIM/treatment_recommender.py |

---

## Tree Sub-project

**Tree** (`Ontogenesis/Tree/`) — построение дерева дифференциации через отслеживание транспланитрованных мутантных митохондрий (mtDNA-трассировка).

**Связь Tree ↔ Ontogenesis:**
- Каждый узел дерева Tree = потенциальный эмпирический переход Ontogenesis (CV/Range algorithm)
- Дерево Tree валидирует последовательность онтогенетических стадий 0–25 лет на клеточном уровне
- Возможная 3D-визуализация дерева Tree в Ontogenesis engine (Three.js)

**Добавлено:** 2026-03-31
