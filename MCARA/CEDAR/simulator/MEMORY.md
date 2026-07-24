# MEMORY.md — CEDAR-v2 Simulator

## Decision History

### 2026-06-27 — Audit and creation of core files
- **Decision:** A deep audit of all projects was conducted. CEDAR_simulator was found without core files.
- **Action:** All 10 core files were created (_pi.md, CONCEPT.md, TODO.md, PARAMETERS.md, MAP.md, STATE.md, MEMORY.md, DESIGN.md, THEORY.md, EVIDENCE.md).
- **Context:** README.md and pyproject.toml already existed.

### Choice of Python (instead of Rust)
- **Decision:** The simulator is written in Python 3.10+, despite the general trend of Rust in the ecosystem.
- **Reason:** NumPy/SciPy ecosystem for scientific computing, development speed, accessibility for the scientific community.
- **Consequences:** Porting to Rust is in the backlog (low priority).

### Private repository
- **Decision:** GitHub repo is private.
- **Reason:** The article has not been published in a peer-reviewed journal.
- **Plan:** Open after publication of the article.

### GPL v3 license
- **Decision:** GPL v3 instead of Apache 2.0 (as in LC).
- **Reason:** Scientific code — requirement of reproducibility.
