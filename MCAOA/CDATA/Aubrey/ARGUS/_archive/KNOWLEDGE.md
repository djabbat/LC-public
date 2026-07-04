# KNOWLEDGE.md — ARGUS

**Версия:** 2.0
**Дата:** 2026-05-15
**Назначение:** Подтверждённые факты, validated references, known constraints. НЕ hypotheses — только то, что проверено.

## Validated hardware facts

1. Микроскоп **Custom inverted microscope (Tsomaia design)** — полностью оригинальная конструкция
2. Lamp housing принимает **OSRAM Halogen-BELLAPHOT 64607, 8V 50W** (verified by physical inspection 2026-04-23, запасная лампа в оригинальной OSRAM-коробке на месте).
3. Filter cube slider p/n **OPTON 46 52 24-01** (2 позиции: DAPI + FITC/GFP) — установлен.
4. Photo port adapter p/n **OPTON 47 17 73-9901** — есть, внутри 3.2× projection lens.
5. **XIMU LGY40-C** manual XY stage (40×40 мм, dovetail, 2 micrometer heads 0.5 мм pitch) — куплен, в квартире.
6. Объективы доступны из manual: Plan 6.3/0.16, Plan 16/0.35, Plan-Neofluar 25/0.8 W Oil, LD-Plan 40/0.6, Plan 40/0.65, Planapo 63/1.4 Oil, Plan 100/1.25 Oil. **Точный набор на турели — надо проверить визуально** (гравировка на объективах).

## Validated control stack capabilities

1. **Claude Code /overnight mode** — подтверждено работает через `~/Desktop/LC/AIM/llm.py` → DeepSeek API (`~/.aim_env → DEEPSEEK_API_KEY`).
2. **DeepSeek models:** `deepseek-chat` (fast) и `deepseek-reasoner` (для сложных задач) — верифицировано 2026-04-23.
3. **LLM router** `llm.py` уже маршрутизирует между DeepSeek / Kimi / Qwen / Groq; fallback logic работает.
4. **Arduino Nano + A4988 + NEMA-8/11 → stepper** — стандартный well-tested паттерн; accelerated через AccelStepper library.
5. **Micro-Manager 2.0 + ToupCam adapter** — community adapter существует (github.com/toupcam/toupcam-mm-plugin).

## Validated references (ARGUS v4.0 — 14 PMIDs, verified via PubMed 2026-05-15)

### Core centriole biology (ARGUS motivation)

| # | PMID | Year | Journal | Key finding |
|---|------|:----:|---------|-------------|
| 1 | **36583780** | 2023 | *Mol Biol Rep* | CDATA Counter #1 — selective inheritance of old centrioles by stem cells |
| 2 | **37882444** | 2023 | *eLife* | Asymmetric centriole inheritance in human neural progenitors (RITE markers) |
| 3 | **16336191** | 2005 | *Biochemistry (Mosc)* | Centriolar mechanisms of morphogenesis |
| 4 | **15886028** | 2005 | *Cell Biol Int* | Determination of morphogenetic status |
| 5 | **21407209** | 2011 | *Nat Commun* | Differential PCM regulation of mother and daughter centrioles in Drosophila |
| 6 | **40243666** | 2025 | *J Cell Biol* | Microtubule and actomyosin centrosome positioning |

### Centriole ablation validation (safety justification)

| # | PMID | Year | Journal | Key finding |
|---|------|:----:|---------|-------------|
| 7 | **17227892** | 2007 | *J Cell Biol* | *De novo* centriole assembly after ablation in normal human cells |
| 8 | **15738265** | 2005 | *Mol Biol Cell* | Centriole ablation does not block cell cycle in HeLa |

### Centriole asymmetry and development

| # | PMID | Year | Journal | Key finding |
|---|------|:----:|---------|-------------|
| 9 | **22683192** | 2012 | *Curr Opin Cell Biol* | Mother-daughter centriole asymmetry in development |
| 10 | **25047620** | 2014 | *Philos Trans R Soc B* | Link between centriole age and cell fate |
| 11 | **33435817** | 2021 | *Open Biol* | Centriolar asymmetry in asymmetric stem cell divisions |
| 12 | **25883936** | 2015 | *Front Cell Dev Biol* | Atypical centrioles in reproduction |
| 13 | **34440763** | 2021 | *Front Cell Dev Biol* | Centriolar asymmetry in early Drosophila oogenesis |
| 14 | **28562636** | 2017 | *PLoS One* | Systematic review of asymmetric organelle inheritance (77% studies low N) |

**Source:** All PMIDs verified via PubMed esummary (2026-05-15). Fabricated PMIDs 38015348 and 38353211 removed.

## MCAOA framework context

