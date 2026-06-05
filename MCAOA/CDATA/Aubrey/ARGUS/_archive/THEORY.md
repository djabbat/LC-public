<!-- AUTO-GENERATED from CONCEPT.md by TBPR orchestrator 2026-05-10 ensure_core (DeepSeek-reasoner). Review and edit as needed. -->

# THEORY.md — Experiment 0: Commissioning Framework

**Версия:** 2.0  
**Статус:** Post TBPR cycle-7 — v4.0 CONCEPT

## 1. Formal Framework

### 1.1 Scope
Experiment 0 (E0) is a **commissioning theory** — not a biological hypothesis. It defines the formal requirements for validating an AI‑agent‑controlled precision microscopy system under 24/7 autonomous operation.

### 1.2 Mathematical Preliminaries
Let $S$ be the system state space:  

$$
S = \{ \text{stage\_position}, \text{laser\_state}, \text{camera\_state}, \text{environmental\_sensors}, \text{interlock\_status} \}
$$

Let $A$ be the set of agent actions (tool functions):  

$$
A = \{ \text{move\_stage}(x,y), \text{fire\_laser}(t), \text{capture\_image}(params), \text{detect\_targets}(img), \text{log\_event}(msg) \}
$$

The agent $G$ (Claude Code + DeepSeek router) implements a decision policy $\pi: S \rightarrow A$.

### 1.3 Validation Objective
For a test period $T = 6$ months, we require:

$$
\forall t \in [0, T] : \text{system\_safe}(t) \land \text{agent\_responsive}(t) \land \text{data\_integrity}(t)
$$

where  

- **system\_safe**($t$) = all interlock conditions satisfied at time $t$  
- **agent\_responsive**($t$) = agent completes action within $\tau_{\max}$  
- **data\_integrity**($t$) = captured images stored without corruption

## 2. Core Axioms

### Axiom 1: Layered Autonomy
The system comprises three independent layers:  

- **L0 (Realtime):** Arduino Nano firmware — guarantees deterministic response (< 1 ms) for safety‑critical paths.  
- **L1 (Controller):** Python tool‑function API — mediates between agent and hardware.  
- **L2 (Decision):** AI agent — executes high‑level plans and adapts to observations.

### Axiom 2: Fail‑Safe Default
In absence of agent command within $\tau_{\text{watchdog}}$, every subsystem reverts to a passive safe state: stage stops, laser turns off, camera idle.

### Axiom 3: Measurement Fidelity
All sensor readings (temperature, humidity, vibration, laser power) shall be logged with uncertainty $\pm \epsilon$ and timestamped to the system clock with drift $< 1$ s/day.

## 3. Derived Properties

### 3.1 Stability Condition
The rig must maintain thermal equilibrium such that:

$$
\Delta T_{\text{objective}} < 0.1\,^\circ\text{C/min}
$$

to prevent focal drift.

### 3.2 Laser Budget Constraint
Maximum cumulative laser exposure per sample per hour:

$$
E_{\text{max}} = \frac{P_{\text{laser}} \cdot t_{\text{on}}}{\text{area}_{\text{spot}}} \leq 10\, \text{J/cm}^2
$$

(empirical limit for Elodea chloroplast viability under 450 nm CW).

### 3.3 Agent Circle Time
Expected agent decision latency:

$$
\tau_{\text{agent}} = \tau_{\text{LLM}}(G) + \tau_{\text{API}} + \tau_{\text{hardware}}
$$

must be $< 10$ s for real‑time tracking performance.

## 4. Falsifiable Predictions

| Prediction | Test | Falsification |
|------------|------|---------------|
| P1: Agent completes 1000 consecutive autonomous cycles without safety override | Run $N=1000$ cycle test | Any manual interlock trigger |
| P2: Stage positioning repeatability $< 1\,\mu$m over 24 h | Measure 20 positions before/after 24 h drift test | $\sigma > 1\,\mu$m |
| P3: Data pipeline stores $> 10^5$ images without corruption | Write‑and‑read hash verification | Any hash mismatch |
| P4: Laser power stability $\pm 5\%$ over 1 h | 3600 measurements at 1 Hz | $\text{CV} > 5\%$ |
| **P5: Discrimination accuracy $\ge 95\%$ old vs new beads** | $n \ge 500$ cycles, Holm-Bonferroni | $<95\%$ accuracy at $p<0.05$ |
| **P6: CUSUM zero crossings** | Sliding window 50 cycles over 6 months | Any crossing of 93% boundary |
| **P7: Physical Beacon blocks laser** | 10+ tests with beacon hidden | Laser fires without beacon |

## 5. Connection to Existing Theories

- **Real‑Time Control Theory:** The L0 firmware implements a finite state machine (FSM) with bounded response time.  
- **Multi‑Agent Systems:** The DeepSeek router acts as a meta‑layer for task decomposition (not implemented in E0 – single agent).  
- **Uncertainty Quantification:** All sensor logs include measurement uncertainty – future Bayesian calibration.

---

**Note:** No biological theory is proposed. See `PEER_REVIEW_DRAFT.md` for surrogate gap discussion.

---
---

## v4.0 Update (2026-05-15): MCAOA framework & TBPR-driven redesign

### MCAOA framework context

ARGUS operates within the **Multi-Counter Architecture of Organismal Aging (MCAOA)**
framework (Tqemaladze J. 2026, Zenodo DOI: 10.5281/zenodo.20055806):
- Organismal aging = parallel accumulation of damage across multiple molecular counters
- **Centriolar counter (#1)** = key limiter of stem cell replicative potential
- ARGUS = empirical validation tool for counter #1 predictions through autonomous laser ablation

### Key theoretical additions from TBPR cycle-7

1. **Physical Beacon (§11 CONCEPT):** Hardware-level AI hallucination block —
   LED with 10 Hz encoded flash; laser fires ONLY if beacon detected + frequency matches.
   Formal: $\text{laser\_enable} = \text{AI\_decision} \land \text{beacon\_detected}$

2. **CUSUM control chart (§5):** Cumulative Sum monitoring of AI decision accuracy.
   If $P_{\text{correct}} < 93\%$ on sliding window $k=50$, auto-stop.
   Formal: $C_t = \max(0, C_{t-1} + (x_t - \mu_0) - k)$ where $x_t = 1$ if correct.

3. **AI Constitution (§12):** 5 prohibited actions: no laser power change >±10%, no
   disable beacon check, no safety param changes, no code without static analysis,
   no operation under any failure mode from FMEA.

4. **FMEA table (DESIGN.md §5):** 7 failure modes with RPN < 100 threshold.

### Centriole ablation safety (ablation validation literature)

| PMID | Finding | Implication |
|:----:|---------|-------------|
| 15738265 | Centriole removal in HeLa does not block cell cycle | Ablation not lethal in transformed cells |
| 17227892 | Normal human cells assemble centrioles *de novo* after ablation | Justifies BJ-hTERT in Phase B |
| 37882444 | Older mother centriole required for human NPC self-renewal | ARGUS must distinguish old vs new |
| 28562636 | 77% of studies have low N (<10 replicates) | ARGUS n=500 exceeds standard |
