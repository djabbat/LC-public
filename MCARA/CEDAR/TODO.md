# CEDAR — TODO

> **📄 Articles and publications:** see `~/Desktop/Services/publications/PUBLICATIONS_TRACKER.md`

## 🟢 2026-10-15 — EVM-пилот (детерминистический явно-электронный движок, Anastasov)
<!-- lang:ru -->
- [x] **Пилот выполнен:** глицин + Gly-Gly стабильны (RMSD 0.028 / 0.101 Å) — `scripts/evm_pilot/REPORT.md` (мастер: `~/Desktop/Services/docs/ANALYSIS_2026-10-15_Anastasov_DeterministicPhysics.md`)
- [ ] **Следующий шаг:** пилот на коротком пептиде 3–5 остатков (напр. полиглицин) или Aβ(16–22) — минимальная модель агрегации белков (протеостаз)
- [ ] **Сравнительный тест:** та же система в GROMACS (классический MD) vs EVM — стабильность/геометрия
- [ ] **Ограничения учтены:** EVM не для долгих MD (дрейф RMSD) и не для энергий/переходных состояний; использовать как фильтр стабильности + генератор стартовых конформаций
<!-- /lang:ru -->

## 🔴 2026-10-05 — Аларт: центросомная наследственность эмбриона × ооцитарная точность
<!-- lang:ru -->
- [ ] **Анализ:** `docs/ANALYSIS_2026-10-05_CentrosomeInheritance_OocyteFidelity.md` (6 статей аларта + 12+ подтверждающих PMID; верифицировано 2026-10-05)
- [ ] **Эксперимент 4** (КРС-модель, ~$41K): перенос отцовской центриолярной энтропии в зиготу — polyGlu/окисление на проксимальной центриоли × возраст отца × качество делений (добавлен в EXPERIMENTAL_DESIGNS). Найти партнёра с КРС-IVF.
- [ ] **Новый фальсификатор:** «D≈0» зиготы — проверить, что зигота стартует с отцовской центриоли, а не с чистого листа (Uzbekov & Avidor-Reiss 2024, PMID 38442864)
- [ ] **Уточнение модели времени CEDAR:** центриоль = автономный осциллятор (Ow & Aydogan 2026, PMID 42782439) → дополнить S₀+β·t осцилляторной сопряжённостью (THEORY.md)
- [ ] **huoMTOC/aMTOC для «ацентриолярного окна»** протокола LERR: белки TACC3/CCP110/CKAP5/DISC1 (Wu 2022, PMID 36395215) — кандидаты на промежуточную стадию Rebuild
- [ ] Проверить стык с Экспериментом 2 (Центросомный IP-MS): состав центросомы зиготы старых vs молодых отцов
<!-- /lang:ru -->

## 🔴 2026-09-30 — Literature: oscillators & organelle hub (CEDAR)
<!-- lang:ru -->
- [ ] **Review file created:** `docs/LITERATURE_REVIEW_2026-09-30_Oscillators_OrganelleHub.md` (Ortiz-Álvarez & Al Jord 2026; Yin & Spradling 2026)
- [ ] Взять: de novo/деутеросомный путь как режим «сброса» vs шаблонная дупликация (статья 1, PMID 28982797, 38811726)
- [ ] Взять: QC числа центриолей (PIDDosome-p53, PMID 28130345) как отдельный контур
- [ ] Взять: MTOC-организация митохондрий как механизм Γ₁₃ (мост CEDAR↔MitoROS)
- [ ] Гипотеза: quiescence ооцитов ≠ накопление временнóй энтропии — калибровка division- vs time-компоненты
- [ ] Проверить: активация de novo-пути (PLK4) при Entropy Reset; возрастная динамика ацетилирования тубулина (HDAC6/ATAT1)
<!-- /lang:ru -->

