# Proteostasis — Memory

## 2026-07-13 — Анализ Research Feed: ClpP/ClpXP и митохондриальный протеостаз

- **Событие:** Анализ статей из ленты Jaba + PubMed-поиск.
- **КЛЮЧЕВЫЕ НАХОДКИ ДЛЯ PROTEOSTASIS:**

### Серия ClpP/ClpXP — митохондриальный протеолитический комплекс:
| PMID | Статья | Год | Суть |
|------|--------|-----|------|
| 42281331 | Feng HW et al. ClpP → mitochondrial integrity, meiotic progression — Andrology | 2026 | cKO ClpP в сперматоцитах: дефекты митохондрий, мейотический блок |
| 37798322 | Guo C et al. ClpP/ClpX → mTORC1 signaling — Commun Biol | 2023 | mTOR-ось: протеостаз митохондрий → mTORC1 |
| 23851121 | Gispert S et al. Clpp null → infertility, mtDNA accumulation — Hum Mol Genet | 2013 | Первая характеристика Clpp KO |
| 38927630 | Key J, Gispert S, Auburger G. CLPP/CLPX in IMM matrix condensates — Genes | 2024 | Молекулярный механизм: CLPP/CLPX в конденсатах матрикса |
| 41655698 | Currie SQW et al. Mitochondrial AAA+ proteases mechanisms — J Biol Chem | 2026 | Обзор молекулярных механизмов AAA+ протеаз |
| 40903791 | Nandha SR et al. CLPP + LONP1 → proteotoxic stress — Cell Commun Signal | 2025 | Таргетирование CLPP/LONP1 → протеотоксический стресс → подавление опухоли |

- **Значение для Proteostasis:** ClpXP — протеолитический комплекс внутренней мембраны митохондрий. Серия из 6 статей показывает: ClpP дефект → накопление повреждённых белков → UPR^mt → mTORC1 → мейотический блок / infertility.
- **Связь с aging:** Нарушение митохондриального протеостаза — один из драйверов возрастной дисфункции герм-клеток.
- **Полный анализ:** `~/Desktop/Services/docs/RESEARCH_FEED_ANALYSIS_2026-07-13.md`

---

## Goals & Tasks

### Project Objectives
- Establish a structured framework for proteostasis research and development.
- Define clear milestones for computational modeling, experimental validation, and data integration.
- Ensure reproducibility and scalability of all project outputs.

### Tasks and Milestones

| Task | Priority | Deadline | Status |
|------|----------|----------|--------|
| Define core proteostasis pathways and key regulators | P0 | after completion of prerequisite tasks | Not started |
| Develop initial computational model of protein folding dynamics | P0 | after previous milestone | Not started |
| Collect and curate experimental datasets for model training | P1 | after previous milestone | Not started |
| Implement validation pipeline using known proteostasis modulators | P1 | next phase | Not started |
| Integrate multi-omics data (transcriptomics, proteomics) | P2 | next phase | Not started |
| Prepare manuscript and release open-source codebase | P2 | next phase | Not started |

## Decision Log

### Upon project initiation — Project Initialization
- **Decision:** Created the Proteostasis project.
- **Rationale:** Need for structured organization of proteostasis-related research and development.
- **Status:** Completed

### After initial planning — Scope Refinement
- **Decision:** Focus on chaperone-mediated folding and aggregation pathways.
- **Rationale:** Initial literature review highlighted these as most tractable for computational modeling.
- **Status:** In progress

### After scope refinement — Tool Selection
- **Decision:** Adopt Python-based simulation framework (PyRosetta, AlphaFold2 interface).
- **Rationale:** Community support, existing infrastructure, and flexibility for custom extensions.
- **Status:** Planned
## 2026-07-09 — Глубокий аудит MCARA
- **Находка:** ClpP, YME1L, LONP1, PARL — митохондриальные протеазы, критичные для cell fate
- **Гипотеза:** Протеостатическая ось митохондрий — универсальный механизм cell fate control
- **Решение:** Добавлены находки в EVIDENCE.md §v4
