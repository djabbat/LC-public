# EpigeneticDrift — Project Status File (STATE.md)

**Version:** 0.2.0  
**Status:** 🟡 In Development (Phase 1 – Foundation)  
**Last Updated:** 2025-03-15  

---

## 1. Executive Summary

EpigeneticDrift is a computational research initiative to build an open-source pipeline for detecting, quantifying, and visualizing age-related epigenetic drift from DNA methylation data. The project targets three primary deliverables: (1) a modular Python/R pipeline for methylation pattern analysis, (2) a curated reference database of ≥100 age-associated CpG markers, and (3) an interactive visualization dashboard. The ultimate goal is to publish a peer-reviewed manuscript describing the methodology and validation results. The project is currently in Phase 1 (Foundation), with all P0 tasks actively underway. The team consists of five part-time members (20% effort each) and operates with a $500 cloud computing budget. Key milestones are set for March 31 (Phase 1), May 15 (Phase 2), and July 31 (Phase 3).

---

## 2. Current Status & Progress Indicators

### 2.1 Phase 1: Foundation (In Progress – 60% complete)

| Task ID | Description | Owner | Progress | Status |
|---------|-------------|-------|----------|--------|
| T0.1 | Finalize CONCEPT.md with detailed objectives, deliverables, success criteria, and ethical considerations. | Project Manager | 80% | 🟢 On track – draft reviewed by PI; final revision in progress. |
| T0.2 | Set up project repository structure (Git), including directories for code, data, docs, tests, and CI/CD configuration. | Lead Developer | 100% | ✅ Completed – repository operational at github.com/epigeneticdrift/pipeline. |
| T0.3 | Define coding standards, contribution guidelines, and issue templates. | Lead Developer | 50% | 🟡 In progress – CONTRIBUTING.md drafted; awaiting team feedback. |
| T0.4 | Establish communication channels (Slack, weekly stand-ups, shared calendar). | Project Manager | 100% | ✅ Completed – Slack channel #epigenetic-drift active; weekly stand-ups scheduled (Mondays 10:00 AM). |

**Overall Phase 1 Completion:** 60% (3 of 4 tasks started; 2 fully complete).  
**Sprint (Mar 10–Mar 21):** Focus on finalizing T0.1 and T0.3. Expected completion by Mar 20.

### 2.2 Phase 2: Core Development (Not Started – 0%)

Scheduled to begin immediately after Phase 1 closure (target: Mar 31). Pre-planning has identified key algorithm design decisions (e.g., beta-value difference vs. M-value transformation, smoothing kernel selection). A technical design document will be drafted during Phase 1 final week.

### 2.3 Phase 3: Validation & Publication (Not Started – 0%)

Dependent on Phase 2 completion. Preliminary literature review for marker database has identified 47 candidate CpGs from three published studies (Hannum et al., Horvath et al., Levine et al.). This will accelerate T2.1.

---

## 3. Completed Tasks

| Task ID | Description | Completion Date | Notes |
|---------|-------------|-----------------|-------|
| T0.2 | Repository setup (Git, CI/CD, directory structure) | 2025-03-05 | Includes GitHub Actions workflow for linting and unit tests. |
| T0.4 | Communication channels established | 2025-03-07 | Slack, calendar, and first stand-up held (Mar 10). |
| — | Project kickoff meeting | 2025-03-01 | All team members attended; scope and roles confirmed. |
| — | CONCEPT.md initial draft | 2025-03-10 | Draft reviewed by PI; feedback incorporated. |

---

## 4. Current Sprint / Phase Tasks (Mar 10 – Mar 21)

### P0 – Foundation (Must Complete Before Phase 2)

| Task ID | Description | Owner | Dependencies | Effort Remaining | Success Criteria |
|---------|-------------|-------|--------------|------------------|-----------------|
| T0.1 | Finalize CONCEPT.md | Project Manager | None | 1 day | Document approved by PI and stakeholders; published in repo. |
| T0.3 | Finalize coding standards & contribution guidelines | Lead Developer | T0.2 | 0.5 day | CONTRIBUTING.md merged; issue templates active. |

### P1 – Core Development (Pre-planning)

