# AICoordinator — Claude-Code Overnight Agent for Adaptive Experimentation

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

A 72-hour lineage tracking experiment must make **thousands of small decisions** (which daughter to keep, when to re-focus, when to abort a branch, when to increase laser power, when to switch to mitotic burst mode). Delegating these to a human is impossible; delegating them to pure rule-based code is brittle. The AICoordinator leverages Claude Code's `/overnight` mode + a domain-specific `PROMPT.md` orchestration document to act as a domain-aware autonomous agent, issuing commands to `MicroscopeController` in real time.

## §2 Scientific basis / mechanism

The AICoordinator is not an ML model per se — it is an **LLM-as-orchestrator** that consumes:

- Segmentation output (CellPose masks + centriole positions)
- Lineage state (partial GenealogyReconstruction graph)
- Experiment policy (PROMPT.md: keep tree balanced, prune >5 siblings, prioritize red-centriole daughters, etc.)
- Live event log (errors, timings)

…and emits structured commands (`{"action": "ablate", "target_id": 42, "dose_mW": 10}`) that the controller executes. It can call DeepSeek API for heavy reasoning and Claude's `/overnight` protocol for session persistence and retry logic. Self-correcting: uses post-ablation imaging to verify effect and retries if needed.

## §3 Current state of the art

- Claude Code `/overnight` protocol (Anthropic, internal) + SESSION_STATE.md pattern
- LLM-based lab automation: Coscientist (Boiko et al. 2023 *Nature*) [PMID: 38123806]
- SmartACM / autonomous microscopy — emerging field [REF-PENDING]

## §4 Integration with other CytogeneticTree technologies

- **CellPose_Segmentation** — input stream of masks + spots
- **GenealogyReconstruction** — input: current lineage graph state
- **MicroscopeController** — receives structured commands
- **LaserAblation_405** — dispatch target
- **StatisticalAnalysis** — end-of-run consumer of decision log
- **RITE_Centriole** — decisions keyed to red/green centriole age

## §5 Known gaps + what this subproject builds

**Gaps:**
1. No standard protocol for LLM-driven lab automation at this scale
2. Safety + reversibility requires careful tool-call design (dry-run, confirmation gates)
3. Latency (LLM round-trip ~ seconds) limits decision frequency to ~ 1 per minute

**Deliverables (Phase A):**
- `PROMPT.md` orchestration spec (policies, invariants, safety rules)
- Claude Code skill that reads zarr store + emits JSON commands
- Dry-run harness (policies tested on synthetic data from GenealogyReconstruction simulator)
- Live 72 h co-driven run with human-in-the-loop oversight
- Open-source prompt + agent scaffolding on GitHub

## Falsifiability

The AICoordinator's decision-making will be evaluated against the following falsifiable hypotheses:

1. **Decision latency hypothesis**: The LLM-based orchestrator will produce a valid action command within ≤30 seconds for ≥90% of decision points (N≥100 decisions).
2. **Ablation accuracy hypothesis**: The proportion of successful ablations (target cell eliminated within 2 attempts) will be ≥0.85 (p<0.001 vs. random targeting baseline of 0.05).
3. **Tree balance hypothesis**: The lineage tree depth will not exceed 12 generations in ≥80% of branches over a 72-hour run (N≥3 independent runs).

All thresholds are pre-specified; failure to meet any threshold constitutes falsification of the corresponding claim.

## Pre-registration plan

- **OSF ID**: `osf.io/TBD` (placeholder; to be registered prior to data collection)
- **Planned registration date**: `2025-09-01`
- **Scope**: Primary endpoint (ablation success rate), secondary endpoints (decision latency, tree balance), and analysis plan will be pre-registered. If pre-registration is not feasible due to exploratory nature, a registered report format will be considered.

## Sample size calculation

For the primary endpoint (ablation success rate), a power analysis is performed:

- **Expected effect**: improvement from baseline 0.05 (random) to 0.85 (LLM-guided)
- **Significance level**: α = 0.001 (Bonferroni-corrected for multiple comparisons)
- **Power**: 1−β = 0.80
- **Formula**: n = (Z_α/2 + Z_β)² · (p₁(1−p₁) + p₂(1−p₂)) / (p₁−p₂)²
  where Z_α/2 = 3.29 (for α=0.001), Z_β = 0.84 (for 80% power), p₁=0.05, p₂=0.85
- **Result**: n ≈ 12 per group (rounded up to 15 to account for attrition)
- **Total decisions required**: N≥30 (15 LLM-guided + 15 random baseline)

If sample size calculation is not applicable (e.g., due to single-run constraints), a justification for N/A will be provided.

## Risk matrix

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| LLM API failure (timeout/error) | Medium (0.3) | High — experiment stalls | Fallback to rule-based controller; retry with exponential backoff; human alert after 3 failures |
| Incorrect ablation decision (wrong cell) | Low (0.1) | High — sample damage | Dry-run validation before each ablation; confirmation gate requiring ≥0.9 confidence; post-ablation verification imaging |
| Communication loss with microscope | Low (0.05) | Critical — experiment abort | Heartbeat check every 10s; auto-save state; resume from last checkpoint on reconnect |
| Time budget exceeded (72h run) | Medium (0.4) | Medium — incomplete data | Dynamic priority scheduling; abort low-value branches early; extend to 96h if needed (with pre-approval) |
| Sample degradation (phototoxicity) | Medium (0.3) | High — data quality loss | Laser power capped at 10 mW; adaptive dose reduction based on cell health markers; abort if viability <70% |
| LLM hallucination (invalid command) | Low (0.15) | Medium — wasted time | Schema validation of all outputs; replay buffer for rollback; human-in-the-loop for novel actions |

