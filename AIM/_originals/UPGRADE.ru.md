---

## TBPR Структурные рекомендации — 2026-05-13 22:25

- **Проект:** `LC/AIM`
- **Текущий score:** 28/55 (REVISE_MAJOR) на L1
- **Цель:** verdict **ACCEPT** (≥48/55)
- **Разрыв:** 20 баллов
- **Cycle dir:** `results/20260513_221810__LC_AIM/`

### 🔴 БЛОКИРУЮЩИЕ

**🔒 Воспроизводимость**

- [ ] Reproducibility Crisis (High) · _(EN)_
      > Источник: review_C blocking

**📐 Структура документа**

- [ ] Bus Factor = 1 (Critical) · _(EN)_
      > Источник: review_C blocking
- [ ] Data Privacy & Compliance (High) · _(EN)_
      > Источник: review_C blocking
- [ ] Incident Response Plan (Critical) · _(EN)_
      > Источник: review_C blocking
- [ ] Adversarial Threat Model (High) · _(EN)_
      > Источник: review_C blocking

### 🟠 КРИТИЧНЫЕ

**🧑‍🔬 PI / Команда**

- [ ] **Критерий:** Зелёный pipeline на GitHub Actions  ### F2: **Реализовать и протестировать χ_Ze калькулятор** (Priority: CRITICAL, Effort: 3h) · _(EN)_
      > Источник: review_A top-3
- [ ] **Критерий:** `pip-audit` проходит без ошибок, `.env.example` существует  --- · _(EN)_
      > Источник: review_A top-3
- [ ] **Mitigate the Bus Factor:**     - **Action:** Immediately assign a PI (Principal Investigator) and at least one co-maintainer. Update the `CONCEPT.md` with their names, ORCIDs, and affiliations.     - **Rationale:** This is a critical governance issue. The project cannot be a one-person show. A sec · _(EN)_
      > Источник: review_C top-3

**📐 Структура документа**

- [ ] Создать `.github/workflows/ci.yml` (код уже есть в CONCEPT.md §10.4) · _(EN)_
      > Источник: review_A top-3
- [ ] Создать `pyproject.toml` (код уже есть в CONCEPT.md §11.1) · _(EN)_
      > Источник: review_A top-3
- [ ] Создать `requirements.txt` (код уже есть в CONCEPT.md §11.2) · _(EN)_
      > Источник: review_A top-3
- [ ] Убедиться, что `ruff check .` и `mypy profiler/` проходят · _(EN)_
      > Источник: review_A top-3
- [ ] Создать `profiler/chi_ze.py` (код уже есть в CONCEPT.md §11.3, но нужно доработать) · _(EN)_
      > Источник: review_A top-3
- [ ] Исправить `test_chi_ze.py`:    - Удалить `test_missing_data_none` (TypeError) или добавить type checking    - Завершить `test_invalid`    - Добавить тесты: `test_boundary_low_0_0001`, `test_boundary_high_0_9999`, `test_interpret_normal_0_049`, `test_interpret_slow_negative` · _(EN)_
      > Источник: review_A top-3
- [ ] Добавить логирование и обработку NaN/Inf · _(EN)_
      > Источник: review_A top-3
- [ ] **Критерий:** `pytest profiler/test_chi_ze.py --cov=profiler --cov-fail-under=80` проходит  ### F3: **Обновить зависимости и добавить security** (Priority: HIGH, Effort: 2h) · _(EN)_
      > Источник: review_A top-3
- [ ] Обновить numpy до 1.26.5+ (fix CVE-2024-21503) · _(EN)_
      > Источник: review_A top-3
- [ ] Создать `.env.example` с шаблоном для конфигурации · _(EN)_
      > Источник: review_A top-3
- [ ] Добавить `core/disclaimers.py` с middleware для medical disclaimer · _(EN)_
      > Источник: review_A top-3
- [ ] Добавить audit logging (минимум: timestamp, request, response hash) · _(EN)_
      > Источник: review_A top-3
- [ ] **Implement a Robust Incident Response Plan:**     - **Action:** Create a `SECURITY.md` file in the repository root.     - **Content:** Определить a clear process for reporting security vulnerabilities (e.g., a dedicated email address, a PGP key for encrypted communication). Outline a triage process (e.
      > Источник: review_C top-3
- [ ] **Implement a "Knowledge Graph Integrity" Check:**     - **Action:** Create a new validation script, `validate/knowledge_integrity.py`.     - **Functionality:** This script should:         1.  Parse the `registry.json` to get the list of sub-projects.         2.  For each sub-project, fetch the `CON · _(EN)_
      > Источник: review_C top-3

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

