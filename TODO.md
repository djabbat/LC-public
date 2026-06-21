# LC · TODO (v7.0: Organismal Aging — единый проект)

**Дата:** 2026-06-21 (полное переосмысление)

---

## Phase 0 — Объединение проектов (Q3 2026) 🔴

- [ ] Переписать LC/CONCEPT.md → Organismal Aging как единый интегратор ✅ (2026-06-21)
- [ ] Обновить LC/THEORY.md → единый формализм ✅ (2026-06-21)
- [ ] Обновить LC/MAP.md → новая структура ✅ (2026-06-21)
- [ ] Обновить LC/PARAMETERS.md → единые параметры
- [ ] Обновить LC/STATE.md → статус интеграции
- [ ] Обновить LC/MEMORY.md → история решения об объединении
- [ ] Создать Organismal_Aging/core-файлы (THEORY ✅, PARAMS, STATE, MEMORY, TODO, EVIDENCE)
- [ ] Архивация старых CONCEPT.md подпроектов → `_archive/subprojects_concepts/`
- [ ] Решение о миграции кода: cell_dt + mcoa → sim_core (архитектурное)

## Phase 1 — sim_core: Уровни #1 + #2 (Q4 2026) 🟡

- [ ] Интеграция cell_dt crates → `sim_core/centriole/`
- [ ] Интеграция mcoa crates → `sim_core/counters/`
- [ ] Унификация API между уровнями #1 и #2
- [ ] Тесты интеграции centriole ↔ counters
- [ ] Бенчмарки: симуляция 120 лет за < 10 минут

## Phase 2 — Уровень #3 + Онтогенез (Q1 2027) 🟡

- [ ] 8 тканей: структуры, параметры τ_renewal, веса wᵢ
- [ ] Z_conflict(i,j,t): формализм + тесты
- [ ] Онтогенез: зигота → эмбрион → взрослый
- [ ] Микробиом: кишечник, кожа, рот
- [ ] Макробиом: INFOGEST-совместимая модель питания

## Phase 3 — Самообучение + ARGUS (Q2 2027) 🟡

- [ ] Байесовское обновление параметров (PyMC/Stan)
- [ ] ARGUS-LP мост: протокол, команды, парсинг
- [ ] INFOGEST мост: стандартный протокол
- [ ] Цикл гипотеза → эксперимент → обновление

## Phase 4 — Виды + Веб (Q3 2027) 🟡

- [ ] Видовая параметризация: человек, мышь, C. elegans
- [ ] Одноклеточные (E. coli, дрожжи) — без центриолей
- [ ] Phoenix LiveView: 4D-визуализация
- [ ] Python bindings (PyO3)

## Phase 5 — Валидация + Публикация (Q4 2027) 🟡

- [ ] Калибровка на GTEx + UK Biobank
- [ ] Валидация Z_conflict на реальных данных
- [ ] Статья в Nature Computational Science
- [ ] Открытый релиз v1.0 (Apache 2.0)

---

## Гранты и дедлайны

- [ ] **2026-07-30** — German Embassy (Ze)
- [ ] **2026-09-15** — Czech Embassy
- [ ] **2026-10-15** — SRNSFG AR-2026, National Geographic
- [ ] **2026-10-28** — EIC Pathfinder Challenges (WP1 CDATA + WP2 Pinekan + WP3 Aqtivirebuli + WP4 MIC)
- [ ] **2026-10-30** — EU4Business Georgia

---

*LC TODO v7.0 — 2026-06-21.*
