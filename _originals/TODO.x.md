# LC · TODO

**Last updated:** 2026-04-28 (CONCEPT v5.6)

---

## Phase 0 — Umbrella core regeneration ✅ DONE 2026-04-28

- [x] Archive old core.md → `_archive/v_pre_2026-04-28/`
- [x] CONCEPT.md from article v5.6
- [x] THEORY.md, DESIGN.md, PARAMETERS.md, MAP.md (all regenerated)
- [x] EVIDENCE.md (verified PMID/DOI/arXiv only)
- [x] OPEN_PROBLEMS.md (cross-cutting open issues)
- [x] STATE.md, TODO.md (this), README.md
- [ ] Pin `md5sum ~/Desktop/LC.md` in CONCEPT.md §10 (after first commit)

---

## Phase 1 — Subproject CONCEPT cross-check (immediate, no code)

- [ ] Read `Ze/CONCEPT.md` — patch any "DERIVE" → "POSTULATE" if present
- [ ] Read `BioSense/CONCEPT.md` — ensure "hypothesis-stage exploratory" prominent in §1; weights post-hoc disclosed
- [ ] Read `FCLC/CONCEPT.md` — threat model uniformly "semi-honest server only; not active"
- [ ] Read `MCAOA/CONCEPT.md` — add M4 operational threshold (N≥2000, α=0.001, partial r²<0.05)
- [ ] Read `CDATA/CONCEPT.md` — status "inconclusive (Sobol p=0.12)" added
- [ ] (none of these block runtime; documentation alignment only)

## Phase 2 — Social layer code edits (per DESIGN §5)

### Server (Rust)
- [ ] `server/src/handlers/biosense.rs` add disclosure header
- [ ] `server/src/handlers/dashboard.rs` strings update
- [ ] `server/src/handlers/ze_guide.rs` system prompt updated (no medical advice; v1 NULL deprecated)
- [ ] new endpoint `/api/disclosures/v5_changes`
- [ ] `migrations/003_health_factors.sql` comment

### Web (React/TS)
- [ ] `web/src/pages/Dashboard.tsx` banner
- [ ] `web/src/pages/Studies.tsx` per-study disclosure
- [ ] `web/src/pages/Profile.tsx` χ_Ze tooltip
- [ ] `web/src/components/feed/PostComposer.tsx` DOI warn

### Realtime (Elixir)
- [ ] `realtime/config/dev.exs` port 4001 → 4500 (Ze conflict)
- [ ] BioSense channel disclosure metadata

### Deploy
- [ ] `deploy/docker-compose-all.yml` port + version env

## Phase 3 — Quality / governance

- [ ] CI: umbrella integration test (subprojects + social server + web)
- [ ] `scripts/regen_umbrella_core_from_article.sh` (helper)
- [ ] Mock layer for BioSense in social server tests
- [ ] Commit "Regenerate umbrella from article v5.6"

## Phase 4 — Strategic (grant + scientific)

- [ ] **2026-04-28 12:30** Joao Gonzalez meeting — coordinator confirmation (today, gating)
- [ ] **2026-05-01** Tsomaia Q1-Q6 freeze (E0 Phase 1 ordering)
- [ ] **2026-05-12** EIC Pathfinder Open deadline — NOT participating (deferred)
- [ ] **2026-08-15** all 3 EU-MS LoIs signed (Geiger ✅; Janke pending; Joao TBD)
- [ ] **2026-09-15** EIC Part B draft ready
- [ ] **2026-10-28** EIC Pathfinder Challenges submission
- [ ] **Q1 2027** FCLC v14 malicious-secure migration

## Phase 5 — Long-term scientific

- [ ] N≥2000 pre-registered cohort acquisition (UK Biobank wearable subset OR All-of-Us OR Аqтивиребули pilot)
- [ ] Cell-DT v4.0 build for full Sobol decomposition (CDATA)
- [ ] Swept-v* falsification protocol on N≥2000 (BioSense)
- [ ] Bridge to CDATA simplification (5 params → 2 params or theory-derived)
- [ ] EEGLAB/EDF Rust readers (BioSense Phase 2)

## Deferred / not now

- HAP / Ontogenesis — toxic; restoration requires complete EVIDENCE rebuild from verified PubMed
- Plotly upgrade for web charts
- Public deployment of subproject backends (TLS, auth, rate limiting)
- Hevolution NIA — requires Independent Asst/Assoc Prof appointment (Tqemaladze не подходит)
- Stipendium Hungaricum — call November 2026
