# Aubrey — 6-Month Centriole Age Tracking Platform

> **⚠️ АППАРАТНАЯ ЧАСТЬ ОБНОВЛЕНА (2026-06-27).**  
> **v1.0 (ниже, 2026-05-15):** 100× Oil, sCMOS ORCA-Flash4, RTX 4090, Claude/Gemini API, BSL-2, €13,850/ед.  
> **v2.0 (актуальная):** 40× сухой / 60×/1.2 Water Immersion, RasPi 5 + SSD снаружи, LED/лазеры, BSL-1, $2,045–5,945.  
> **Актуальный BOM и компоновка:** `~/Desktop/Marketing/ARGUS-LP/docs/correspondence/ARGUS_LP_components_attachment_2026-06-27.md`  
> **Актуальные параметры:** `PARAMETERS.md` (обновлён 2026-06-27)  
> **Научная методология (pre-registration, sample size, phototoxicity budget, blinding, controls, go/no-go) — сохранена и актуальна.**  
> **Данный CONCEPT сохраняется для научного содержания; аппаратные спецификации в секциях §Hardware, §Budget — исторические (v1.0).**

**Formal grant title (per signed LoS Parrish 2026-04-22 and Geiger 2026-04-23):** *AI-Directed Pure-Lineage Test of Counter #1 (Centriolar) in the Multi-Counter Architecture of Organismal Aging (MCARA)*

**Phase A equipment name:** **ARGUS-LP** — *AI-Resident Robotic Genealogical Ultra-surveillance for Lineage Purification.* A purpose-built COTS-assembled live-cell imaging station (not a Zeiss retrofit), designed from scratch for 24/7 AI-agent-driven continuous live-cell observation over the 6-month tracking window. The Argus mythological reference is intentional: 24/7 multi-month autonomous surveillance of a slowly-drifting cellular phenotype is a function that no human-operated workflow can deliver. ARGUS-LP is Phase A's capital deliverable; it remains the property of GLA at the end of the grant. Two identical units are built (see §Budget): Unit #1 at GLA Abastumani, Unit #2 at an independent replication site (Zheleznov, LoS pending). Engineering subcontractor  handles design and assembly.

**Parent project:** [CytogeneticTree (CellLineageTree)](../CONCEPT.md)
**Created:** 2026-05-09; **scope revised 2026-05-12: from 48 h pilot → 6-month tracking programme** aligned with the parent project's quantitative predictions P5 (50-passage polyGlu trend; reachable in ~6 mo at 1 division/day from passage 25 → 50) and partial coverage of P11 (relapse half-life ~40–60 divisions; full 80–120 divisions in Phase B). The 48 h figure is retained only as the *duration of one continuous time-lapse imaging block*, not the duration of the whole experiment.
**Status:** Pre-funding proposal stage. No pilot data exist yet; this CONCEPT describes the planned design.


## Статус
🟡 В разработке. Детальная грантовая заявка — см. `docs/Aubrey_Platform_full_grant.md`.

## Суть
Аппаратно-программная платформа для лазерной абляции центриолей в живых клетках (ARGUS — Ablative Removal of Granular Units for Senescence). Часть CEDAR (Counter #1 MCARA).

## Ключевые компоненты
- Лазерная абляция с субклеточным разрешением
- Микрофлюидика для долговременного наблюдения
- Automated microscopy + computer vision (CenFind)
- Анализ асимметричного наследования центриолей

## Команда и партнёры
См. `docs/Aubrey_Platform_full_grant.md`:
- Pierre Gönczy (EPFL) — центриоли
- Wolfgang Wagner (UKA Aachen) — эпигенетические часы
- + TBD
