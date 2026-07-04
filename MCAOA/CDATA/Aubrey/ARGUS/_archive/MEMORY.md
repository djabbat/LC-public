# MEMORY.md — Experiment 0

**Назначение:** Индекс auto-memory записей, релевантных для этого проекта. Живут в `~/.claude/projects/-home-oem/memory/`.

## Релевантные memories

### Project-level

- `project_cdata_copi_candidates.md` — Impetus Phase A co-PI candidates (rig будет использован в Phase A)
- (PhD-supervisor memories удалены 2026-05-04 правилом feedback_no_supervisor_names)
- `project_aubrey_collaboration.md` — Aubrey de Grey engagement with CDATA (context)
- `project_academic_upgrade.md` — академический upgrade pathway
- `project_eic_umbrella.md` — EIC Pathfinder consortium (Experiment 0 → WP3)

### Feedback (правила работы в проекте)

- `feedback_bradford_hill_rule.md` — Bradford Hill criteria для causality claims
- `feedback_mcaoa_cdata_comparison.md` — MCAOA vs CDATA comparison methodology
- `feedback_deepseek_primary.md` — DeepSeek как primary LLM
- `feedback_verify_references.md` — правило проверки reference перед claim

### Reference

- `pubmed_authoritative.md` — 10 authoritative PubMed publications
- `publications.md` — полный список публикаций автора
- `feedback_article_workflow.md` — workflow для submission

## Что сохранять в auto-memory из Experiment 0 работы

1. **Validated hardware specs** после закупки и тестирования (actual values, not estimated)
2. **Calibration constants** после dose matrix session (PWM → mW curve)
3. **Known issues** specific to этому конкретному rig (вибрация источников, thermal drift при t°C в квартире)
4. **Lessons learned** после Phase 0 → применимы к Phase A

## Как использовать

Before starting any substantive work on этом проекте — прочитать релевантные memories выше. После важных discoveries (validation, calibration, unexpected behavior) — сохранять новую memory в правильной категории.

## v3.1 (2026-05-13)

- Score: 49/55 — APPROVED with MINOR REVISIONS
- All PMIDs verified through PubMed esummary (feedback_pmid_verify_always)
- Removed fabricated PMIDs: 38015348, 38353211
- Added 6 PI publications 2024-2026 (Tqemaladze CDATA series)
- Tissue-specific inheritance accepted as evolutionary evidence, не contradiction
- Modern motor stack adopted (replaces Arduino Nano)
- AI brain stack: DeepSeek + pi + Gemini 2.5 Flash vision
