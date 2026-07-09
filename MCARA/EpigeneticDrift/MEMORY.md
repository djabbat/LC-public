# EpigeneticDrift — Memory

## Decision History

### Upon Project Initialization — Project Initialization
- **Decision:** Created the EpigeneticDrift project
- **Rationale:** Need for structured development of an epigenetic drift analysis framework
- **Status:** Completed

## Goals & Tasks

### Project Objectives
1. Develop a computational pipeline to quantify epigenetic drift from DNA methylation data.
2. Validate the pipeline on public datasets (e.g., GEO, TCGA).
3. Build a predictive model for biological age based on drift metrics.
4. Publish results and open-source the code after previous milestone.

### Tasks with Priorities and Deadlines

| Task | Priority | Deadline | Status |
|------|----------|----------|--------|
| Collect and preprocess methylation datasets (GSE40279, GSE55763) | P0 | after completion of prerequisite tasks | Not started |
| Implement drift quantification algorithm (entropy-based) | P0 | after previous milestone | Not started |
| Create unit tests and validation framework | P1 | after previous milestone | Not started |
| Benchmark against existing age predictors (Horvath clock) | P1 | after previous milestone | Not started |
| Develop age prediction model using drift features | P0 | after previous milestone | Not started |
| Write documentation and user guide | P2 | after previous milestone | Not started |
| Prepare manuscript for submission | P1 | after previous milestone | Not started |
| Release v1.0 on GitHub with DOI | P2 | after previous milestone | Not started |
## 2026-07-09 — Глубокий аудит MCARA
- **Обновление:** STATE.md переписан (был устаревший от 2025-03-15)
- **Задача:** Интегрировать Park/Di Stefano (2026) — эпигенетический уровень stem cell exit
- **Задача:** Добавить asymmetric histone inheritance (Ma 2026, PMID: 41872193)
