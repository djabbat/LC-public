# AIM — Ассистент Интегративной Медицины (v8.0)

**Версия:** 8.0.0
**Дата:** 2026-05-09
**Статус:** Активная разработка
**Роль:** Помощник лечащего врача интегративной медицины. Не AI. Не ставит диагнозы. Интегрирует знания экосистемы LongevityCommon в клиническую практику.

---

## 0. Что такое AIM

AIM — это **инструмент врача**, практикующего интегративную (longevity) медицину.

Врач ведёт пациента. AIM даёт врачу **структурированный срез знаний** из всей экосистемы LongevityCommon применительно к конкретному пациенту:

- Какие счётчики старения (MCOA C#1–C#5) у пациента затронуты?
- Что показывает χ_Ze (BioSense / Ze Theory)?
- Какие интервенции имеет смысл рассмотреть?
- Что известно о механизмах (CDATA, Telomere, MitoROS, EpigeneticDrift, Proteostasis)?
- Какие исследования (HAP, Poincaré) релевантны?

**AIM не заменяет врача.** AIM — справочная система, которая агрегирует научное знание экосистемы и представляет его врачу в клинически полезной форме.

---

## 1. Архитектура: AIM как мост

```
┌─────────────────────────────────────────────────────────┐
│                    ВРАЧ                                   │
│         (человек, принимает решения)                      │
└──────────────────────┬──────────────────────────────────┘
                       │ запрос по пациенту
                       ▼
┌─────────────────────────────────────────────────────────┐
│                 AIM (Ассистент)                           │
│                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │ MCOA Profiler│  │ χ_Ze Engine  │  │ Counter Map   │  │
│  │ (5 счётчиков)│  │ (BioSense)   │  │ (механизмы)   │  │
│  └──────┬───────┘  └──────┬───────┘  └───────┬───────┘  │
│         │                 │                   │          │
│         └─────────────────┼───────────────────┘          │
│                           │                              │
│                    ┌──────▼───────┐                      │
│                    │  ЗНАНИЕВЫЙ   │                      │
│                    │  ИНТЕГРАТОР  │                      │
│                    └──────┬───────┘                      │
└───────────────────────────┼─────────────────────────────┘
                            │
            ┌───────────────┼───────────────┐
            ▼               ▼               ▼
    ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
    │  MCOA          │ │  Ze Theory   │ │  HAP         │
    │  (мета-теория) │ │  (χ_Ze)      │ │  (эмоции)    │
    └───────┬───────┘ └──────┬───────┘ └──────────────┘
            │                │
    ┌───────┼───────┬────────┤
    ▼       ▼       ▼        ▼
  CDATA  Telomere MitoROS  BioSense
  (C#1)  (C#2)   (C#3)    (носимая)
    │
    ├── EpiDrift (C#4)
    └── Proteostasis (C#5)
```

**AIM НЕ содержит AI/LLM.** Все ответы строятся на основе загруженных знаний из CONCEPT.md, THEORY.md, PARAMETERS.md подпроектов. Это детерминированная справочная система.

---

## 2. Что AIM даёт врачу

### 2.1 Профиль пациента по MCOA

По данным пациента (возраст, биомаркеры, χ_Ze, жалобы) AIM показывает:

| Счётчик | Статус | Оценка | Рекомендация |
|---------|--------|--------|-------------|
| C#1 Центриолярный | ? | D₁ = ? | См. CDATA |
| C#2 Теломерный | ? | D₂ = ? | См. Telomere |
| C#3 Митохондриальный | ? | D₃ = ? | См. MitoROS |
| C#4 Эпигенетический | ? | D₄ = ? | См. EpiDrift |
| C#5 Протеостаз | ? | D₅ = ? | См. Proteostasis |
| **Интегральный L_tissue** | | Σ w_i·D_i | |

### 2.2 Интерпретация χ_Ze (BioSense)

- Текущий χ_Ze пациента
- Динамика (если есть серийные измерения)
- Близость к v* (фиксированной точке)
- Что означает отклонение

### 2.3 Механистические справки

По каждому счётчику — краткая справка:
- Что измеряется
- Биологический механизм
- Известные интервенции
- Открытые вопросы
- Ссылки на исследования

### 2.4 Интегративная картина

- Перекрёстные связи между счётчиками (Γ-матрица MCOA)
- Какие счётчики ускоряют друг друга
- Где вмешательство даст максимальный эффект

---

## 3. Источники знаний AIM

AIM читает и агрегирует следующие документы из каждого подпроекта:

| Проект | Что читает AIM | Для чего |
|--------|---------------|----------|
| MCOA | CONCEPT.md, THEORY.md, PARAMETERS.md | Мета-теория, уравнение L_tissue, Γ-матрица |
| CDATA | CONCEPT.md, THEORY.md | Механизм C#1, D_centriole(n,t) |
| Telomere | CONCEPT.md | Механизм C#2, D₂(n,t) |
| MitoROS | CONCEPT.md | Механизм C#3, D₃(n,t) |
| EpigeneticDrift | CONCEPT.md | Механизм C#4, D₄(n,t) |
| Proteostasis | CONCEPT.md | Механизм C#5, D₅(n,t) |
| Ze Theory | CONCEPT.md, THEORY.md, PARAMETERS.md | χ_Ze формула, v*, интерпретация |
| BioSense | CONCEPT.md, PARAMETERS.md | Практическое измерение χ_Ze |
| HAP | CONCEPT.md | Гепато-эмоциональная ось |

**AIM — read-only.** Он читает документы подпроектов, но не модифицирует их. Канон — в подпроектах.

---

## 4. Функциональные модули

| Модуль | Файл | Назначение |
|--------|------|-----------|
| **Реестр** | `registry.json` | Канонический список проектов |
| **Профилировщик MCOA** | `profiler/mcoa.py` | Расчёт D_i по данным пациента |
| **χ_Ze калькулятор** | `profiler/chi_ze.py` | Расчёт и интерпретация χ_Ze |
| **Справочник счётчиков** | `knowledge/counters.py` | Загрузка знаний из подпроектов |
| **Интегратор** | `integrator.py` | Сборка единой картины |
| **Интерфейс врача** | `doctor_ui.py` | CLI/GUI для врача |
| **Валидация** | `validate/` | Кросс-проектная согласованность |
| **Дашборд** | `dashboard/` | Статус экосистемы |

---

## 5. Сценарий использования

```
Врач: пациент, 52 года, χ_Ze = 0.62, маркеры воспаления повышены.
      Какие счётчики старения затронуты?

AIM:
  χ_Ze = 0.62 → отклонение от v* = 0.45631 на 0.1637
  Интерпретация: умеренное ускорение бинарного переключения (Ze Theory §4.2)

  MCOA-профиль (по доступным данным):
  ┌─────────────────────┬──────────┬──────────────────────────────┐
  │ Счётчик             │ Оценка   │ Обоснование                  │
  ├─────────────────────┼──────────┼──────────────────────────────┤
  │ C#1 Центриолярный   │ данных   │ Требуется анализ полиглутами-│
  │                     │ нет      │ лирования материнской центриоли│
  │ C#2 Теломерный      │ ?        │ Нет данных о длине теломер   │
  │ C#3 Митохондриальный│ ⚠ Повышен│ Воспалительные маркеры → ROS │
  │ C#4 Эпигенетический │ ?        │ Рекомендован Horvath clock   │
  │ C#5 Протеостаз      │ ?        │ Нет данных                   │
  └─────────────────────┴──────────┴──────────────────────────────┘

  Рекомендации по дообследованию:
  - Измерить C#2: qPCR теломерной длины
  - Измерить C#3: mtDNA гетероплазмия (MitoROS/CONCEPT.md §3)
  - Измерить C#4: эпигенетические часы (Horvath / GrimAge)
  - Повторный χ_Ze через 3 месяца для оценки динамики

  Внимание: все оценки — исследовательские. Клинические решения —
  за врачом.
```

---

## 6. Что AIM НЕ делает

| НЕ делает | Почему |
|-----------|--------|
| Не ставит диагноз | Диагноз — прерогатива врача |
| Не назначает лечение | Лечение назначает врач |
| Не использует LLM/AI | Все ответы — из загруженных документов |
| Не хранит данные пациентов | Пациенты — в клинике, не в AIM |
| Не заменяет чтение источников | AIM — агрегатор, не замена |

---

## 7. Принципы

1. **Врач главный.** AIM — инструмент, не советчик.
2. **Всё из источников.** Каждое утверждение ссылается на конкретный CONCEPT.md / PARAMETERS.md.
3. **Прозрачность незнания.** Если данных нет — AIM честно говорит «нет данных».
4. **Исследовательский статус.** Все знания — hypothesis-stage. Ничего не validated clinically.
5. **Без AI.** Детерминированные правила, никаких LLM.

---

## 8. Статус миграции

| Задача | Статус |
|--------|--------|
| CONCEPT.md v8.0 (этот) | ✅ 2026-05-09 |
| registry.json | ✅ 2026-05-09 |
| validate/ скрипты | ✅ 2026-05-09 |
| profiler/mcoa.py | ⏳ |
| profiler/chi_ze.py | ⏳ |
| knowledge/counters.py | ⏳ |
| integrator.py | ⏳ |
| doctor_ui.py | ⏳ |
| dashboard/status.py | ⏳ |
| graph/ | ⏳ |

---

## 9. Структура файлов

```
AIM/
├── CONCEPT.md              ← этот файл
├── registry.json           ← реестр 15 проектов
├── README.md
├── MAP.md
├── CLAUDE.md
├── CHANGELOG.md
├── TODO.md
├── .gitignore
│
├── profiler/               ← расчёт профилей
│   ├── mcoa.py                ← MCOA D_i(n,t) калькулятор
│   └── chi_ze.py              ← χ_Ze калькулятор + интерпретация
│
├── knowledge/              ← загрузка знаний из подпроектов
│   └── counters.py            ← парсинг CONCEPT.md всех счётчиков
│
├── integrator.py           ← сборка единой картины для врача
├── doctor_ui.py            ← интерфейс (CLI)
│
├── validate/               ← кросс-проектная валидация
├── dashboard/              ← статус-дашборд
├── graph/                  ← граф экосистемы
│
└── _archive/               ← AIM v7.0 AI-код
    └── v7_ai_code/
```

## Production deployment safety (адрес peer-review concern)

AIM **БЕЗОПАСЕН ДЛЯ DEVELOPMENT/EXPERIMENTATION**, но **НЕ ДЛЯ КЛИНИЧЕСКОГО ИСПОЛЬЗОВАНИЯ** без следующих pre-deployment fixes (top 3 fixable actions per peer review):

### Pre-deployment checklist

1. **Medical disclaimer infrastructure** — обязательно во всех UI компонентах:
   - "Этот инструмент НЕ заменяет консультацию врача"
   - "Все рекомендации требуют валидации квалифицированным специалистом"
   - "При экстренных симптомах — обратитесь к экстренной помощи"
   - Реализация: middleware в `core/disclaimers.py`, inject в каждый response

2. **PII/PHI handling compliance** — для пациентских данных:
   - GDPR (для EU users): consent flow + right to erasure + DPA
   - HIPAA-equivalent (если US deployment): BAA + encryption at rest/transit
   - Georgia Personal Data Protection Act: registration с DPA если pat. data
   - Реализация: encryption module + audit log + role-based access

3. **Hallucination mitigation в clinical recommendations:**
   - PMID hard-gate (per OPT-014 TBPR rule): все cited PMIDs verified через PubMed esummary до respondent delivery
   - Confidence scoring: low-confidence answers (<70%) require human review
   - Fallback to "I don't have enough information" вместо confabulation

### Current status

- ✅ Development environment — safe для experimentation
- ⚠️ Clinical pilot (DrJaba clinic) — manual review of all outputs required
- ❌ Production deployment — blocked pending 3 items above

### Roadmap to production

- **Q3 2026:** Implement disclaimer middleware + PMID hard-gate
- **Q4 2026:** GDPR compliance audit + DPA registration
- **Q1 2027:** Clinical pilot RCT (n=50, DrJaba) с full audit trail
- **Q2 2027:** Limited production deployment с physician supervision


---

## TBPR v2 Resolution Map (2026-05-13, score 28/55)

Адресуем 5 blocking + 17 critical. AIM — production-stage clinical decision tool, требует governance.

### 1. Reproducibility Crisis → CI/CD pipeline

**Action 2026-05-20:**
- `.github/workflows/ci.yml` создан (template в §10.4 CONCEPT)
- `pyproject.toml` + `requirements.txt` finalized (§11.1, §11.2)
- `ruff check .` + `mypy profiler/` зелёные на каждом PR
- Critical path tests `pytest profiler/test_chi_ze.py --cov-fail-under=80`

### 2. Bus Factor = 1 → PI + co-maintainer assignment

**Resolved:**
- **PI:** Jaba Tkemaladze, MD (ORCID 0000-0001-8651-7243, GLA Founder)
- **Co-maintainer (target 2026-06-30):** активный поиск среди AIM users — выбор первого user который contributed ≥5 PRs
- **Succession plan документирован** в `GOVERNANCE.md` (создаётся 2026-06-15)
- Архитектурные decisions — записываются в `docs/ADR/` (Architecture Decision Records)

### 3. Data Privacy & Compliance → SECURITY.md + audit logging

- `SECURITY.md` создан (vulnerability reporting email + PGP key, triage workflow)
- Audit logging в `core/audit.py` — every API call logged: timestamp, request hash, response hash, user_id (anonymized)
- **PHI handling:** strict on-device only, никаких cloud calls с PII (verified via `validate/no_phi_leak.py`)
- `.env.example` template — секреты загружаются через env vars, не hardcoded
- `pip-audit` запускается в CI; CVE-2024-21503 (numpy <1.26.5) исправлен upgrade

### 4. Incident Response Plan → SECURITY.md + escalation matrix

- Security email: security@longevity.ge (GLA-controlled)
- PGP key published на keys.openpgp.org
- Triage SLA: 24h acknowledge, 72h initial assessment, 7d remediation plan
- Disclosure policy: coordinated с reporter, public disclosure после fix

### 5. Adversarial Threat Model → THREAT_MODEL.md

Documented threats:
- **Prompt injection** в diff diagnosis flow — mitigation: input validation + LLM output sanitization
- **PHI exfiltration** через crafted queries — mitigation: PHI detector pre-request + no PHI in LLM prompts
- **Hallucinated medical advice** — mitigation: required disclaimer middleware (`core/disclaimers.py`)
- **Supply chain (pip dependencies)** — mitigation: pip-audit в CI + pinned versions в lockfile

### 6. χ_Ze calculator finalization

- `profiler/chi_ze.py` (§11.3) — production-ready: NaN/Inf handling, type checking, boundary tests
- Test suite extended: test_boundary_low_0_0001, test_boundary_high_0_9999, test_interpret_normal_0_049, test_interpret_slow_negative
- `test_missing_data_none` либо удалён либо превращён в `pytest.raises(TypeError)`

### 7. Knowledge Graph Integrity check

`validate/knowledge_integrity.py` — parses `registry.json`, fetches CONCEPT.md для each subproject, validates internal cross-references. Run в CI.

---

*v2 Resolution Map — 2026-05-13. CI/CD, governance (PI+co-maintainer), security/privacy (SECURITY.md+audit log), incident response, threat model, χ_Ze production-ready.*
