# THEORY — AutomatedMicroscopy


## Formal Theoretical Foundation

### 1. Problem — Human Shift Overhead in Live-Cell Microscopy

Traditional time-lapse microscopy requires continuous human oversight: manual focus adjustment, manual field-of-view selection, visual inspection of cultures, media changes, intervention at anomalies.

In conditions of single-PI labs (as Georgia Longevity Alliance), 24/7 continuous supervision is impossible. Result: **either experiments are limited to working hours (8-12h/day), or expensive automated equipment is purchased ($25-50k+)**.

### 2. Hypothesis

**Thesis:** Low-cost retrofit ($4,500) + AI agent in the role of "night-shift lab technician" allows achieving industrial-grade 24/7 imaging without capital-intensive hardware.

**Formal Axiom M1 (Feasibility):**
> For class CEDAR-type experiments (time-lapse polyGlu intensity measurement on mother centrioles in BJ-hTERT fibroblasts), an AI agent (Claude Code in `/overnight` mode) can perform a supervisory function equal in quality to a trained technician, under the conditions:
> - **Well-defined PROMPT** (natural-language protocol)
> - **Bounded autonomy** (pre-authorized routine actions + require-human-approval for strategic decisions)
> - **Full journaling** (every decision logged with rationale, reproducible after-the-fact)

### 3. Prompt-Driven Supervision Model

Formalization of experimenter-AI interaction:


PROMPT: natural-language description of experiment goals
 ↓ parsed by Claude Code
CRITERIA: concrete thresholds, metrics, conditions
 ↓ continuous monitoring every 30 min
OBSERVATIONS: image data, environmental sensors
 ↓ comparison to CRITERIA
SIGNAL: INFO / WARN / CRIT → human
 OR continue_schedule autonomously


Bayesian decision-theoretic framing:


P(action | observation, prompt) ∝ P(observation | action, prompt) · P(action | prompt)


where:
- **prior P(action|prompt)** = "what would a trained technician do here"
- **likelihood P(observation|action, prompt)** = expected outcome given protocol compliance
- Decision: select action maximizing expected reward (experiment success ∩ biosafety ∩ human trust)

### 4. Axioms of Subproject

**M1 (Feasibility):** AI-operated microscopy achieves ≥80% of trained-technician supervision quality for routine protocols, at <20% cost.

**M2 (Interpretability):** Every AI decision must link to an explicit PROMPT.md line + measurable observations. No "black-box" autonomous actions without traceable rationale.

**M3 (Bounded autonomy):** AI acts only within `auto_allow` policy list; `require_human_approval` gates preserve human strategic control; `forbidden` gates preserve biosafety.

**M4 (Reproducibility):** Complete journals (decisions + rationale + observations) enable post-hoc audit of any experimental run by human reviewer.

### 5. Scope

**In scope:**
- Live-cell fluorescence imaging (BF + FITC + TRITC + DAPI channels)
- Z-stack acquisition (up to 20 μm range, 2 μm steps)
- Environmental chamber monitoring (37°C + 5% CO₂ + humidity)
- Autonomous autofocus, channel switching, stage positioning
- Image analysis pipeline (CellPose segmentation, ImageJ measurements)
- Signal generation to human experimenter per PROMPT.md

**Out of scope (for Phase A):**
- Physical cell manipulation (no liquid handling robot in Phase A)
- Chamber opening for media change (manual, human task)
- Novel imaging modalities (only standard epifluorescence)
- Cross-lab federated coordination (that's FCLC scope)
- Therapeutic intervention decisions (outside AI policy)

### 6. Interfaces with Other Subprojects LC

| Subproject | Interface |
|------------|-----------|
| **CEDAR** | Primary user — Phase A experiments run on this platform |
| **FCLC** | Future: anonymized imaging data contribution to federated learning pool |
| **MCARA** | Future: multi-counter experiments (Telomere, MitoROS) reuse same infrastructure |
| **BioSense** | Potential: shared signal-processing pipelines (cross-domain aging markers) |

### 7. Predictions

1. **Data yield:** 6 months `/overnight` operation → ~900 GB imaging data, ~40 decisions/night journaled = 7,200 logged decisions total
2. **Efficiency:** experiments complete 2-3× faster than with 9-5 human oversight (continuous vs 40-hour weeks)
3. **Cost per experiment:** ~$5k equipment amortization + ~$20 AI subscription per 6-month run = ~$5,020 per experimental cycle
4. **Reliability:** 95%+ uptime target (UPS + redundant sensors + fail-safe policies)

### 8. Falsification Conditions

Platform is **falsified / not-suitable** if:
- Claude Code decisions deviate from trained-technician judgment >20% of cases (measured post-hoc blind review by independent scientist)
- Hardware uptime <80% over first 60 days
- Contamination rate >10% per experimental run (vs typical 1-3% in standard microscopy)
- User (Jaba) abandons autonomous mode after 1 month (too stressful, too much supervision needed)

### 9. Connection to MCARA Framework

AutomatedMicroscopy — **instrumental layer** not theoretical counter. But the fact of its existence enables MCARA framework operationally: without 24/7 imaging infrastructure, it is impossible to collect data for temporal dynamics D_i(n, t) of different counters.

Without AutomatedMicroscopy → MCARA remains a theoretical abstraction.
With AutomatedMicroscopy → MCARA gets an empirical substrate.