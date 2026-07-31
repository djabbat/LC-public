# Organismal Aging — MEMORY

**Date:** 2026-07-29

---

## 📚 2026-07-29: Обзор — Hydra: ультраструктура стволовости (Seybold, Salvenmoser et al., Innsbruck)

**Статья:** Seybold A, Salvenmoser W, Pfaller K, Redl S, Hess MW, Hobmayer B. "Ultrastructure of stemness and differentiated state in *Hydra* epithelial cells." bioRxiv 2026-07-23. DOI: `10.64898/2026.07.20.739505`

**Суть:** Эпителиальные клетки Hydra одновременно делятся и выполняют дифференцированные функции. Ядерные маркеры стволовости vs полярность/секреция. Cryofixation.

**Значение для Organismal_Aging:** Hydra — пренебрежимое старение; многофункциональный эпителий — ancestral condition.

**✅ 2026-07-29: Письмо Bert Hobmayer отправлено** — вопрос про центриоли в Hydra. Ждём ответа.

---

## 🔴 2026-07-27: Post-Mortem — Research Square отказ (rs-10483434)

**Тип:** Отказ препринт-платформы.
**Дней до решения:** 3 (24 июл → 27 июл).
**Причина:** «the manuscript type or its content is not suitable for posting as a preprint on Research Square.»

### Что сказали
> «Our screeners have determined that the manuscript type or its content is not suitable for posting as a preprint on Research Square. This decision does not reflect the quality or importance of the work and is made on the basis of our editorial policies with respect to content type and screening.»

### Что мы упустили
1. **Research Square — не для hypothesis papers.** Они принимают research articles с данными. Чистая гипотеза без экспериментальных результатов — не их формат.
2. **Объём.** ~11,000 слов — слишком много для препринта. Многие платформы имеют лимиты.
3. **Упоминание предыдущего отказа.** В Acknowledgments: «the four reviewers of the original manuscript (npj Aging, ID 2e8466c7)» — это красный флаг для скринеров.
4. **Research Square — не лучший выбор для этой статьи.** bioRxiv, arXiv (q-bio), Zenodo или OSF Preprints принимают hypothesis/theory papers.

### Что изменить
- [ ] Для препринта: **Zenodo** (уже есть MCARA там — 10.5281/zenodo.21299683) или **bioRxiv**
- [ ] Убрать из Acknowledgments упоминание npj Aging rejection — не для публичного препринта
- [ ] Medical Hypotheses — ждать решения (подана 24 июл)

### Следующий шаг
1. **Medical Hypotheses** — журнал специально для hypothesis papers. Это правильная цель. Ждать.
2. **Препринт:** залить на Zenodo (как MCARA) или bioRxiv. Не Research Square.

---

## Decision #1: Creating the Organismal Aging Project

**Date:** 2026-06-21
**Initiator:** Dzhaba
**Context:** Working session with pi

**Decisions:**
1. Name: **Organismal Aging** (not Organizmal)
2. Location: within **LC** as a single integrator
3. Aging — **the basis of all diseases** (aging-driven, not age-associated)
4. **Self-learning 4D simulator** (3D + time) with physical robot ARGUS-LP
5. **Three levels:** Centriole → 5 MCARA counters → Tissues + Ze conflicts
6. **8 basic tissues** with expansion capability
7. **Species universality:** human, mouse, C. elegans, unicellular organisms
8. **piRNA (#6) excluded from v1.0** — left as an expansion slot
9. **License:** Apache 2.0
10. **Open project** — all code, data, hardware (ARGUS-LP)

**Dzhaba verbatim:** "A simulator with a robot that tests hypotheses on physical simulators like INFOGEST, self-learns, creates a model of the organism in time and space, its development from zygote and age-related changes, possible injuries, other diseases, microbiome and macrobioime."

---

## Decision #2: Merging LC into a Single Project

**Date:** 2026-06-21
**Initiator:** Dzhaba

**Decision:** LC ceases to be an "ecosystem of subprojects." All former subprojects (CEDAR, MCARA, Ze, BioSense, FCLC, HAP) — components of the unified Organismal Aging system. Old CONCEPT.md → archive.

**Structure:** `sim_core/` (Rust) — core. `argus_bridge/`, `infogest_bridge/` — physical bridges. `biosense/`, `fclc/`, `hap/` — supporting components.

---

*Organismal Aging MEMORY v1.0 — 2026-06-21.*
