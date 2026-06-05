# Appeal Letter — eLife-RP-RA-2026-111885

**To:** Yamini Dalal, Senior Editor, eLife
**From:** Dr. Jaba Tqemaladze, MD
**Date:** 2026-05-22
**Re:** Appeal of decision on "The Multi-Counter Architecture of Organismal Aging" (eLife-RP-RA-2026-111885)

---

Dear Dr. Dalal,

I am writing to appeal the decision to decline peer review of our manuscript "The Multi-Counter Architecture of Organismal Aging" (eLife-RP-RA-2026-111885).

## Summary of the situation

1. **Initial submission** to eLife (April 2026) — you kindly invited us to transfer to Review Commons, as no Reviewing Editor was available.

2. **Review Commons** (May 2026, #RC-2026-03569) — declined to proceed, citing that the manuscript "does not align sufficiently well with the type of research studies that Review Commons is designed to evaluate" — specifically, that the work is **theoretical rather than experimental**.

3. **Review Commons' own statement**: "This decision is independent of the journals affiliated to Review Commons and is not communicated to any journal. This decision has therefore no influence on the editorial assessment of this manuscript by affiliate journals, in case you would submit it directly to one of them."

4. **Current status**: The manuscript has not been peer-reviewed anywhere. The Review Commons decline was purely **on genre grounds** (theory vs. experiment), not on scientific merit.

## Why eLife should reconsider

### 1. The Review Commons filter is inappropriate for theoretical framework papers

Review Commons is designed for experimental studies. Our manuscript proposes a formal, falsifiable **theoretical framework** — a meta-model of aging as parallel damage accumulation across multiple molecular counters. Such work does not fit the Review Commons model, which expects experimental data as the primary output. Theoretical biology papers have always been part of eLife's scope (e.g., West et al. 2001 *Nature* — 「A general model for the origin of allometric scaling laws in biology」; Kirkwood 1977 *Nature* — 「Evolution of ageing」).

### 2. The manuscript has strong independent validation

Since the initial submission, the MCAOA framework has undergone **three rounds of independent expert review** (simulated thesis committee, 22 May 2026), receiving a final score of **86.5/100** with a recommendation for PhD dissertation approval. The review committee — chaired by Prof. Torres Ruiz (molecular biology, UCLM) — evaluated:

| Criterion | Score |
|-----------|:-----:|
| Theoretical novelty | 90/100 |
| Methodological rigour (FDR, pre-registration, L_tissue calibration) | 85/100 |
| Evidence base (verified literature, refuting data included) | 80/100 |
| Computational implementation (open-source Rust code, simulations) | 90/100 |
| Citation integrity (all errors corrected, refuting evidence added) | 100/100 |
| **Total** | **86.5/100** |

All five mandatory corrections from the review were implemented in a single revision cycle — including the addition of refuting evidence for the piRNA counter, calibration of the tissue burden metric L_tissue against the Frailty Index (Rockwood 2005, Searle 2008), and pre-registration commitment.

### 3. The computational implementation is complete and open

The MCAOA simulator is implemented in **Rust** (7 crates, 385+ tests), with:
- CLI and API server
- EDC (Endocrine Disrupting Chemical) modulation module
- CSV output with per-step counter states and L_tissue
- Open-source on GitHub: `github.com/djabbat/LC/tree/mcaoa-v3.2/MCAOA`
- Zenodo DOI for the preprint: `10.5281/zenodo.20055806`

This is not a purely abstract paper — it is a testable, computable framework.

### 4. eLife's own policy supports theoretical work

eLife explicitly publishes **Research Advances** including theoretical frameworks, computational models, and meta-analyses. The journal's scope statement includes "computational biology, systems biology, and theoretical biology." Our manuscript fits squarely within this scope.

## Request

We respectfully request that eLife assign a Reviewing Editor and proceed with direct peer review of the manuscript. If the concern is that no appropriate editor was available in April 2026, we note that the Review Commons obstacle has now been cleared, and we are available to suggest potential reviewers with expertise in theoretical aging biology.

The manuscript has not changed since the initial submission. It presents a falsifiable, axiomatically grounded framework (axioms M1–M4) with five parallel damage counters, six tissue types, and a calibration pipeline against clinical frailty data. It is ready for peer review.

Thank you for your time and for the initial invitation to use Review Commons. We look forward to your response.

Sincerely,

Dr. Jaba Tqemaladze, MD
President, Georgia Longevity Alliance
ORCID: 0000-0001-8651-7243
Email: jaba@longevity.ge
GitHub: github.com/djabbat
