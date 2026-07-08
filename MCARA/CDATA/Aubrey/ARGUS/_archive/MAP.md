# MAP.md — ARGUS

**Версия:** 2.0
**Дата:** 2026-05-15
**Назначение:** Файловая структура проекта + связь с экосистемой. Источник истины — `CONCEPT.md`.

## Файловая структура

```
~/Desktop/E0/
├── INDEX.md                         — навигация для внешнего рецензента
├── CONCEPT.md                       — концепция, цели, границы
├── README.md                        — project overview (legacy)
├── CLAUDE.md                        — правила работы Claude Code в проекте
├── MAP.md                           — этот файл
├── MEMORY.md                        — ссылки на auto-memory записи
├── KNOWLEDGE.md                     — подтверждённые факты, verified references
├── PARAMETERS.md                    — численные параметры (optics, laser, camera)
├── TODO.md                          — задачи
├── UPGRADE.md                       — roadmap улучшений Phase 0 → Phase A
├── LINKS.md                         — внешние ссылки (AliExpress, eBay, PMIDs)
│
├── 00_EXECUTIVE_SUMMARY.md          — 1-page pitch для specialist
├── 01_CONTROL_ARCHITECTURE.md       — Claude + Arduino + Python stack
├── 02_HARDWARE_INVENTORY.md         — inventory + фото-ссылки
│
├── BOM.md                           — Bill of Materials (AUTHORITATIVE)
├── ENCLOSURE.md                     — light-tight box CAD
├── CLAUDE_AGENT.md                  — Claude agent implementation detail
├── Phase_0_Prototype.md             — cheaper prototype alternative
├── Полное_Описание.md               — full 1000-line reference
├── Техническая_реализация.md        — deep engineering
├── Покупки_Китай.md                 — Taobao shopping через брата
├── Adapter_для_токаря.md            — LED mount adapter чертёж
├── PEER_REVIEW_DRAFT.md             — самокритика / known limitations
│
└── photos/                          — фото-инвентарь + video
    ├── INDEX.md                     — описание каждого фото
    ├── photo_2026-04-23_*.jpg       — 35 фотографий
    └── video.mp4                    — обзорное видео
```

## Связь с экосистемой (другие репо/директории)

| Внешний ресурс | Связь |
|---|---|
| `~/Desktop/LC/CDATA/` | Научная основа (CDATA theory) |
| `~/Desktop/LC/AutomatedMicroscopy/` | Параллельный project про AI microscopy |
| `~/Desktop/LC/MCAOA/` | Multi-Counter Architecture of Organismal Aging — meta-framework, Counter frameworks |
| `~/Desktop/LC/AIM/llm.py` | LLM router (DeepSeek primary) для Claude agent |
| `~/.aim_env` | DEEPSEEK_API_KEY |
| `~/Desktop/PhD/docs/` | Gitignored переписка, не для этого проекта |

## Что НЕ в E0 (external dependencies)

- **Impetus LOI** → `~/Documents/Submissions/2026-04-25_Impetus_CDATA/` (NOT here, отдельный grant submission)
- **MCAOA submission** → `~/Documents/Submissions/2026-04-25_NatureAging_MCAOA/`
- **PhD dissertation** → `~/Desktop/PhD/dissertation/`

## Git tracking

`~/Desktop/E0/` → `djabbat/E0-private` (выделено из PhD/microscope/Experiment_0 — 2026-04-26).

## v3.1 Project Map (2026-05-13)

- ARGUS = construction phase (simulator-only)
- Parent: Aubrey = biological pilot (BJ-hTERT + hypoxia + Centrin1-Kaede)
- Grandparent: CellLineageTree = methodology umbrella
- Theory: CDATA Counter #1 (centriolar damage accumulation)
- Meta: MCAOA (Multi-Counter Architecture of Organismal Aging)
- Application: Impetus Phase A grant ($80K)
- Phase B: Geiger lab Ulm (€100K, conditional на Phase A Go)
