# Organismal Aging — STATE

**Дата:** 2026-06-21
**Статус:** 🟡 Pre-Alpha — проектирование архитектуры

---

## Текущее состояние

| Артефакт | Статус |
|---|---|
| CONCEPT.md | ✅ v2.0 (2026-06-21) |
| THEORY.md | ✅ v1.0 (2026-06-21) |
| PARAMETERS.md | ✅ v1.0 (2026-06-21) |
| MAP.md | ⏳ Требуется |
| STATE.md | ✅ Этот файл |
| MEMORY.md | ✅ (2026-06-21) |
| TODO.md | ⏳ Требуется |
| EVIDENCE.md | ✅ v1.0 (2026-06-21) |
| README.md | ⏳ Требуется |
| _pi.md | ⏳ Требуется |
| OPEN_PROBLEMS.md | ⏳ Требуется |
| DESIGN.md | ⏳ Требуется |

---

## Код

| Компонент | Статус |
|---|---|
| sim_core | 🔴 Не начат — проектирование |
| argus_bridge | 🟡 ARGUS-LP существует (Aubrey), мост не написан |
| infogest_bridge | 🔴 Не начат |
| species (виды) | 🔴 Не начат |
| learning (самообучение) | 🔴 Не начат |

---

## Интеграция с LC

| Шаг | Статус |
|---|---|
| LC/CONCEPT.md → v7.0 (единый интегратор) | ✅ |
| LC/MAP.md → новая структура | ✅ |
| LC/THEORY.md → единый формализм | ✅ |
| LC/TODO.md → обновлён | ✅ |
| LC/STATE.md → обновлён | ✅ |
| LC/MEMORY.md → зафиксировано решение #28 | ✅ |
| Архивация старых CONCEPT.md подпроектов | ⏳ |

---

## Ближайшие шаги

1. Завершить core-файлы Organismal_Aging (MAP, TODO, README, _pi)
2. Архитектурное решение по миграции кода cell_dt + mcoa → sim_core
3. Создать `sim_core/Cargo.toml`
4. Начать реализацию Уровня #1 (центриоль)

---

*Organismal Aging STATE v1.0 — 2026-06-21.*
