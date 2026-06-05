# pi.md — LC

> ## 🔴 ЖЕЛЕЗНОЕ ПРАВИЛО: ПАМЯТЬ ПРЕЖДЕ ДЕЙСТВИЯ
> 
> **Перед любым действием (чтение файла, запуск команды, конвертация, ответ пользователю) — ОБЯЗАТЕЛЬНО прочитать:**
> 1. \`_pi.md\` — этот файл (правила, структура, команды)
> 2. \`MAP.md\` — карта проекта
> 3. \`MEMORY.md\` — история решений, запреты, токены
> 
> Нарушение → игнорирование памяти → потеря контекста → ошибки.
> 
> ---

Памятка агента pi. Создан 2026-05-11.

## Суть

Hypothesis-stage framework: 5 подпроектов (MCAOA, CDATA, Ze, BioSense, FCLC) + социальный слой (server/web/realtime). Все AUC — exploratory, не confirmatory. Pre-registered тесты χ_Ze — NULL (deprecated). Публикации НЕ peer-reviewed.

## EIC Pathfinder Challenges 2026

- Deadline: **28 Oct 2026**, бюджет до €4M
- Variant C: WP1-5 (FCLC + Ze + CDATA + BioSense + Activatus), €3.0M / 36 mo
- Партнёры: Geiger (Ulm, ✅ LoS 23.04.2026), Janke Curie (advisory + его co-PI как real partner), COSIC/Preneel KU Leuven
- Miguel Angel Gonzalez Ballester (UPF Barcelona) — встреча 28.04.2026, ждём ответа

## CDATA (из памяти Claude, 2026-05-17)

### Статус: 439 тестов, GUI 7-язычная, 6 aging tracks (A-F)

### C1-C4 critical issues:
- C1: p21/p16 не влияют на damage accumulation rate
- C2: ×4.2 calibration circular — не валидирован против CEP135/SAS-6
- C3: ROS-mtDNA формула linear — нужен sigmoid с threshold 20%
- C4: ZeHealthState computed but never read

### Исследования
- **Aubrey de Grey** — active dialog: division-counted vs time-counted P(n,t) = P₀ + α·n + β·t
- **Carsten Janke (Curie)** — сам не может (CoI), но advisor по polyglutamylation + intro своей co-PI
- **Hartmut Geiger (Ulm)** — Phase B Co-PI ✅ (LoS signed 23.04.2026)
- **Liz Parrish (BioViva)** — Strategic Co-PI (support letter signed 22.04.2026)

### Tsomaia E0 design (laser ablation rig)
- Phased Gateway Validation: 4 phases (optical → ablation → mechanics → automation)
- 3-Objective Architecture (no dichroic cascade)
- Total cost если все phases pass: $720-1500
- Team: Tsomaia (lead) + Zheleznov (junior)
- Встречи каждую субботу 15:00 Tbilisi

### Корреспонденция
- Janke ответил 28.04.2026 → ждём intro его co-PI
- Geiger письмо отправлено 15.05.2026 (advisory Phase A)
- Liz Parrish: 4 quarterly review calls over 18 mo

### GitHub
- CDATA-Aubrey: github.com/djabbat/CDATA-Aubrey (public, CC BY 4.0)
- OSF: https://osf.io/kqby4/ (DOI 10.17605/OSF.IO/KQBY4)

## HAP — HALTED (с 2026-04-21)
- Deep audit: 10/10 PMIDs fabricated, 56-taxa CSV missing
- Quarantined: EVIDENCE.md → .QUARANTINED_2026-04-21
- Halt lifted когда: EVIDENCE.md rebuilt from verified PMIDs

## Ontogenesis — HALTED (с 2026-04-21)
- Deep audit: 6/6 PMIDs fabricated
- Missing: ABCD Study, ALSPAC, Bethlehem 2022 и др.
- Ethics section: 6 lines — недостаточно

## Структура

19 поддиректорий (core), + корневые .md файлы. См. CONCEPT.md для authority order.

### Основные подпроекты (5)
- `MCAOA/` — Multi-Counter Architecture of Aging (ранее MCAOA)
  - `CDATA/` — Centriolar Damage Accumulation Theory
    - **Aubrey/** → вынесен в отдельный проект `~/Desktop/Aubrey/`
- `Ze/` — Entropy-geometric ansatz dτ/dt = −α·I(Z)
- `BioSense/` — wearable-платформа, χ_Ze биомаркер
- `FCLC/` — Federated Learning
- `AIM/` — Integrative Medicine Assistant (AI-ядро, v8.0)

### Социальный слой (3)
- `server/` — Rust/axum REST API
- `web/` — React+TS PWA (social layer UI)
- `realtime/` — Phoenix Channels (WebSocket)

### Вспомогательные (11)
- `_archive/`, `_audits/`, `audits/`, `_originals/
- `deploy/` — деплой-конфиги
- `docs/` — документация
- `refs/` — PMID-файлы (17 references, with README)
- `scripts/` — скрипты
- `services/` — (пусто, задел под миграцию Services)
- `shared-types/` — Rust shared types (Cargo.toml + src/)
- `__pycache__/` — кэш Python