| Task ID | Description | Owner | Dependencies | Effort Remaining | Success Criteria |
|---------|-------------|-------|--------------|------------------|-----------------|
| T1.1 | Create comprehensive TODO list for Phase 2 | Project Manager | T0.1 | 1 day | TODO.md approved by team; includes algorithm, database, dashboard tasks. |
| T1.2 (prep) | Draft algorithm design document | Lead Developer | T0.1 | 2 days | Design doc outlines comparison methods, statistical tests, and data flow. |

---

## 5. Blockers & Risks

### 5.1 Active Blockers

- **None currently.** All P0 tasks are progressing without external dependencies. Potential blocker: PI availability for CONCEPT.md final review (scheduled for Mar 18).

### 5.2 Risk Register

| Risk | Likelihood | Impact | Mitigation | Owner | Trigger |
|------|------------|--------|------------|-------|---------|
| Algorithm fails to detect known drift signals | Medium | High | Use synthetic data with known ground truth; test on multiple public datasets (GEO, TCGA); consult domain expert (Dr. X). | Lead Developer | First algorithm test (Apr 15) |
| Data format incompatibility | Low | Medium | Build flexible ingestion module supporting IDAT, CSV, BED; document required columns; include format conversion utilities. | Data Scientist | Data ingestion test (Apr 1) |
| Team member availability changes (e.g., illness, conflicting projects) | Medium | Medium | Cross-train tasks; maintain documentation; have backup owners identified for each task. | Project Manager | Any unplanned absence >1 week |
| Publication delay due to journal review | High | Low | Submit to preprint server (bioRxiv) immediately after Phase 3; target open-access journals with fast review (e.g., PLOS ONE, BMC Bioinformatics). | Principal Investigator | Manuscript submission (Jul 31) |
| Dashboard performance with large datasets (>1000 samples) | Low | Medium | Implement data aggregation and caching; test with simulated 2000-sample dataset; use Plotly/Dash with server-side filtering. | Frontend Developer | Dashboard prototype (Jun 1) |
| Cloud credit overrun | Low | Low | Monitor usage weekly; use spot instances for batch processing; prioritize free-tier resources. | Lead Developer | Monthly cost >$100 |

---

## 6. Next Steps (Immediate Priorities)

### P0 (Must Do This Week)

1. **Finalize CONCEPT.md** – Project Manager to incorporate PI feedback and publish final version. Deadline: 2025-03-18.  
2. **Complete CONTRIBUTING.md** – Lead Developer to finalize coding standards and merge. Deadline: 2025-03-19.  
3. **Phase 1 closure review** – Schedule meeting with PI to confirm Phase 1 completion. Target: 2025-03-21.

### P1 (Should Do Next Week)

4. **Draft algorithm design document** – Lead Developer to produce technical specification for core algorithm. Deadline: 2025-03-25.  
5. **Create TODO.md for Phase 2** – Project Manager to break down T1.2–T1.5 into sub-tasks. Deadline: 2025-03-22.  
6. **Begin literature review for marker database** – Data Scientist to compile list of ≥100 candidate CpGs from at least 5 published studies. Deadline: 2025-03-28.

---

## 7. Milestones & Timeline

| Milestone | Target Date | Deliverables | Exit Criteria | Status |
|-----------|-------------|--------------|---------------|--------|
| Phase 1 Complete | 2025-03-31 | CONCEPT.md finalized, repository operational, CI/CD running, communication channels active | All P0 tasks closed; team aligned on scope; design doc initiated | 🟡 On track |
| Phase 2 Complete | 2025-05-15 | Core algorithm implemented, unit tests passing (≥80% coverage), data ingestion module functional, CI pipeline green | All P1 tasks closed; algorithm validated on synthetic data; TODO.md complete | 🔴 Not started |
| Phase 3 Complete | 2025-07-31 | Reference database (≥100 markers), dashboard deployed, manuscript draft submitted to journal | All P2 tasks closed; manuscript submitted; package installable via pip | 🔴 Not started |

**Detailed Timeline (Gantt-style):**