- **MCAOA** = Multi-Counter Architecture of Organismal Aging (Tqemaladze J. 2026, Zenodo DOI: 10.5281/zenodo.20055806)
- ARGUS targets empirical validation of **centriolar counter (#1)** predictions
- CDATA = Centriolar Damage Accumulation Theory of Aging [PMID: 36583780] — theoretical basis for counter #1
- MCAOA (Multi-Counter Architecture of Organismal Aging, formerly MCAOA) — renamed to avoid confusion with bioinformatics abbreviations. Zenodo DOI: 10.5281/zenodo.20055806

## Key experimental precedent: ablation validation

1. **La Terra 2005 [15738265]:** Centriole removal in HeLa cells does not block cell cycle; cells assemble new centrioles *de novo* — confirms ablation is not a lethal intervention in transformed cells.
2. **Uetake 2007 [17227892]:** Normal human cells progress through G1 without centrioles and assemble them *de novo* — justifies working with normal fibroblasts (BJ-hTERT) in Phase B.
3. **Royall 2023 [37882444]:** Asymmetric inheritance of the older mother centriole is required for human neural progenitor self-renewal — central justification: ARGUS must distinguish old vs new centrioles.
4. **Systematic review [28562636]:** 77% of asymmetric organelle inheritance studies have low statistical reliability (<10 replicates). ARGUS Phase A exceeds this standard (n=500 cycles).

## External references (non-author, используемые в обосновании)

Все PMID проверены 2026-04-23 (см. `Полное_Описание.md` §7):

1. **Royall L, Machado D, Jessberger S, Denoth-Lippuner A** (2023) eLife 12:e83157. **PMID 37882444** — Centriolin-RITE tool. Для Experiment A.
2. **Mangione F, D'Antuono R, Tapon N** (2022) Front Physiol 13:1093303. **PMID 36685184, PMC9845895** — 405 nm ns ablation benchmark. <!-- corrected 2026-04-26: prior wording cited fabricated «Strunov 2022 PMID 36685234»; real authors/PMID per PubMed verification, see docs/REFERENCE_AUDIT_2026-04-26.md row 7 -->
3. **Bürgy L et al** (2023) BMC Bioinformatics 24:120. **PMID 36977999, PMC10045196** — CenFind centriole detection CNN.
4. **Zeigler MB, Chiu DT** (2009) Photochemistry and Photobiology 85:1218-1224. **PMID 19558419, PMC5600466** — laser selection / cell viability.
5. **Botvinick EL, Venugopalan V, Shah JV, Liaw LH, Berns MW** (2004) Biophys J 87:4203-12. **PMID 15454403**, DOI 10.1529/biophysj.104.049528 — picosecond laser microtubule ablation, gold standard for sub-μm cellular surgery. <!-- corrected 2026-04-26: prior wording cited fabricated «Maiato/Khodjakov 2004 PMC1304929»; real authors/PMID per docs/REFERENCE_AUDIT_2026-04-26.md row 4 -->
6. **Yamashita YM, Jones DL, Fuller MT** (2003) Science 301:1547-1550. **PMID 12970569** — Drosophila GSC centrosome asymmetric division.
7. **Verzijlbergen KF et al** (2010) PNAS 107:64-68. **PMID 20018668** — Original RITE.
8. **Icha J et al** (2017) BioEssays 39:e201700003. **PMID 28749075** — phototoxicity guide. <!-- corrected 2026-04-26: PMID was 28749007 (off-by-68, that PMID = Aitken sperm ROS paper); correct PMID per docs/REFERENCE_AUDIT_2026-04-26.md row 21 -->
9. **Vogel A et al** (2005) Applied Physics B 81:1015-1047 — fs laser nanosurgery mechanisms.
10. **Laissue PP et al** (2017) Nat Methods 14:657-661. **PMID 28661494** — phototoxicity assessment. <!-- corrected 2026-04-26: PMID was 28661495 (off-by-1, that PMID = Zoppè size perception); correct PMID per docs/REFERENCE_AUDIT_2026-04-26.md row 22 -->

## Known physical constraints

- **Vibration:** квартирный стол, нет оптического стола. Нужны резиновые виброподкладки 10мм. Мitigation частичная — для 100×/1.4 + laser ablation может быть критично, для 40×/0.65 ok.
- **Thermal drift:** отсутствует активное охлаждение. В жаркую погоду +2°C = stage drift ~5 µm за 1 час. Для 30-min intervals приемлемо.
- **Phototoxicity:** 450 nm CW фототоксичен. Dose matrix calibration обязательна (см. `Полное_Описание.md` §4.4).
- **Elodea стохастичность:** chloroplast inheritance stochastic, НЕ моделирует детерминированную центриолярную асимметрию (см. PEER_REVIEW_DRAFT).

## Known software constraints

- Claude Code session timeout (длинные сессии могут прерываться) — mitigation через /overnight + systemd restart.
- DeepSeek API rate limits — не должно быть проблемой на этом masштабе.
- Arduino Nano 2KB RAM — ограничивает sophisticated motion planning; достаточно для Step/Dir + interrupt handlers.
- Micro-Manager MDA (Multi-Dimensional Acquisition) надёжность на 6-месячных сессиях не верифицирована — risk.

## Validated budget estimates

См. `PARAMETERS.md` раздел "Бюджет" + `BOM.md` — итоговый диапазон $881–$1687 для minimum и optimum комплектов.

## References not yet verified

- ~~Точные параметры OSRAM 64607 base type (prefocus)~~ — требует измерения штангенциркулем
- ~~Photo port thread type для C-mount adapter~~ — требует фото open port изнутри
- ~~Объективы на турели~~ — требует визуальной проверки гравировки

## v3.1 KEY UPDATES (2026-05-13)

### Tissue-specific centriole inheritance
- Drosophila neuroblasts retain DAUGHTER centrosome (Januschke 2011, PMID 21407209)
- Human NPCs inherit MOTHER centrosome (Royall 2023, PMID 37882444)
- ARGUS-Aubrey targets human model (consistent с Royall)

### Centrosome positioning mechanisms (Schaeffer 2025, PMID 40243666)
- Microtubule-anchored forces damped by actin network
- Actomyosin contractility generates centripetal flow → centrosome moves independent of MT
- Phase B control required: blebbistatin 50 µM

### Centriolin-RITE limitations (Royall 2023)
- Centriolin stable >20 days после рекомбинации
- Mixed tdTomato/NeonGreen signals при age determination
- Secondary marker: Centrin-1-GFP (turnover ~24h)
