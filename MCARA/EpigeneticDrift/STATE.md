# EpigeneticDrift — Project Status (STATE.md)

**Дата:** 2026-07-09 | **Аудит:** pi (глубокий аудит MCARA)
**Статус:** 🟡 Активен (Counter #4 в MCARA)

---

## Текущее состояние

EpigeneticDrift — Counter #4 (Эпигенетический) в архитектуре MCARA. Отвечает за моделирование возрастного дрейфа эпигенетических меток (DNA methylation clocks, histone modifications) как одного из параллельных счётчиков старения.

## Core-файлы
- ✅ _pi.md, CONCEPT.md, TODO.md, PARAMETERS.md, MAP.md, STATE.md, MEMORY.md, README.md
- ✅ DESIGN.md, THEORY.md, EVIDENCE.md

## Инфраструктура
- ✅ backend/ (Cargo.toml, Dockerfile)
- ✅ frontend/ (mix.exs)
- ✅ crates/epigenetic_counter/
- ✅ data/ (PARAMETERS_calibrated.json)

## Последние обновления
- 2026-07-09: Глубокий аудит MCARA — core-файлы подтверждены, требуется обновление CONCEPT.md новыми данными (hnRNPs, Park/Di Stefano 2026)
- 2026-06-16: Аудит pi — состояние подтверждено

## Ближайшие задачи
1. Интегрировать данные из Park/Di Stefano (2026) — 5 уровней stem cell exit включают эпигенетический
2. Обновить ссылки на Horvath clock, GrimAge v2, DunedinPACE
3. Добавить asymmetric histone inheritance (Ma et al. 2026, PMID: 41872193)

## Связи с другими Counter'ами
- **Counter #1 (CEDAR):** Центриоли ↔ хроматин через PCM1, Oct4
- **Counter #3 (MitoROS):** Митохондриальные метаболиты → epigenetic enzymes (TET, KDMs)
- **Counter #5 (Proteostasis):** UBE2G1 → signalling ↔ epigenetics
