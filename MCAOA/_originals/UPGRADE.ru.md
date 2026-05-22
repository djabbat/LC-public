---

## TBPR Структурные рекомендации — 2026-05-13 22:25

- **Проект:** `LC/MCAOA`
- **Текущий score:** 29/55 (REVISE_MAJOR) на L0
- **Цель:** verdict **ACCEPT** (≥48/55)
- **Разрыв:** 19 баллов
- **Cycle dir:** `results/20260513_222247__LC_MCOA/`

### 🔴 БЛОКИРУЮЩИЕ

**💰 Бюджет**

- [ ] BUDGET INFLATION WITH FABRICATED EQUIPMENT COSTS** — qFISH station at €20k (real cost · _(EN)_
      > Источник: review_A blocking

**📊 Данные / Когорта**

- [ ] ZERO VERIFIABLE PRELIMINARY DATA** — Section 11 · _(EN)_
      > Источник: review_A blocking
- [ ] NO PRELIMINARY DATA FOR CORE CLAIMS** — The document explicitly states "no MCAOA-specific experimental data exist · _(EN)_
      > Источник: review_C blocking

**🧪 Falsifiability**

- [ ] NO NEGATIVE RESULTS CONSIDERED** — The document never asks · _(EN)_
      > Источник: review_C blocking

**🧑‍🔬 PI / Команда**

- [ ] PI IDENTITY FABRICATION** — The lead PI "Dr · _(EN)_
      > Источник: review_A blocking
- [ ] ORCID FABRICATION** — The ORCIDs provided (0000-0002-1234-5678, 0000-0003-4567-8910) appear to be fabricated sequential patterns · _(EN)_
      > Источник: review_A blocking
- [ ] PI COMMITMENT UNCERTAIN** — Lead PI Solovei has NOT signed consortium agreement (risk score 4/5) · _(EN)_
      > Источник: review_C blocking

**🔒 Воспроизводимость**

- [ ] PUBLICATION BIAS IGNORED** — No mention that positive results (e · _(EN)_
      > Источник: review_C blocking

**⚠️ Риски / Этика**

- [ ] CONSORTIUM DOES NOT EXIST** — All named partners (Ito, Mann, Melzer, Gladyshev) are listed without LoIs, without consent, and with explicit caveat "naming is conditional on written consent · _(EN)_
      > Источник: review_A blocking

**📐 Структура документа**

- [ ] PARAMETER OVERFITTING DANGER** — The model has ~25+ free parameters (5 counters × 3 rates + coupling matrix + weights + transfer functions + thresholds) · _(EN)_
      > Источник: review_C blocking
- [ ] CENTRIOLE AS "MASTER COUNTER" IS UNSUPPORTED** — The claim that centriole polyglutamylation is the primary division counter has ZERO published evidence in aging · _(EN)_
      > Источник: review_C blocking
- [ ] SURVIVOR BIAS IN THEORY COMPARISON** — §10 compares MCAOA to 5 theories but doesn't mention the dozens of aging theories that failed (e · _(EN)_
      > Источник: review_C blocking

### 🟠 КРИТИЧНЫЕ

**💰 Бюджет**

- [ ] **REPLACE CONTINGENCY WITH REAL COSTS**: Reduce contingency from 15% to 5% (€37,500). Добавить the €75,000 saved to specific line items: (a) increase OA fees to €15,000 (3 papers × €5k), (b) Добавить software developer (€50k for 6 months to build mcoa-core), (c) add ethics consultation (€10k). This shows you 
      > Источник: review_B top-3

**📚 Литература / PMID**

- [ ] **ADD SYSTEMATIC NEGATIVE RESULTS SECTION** — For each counter, cite studies that FAILED to find correlation with aging. For example: "Telomere length does not predict mortality in centenarians [PMID: X]" or "Antioxidant supplementation does not extend lifespan in humans [PMID: Y]." Show that MCAOA c · _(EN)_
      > Источник: review_C top-3

**🧪 Falsifiability**

- [ ] **REDUCE PARAMETER COUNT WITH CROSS-VALIDATION PLAN** — Уточнить how parameters will be constrained: e.g., "We will measure n_i* and τ_i in 3 independent cell types and require <20% variation across types. If variation >20%, the dimensionless assumption is falsified." Pre-register parameter ranges be
      > Источник: review_C top-3

**🧑‍🔬 PI / Команда**

- [ ] **Replace PI with real, consenting researcher** — Identify a genuine aging researcher (e.g., from MPI Cologne, University of Cologne, or other institution) who has agreed to serve as PI. Привести verifiable ORCID, Scopus ID, publication list, and grant history. This is **mandatory** — the current PI 
      > Источник: review_A top-3
- [ ] **Generate preliminary data before submission** — Execute Test 4 (Division vs Time) as a standalone pilot using the claimed €100k seed funding. Привести actual results (even negative) rather than "no data exist." Alternatively, reduce TRL claim to TRL 2 (theoretical framework only) and Удалить experim
      > Источник: review_A top-3
- [ ] **GET SOLOVEI'S REAL PUBLICATIONS**: Replace all [TBD] with actual PMIDs and titles. If Solovei hasn't consented, Удалить her as Lead PI and find a real PI with verifiable track record. This is non-negotiable.
      > Источник: review_B top-3
- [ ] **OBTAIN PI COMMITMENTS AND PRELIMINARY DATA** — Before resubmission, secure signed LoIs from Solovei and at least 2 named collaborators. Execute Test 4 (Division vs Time) and publish results. Without this, the grant is unfundable. · _(EN)_
      > Источник: review_C top-3

**📐 Структура документа**

- [ ] **Obtain and attach actual Letters of Intent (LoIs)** — Contact real labs (Ito, Mann, Melzer, Gladyshev or alternatives) and secure written confirmation of participation. Without LoIs, the consortium is fictional. This is **mandatory** for any fundable proposal. · _(EN)_
      > Источник: review_A top-3
- [ ] **SECURE AT LEAST 2 SIGNED LoIs**: Drop the pretense of 4 partners. Focus on Ito (iPSC organoids — most critical for Test 4) and one other. Get written agreements. Without LoIs, the consortium is fictional. · _(EN)_
      > Источник: review_B top-3

_Сгенерировано: TBPR structural_writeback.py · 2026-05-13 22:25_


## v3 Update (2026-05-13)

CONCEPT.md updated with TBPR peer-review responses:
- Verified PMIDs through PubMed esummary (per `feedback_pmid_verify_always`)
- Removed fabricated references
- Addressed top blocking/critical reviewer concerns
- Statistical protocol additions where applicable
- Honest TODO sections для unmet requirements

See `CONCEPT.md` Section с пометкой "v3" / "Адрес peer-review concerns"
для project-specific changes.

