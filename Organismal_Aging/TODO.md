# Organismal Aging — TODO

**Дата:** 2026-06-21

---

## 🔴 Phase 0: Core-файлы + Архитектура (Q3 2026)

- [x] CONCEPT.md v2.0 ✅
- [x] THEORY.md v1.0 ✅
- [x] PARAMETERS.md v1.0 ✅
- [x] EVIDENCE.md v1.0 ✅
- [x] STATE.md v1.0 ✅
- [x] MEMORY.md v1.0 ✅
- [ ] MAP.md
- [ ] README.md
- [ ] _pi.md
- [ ] OPEN_PROBLEMS.md
- [ ] DESIGN.md
- [ ] Архивация старых CONCEPT.md подпроектов → `_archive/subprojects_concepts/`
- [ ] Архитектурное решение: миграция cell_dt + mcoa → sim_core

## 🟡 Phase 1: Ядро sim_core (Q4 2026)

- [ ] `sim_core/Cargo.toml` + структура крейта
- [ ] `centriole/` — Уровень #1: энтропия, деление, polyGlu
- [ ] `counters/` — Уровень #2: 5 счётчиков + агрегатор L_tissue
- [ ] Интеграция с существующим кодом cell_dt + mcoa
- [ ] Тесты: центриоль, счётчики, интеграция
- [ ] Бенчмарк: 120 лет за < 10 минут

## 🟡 Phase 2: Ткани + Ze + Онтогенез (Q1 2027)

- [ ] `tissue/` — 8 тканей с τ_renewal, w_i, L_crit
- [ ] `tissue/ze_conflict.rs` — Z_conflict(i,j,t)
- [ ] `organism/development.rs` — зигота → эмбрион → взрослый
- [ ] `microbiome/` — кишечник, кожа, рот
- [ ] `macrobiome/` — INFOGEST-совместимая модель
- [ ] `spatial/` — 3D-анатомия (базовая)

## 🟡 Phase 3: Самообучение + ARGUS (Q2 2027)

- [ ] `learning/bayesian.rs` — MCMC обновление параметров
- [ ] `argus_bridge/` — протокол, команды, парсинг
- [ ] `infogest_bridge/` — стандартный протокол INFOGEST
- [ ] Цикл: гипотеза → ARGUS → результат → обновление

## 🟢 Phase 4: Виды + Веб (Q3 2027)

- [ ] `species/human.rs` — базовая параметризация
- [ ] `species/mouse.rs` — мышь
- [ ] `species/celegans.rs` — C. elegans
- [ ] `species/unicellular.rs` — одноклеточные (без центриолей)
- [ ] `sim_gui/` — Phoenix LiveView, 4D-визуализация
- [ ] `sim_py/` — Python bindings

## 🔵 Phase 5: Валидация + Публикация (Q4 2027)

- [ ] Калибровка на GTEx
- [ ] Валидация Z_conflict
- [ ] Статья → *Nature Computational Science*
- [ ] Открытый релиз v1.0

---

*Organismal Aging TODO v1.0 — 2026-06-21.*
