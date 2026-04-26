# AutomatedMicroscopy — CONCEPT

*(This document is the authoritative CONCEPT.md for AutomatedMicroscopy subproject. Synthesized 2026-04-21 from AUTOMATED_MICROSCOPY_SETUP.md engineering specification.)*

## Parent framework

AutomatedMicroscopy is a **LongevityCommon infrastructure subproject** providing experimental automation for MCOA Counter validation experiments (CDATA Phase A being the first user).

**Positioning in MCOA framework:**
- Not a damage counter itself (no D_i equation)
- Operational infrastructure enabling collection of time-series data D_i(n, t) для all biological counters
- Serves all subprojects requiring live-cell time-lapse imaging: CDATA, Telomere, MitoROS, EpigeneticDrift, Proteostasis

## Core concept

Retrofit of existing Zeiss IM 35 inverted microscope ($4,500 BOM, open-source DIY) with:
1. Motorized XY+Z stage (Arduino-based steppers)
2. FLIR Blackfly S USB3 scientific camera
3. LED fluorescence illuminator (ThorLabs M470L4 + M565L3)
4. DIY environmental chamber (37°C + 5% CO₂ + 90% humidity)
5. UPS + NAS backup + WireGuard VPN remote monitoring

**Unique innovation:** Claude Code agent operating в `/overnight` mode serves as AI night-shift technician, interpreting natural-language PROMPT.md per experiment and making routine decisions (focus adjustment, ROI selection, channel switching) autonomously while signaling human only для strategic decisions.

## Axioms

**M1 (Feasibility):** AI-operated microscopy achieves ≥80% trained-technician supervision quality at <20% capital cost для routine CDATA-class protocols.

**M2 (Interpretability):** Every AI decision links to explicit PROMPT.md line + measurable observations. Full audit trail.

**M3 (Bounded autonomy):** AI acts only within `auto_allow` policy; `require_human_approval` for strategic; `forbidden` for biosafety.

**M4 (Reproducibility):** Complete journals (decisions + rationale + observations) enable post-hoc blind audit.

## Hypothesis

Low-cost retrofit ($4,500) + prompt-driven AI supervision replicates industrial-grade automated microscopy ($25-50k) for class of protocols where:
- Sample stability ≥ 3 weeks
- Imaging frequency ≤ 2/hour
- Environmental stability required ±0.5°C, ±0.5% CO₂
- No physical sample manipulation on-platform (media changes human-performed)

## Primary use case

CDATA Phase A fibroblast experiment (Impetus Grant 2026-04-25):
- BJ-hTERT fibroblasts, 20% vs 3% O₂
- Aim A.1: α vs β discrimination (division vs time accumulation)
- Aim A.2: Parrinello paradox test
- Aim A.3: CCP1/TTLL6-OE causality
- 6 months, 24/7 imaging, >900 GB data, >7,200 AI decisions journaled

## Predictions / success metrics

1. Platform uptime ≥95% over 180 days
2. Claude decisions concordant with trained-technician judgment ≥80% (blind review by 3 external scientists post-hoc)
3. Contamination rate ≤3% per experimental run
4. Cost per 6-month run: $5,020 total ($5,000 amortized equipment + $20 Claude subscription)
5. Bill of materials + policy file + tool function code released open-source concurrent с Phase A preprint

## Falsification conditions

Platform not-suitable если:
- AI decisions deviate >20% from trained-technician judgment
- Hardware uptime <80%
- Contamination rate >10%
- User abandons autonomous mode within 1 month

## Budget

$4,500 (Вариант A DIY) allocated within CDATA Phase A Impetus grant line "Microscope Automation & Upgrade" (reallocation from Consumables).

## Scope exclusions

- Physical cell manipulation (no liquid handling robot in Phase A)
- Chamber opening for media change (human task)
- Novel imaging modalities (only standard epifluorescence)
- Cross-lab federated coordination (that's FCLC)
- Therapeutic intervention decisions (outside AI policy)

## Cross-ecosystem references

- **Uses FCLC**: anonymized imaging data будут contributed to federated learning pool post-Phase A
- **Enables MCOA**: provides empirical substrate for temporal D_i(n, t) dynamics
- **Enables CDATA**: Phase A experiments impossible without 24/7 imaging
- **Enables future counter subprojects** (Telomere, MitoROS, EpigeneticDrift, Proteostasis): same platform reused

## Related documents

- `AUTOMATED_MICROSCOPY_SETUP.md` — detailed engineering specification (BOM, wiring, software stack)
- `THEORY.md` — formal axioms and predictions
- `EVIDENCE.md` — verified refs + internal data
- `OPEN_PROBLEMS.md` — honest list of risks and unresolved questions
- `DESIGN.md` — code architecture + file tree

---

*CONCEPT v1.0, 2026-04-21. Part of LongevityCommon ecosystem per `~/Desktop/CommonHealth/CONCEPT.md`.*