```
Mar 1–15: Phase 1 (T0.1–T0.4)
Mar 16–31: Phase 1 wrap-up + Phase 2 prep (T1.1, design doc)
Apr 1–15: Phase 2 – Algorithm implementation (T1.2)
Apr 16–30: Phase 2 – Data ingestion module (T1.3)
May 1–15: Phase 2 – Unit tests & CI hardening (T1.4, T1.5)
May 16–Jun 7: Phase 3 – Marker database (T2.1)
Jun 8–Jul 5: Phase 3 – Validation on public datasets (T2.2)
Jul 6–Jul 26: Phase 3 – Dashboard development (T2.3)
Jul 27–Aug 10: Phase 3 – Manuscript writing (T2.4)
Aug 11–Aug 25: Phase 3 – Open-source release (T2.5)
```

---

## 8. Key Performance Indicators (KPIs)

| KPI | Target | Measurement Method | Current Value | Frequency |
|-----|--------|--------------------|---------------|-----------|
| Code coverage | ≥80% by end of Phase 2 | pytest-cov report | 0% (no code yet) | Weekly during Phase 2 |
| Pipeline accuracy (synthetic data) | ≥90% | Correlation between predicted and true age | N/A | After T1.2 completion |
| Pipeline accuracy (real data) | ≥80% age correlation | Pearson r between predicted and chronological age | N/A | After T2.2 completion |
| Database size | ≥100 curated markers | Count of markers with metadata | 47 (preliminary) | Monthly |
| Dashboard uptime | ≥99% during review period | Uptime monitoring (e.g., UptimeRobot) | N/A | Daily during Phase 3 |
| Manuscript submission | Within 2 weeks of Phase 3 completion | Date of submission | N/A | Once |
| Team satisfaction | ≥4/5 in monthly survey | Anonymous survey | N/A | Monthly |

---

## 9. Resource Allocation

### 9.1 Personnel

| Role | Name (Alias) | Effort | Responsibilities |
|------|--------------|--------|------------------|
| Project Manager | A. Smith | 20% | Coordination, documentation, stakeholder communication |
| Lead Developer | B. Jones | 20% | Algorithm implementation, CI/CD, code review |
| Data Scientist | C. Lee | 20% | Data ingestion, marker database, validation |
| Frontend Developer | D. Patel | 20% | Dashboard development (Plotly/Dash) |
| Principal Investigator | E. Garcia | 10% | Scientific oversight, manuscript writing |

### 9.2 Computing & Tools

- **Cloud credits:** $500 (AWS/GCP) – used for testing large datasets.
- **Software stack:** Python 3.11, R 4.3, Git, GitHub Actions, Plotly/Dash, LaTeX.
- **Data sources:** GEO (GSE series), TCGA, ENCODE – all publicly available and free.

### 9.3 Budget

| Category | Allocated | Spent to Date | Remaining |
|----------|-----------|---------------|-----------|
| Cloud computing | $500 | $0 | $500 |
| Software licenses | $0 | $0 | $0 (all open-source) |
| Publication fees | $2,000 (pending grant) | $0 | $2,000 |

---

## 10. Communication Plan

- **Weekly stand-up:** Every Monday 10:00 AM (15 min) – status updates, blockers, next steps.
- **Daily updates:** Slack channel #epigenetic-drift – async check-ins.
- **Monthly review:** Last Friday of each month with PI and stakeholders – milestone progress, risk review.
- **Decision log:** All decisions documented in GitHub Wiki (Decisions page).
- **Issue tracking:** GitHub Issues with labels (P0, P1, P2, bug, enhancement).

---

## 11. Appendices

### A. Glossary

- **CpG:** Cytosine-phosphate-Guanine dinucleotide; site of DNA methylation.
- **Beta-value:** Ratio of methylated signal to total signal (0–1).
- **Epigenetic drift:** Age-related changes in DNA methylation patterns.
- **GEO:** Gene Expression Omnibus (public repository).
- **TCGA:** The Cancer Genome Atlas.

### B. References (Preliminary)

- Hannum, G. et al. (2013). Genome-wide methylation profiles reveal quantitative views of human aging rates. *Molecular Cell*, 49(2), 359–367.
- Horvath, S. (2013). DNA methylation age of human tissues and cell types. *Genome Biology*, 14(10), R115.
- Levine, M.E. et al. (2018). An epigenetic biomarker of aging for lifespan and healthspan. *Aging*, 10(4), 573–591.

---

*This STATE.md is a living document. Update after each milestone review or when significant changes occur. Next scheduled update: 2025-03-31 (Phase 1 completion).*