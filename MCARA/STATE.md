# STATE — MCARA

**Date: 2026-07-24
**Status:** 🟢 Active. Pierre Gönczy's response received — centrioles are preserved for a reason (signaling or centrosomal). 🆕 Iron-Positive Centriole Remnant hypothesis added to CONCEPT.md §0.


## 2026-07-23: ERR repo created
- GitHub: https://github.com/Georgia-Longevity-Alliance/ERR
- Marketing folder: ~/Desktop/Marketing/ERR/ (renamed from MCARA_EIC_Pathfinder)
- README contains Cologne 2026 handout
## Counters
- C1: CEDAR — centriolar
- C2: Telomere
- C3: MitoROS
- C4: EpigeneticDrift
- C5: Proteostasis

## Current focus
- Consortium: Gönczy (EPFL) responded — field is open, mechanism is unknown, recommended Pimenta-Marques 2023
- Article v8: in Biogerontology (ID 7cc6de62), status 🟡 With Editor
- EIC Pathfinder grant: deadline October 28, 2026
- CIRCBIO-07: deadline September 17, 2026

## Latest changes (2026-07-19 — major update)

### Exchange with Pierre Gönczy
- Three questions — three answers. Field confirmed as open
- Recommended: Pimenta-Marques 2023 (ANA1/CEP295) + Kalbfuss & Gönczy review
- Follow-up letter ready

### In-depth audit (41 PMID)
- 93% verified (38/41)
- 🔴 Lindhout 2021 — corrected (contradicted CEDAR)
- ⚠️ Bodnar/Parrinello overstatements corrected
- Bradford Hill: 5/9 → 6/9 (Temporality 🟢, Specificity 🟡)

### New articles integrated
- **Centriculum** (Maheshwari/Cohen-Fix 2023 Curr Biol, 2026 J Cell Sci) — ER-membrane reticulum around centrosome
- **Spermatogenesis** (Ishida & Shibuya 2026) — gamete asymmetry: oogenesis (elimination) vs spermatogenesis (preservation)
- **Tweedell** — not found on PubMed (old article without indexing)

### File versions
- CONCEPT.md: v4.7 (+Centriculum, +Spermatogenesis, +Three-stage model)
- THEORY.md: v4.7 (+Centriculum, +Gamete asymmetry)
- EVIDENCE.md: v4.7 (§11 Gönczy, §12 Centriculum, §13 Spermatogenesis)
- MEMORY.md: updated
- Audit: `audits/MCARA_Evidence_Audit_2026-07-19.md`

## Next steps
- [x] Follow-up to Pierre sent (July 19)
- [x] Pierre's response received (July 21) — centrioles are preserved for a functional reason
- [ ] Integrate Pierre's response into EVIDENCE.md §11
- [ ] CCP1-KO experiment as the next critical step
- [ ] Update article v8 with new data
## Simulator v0.5 — 2026-08-08
- Эпигенетический счётчик калиброван: τ=100 лет, β=1.0, d_critical=0.75 (EpigeneticDrift).
- Геометрический счётчик центриоли: модель Орнштейна–Уленбека (α≈0.97), наследование геометрии мать→дочь (Wang 2014; Panda 2024).
- Асимметричное деление в стволовых тканях: старейшая центриоль остаётся в СК → retention_boost ×1.6, α_eff=0.985.
- Функции цилии (порог damage 0.30) и центросомы (порог 0.60) деградируют с накоплением.
- Тесты: 18/18 проходят (mcara_core 6, mcara_simulation 9, mcara_compare 3).
