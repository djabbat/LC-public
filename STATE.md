<!-- AUTO-TRANSLATED via DeepSeek 2026-05-13. Source language: mixed. Original preserved at STATE.x.md. -->

# LC · STATE

**Last touched:** 2026-05-24

## 👤 Статус автора
- **Jaba Tqemaladze, MD** — начал PhD программу в **UNED Madrid** осенью 2026
- Заявка UNED создана (Nº 712356513), оплачена 32€, PDF отправлены

**Last touched:** 2026-06-02
**CONCEPT version:** v5.6 (regenerated from article `~/Desktop/LC.md` on 2026-04-28)

---

## §0. New contact — Family Offices via Vortex Capital Learn (2026-06-02)

- **Roberts Reizins**, Vortex Capital Learn (roberts.c@vortexcapitallearn.com) — бутиковый инвестбанк
- Представляют Family Offices, заинтересованные в Longevity Clinic, Inc, Georgia
- Jaba ответил, предложил время на 9–11 June GMT+4
- Roberts подтвердил, запросил introductory meeting
- **Status:** ✅ Письмо отправлено Roberts (Reizins). Вторник 9 июня 15:00 GMT+4. IM приложен.

### 🔥 Новый контекст (2026-06-02, разговор с Джабой):
- **Клиника выбрала место** — напротив флагманской гостиницы **Paragraph Hotel** на курорте **Абастумани**
- Абастумани — лучший горный курорт Грузии (уровня Давоса, Швейцария)
- Идут переговоры по покупке земли
- Курорт начнёт принимать мировую элиту через ~2 года (€500M+ development)
- **Вторник 9 июня 15:00 GMT+4** — подтверждённая встреча с Roberts (Vortex Capital)
- Investment Memorandum обновлён (добавлен Abastumani контекст)

## §1. What was done in this iteration (2026-04-28)

- ✅ Previous CONCEPT/THEORY/DESIGN/EVIDENCE/OPEN_PROBLEMS archived in `_archive/v_pre_2026-04-28/`
- ✅ New core .md files regenerated from article v5.6: CONCEPT, THEORY, DESIGN, PARAMETERS, MAP, EVIDENCE, OPEN_PROBLEMS, STATE (this), TODO, README
- ✅ Article on Desktop as `LC.md` (50 KB) + .docx (40 KB)
- ✅ Three iteration outputs (v1/v2/v3 articles + peer reviews + audits) saved on Desktop as backup

## §2. Subproject status snapshot

| Subproject | Status | Last updated |
|------------|--------|--------------|
| **MCAOA** | submitted Nature Aging NATAGING-P13741, NOT peer-reviewed | (per article) |
| **CDATA** | inconclusive (Sobol p=0.12 after correction; full S2/ST pending) | (per article v5) |
| **Ze** | regenerated 2026-04-28; simulator + backend + Phoenix live; F-tests partial pass | 2026-04-28 |
| **BioSense** | regenerated 2026-04-28; simulator + backend + Phoenix live; B1-B6 + datasets crate; γ velocity convention applied | 2026-04-28 |
| **FCLC** | v13.4 PASS milestone; semi-honest only; v14 planned Q1 2027 | 2026-04-26 (server deployment) |
| **Activated** | clinical pilot ready (Shashviashvili) | (extern) |
| **AIM** | ✅ **Декомпозирован 2026-06-02** — компоненты распределены по проектам | removed |
| HAP / Ontogenesis | TOXIC — not in v5.6 ecosystem | halted 2026-04-21 |

## §3. Open blockers

### Top 3 critical:
1. **EIC consortium 0 signed EU LoIs** — Miguel Angel González Ballester meeting today 12:30 gating
2. **N≥2000 pre-registered cohort** — all scientific claims hypothesis-stage until this
3. **FCLC malicious-secure (v14)** — GDPR Art. 9 blocker; Q1 2027 timeline

Full list: `OPEN_PROBLEMS.md`.