## Consortium / partners

- **Partner A** (placeholder): Development of PROMPT.md orchestration spec and safety rules
- **Partner B** (placeholder): Testing on real microscope hardware (MicroscopeController integration)
- **Partner C** (placeholder): Statistical analysis and validation of decision logs
- **Partner D** (placeholder): Independent replication of key results on different microscope platform

Formal collaboration agreements and data-sharing MOUs to be established prior to data collection.

## Evidence base & meta-analysis

### Key claims and supporting evidence

1. **LLM-based lab automation is feasible**
   - Boiko et al. 2023 *Nature* [PMID: 38123806] — Coscientist: LLM-driven chemical synthesis planning and execution
   - BioAutoMATED (Soenksen et al. 2022 *Nature Communications*) — automated ML pipeline for biology [PMID: 36456553]
   - RoboChem (Coley et al. 2019 *Science*) — autonomous chemical synthesis platform [DOI: 10.1126/science.aax1566]

2. **LLM-as-orchestrator for microscopy**
   - No direct precedent found; this is a novel application. Related work: autonomous microscopy for cell tracking (e.g., [PMID: 32661310] — real-time adaptive microscopy)

3. **Autofocus/drift correction policies**
   - Standard methods exist (e.g., [PMID: 30531955] — deep learning-based autofocus)

### Systematic review / meta-analysis
No systematic review or meta-analysis specifically addressing LLM-driven microscopy automation was identified. A PRISMA-style search is planned prior to final submission.

### Contradictory evidence
- **Latency vs. real-time requirement**: LLM inference latency (1–10s) may be too slow for millisecond-scale decisions (e.g., drift correction). Mitigation: hybrid approach — fast rule-based for low-level control, LLM for high-level decisions.
- **Reliability concerns**: LLMs are known to hallucinate; structured output schemas and validation gates are essential.

### State of the art
The field of LLM-driven lab automation is nascent. Coscientist (2023) demonstrated feasibility for chemical synthesis; this project extends the paradigm to live-cell microscopy, which presents unique challenges (real-time constraints, safety-critical decisions, complex lineage dynamics).

## Methodology depth

### Step-by-step protocol
1. **Initialization**: Load experiment policy from PROMPT.md; connect to MicroscopeController; verify segmentation pipeline (CellPose) is running.
2. **Decision loop** (every 60s):
   a. Read current lineage state from GenealogyReconstruction
   b. Read latest segmentation masks and centriole positions
   c. Query LLM (Claude Code `/overnight`) with state + policy → structured command
   d. Validate command schema (action, target_id, parameters)
   e. Dry-run: simulate command outcome on current state
   f. If dry-run passes: execute command; if fails: request alternative from LLM
   g. Post-execution: verify outcome via imaging; log result
3. **Error handling**: On any failure, retry up to 3 times with exponential backoff; escalate to human if persistent.
4. **Termination**: After 72h or when all branches are complete; save full decision log.

### Statistical Analysis Plan (SAP)
- **Primary endpoint**: Ablation success rate (proportion of target cells successfully ablated within 2 attempts)
- **Secondary endpoints**: Decision latency (seconds), tree balance (max depth), sample viability over time
- **Multiple comparisons**: Bonferroni correction for 3 endpoints (α = 0.05/3 = 0.017)
- **Missing data**: Last observation carried forward for incomplete branches; sensitivity analysis with complete-case only

### Controls
- **Positive control**: Known ablation target (e.g., cell with red centriole) — expected success rate ≥0.9
- **Negative control**: Random targeting (no LLM guidance) — expected success rate ~0.05
- **Replication strategy**: 3 independent runs on different days; split-half analysis for internal consistency

### Blinding / randomisation
Not applicable (single-arm, open-label design). Justification: blinding is not feasible due to the nature of the intervention (LLM vs. human decision-making is inherently observable).

## Reproducibility & open science

### Code repository

- All agent scaffolding, PROMPT.md, and evaluation harness will be released on GitHub under MIT license upon manuscript acceptance (or earlier as preprint).
- Repository URL: TBD (placeholder).

### Data deposit plan

- Raw decision logs, segmentation outputs, and lineage graphs will be deposited on Zenodo (TBD) or OSF (osf.io/TBD).
- Data will include: (1) full decision log with timestamps, (2) synthetic test scenarios, (3) evaluation results.

### Pre-registration

- Pre-registration available at osf.io/TBD (placeholder).
- Planned registration date: 2025-09-01.

### Materials transparency

- Python environment: requirements.txt with pinned versions (to be provided).
- PROMPT.md orchestration spec: version-controlled in repository.
- Protocols: step-by-step protocol available on protocols.io (TBD).
