**Last update: 2026-07-24

# STATE — HAP Project

**Date: 2026-07-24
**Status:** 🟢 Active

## ⚠️ BSPC — formal rejection (July 1, 2026)

- **July 1, 2026** — received formal rejection from BSPC (BSPC-D-26-11119)
- **Editor:** Mathias Baumert (Executive Editor)
- **Reason:** prescreen — «threshold for acceptance is high»
- **This is the 9th rejection** (JAD, PNEC, BioSystems, CSF, MedHyp, BMB, BBS, BSPC + CSF dual)
- **Current active submission:** Mathematical Biosciences (MBS-D-26-00817, June 27)

> **📄 Articles and publications:** see `~/Desktop/Services/publications/PUBLICATIONS_TRACKER.md`

## Technical status

| Component | Status |
|-----------|:------:|
| ODE simulation (6 variables) | ✅ Done |
| Morris + Sobol sensitivity | ✅ θ_L dominates (ST=0.75) |
| Stochastic robustness | ✅ CV<1%, robustness=0.992 |
| Phase portraits | ✅ Generated |
| Bifurcation analysis | ✅ Saddle-node at L_basal≈0 |
| Plots | ✅ 10 plots |
| Evidence base | ✅ 6 categories confirmed |
| GitHub | ✅ https://github.com/djabbat/hap-dynamics |

## Session 2026-06-15 (outcome): Full revision cycle completed

### Done
- [x] Morris + Sobol sensitivity: θ_L dominates (ST=0.75)
- [x] Stochastic: white noise CV=0.78%, colored (OU) CV=1.59%
- [x] 2D parameter scan (L_basal × θ_L, 900 runs)
- [x] Phase portraits (L-A, S-A)
- [x] Manuscript v3: 471 lines, 26 references (all verified)
- [x] NHAM → HAP, Afaf removed, Longevity Horizon hidden
- [x] El Fettahi disclosure + email correspondence
- [x] 3 rounds of peer review (IF 18+ level)
- [x] Review #3: verdict ACCEPT (with recommendations)
- [x] New references: PMID 40362260, 39566821, 41465592, 41459016 + Phytomedicine 2024

## Session 2026-05-30: Project creation + simulation prototype + evidence search

### What's done
- [x] Project created with 8 core files
- [x] ODE simulation prototype written (Python, 6 state variables)
- [x] Simulation reproduces HAP predictions (ablation before/after τ_crit)
- [x] Bifurcation analysis — saddle-node at L_basal≈0, k_A_L≈0
- [x] 10 plots generated (trajectory, ablation, bifurcation)
- [x] **PubMed search with 15 queries** — all categories confirm HAP
- [x] **Evidence report** — docs/evidence_hap_confirmation.md (6 sections, 20+ new PMIDs)
- [x] Afaf left the project (Biomarker Review). Decision: freeze, do solo when returning.

### Key evidence search findings

| Category | Status | New articles |
|-----------|:------:|:------------:|
| NAFLD ↔ depression | ✅ Confirmed | 6 (2022-2026) |
| Bile acid → FXR/TGR5 → mood | ✅ Confirmed | 4 (2024-2026) |
| Drosophila: fat body + ecdysone → affect | ✅ Confirmed | 6 (2017-2026) |
| C. elegans: no liver = no affect | ✅ Confirmed | 3 (2025-2026) |
| Liver Tx → mood | ✅ Confirmed | 4 (2015-2026) |
| Critical window: steroids → affect | ✅ Confirmed | 3 (2016-2022) |