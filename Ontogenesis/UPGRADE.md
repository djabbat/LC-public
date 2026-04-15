# UPGRADE.md — Ontogenesis v4.1

Suggestions for project development from external analysis, literature, and cross-project review.

**Format:**
```
## [YYYY-MM-DD] Title
**Source:** [what triggered this]
**Status:** [ ] proposed | [✓ approved YYYY-MM-DD] | [✓✓ implemented YYYY-MM-DD]
```

---

## [2026-03-29] AI-assisted personalized trajectory prediction
**Source:** Cross-project analysis with AIM — personalized medicine patterns
**Status:** [ ] proposed

Using the trained transition model, enable prediction of individual developmental
trajectory based on early measured parameters (height, weight, cognitive milestones).
Allows identification of atypical development patterns for clinical follow-up.

---

## [2026-03-29] Integration with AIM diagnostic engine
**Source:** AIM ecosystem architecture — Ontogenesis as age-normalization module
**Status:** [ ] proposed

Expose Rust core as WASM module callable from AIM's `lab_reference.py`.
Would provide age-normalized reference ranges for 24 parameters across 0–25 yr,
improving accuracy of pediatric/adolescent diagnoses in AIM.

---

## [2026-03-29] Mobile 3D viewer for patient education
**Source:** Space project cross-pollination — 3D animations on mobile
**Status:** [ ] proposed

Lighter Three.js version for mobile (React Native via Expo, similar to Space project).
Parents/pediatricians could visualize normal developmental trajectories on mobile.

---

## [2026-03-29] Expand age range to 0–80 yr (full lifespan)
**Source:** CDATA project — aging trajectory modeling; Ontogenesis covers 0–25 yr only
**Status:** [ ] proposed

Merge Ontogenesis (development) with CDATA (aging) data models to create a
continuous 0–80 yr lifespan simulator. Development phase uses CV/Range algorithm;
aging phase uses CDATA centriole damage accumulation model.