## 🔴 2026-08-13 — Meyer-protocol → design article
<!-- lang:ru -->
- [ ] 📄 Submit `Testing_the_Hypothesis_of_Centriolar_Entropy...` as a design article (candidates: Cell Reports Methods / eLife / Nature Protocols-style)
- [ ] 🔬 Phase 0: GEO re-analysis (centriolar genes vs age) — first pilot signal
- [ ] 🔬 Phase 0: SILAC scaffold turnover + U-ExM passage 5 vs 25
- [ ] 📎 Link to protocol in CEDAR/docs/ and ERR/wp1_cedar/docs/
<!-- /lang:ru -->

## Active Tasks (2026-07-26)
### 🟢 2026-08-09 — bioRxiv preprint submitted
- [x] **bioRxiv BIORXIV/2026/743702** — Version 1 submitted (files: `articles/centriole-division-counter-biorxiv/`)
- [x] bioRxiv acknowledgment received (9 Aug, screening 24–72 h)
- [ ] Wait for bioRxiv screening → preprint + DOI 10.1101/...
- [ ] Consider bioRxiv → journal/Review Commons transfer (send_paper_away queue)
- [ ] After preprint: journal-fit.sh + pre-submission inquiry (MANDATORY) + journal submission



### 🔴 P0 — Critical
- [x] **rDNA clock (TRCS) integrated** — SenescenceTrigger::RdnDnaShortening + rdna_copy_number (v4.7, 2026-08-08, 547 tests pass)
- [x] **Publish CEDAR code** — GitHub (LC-public, tag v0.4.8-rdna-clock) + Zenodo DOI 10.5281/zenodo.21852388 (2026-08-08)
- [x] **Fix PARAMETERS.md** — replace generic template with centriole aging parameters (autofix cycle 1) — done 2026-07-26
- [x] **Fix MAP.md** — update to match current directory structure (autofix cycle 1) — done 2026-07-26
- [x] **Verification doc** — review VERIFICATION_CENTRIOLE_LAND_WATER_2026-07-26.md for integration into THEORY.md — **done 2026-08-15** (new section “Evolution of Centriole Loss: Land vs Water” in THEORY.md)
- [ ] **CIRCBIO-07 Part B** — draft due 1 Aug 2026 (5 days)

### 🟡 P1 — Important
- [ ] **LLPS section in THEORY.md** — add centrosome LLPS mechanisms (6 PMIDs verified)
- [x] **PARAMETERS.md** — add LLPS/centrosome section (done 2026-07-26)
- [ ] **npj Aging `2e8466c7`** — wait for peer review decision (since 12 Jun)
- [ ] **Article split** — CEDAR hypothesis (3000 words) → Differentiation
- [x] **DESIGN.md expansion** — architecture overview (done 2026-07-26)

### 🟢 P2 — If Possible
- [ ] **Git commit** — commit all autofix changes
- [ ] **Update STATE.md** — reflect PARAMETERS.md fix and MAP.md update
- [ ] **Dictyostelium note** — add to THEORY.md as centrosome-without-centriole model
- [ ] **Autofix script** — re-run until 95+ (currently 81, target 95)
- [ ] **Cell‑DT v4.0 calibration** — ep_age(t) coupling model

### 🟣 P3 — Future
- [ ] **Experimental design** — 13-group centriole elimination + reprogramming
- [ ] **Moss (Physcomitrella) centriole genomics** — comparative analysis
- [ ] **Ca²⁺/centrin → centriole** — literature review for THEORY.md

## Completed

### 2026-07-26
- [x] VERIFICATION_CENTRIOLE_LAND_WATER_2026-07-26.md — scientific verification of LLPS/centriole/land-water hypothesis
- [x] PARAMETERS.md — rewritten with centriole aging parameters (was: generic template)
- [x] MAP.md — updated to match current structure
- [x] DESIGN.md — expanded from 353 → 1700+ bytes
- [x] cedar_autofix.sh — CEDAR-specific autofix script created

### Earlier
- [x] BioEssays post-mortem analysis (MEMORY.md, 2026-07-15)
- [x] Peer Review v2 — 55 PMIDs verified (2026-07-05)
- [x] C1/C2 axioms + M1-M9 mechanisms (THEORY.md v5.6)
- [x] Chk1 molecular mechanism for M1 (PMID 41844775, 2026-07-25)
- [x] MCARA Biogerontology appeal filed (2026-07-15)