### Корневые файлы
`CONCEPT.md`, `README.md`, `TODO.md`, `THEORY.md`, `KNOWLEDGE.md`, `MEMORY.md`, `STATE.md`, `DESIGN.md`, `EVIDENCE.md`, `MAP.md`, `OPEN_PROBLEMS.md`, `PARAMETERS.md`, `PI_TRACK_RECORD.md`, `PROJECT_AUDIT_2026-05-12.md`

## Различия локальной и серверной копий

Локальная (`~/Desktop/LC/`) содержит **не все** подпроекты сервера.

### Есть на сервере, НЕТ локально:
- `CDATA/` — Centriolar Damage Accumulation Theory
- `AutomatedMicroscopy/` — автоматизация микроскопии (новый подпроект)
- `CytogeneticTree/` — цитогенетическое дерево
- `EpigeneticDrift/` — эпигенетический дрейф
- `HAP/` — HAP-исследования
- `MitoROS/` — митохондриальный ROS
- `Proteostasis/` — протеостаз
- `Telomere/` — теломеры

### Есть локально, НЕТ на сервере:
- `FCLC/` — Federated Learning Cloud
- `shared-types/` — Rust shared types
- `services/` — (пусто)
- `MCAOA/` — (на сервере `MCAOA/`)

### Связь с сервером

- `AIM/` → симлинк → `~/hive_queen/AI` (на сервере)
- Локальная копия (`~/Desktop/LC/`) — сервер (`~/LC/`) — git-оригинал

## Команды

```bash
# Пуш на сервер
cd ~/Desktop/LC && git push server main
```

---

## Заметки
## 2026-05-15

## 2026-05-15 — HARD RULE: Rust + Phoenix везде

ЖЕЛЕЗНОЕ ПРАВИЛО: Весь новый код — только Rust (backend, API, core) + Phoenix/Elixir (frontend, web). Никакого Python/Flask/Node.js в production. Исключения — только по прямому указанию Jaba.

## 2026-05-15

## 2026-05-15: Aubrey tbpr-project overnight loop

Запущен `/home/oem/overnight/overnight --id aubrey-tbpr-project` с `tbpr-project --dir ~/Desktop/Aubrey --name Aubrey --provider deepseek -v`.
PID: 392762. Лог: /tmp/overnight/aubrey-tbpr-project/output.log

Двухфазная архитектура (CONCEPT+core → packet, rest → inventory) уже применена.
Предыдущие вердикты: REVISE_MAJOR (21/55) — ожидается улучшение после интеграции документа инженера (L1 mEos3.2 validation).

## 2026-05-15

## 2026-05-15

## Aubrey/ARGUS файлы на Desktop
- `~/Desktop/Aubrey_CONCEPT_v5_ENG.md` (70KB) — основной концепт Aubrey v5
- `~/Desktop/Aubrey_META-REVIEW_v5.md` (8.8KB) — meta-review
- Путь из CLAUDE.md: `~/Desktop/LC/CDATA/experiments/CellLineageTree/Aubrey/` — НЕ существует локально (CDATA только на сервере)
- Файлы лежат на верхнем уровне Desktop, не в LC

## 2026-05-14

2026-05-14 — Исправлена расшифровка AIM в README.md

Было: "Architecture Integration Matrix"
Стало: "Integrative Medicine Assistant" — соответствует каноническому CONCEPT.md

## 2026-05-15

2026-05-15 — Обновлена структура (pi):
- 19 поддиректорий (было 18): добавлены `refs/` (17 PMID), `shared-types/` (Rust), `services/` (пусто)
- MCAOA/ (было MCAOA) — подтверждено
- AIM расширен: `_archive/aim-web_2026-05-07/` с deps/bandit
- Добавлены корневые файлы: STATE.md, DESIGN.md, EVIDENCE.md, MAP.md, OPEN_PROBLEMS.md, THEORY.md, TODO.md

## 2026-05-14

## Rust/Phoenix rule for gabro — 2026-05-14
Все новый код для gabro пишется ТОЛЬКО на Rust (core/backend) или Phoenix/Elixir (frontend/UI).
Другие языки — только по прямому указанию pi.
Rust library: ~/Desktop/Services/gabro-erudition/ — self-optimizing erudition layer.

## 2026-05-13

## 2026-05-14 — Gabro Intelligence Imperative

**Каждое действие pi — делать gabro умнее.** Это absolute priority.

Gabro — локальный Triada agent (amigdala + PEL + TBPR + gabro).
Ключевые improvements:
1. SYSTEM_PROMPT — identity + Triada + tools + правила
2. `_project_tool_query()` — pre-gather данных через bash/sqlite/git/ps
3. `_build_smart_prompt()` — сборка system + lang + data + CoT
4. Chain-of-thought: 1) данные 2) анализ 3) вывод
5. Правило "не выдумывай" — только из [DATA] секции
6. Auto-switch на deepseek-r1:14b для сложных запросов

Никаких изменений без улучшения gabro.

## 2026-05-13

## 2026-05-13 — Критическое правило: PMID всегда проверять
Перед вставкой любого PMID/DOI — проверить:
1. Что DOI/PMID реально существует (resolvable)
2. Что содержание статьи соответствует контексту (не fabricated как с DOI:10.1038/s41598-019-53456-3 — микрофлюидика вместо stage)
3. Fabricated reference = REVISE_MAJOR минимум. 3+ = TOXIC_WITHDRAW.
