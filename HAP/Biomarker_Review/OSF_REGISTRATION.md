# OSF Registration — Step-by-Step

## 1. Создать проект на OSF

1. Зайти на https://osf.io/
2. Войти (Sign In) — можно через ORCID: 0000-0001-8651-7243
3. Нажать «Create new project»
4. Заполнить:

### Project Details

| Поле | Значение |
|------|----------|
| **Title** | Dynamic Biomarkers of Affective-Cognitive Regulation: A Systematic Map with Quality Assessment |
| **Description** | A systematic map with quality assessment investigating which peripheral and central biomarkers index dynamic transitions in affective-cognitive regulation in healthy adults under acute stress, uncertainty, or cognitive challenge. Protocol follows PRISMA 2020 + SWiM guidelines with JBI Critical Appraisal. Includes laboratory challenge paradigms (TSST, n-back, emotion regulation tasks) and ambulatory real-world assessment (EMA, wearable-based monitoring). Multi-language search (EN, DE, FR, ZH, ES) across PubMed, Embase, Scopus, PsycINFO, Web of Science, ProQuest Dissertations, and Google Scholar. |
| **Category** | Project |
| **License** | CC-By Attribution 4.0 International |

### Contributors

| Имя | ORCID | Роль |
|-----|-------|------|
| Jaba Tqemaladze | 0000-0001-8651-7243 | Administrator |

---

## 2. Загрузить протокол

1. В проекте выбрать вкладку «Files»
2. Нажать «Upload» → выбрать файл `PROTOCOL_v2.3.md`
3. Переименовать (опционально): `Protocol_Dynamic_Biomarkers_v2.3.pdf`

*Примечание: лучше конвертировать .md → .pdf перед загрузкой (через md2docx → .docx → Print to PDF, или через pandoc: `pandoc PROTOCOL_v2.3.md -o PROTOCOL_v2.3.pdf`)*

---

## 3. Создать Registration (пре-регистрация)

1. В проекте нажать «Registrations» (левое меню)
2. «New Registration»
3. Форма: выбрать **«Open-Ended Registration»** (наиболее гибкая для scoping/systematic map)
   - Или: **«OSF Preregistration»** — стандартная форма
4. Заполнить по шаблону ниже ↓

---

## 4. Текст для формы OSF Registration

### Study Information

**Title:**
Dynamic Biomarkers of Affective-Cognitive Regulation: A Systematic Map with Quality Assessment

**Authors:**
Jaba Tqemaladze (Georgia Longevity Alliance, ORCID: 0000-0001-8651-7243)

**Date:** 2026-06-15

### Research Question
Which peripheral and central biomarkers index dynamic transitions in affective-cognitive regulation in healthy adults, particularly under acute stress, uncertainty, or cognitive challenge?

### PCC Framework
- **Population:** Healthy adults (≥18 yr), human studies only. Mixed clinical-healthy samples included only if healthy controls reported separately.
- **Concept:** Dynamic transitions (baseline → activation → adaptation/recovery) indexed by biomarkers (HRV, cortisol, IL-6, CRP, tryptophan/kynurenine, EEG/ERP).
- **Context:** Acute stress, uncertainty, cognitive challenge paradigms (laboratory) + ambulatory real-world assessment (EMA, wearables).

### Hypotheses
N/A — this is a systematic map (descriptive), not hypothesis-testing. We map the evidence landscape; we do not test directional predictions.

### Study Design
Systematic map with quality assessment. Data sources: published articles, preprints (medRxiv, PsyArXiv), doctoral dissertations (ProQuest). No primary data collection.

### Inclusion Criteria
1. Human studies, healthy adult participants (≥18 yr)
2. Mixed clinical-healthy samples — only if healthy controls reported separately
3. At least one biomarker measured dynamically (≥2 time points, min 5-min interval within session OR repeated measures across conditions)
4. At least one measure of affective or cognitive state
5. Dynamic perturbation protocol (laboratory challenge OR ambulatory real-world assessment)
6. Languages: English, German, French, Chinese, Spanish (DeepL-assisted translation for non-English)
7. Peer-reviewed primary research, preprints (medRxiv, PsyArXiv), doctoral dissertations (ProQuest)
8. Publication date: 2000–2026

