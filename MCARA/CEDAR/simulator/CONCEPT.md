# CONCEPT.md — CEDAR-v2 Simulator

**Version:** 1.0

## Title
**CEDAR-v2: Stochastic, ABC-SMC-calibrated, GSA-validated model of centriole-mediated stem cell depletion with dual Aurora A-mediated p53 inactivation**

## Author
Jaba Tqemaladze, MD — Georgia Longevity Alliance

## Problem
Stem cell pool depletion is a fundamental mechanism of aging. The centriolar apparatus plays a key role in cell cycle regulation and damage response, but a quantitative model linking centriole dynamics to stem cell depletion was lacking.

## Solution
CEDAR-v2 is a stochastic simulation model that simulates:
- Centriolar apparatus dynamics (mother/daughter centrioles)
- Two Aurora A → p53 inactivation pathways (Ser315 degradation + Ser215 transcriptional silencing)
- ATM → p53(Ser15) → centrosome (spindle checkpoint → senescence)
- CCP5/AGBL5 deglutamylase activity → damage accumulation
- CEP295 → Polo/PLK1 mother centriole maturation

## Methodology
1. **ABC-SMC calibration**: 14 parameters against 2400 observations (3 cell types × 4 endpoints)
2. **Sobol GSA**: global sensitivity analysis, identification of dominant parameters
3. **Stochastic modeling**: Monte Carlo method, configurable number of runs

## Key Findings
- Aurora A → p53(Ser215) is the dominant pathway (confirmed by GSA)
- CCP5/AGBL5 is a critical parameter determining the rate of damage accumulation
- The model reproduces the Hayflick limit for three cell types

## Status
- Published as preprint (2026)
- Code: open source (GPL v3), Python 3.10+
- Installation: `pip install git+https://github.com/djabbat/CEDAR-sim.git`
- DOI: pending

## Links to Other Projects
- **LC/MCARA/CEDAR** — theoretical foundation
- **PhD** — dissertation work
- **Marketing/Aubrey** — application in grant proposals

## Consumables (annual)

| **Office consumables** (printing, stationery, toner) | **$300** |

## Hypothesis

*To be specified — see CONCEPT.md §1 for project rationale.*

## References

*See project MEMORY.md for reference history.*


## Budget

| Item | Amount |
|------|--------|
| Computational resources | $500/yr |
