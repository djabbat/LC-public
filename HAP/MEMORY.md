# MEMORY.md — HAP Project

**Purpose:** What needs to be remembered between sessions. New on top.

## 2026-08-11 — OSF Projects will be phased out (no action needed for HAP)

**Source:** COS announcement email 2026-08-11 (osf.io).
- **2026-11-16:** no new OSF projects/components can be created.
- **2027-02-19:** all OSF projects (public and private) become **read-only**.
- **Stays:** OSF Registries (pre-registrations) keep working — content remains persistent.

**HAP-related OSF objects (both verified live, HTTP 200):**
- `osf.io/dqy38` — HAP Biomarker_Review registration (Open-Ended, 2026-06-15) → **registration, stays, no action**.
- `osf.io/mgzt5` — HAP Biomarker_Review project (public) → will become read-only 2027-02-19. **Confirmed: no project files stored there** (Jaba checked 2026-08-11) → nothing to export.

**Decision:** No export needed, no doc link updates needed (registrations persist). If a new OSF project is ever needed — create it before 2026-11-16.
## 2026-07-01 — BSPC formal rejection (9th rejection)

- **July 1, 2026** — formal rejection of BSPC (BSPC-D-26-11119), Executive Editor Mathias Baumert
- **Reason:** prescreen — high acceptance threshold, does not fit the scope
- **Submitted:** June 19 through Elsevier Article Transfer from BioSystems
- **Total 9 rejections:** JAD → PNEC → BioSystems → CSF → MedHyp → BMB → BBS → BSPC → (CSF dual)
- **Current:** Mathematical Biosciences (MBS-D-26-00817, June 27) — With Editor
- **Next options in case of rejection:** Discover Aging (Springer), J. Biological Dynamics (T&F)
## 2026-06-19 — Submitted to Biomedical Signal Processing and Control (7th attempt)

- **19 Jun 2026, 21:40** — BSPC-D-26-11119 confirmed, Submitted to Journal
- **Journal:** Biomedical Signal Processing and Control (Elsevier)
- **Path:** Transferred from Elsevier's BioSystems (BIOSYS-D-26-00840 desk reject on 16 Jun) → BSPC
- **Cover letter:** Adapted for biomedical signal processing
- **Files:** HAP_Dynamics.docx + highlights + title_page + declaration + 20 figures
- **This is the 7th attempt after 6 rejections (all desk rejects due to scope, not quality)**
## 2026-06-18 — Medical Hypotheses: Handling Editor → Instant Reject

- **18 Jun 2026, 14:57** — YMEHY-D-26-00985: Editor-in-Chief Dr Sachin Sarode appointed handling editor
- **18 Jun 2026, 15:00** — Rejected (in 3 minutes!), desk reject without reviews
- **Reason:** Standard wording ("reviewers recommend against publishing"), but reviewers were not assigned → desk reject
- **Irony:** Review was requested on 17.06, but not processed — manuscript still received a rejection
- **5th rejection in a row** (JAD, PNEC, BioSystems, CSF, Medical Hypotheses) — all due to scope/format
- **Next:** Journal of Biological Dynamics (T&F, subscription, IF ~2.8, not Elsevier)
- **Cover letter:** `~/Desktop/cover_letter_JBD.md` is already ready
## 2026-06-17 — CSF Rejected (dual submission) + Medical Hypotheses withdrawal

- **17 Jun 2026** — CSF (CHAOS-D-26-05728) rejected: Elsevier discovered dual submission
- **Reason:** Medical Hypotheses (YMEHY-D-26-00985) did not close the manuscript during transfer to Psychoneuroendocrinology
- **Medical Hypotheses** withdrawal sent (17.06) via Editorial Manager
- **Apology letter** sent to Marcel Clerc (CSF Editor-in-Chief)
- **Lesson:** When transferring within Elsevier, always check that the manuscript is closed in the original journal
- **Decision:** Submission to Journal of Biological Dynamics (T&F, not Elsevier, subscription, IF ~2.8)
- **Cover letter:** `~/Desktop/cover_letter_JBD.md`
- **BBS:** «Sensation, Feeling, Abstraction» (BBS-D-26-00814) — also rejected 17.06 (scope)
## 2026-06-16 — Third desk reject: Psychoneuroendocrinology

- **16 Jun 2026** — rejection from Psychoneuroendocrinology (PNEC-D-26-00481)
- **Editor:** Elizabeth (Birdie) Shirtcliff (Editor-in-Chief)
- **Reason:** «falls outside of the scope of this journal»
- **This is the 3rd desk reject in a row** (J. Affective Disorders → Medical Hypotheses → Psychoneuroendocrinology)
- **All three — due to scope, not quality** (the article passed 3 rounds of internal revision, verdict ACCEPT)
- **Problem:** HAP — interdisciplinary work (evolutionary biology + mathematical modeling + affective neuroscience), it's hard to fit it into a narrow journal scope
- **Solution:** look for journals that specifically accept theoretical/mathematical works (J. Theoretical Biology, Biosystems) or multidisciplinary ones (PLOS ONE, PeerJ)
## 2026-06-15 — Afaf withdrew from Biomarker Review

- **15 Jun 2026** — Afaf El Fettahi stepped down from Biomarker Review (Dynamic Biomarkers Systematic Map)
- **Reason:** the protocol (v2.4) became too extensive for a duo: 3,499 hits → ~350-500 eligible → 5 biomarker domains → many databases → quality assessment → synthesis
- **Recommendation:** find a systematic review methodologist or laboratory
- **Letter:** `Biomarker_Review/email_from_Afaf_2026-06-15.md`
- **Consequences:**
  - Remove Afaf's name from the protocol, OSF registration, all documents
  - Need a new collaborator OR narrow down the protocol to 1-2 biomarker domains
  - Embase access now needs to be sought independently
## 2026-06-15 — J. Affective Disorders Rejected → Medical Hypotheses

- **15 Jun 2026, 7:31 AM** — rejection from J. Affective Disorders (JAFD-D-26-06247)
- **Editor:** Benjamin Goldstein (Deputy Editor)
- **Reason:** "Lack of sufficient novelty" (desk reject, without reviews)
- **Rejection reason:** Mismatch of topic — J. Affective Disorders is a clinical journal, not for ODE models
- **Decision:** Submit to **Medical Hypotheses** (Elsevier, IF ~4.7) — a journal specifically for bold theories
- **Cover letter prepared:** `~/Desktop/LC/HAP/docs/cover_letter_Medical_Hypotheses.md`
## 2026-05-30 — Creation of HAP project

- HAP separated from PhD into an independent project (~/Desktop/HAP/)
- Simulation (src/) and documentation (docs/) have been transferred
- All core files have been created
- Simulation prototype is working and reproduces HAP Predictions
## 2026-05-30 — Afaf's letter sent

- Afaf suggested: nonlinear dynamics, allostasis, feedback loops (not quantum)
- Plan: simulation → data (not the other way around)
- Response letter sent: agreement on direction, proposal to start with simulation
- Response expected
## Permanent Rules

- **HAP Strong Version** has already been published (DOI: 10.65649/d76f6c48) — this is the foundation
- **HAP is more fundamental than NHAM** — HAP gives a necessary condition, NHAM — a mechanism
- **Simulation before data** — do not look for data until the model is ready
- **Afaf** — the main collaborator on the second article