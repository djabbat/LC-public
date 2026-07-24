# BioSense Improvement Plan

## P0 — Blockers (without them the project is unacceptable for production)

### P0.1 Single Source of Truth for v* and Constants
- Create `config/constants.toml` with v*_active, f_opt, dataset parameters.
- Rust: generate `src/constants.rs` through `build.rs` (or read `.toml` through `serde`).
- Python: read the same file or import from `constants.py`, which parses `.toml`.
- Remove duplicates from `PARAMETERS.md`, `KNOWLEDGE.md`, `MEMORY.md`, `main.rs`, scripts.
- **Affected files:** `backend/src/main.rs`, `src/eeg_ze_processor.py`, `src/*.py`, `config/constants.toml` (new), `backend/build.rs` (new), `PARAMETERS.md`, `KNOWLEDGE.md`, `MEMORY.md`.
- **Estimation:** M (2 days); **Risk:** M (necessary to synchronize all links).

### P0.2 Full Test Coverage (Rust + Python)
- Rust backend: unit tests for core logic (e.g., `compute_chi_ze`), integration tests through `axum::test`, coverage >80%.
- Python: unit tests for `eeg_ze_processor.py` (`ze_cheating_index`, `narrowband_ze`, `group_statistics`), mock for data loading.
- CI: `cargo test`, `pytest` are mandatory before merge.
- **Affected files:** `backend/src/main.rs` (add `#[cfg(test)]` modules), `backend/tests/` (new), `src/tests/` (new), `.github/workflows/ci.yml` (new).
- **Estimation:** M (3 days); **Risk:** M (will reveal existing bugs).

### P0.3 Input Validation in Rust Endpoints
- Add deserialization with validation: `NaN`, `Inf`, ranges (e.g., `v` ∈ [0,1], `age` ∈ [0,150]).
- Return `422 Unprocessable Entity` with error description.
- **Affected files:** `backend/src/main.rs` (add `#[derive(Deserialize)]` with `#[serde(deny_unknown_fields)]` and custom `deserialize_with`).
- **Estimation:** S (0.5 days); **Risk:** L (isolated change).

### P0.4 Structuring Python Code into a Package
- Create `src/biosense/` with subfolders `core/` (eeg_ze_processor), `analysis/` (ze_cuban_analysis, etc.), `utils/` (data loading).
- Add `__init__.py` to each folder; imports — through `from biosense.core import ...`.
- Update all scripts and `biosense.sh`.
- **Affected files:** `src/` (rename and move), `README.md` (update structure), `MAP.md`.
- **Estimation:** M (1 day); **Risk:** L (isolation, regression unlikely).

### P0.5 Clear Exclusion Rule for Python in TODO.md
- In `TODO.md` change "if not explicitly stated — Rust" to "Python is allowed ONLY for EEG/HRV analysis (scientific scripts) and for AIM ML router. All production code (backend, deployment) — Rust."
- **Affected files:** `TODO.md`, `CLAUDE.md`.
- **Estimation:** S (0.1 days); **Risk:** L.

### P0.6 Fixing ChiZeRequest and `/api/v_star` Conversion
- Add to `backend/src/main.rs` the `ChiZeRequest` structure with `#[serde(alias)]` for legacy fields.
- Implement `/chi_ze`: calculate χ_Ze by the formula (Python form), but return in Article form (multiply? according to PARAMETERS.md: Article = 2·Python − 1). Clarify with the author.
- Ensure that `/api/v_star` returns Article form `-0.08738` and the documentation `CLAUDE.md` explicitly indicates the format.
- **Affected files:** `backend/src/main.rs` (add `ChiZeRequest`, implementation), `CLAUDE.md`.
- **Estimation:** S (1 day); **Risk:** M (high probability of mismatch with client expectations).

---

## P1 — Important (significantly affects development and maintenance)

### P1.1 .gitignore
- Add `data/`, `__pycache__/`, `*.pyc`, `*.egg-info`, `target/`, `*.mat`, `.env`.
- **Affected files:** `.gitignore` (new).
- **Effort:** S (0.1 days).

### P1.2 Dependency Version Fixation
- `src/requirements.txt`: replace `>=` with `==` with specific versions (e.g., `mne==1.6.1`). Use `pip freeze`.
- `backend/Cargo.toml`: fix versions through `major.minor.patch` (but already sufficient).
- **Affected files:** `src/requirements.txt`, possibly `backend/Cargo.lock` (already lock).
- **Effort:** S (0.5 days).

### P1.3 Moving Organizational Rules from TODO.md
- From `TODO.md` move the section "📌 Rule: language..." and "📌 Rule: DeepSeek..." to `CLAUDE.md`.
- In `TODO.md` leave only tasks, and rules — move to `RULES.md` or `CLAUDE.md`.
- **Affected files:** `TODO.md`, `CLAUDE.md`.
- **Effort:** S (0.2 days).

### P1.4 CI Pipeline
- GitHub Actions: `cargo build`, `cargo test`, `cargo clippy`, `pytest`, `flake8` (or `ruff`).
- Add badge to `README.md`.
- **Affected files:** `.github/workflows/ci.yml` (new), `README.md`.
- **Effort:** M (1 day).

### P1.5 JSON Result Management
- Move all `.json` (except service ones) to `results/` (already partially). Check that no extra `.json` are committed to the root.
- Add `results/*.json` to `.gitignore`, if they are generated; otherwise, leave, but add `**/results/*.json` to `git lfs` or limit the size.
- **Affected files:** `.gitignore`, possibly `results/`.
- **Effort:** S (0.5 days).

---

## P2 — Nice-to-have (improvements that can be postponed)

### P2.1 OpenAPI Specification
- Create `openapi.yaml` for Rust backendс описанием всех эндпоинтов.
- **Затронутые файлы:** `docs/openapi.yaml` (новый).
- **Трудоёмкость:** M (1-2 дня).

### P2.2 Logging и метрики в Python
- Заменить `print` в Python-скриптах на `logging` с уровнями `INFO`, `DEBUG`, `ERROR`.
- **Затронутые файлы:** `src/biosense/core/*.py`, `src/biosense/analysis/*.py`.
- **Трудоёмкость:** S (0.5 дня).

### P2.3 Мониторинг для Rust backend
- Добавить метрики (`/metrics` с prometheus), трейсинг (OpenTelemetry).
- **Затронутые файлы:** `backend/src/main.rs` (добавить `axum-prometheus`).
- **Трудоёмкость:** M (1-2 дня).

### P2.4 Удаление dead code и выравнивание import-ов
- Проверить Python-скрипты на неиспользуемые функции, дублирование логики (например, загрузка данных).
- **Затронутые файлы:** все `.py`.
- **Трудоёмкость:** S (0.5 дня).

### P2.5 Автоматическая генерация документации
- `cargo doc`, `pydoc` или `sphinx` для Python.
- **Затронутые файлы:** `Makefile` или `justfile`.
- **Трудоёмкость:** M (1 день).