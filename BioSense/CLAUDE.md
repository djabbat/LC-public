# BioSense — Носимый браслет: EEG · HRV · Запах

## Backend port (decided 2026-05-07; production state corrected 2026-05-08)

**Production state:** `:4502` is **already** held by a Docker container
`deploy-biosense-backend-1` (image `deploy-biosense-backend`, running
since Apr 30 2026) which serves the existing chi_ze API consumed by
`biosense-web` Phoenix on `:4501`. Field shape: input expects `eeg/hrv/...`
NOT `v_eeg/v_hrv/...`.

The native Rust crate at `BioSense/backend/` (Phase 4.4 work) cannot
co-exist on `:4502` without breaking biosense-web. Resolution:

- **`BioSense/backend/`** (native Rust, Phase 4.4 idiomatic shape with
  `v_eeg` field naming) is **not** the production backend until field-
  name reconciliation is done. Builds, tests pass, but no systemd unit
  enabled on production.
- **Docker `deploy-biosense-backend-1`** remains canonical. nginx
  `biosense.longevity.ge.conf` /api/ → :4502 → that container.
- Per memory `feedback_no_docker` rule (no-Docker), the container is
  tech-debt to retire. Migration plan:
  1. Reconcile field names (`v_eeg` ↔ `eeg`) — pick one, update both
     biosense-web client + native Rust backend.
  2. Stop & remove `deploy-biosense-backend-1` container.
  3. Enable `biosense-backend.service` systemd unit on :4502.

Until then BioSense/backend/ is **dev-only**. Run with custom port
(e.g. `PORT=14502 ./target/release/biosense-backend`) for local tests.

---


## 📌 Правило: DeepSeek для нетехнических задач

**Код (Python/Rust) — Claude. Всё остальное — DeepSeek API.**
Примеры: статьи о χ_Ze, peer review, введение/обсуждение, переводы.
**Ключ:** `~/.aim_env → DEEPSEEK_API_KEY` · **Вход:** `~/Desktop/AIM/llm.py`
**Модели:** `deepseek-chat` (быстро) · `deepseek-reasoner` (научные рассуждения)

---

## Проект

Верификация Ze-теории на EEG-данных. Гипотеза: χ_Ze(молодые) > χ_Ze(пожилые).
Статья `ze_eeg_paper.docx` v8 (410KB) — почти готова к отправке.

## Связь с AIM

`ze_ecg.py` → AIM HRV анализ пациентов (χ_Ze, v*, RMSSD).
