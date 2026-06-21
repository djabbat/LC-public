# LC · STATE (v7.0 — Organismal Aging, единый проект)

**Дата:** 2026-06-21 (тектонический сдвиг — объединение всех подпроектов)
**CONCEPT:** v7.0 (единый интегратор)
**Git:** требует реорганизации

---

## §0. КЛЮЧЕВОЕ РЕШЕНИЕ (2026-06-21)

**LC больше не экосистема подпроектов. LC = Organismal Aging — единый самообучающийся 4D-симулятор организма.**

Все бывшие подпроекты (CDATA, MCAOA, Ze, BioSense, FCLC, HAP) — компоненты единой системы. Старые CONCEPT.md подпроектов → архив.

**Причина:** Фрагментация мешала видеть целое. Organismal Aging объединяет три уровня (центриоль → счётчики → ткани/Ze-конфликты) в единый симулятор, интегрированный с физическим роботом ARGUS-LP.

---

## §1. Статус компонентов (после объединения)

| Компонент | Код | Статус |
|---|---|---|
| **sim_core** (ядро) | Проектируется | 🔴 Новый |
| ├─ centriole (Уровень #1) | cell_dt 10 crates | 🟡 Интегрируется |
| ├─ counters (Уровень #2) | mcoa 6 crates | 🟡 Интегрируется |
| ├─ tissue (Уровень #3) | Ze формализм | 🔴 Новый код |
| ├─ organism (интеграция) | — | 🔴 Новый код |
| ├─ species (виды) | — | 🔴 Новый код |
| ├─ microbiome | — | 🔴 Новый код |
| ├─ macrobiome | — | 🔴 Новый код |
| ├─ learning (самообучение) | — | 🔴 Новый код |
| └─ spatial (3D) | — | 🔴 Новый код |
| **argus_bridge** | ARGUS-LP (Aubrey) ✅ | 🟡 Мост не написан |
| **infogest_bridge** | — | 🔴 Новый |
| **biosense** | backend + frontend ✅ | 🟢 Работает |
| **fclc** | v13.4 ✅ | 🟢 Production |
| **hap** | Статья ✅ | 🟡 Готовится к сабмиту |
| **sim_gui** | Phoenix LiveView | 🟡 Проектируется |

---

## §2. Что работает сейчас

| Сервис | Порт | Статус |
|---|---|---|
| Ze backend | :4001 | 🟢 up |
| Ze Phoenix | :4000 | 🟢 up |
| BioSense backend | :4101 | 🟢 up |
| BioSense Phoenix | :4100 | 🟢 up |
| FCLC | :4002 | 🟢 production (сервер) |
| AIM Phoenix | :4040 | 🟢 up (сервер) |

---

## §3. Ближайшие действия

1. ✅ CONCEPT v7.0 написан
2. ✅ MAP v7.0 обновлён
3. ✅ THEORY v7.0 обновлён
4. ✅ TODO v7.0 обновлён
5. ⏳ PARAMETERS — требуется обновление
6. ⏳ MEMORY — зафиксировать решение
7. ⏳ Архивация старых CONCEPT.md подпроектов
8. ⏳ Архитектурное решение по миграции кода

---

## §4. Критические дедлайны

- German Embassy — 30 Jul 2026
- Czech Embassy — 15 Sep 2026
- EIC Pathfinder Challenges — 28 Oct 2026
- EU4Business Georgia — 31 Jul / 30 Oct 2026

---

## §5. Публикации в пайплайне

- CDATA → Nature Aging (доработка языка)
- HAP → Psychoneuroendocrinology (сабмит)
- Ze → Physica D (ожидание)
- MCAOA → после desk-reject — новый журнал
- Entropy in Aging → npj Aging (сабмит)

---

*LC STATE v7.0 — 2026-06-21.*