### Exclusion Criteria
1. Clinical populations only (unless healthy controls reported separately)
2. Static/baseline biomarker measurement only (single time point)
3. Animal studies
4. Case reports, commentaries, editorials
5. No affective/cognitive outcome measure
6. Pharmacological intervention (unless biomarker response to challenge is primary outcome)
7. Chronic stress focus (not acute transitions)
8. Resting-state only (no dynamic manipulation)

### Databases
PubMed/MEDLINE, Embase (mandatory), Scopus, PsycINFO, Web of Science, ProQuest Dissertations, Google Scholar (first 200 results), backward citation search.

### Search String (PubMed — primary)
See attached protocol §9. Combines MeSH terms for Heart Rate, Hydrocortisone, Interleukin-6, C-Reactive Protein, Tryptophan, Kynurenine, Electroencephalography, Emotional Regulation, Stress (Psychological), Cognition with tiab terms for specific biomarkers and paradigms. Date filter: 2000–2026.

### Screening Procedure
Dual independent screening with Rayyan. Pilot calibration: 50 random abstracts, Cohen's κ ≥ 0.70 before full screening. Conflict resolution by discussion; unresolved → flagged as «uncertain».

### Quality Assessment
JBI Critical Appraisal Checklists (revised 2026) — design-specific modules (cross-sectional, quasi-experimental, cohort) within unified framework. Quality score normalised as %; does NOT exclude studies; informs narrative synthesis.

### Extraction
See protocol §12 for 14-field extraction table including: study ID, sample, design, perturbation type, setting (lab/ambulatory), biomarker(s), temporal structure code (BA/BAR/RM/LD/ESM), affective/cognitive measure, key finding, quality score (JBI %), language.

### Synthesis Plan
1. Narrative synthesis by biomarker domain
2. Transition matrix (biomarker × temporal structure)
3. Setting comparison (lab vs ambulatory/ESM)
4. Quality-stratified synthesis (JBI ≥ 80% vs < 80%)
5. Gap analysis
6. Meta-analysis feasibility assessment
7. Future directions (gut-derived biomarkers)

### Target Journals
*Neuroscience & Biobehavioral Reviews* (IF 8.5), *Psychoneuroendocrinology* (IF 3.5), *Psychophysiology* (IF 3.7), *Biological Psychology* (IF 3.0). Final choice after feasibility pilot.

### Timeline
- Protocol finalised + OSF registration: June 2026
- Pilot searches (PubMed, Scopus, Embase): Weeks 1–3
- Pilot report: Week 3
- Title/abstract screening: Weeks 4–6
- Full-text screening + quality assessment: Weeks 7–10
- Extraction: Weeks 10–12
- Synthesis + manuscript: Weeks 13–16

### Funding
None.

### Competing Interests
None declared.

### Data Management
- Screening decisions: Rayyan (cloud backup)
- Extraction + quality ratings: Google Sheets (shared, version-controlled)
- Protocol + amendments: OSF (versioned)
- Final dataset: OSF supplementary material

---

## 5. После регистрации — действия

1. ✅ OSF выдаст **DOI** для регистрации
2. ✅ URL проекта: https://osf.io/mgzt5 (проект), https://osf.io/dqy38 (регистрация)
3. 🔗 Отправить ссылку заинтересованным коллегам
4. 📋 Добавить DOI в протокол (строка «Registration»)
5. ⏱️ Добавить соавторов на OSF при необходимости (Settings → Contributors)

## Статус (2026-06-15)
- ✅ Проект создан: https://osf.io/mgzt5 (публичный)
- ✅ Регистрация создана: https://osf.io/dqy38 (Open-Ended Registration)
- ✅ Файл PROTOCOL_v2.3.pdf загружен в проект и регистрацию
- ✅ Metadata: Resource type = Protocol, Language = English
- ⏳ DOI: ожидается

---

## 6. Файлы для загрузки

| Файл | Действие |
|------|----------|
| `PROTOCOL_v2.3.md` | Загрузить в OSF Files |
| `PROTOCOL_v2.3.pdf` | Сконвертировать и загрузить (красивее для sharing) |
| `PRISMA 2020 checklist` | Загрузить как supplementary |
| `SWiM reporting guideline` | Загрузить как supplementary |
| `JBI checklists (3 модуля)` | Загрузить как supplementary |