## §4. Live services

| Service | Status | URL |
|---------|--------|-----|
| Ze backend | up :4001 | http://127.0.0.1:4001/healthz |
| Ze Phoenix | up :4000 | http://127.0.0.1:4000 |
| BioSense backend | up :4101 | http://127.0.0.1:4101/healthz |
| BioSense Phoenix | up :4100 | http://127.0.0.1:4100 |
| BioSense /datasets | up :4100/datasets | 12 datasets registry |
| FCLC (server-resident) | up :4002 server-side | https://fclc.longevity.ge |
| Social server (`server/`) | not started this session | — |
| Social web (`web/`) | not started this session | — |
| Social realtime | not started; port conflict pending fix | — |

## §5. Code that may need changes after CONCEPT v5.6 (audit list)

Per `DESIGN.md §5`:

### §5.1 Server (`~/Desktop/LC/server/src/`)
- [ ] `handlers/biosense.rs` — add header `X-LC-Status: hypothesis-stage-exploratory`
- [ ] `handlers/dashboard.rs` — strings "biological age" → "exploratory aging activity index (research only)"
- [ ] `handlers/ze_guide.rs` — system prompt update: explicit "research-grade exploratory metric, not validated medical biomarker; v1 NULL retracted; v2 post-hoc; no medical advice; decline confirmatory clinical claims"
- [ ] new endpoint `GET /api/disclosures/v5_changes` returning changelog
- [ ] `migrations/003_health_factors.sql` — add comment "thresholds exploratory, see CONCEPT v5.6 §2"

### §5.2 Web (`~/Desktop/LC/web/src/`)
- [ ] `pages/Dashboard.tsx` — add banner "⚠ Hypothesis-stage research platform. Metrics are exploratory, not clinical advice."
- [ ] `pages/Studies.tsx` — each study card: "v1 NULL deprecated/superseded; v2 multimodal post-hoc"
- [ ] `pages/Profile.tsx` — tooltip on χ_Ze: "exploratory metric; not validated on N≥2000 pre-registered cohort"
- [ ] `components/feed/PostComposer.tsx` — DOI Crossref check; warn if DOI is Longevity Horizon

### §5.3 Realtime (`~/Desktop/LC/realtime/`)
- [ ] `config/dev.exs` — port 4001 → 4500 (avoid Ze conflict)
- [ ] BioSense live stream channel — add metadata `{disclosure: "exploratory"}`

### §5.4 Deploy
- [ ] `deploy/docker-compose-all.yml` — port + service name updates per §5.3
- [ ] env `LONGEVITYCOMMON_VERSION=v5.6`

### §5.5 Subproject CONCEPTs cross-check
- [ ] `Ze/CONCEPT.md` — pull latest cross-cutting status from umbrella
- [ ] `BioSense/CONCEPT.md` — same; ensure χ_Ze "hypothesis-stage" + post-hoc multimodal disclosed
- [ ] `FCLC/CONCEPT.md` — threat model wording aligned ("semi-honest only; not active server collusion")
- [ ] `MCAOA/CONCEPT.md` — M4 operational threshold (N≥2000, α=0.001, partial r²<0.05) added
- [ ] `CDATA/CONCEPT.md` — status "inconclusive" added; ABL-2 explanation; Sobol full decomp deferred

**None of these block scientific layer functioning** — all are documentation/disclosure consistency updates.

## §6. Versions

- CONCEPT: v5.6 (this regeneration; supersedes v4.0 in `_archive/v_pre_2026-04-28/CONCEPT.md`)
- Article: v5.6 + iterations 1-3 (article on Desktop as `LC.md`; backups: `LC_Article_v3_2026-04-28.md`)
- Subproject CONCEPTs: pending cross-check (see §5.5)
- Social layer code: pending point-edits (see §5.1-§5.4)

## §7. Pinning

To make CONCEPT verifiable later, record source-document md5:
```
md5sum ~/Desktop/LC.md
```
[record on next commit]