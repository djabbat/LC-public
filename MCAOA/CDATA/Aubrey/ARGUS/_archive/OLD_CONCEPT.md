# CONCEPT.md — ARGUS (AI-Resident Robotic Genealogical Ultra-surveillance for Lineage Purification)

**Version:** 4.1 (2026-05-15, post TBPR cycle-9 — added budget, WP, risk matrix; removed embedded rebuttal)
**Status:** Construction phase — **simulator-only**, no live cells

## 0. MCAOA context

ARGUS operates within the **Multi-Counter Architecture of Organismal Aging (MCAOA)** framework (Tkemaladze J. 2026, DOI: 10.5281/zenodo.20055806). The centriolar **Counter #1** is one of the key limiters of stem cell replicative potential; ARGUS is the empirical validation tool for its predictions.

*Note: MCAOA is used here as a biological-motivation context only. Phase A is pure engineering — no living cells, no MCAOA validation claims.*


## 1. Scope — Construction Phase

### ✅ In scope

- **Simulator**: Kaede-beads ($300-500) or fluorescent FluoSpheres ($250)
- **Fluorescence imaging YES** — but **on simulator**, not on cells
- **AI agent**: Claude Code /overnight orchestration on simulated targets
- **Mechanical calibration**: XY stage repeatability on static beads
- **Optical alignment**: dichroic + bandpass on known fluorescent beads
- **Safety interlock**: hardware kill, OD 4+ goggles, UPS, **Physical Beacon** (§11)

### ❌ Out of scope for construction phase

- NOT live cells (BJ-hTERT, iPSC, Elodea)
- NOT cell staining (RITE, IF — postponed to Aubrey biology phase)
- NOT laser ablation on biological targets
- NOT biology claims (Impetus / EIC Pathfinder positioning — separate proposal)


## 2. Biological motivation (context, not scope)

Centriole inheritance is tissue-specific, motivating the need for a precision ablation tool for future centriole biology studies [PMID: 37882444].

**Validation of centriole ablation approach (justification for Phase B):**

| Study | Key finding | Relevance to ARGUS |
|-------|-------------|---------------------|
| **La Terra S. et al. (2005) *Mol Biol Cell* PMID: 15738265** | Centriole removal in HeLa cells **does not block cell cycle**; cells assemble new centrioles *de novo* | Confirms ablation is not a lethal intervention in transformed cells |
| **Uetake Y. et al. (2007) *J Cell Biol* PMID: 17227892** | Normal human cells progress through G1 without centrioles and assemble them *de novo* | Justification for working with normal fibroblasts (BJ-hTERT) in Phase B |
| **Royall LN et al. (2023) *eLife* PMID: 37882444** | Asymmetric inheritance of the **older** mother centriole is required for human neural progenitor self-renewal | Central justification: ARGUS must distinguish old vs new centrioles |
| **Tkemaladze J. (2023) *Mol Biol Rep* PMID: 36583780** | Old centrioles are potentially the primary aging structure; during asymmetric stem cell division, the daughter cell retaining stem cell potential selectively conserves old (mother) centrioles | Theoretical justification for MCAOA counter #1 |


## 3. Verified PMIDs (2026-05-15 — full verification via PubMed)

| # | PMID | Article | Key finding for ARGUS |
|---|------|---------|----------------------|
| 1 | **36583780** | Tkemaladze J. *Mol Biol Rep* 2023 | CDATA Counter #1 — selective inheritance of old centrioles by stem cells |
| 2 | **37882444** | Royall LN et al. *eLife* 2023 | Asymmetric centriole inheritance in human neural progenitors |
| 3 | **16336191** | Tkemaladze & Chichinadze 2005 *Biochemistry (Mosc)* | Centriolar mechanisms of morphogenesis |
| 4 | **15886028** | Tkemaladze J. 2005 *Cell Biol Int* | Determination of morphogenetic status |
| 5 | **21407209** | Januschke J. et al. 2011 *Nat Commun* | Differential PCM regulation of mother and daughter centrioles in Drosophila |
| 6 | **40243666** | Schaeffer A. et al. 2025 *J Cell Biol* | Microtubule and actomyosin positioning |
| 7 | **17227892** | Uetake Y. et al. 2007 *J Cell Biol* | *De novo* centriole assembly after ablation in normal human cells |
| 8 | **15738265** | La Terra S. et al. 2005 *Mol Biol Cell* | Centriole ablation does not block cell cycle in HeLa |
| 9 | **22683192** | Pelletier L. et al. 2012 *Curr Opin Cell Biol* | Mother-daughter centriole asymmetry in development |
| 10 | **25047620** | Reina J. et al. 2014 *Philos Trans R Soc B* | Link between centriole age and cell fate |
| 11 | **33435817** | Chen C. et al. 2021 *Open Biol* | Centriolar asymmetry in asymmetric stem cell divisions |
| 12 | **25883936** | Avidor-Reiss T. et al. 2015 *Front Cell Dev Biol* | Atypical centrioles in reproduction |
| 13 | **34440763** | — *Front Cell Dev Biol* 2021 | Centriolar asymmetry in early Drosophila oogenesis |
| 14 | **28562636** | — *PLoS One* 2017 | Systematic review of asymmetric organelle inheritance |

**Removed (fabricated):** ~~38015348~~, ~~38353211~~ (not Tkemaladze, not centriole biology)


## 4. PI publications — scope of self-citation

The PI's own work is referenced only where directly relevant:
- **PMID 36583780** — CDATA Counter #1 conceptual framework (used for biological motivation in §2)
- **MCAOA preprint** (Zenodo DOI: 10.5281/zenodo.20055806) — acronym definition only (§0), no experimental claims
- All other references are independent third-party publications (see §3)


## 5. Statistical Protocol (Phase A commissioning)

**Primary endpoint:** XY stage repeatability
- Expected SD: ±2 µm (manufacturer spec)
- Margin of non-inferiority: ±5 µm vs spec
- **Power calculation:** n=50 measurements (α=0.05, β=0.2, effect size 1.5σ)

**Secondary endpoint (updated according to biological justification from Royall 2023 and Tkemaladze 2023):**
- **Discrimination between 'old' (photobleached) and 'new' (fresh) centriole mimetic beads**
- Target: ≥95% correct classification on ≥500 simulated ablation cycles
- **Power calculation:** n=500 cycles, α=0.05 (Holm-Bonferroni corrected), power 0.9 for detecting 5% deviation from 95%

**Tertiary endpoint (new — CUSUM AI quality control chart):**
- CUSUM (Cumulative Sum) control chart for monitoring AI performance drift
- If accuracy falls below 93% on a sliding window of 50 cycles → automatic stop test
- **Acceptance:** 0 crossings of the lower control limit over 6 months

**Blinding protocol:**
- Operator blinded to experimental condition
- AI agent receives no metadata about condition
- 10% of cycles reviewed by second AI model (cross-check)

**Meta-analytic note:** According to a systematic review of asymmetric organelle inheritance [PMID: 28562636], most studies (77%) have unclear or high statistical reliability due to low numbers of technical replicates (<10). ARGUS Phase A exceeds this standard, offering n=500 cycles and formal protocol pre-registration.


## 6. Laser specification

| Parameter | Phase A (simulator) | Phase B (biology) | Status |
|-----------|---------------------|-------------------|--------|
| Wavelength | 450 nm CW | **355 nm Q-switched** | Phase B upgrade |
| Pulse energy | N/A | >1 µJ | Required |
| Spot size | ~50 µm | ~200 nm (diffraction-limited) | Requires UV objective |
| Cost | $500 | $15,000-20,000 | Budgeted |


## 7. Modern motor stack

| Legacy (rejected) | Modern (adopted) | Why |
|-------------------|------------------|-----|
| Arduino Nano + L298N | ESP32-S3 + TMC2209 | OTA updates, silent step |
| Open-loop steppers | Closed-loop MKS SERVO42 | Position feedback |
| Single MCU PWM-only | FreeRTOS on STM32/RP2040 | Deterministic timing |
| Manual GRBL config | Klipper / Reach firmware | Lookahead motion planning |


## 8. Team

| Role | Name | Institution | Responsibility |
|:----:|------|-------------|---------------|
| **PI** | Jaba Tkemaladze, MD, PhD | Georgia Longevity Alliance | System architecture, AI agent integration, hardware assembly, project lead |

**No external Co-PIs or partners are required for ARGUS Phase A.** All engineering is executed by the PI with AI-assisted tooling (Claude Code for firmware/Python scaffolding, Gemini for vision). GLA provides lab space and administrative support in-kind.


## 9. Grant context — external commitments (Phase B downstream)

The following signed Letters of Support are held for the **Aubrey biology program** (Phase B downstream of ARGUS). They are not part of this ARGUS Phase A application:

- **BioViva Sciences LLC (CEO Elizabeth Parrish)** — Signed LoS 2026-04-22. Parrish serves as **Advisor & Co-PI on the Aubrey program** (strategic oversight, regulatory guidance, commercialisation pathway for CCP1/AGBL5 therapeutic targets). No role in ARGUS Phase A engineering.
- **Prof. Hartmut Geiger (Univ. Ulm)** — Signed LoS 2026-04-23. Phase B subcontracted in vivo HSC work (€90K), conditional on ARGUS Phase A Go decision.

ARGUS Phase A is fully self-contained. These commitments become active only upon successful commissioning (Phase A → Aubrey transition).


## 10. Execution credibility — why a theorist can build this

ARGUS Phase A is an **engineering project assembled from off-the-shelf components**, not a novel hardware invention. The execution risk is low because:

1. **Standard parts, standard protocols** — ESP32-S3 (Arduino-compatible), TMC2209 (documented driver), MKS SERVO42 (closed-loop with config), 450 nm laser (TTL-modulated diode), ToupCam (USB camera with µManager plugin). Every component has open-source example code and active community support.
2. **AI-assisted development** — Claude Code handles firmware scaffolding, Python API wiring, and debug. PI reviews and tests. This is the standard workflow for solo hardware developers in 2026.
3. **Open-source reuse** — µManager for camera control, FreeRTOS for ESP32, Klipper for motion planning. No code written from scratch.
4. **6-month timeline with acceptance gates** — each WP has a clear go/no-go criterion (see §15). If any gate fails, the project stops.

**What Phase A does NOT require:** custom PCB design, FPGA programming, wet-lab cell culture, regulatory approval, novel optical design. All engineering risk is contained in integration — which is precisely what is being validated.

*For Phase B (biology), the PI will collaborate with the Geiger lab (Ulm) which has established iPSC and imaging infrastructure. Phase A is the prerequisite proving that the tool physically works.*

## 11. Honest comparison with existing autonomous-imaging stacks

| Existing tool | What it does | Why ARGUS is distinct |
|---------------|--------------|----------------------|
| **μManager** | Z autofocus for timelapse | Reused; ARGUS adds ablation-gating layer |
| **ImSwitch** [PMID: 33677484] | Modular Python control for super-resolution | ARGUS adds photoconvertible-event-driven ablation |
| **AutoPilot** [PMID: 30038365] | Adaptive imaging on light-sheet | No ablation channel |
| **OpenFlexure** [PMID: 27782608] | Open-hardware standalone microscope | ARGUS uses custom inverted microscope (Tsomaia design) |

*Note on contemporary analogues (EAA Argonne 2026, Osaka SLM Agent 2026): These systems are described in the literature as preprints or personal communications; their engineering solutions are not the subject of this proposal. ARGUS is unique in its focus on 355 nm UV ablation for centriolar targets.*


## 12. Single-configuration calibration scope

**Phase A fixed configuration:** custom 40× objective (Tsomaia design), Semrock dichroic FF497-Di01-25×36, emission bandpass FF01-525/45. All acceptance criteria are measured only on this configuration. Multi-config validation is outside Phase A.


## 13. Physical Beacon — hardware blocking of AI hallucinations

**Problem:** AI may hallucinate — "seeing" a target on an empty slide.

**Solution:** A **Physical Beacon** is placed in the camera's field of view — an LED with known flashing frequency (10 Hz, encoded signal).

**Hardware blocking rule:** The laser can be activated **ONLY** if:
1. AI detects a target for ablation AND
2. Beacon is detected in the correct position AND
3. Beacon flashing frequency matches calibration

**If beacon is not visible → hardware laser kill, independent of AI decision.**


## 14. AI Brain Stack + Constitution

| Role | Primary | Backup |
|------|---------|--------|
| Strategic reasoning | DeepSeek `deepseek-reasoner` | deepseek-chat |
| Vision | Gemini 2.5 Flash | Llama 3.2 90B Vision |
| Code generation | Claude Code | DeepSeek chat |
| Real-time orchestration | Claude Code /overnight | — |

**Constitution for AI Scientists (prohibited actions):**
- Change laser power by more than ±10% from calibration
- Disable Physical Beacon check
- Change safety parameters (timeouts, motor current)
- Execute code without static analysis (bandit/Pyre)
- Continue operation under any failure mode from FMEA


## 15. Phase-graduated scenario

| Phase | Sample | Cost | Goal |
|-------|--------|:----:|------|
| 0a | TetraSpeck beads | $280 | Optical alignment |
| 0b | FluoSpheres 488/520 | $250 | Fluorescence baseline |
| 1 | Kaede-beads simulator | $300-500 | AI/mechanical calibration |
| 2 | Phase A commissioning complete | included | Ready for biology |
| 3 | Aubrey biology phase (separate) | $3K+ | BJ-hTERT + Centrin1-Kaede |

## 16. Budget Table (Phase A)

| Category | Item | Cost |
|----------|------|:----:|
| **Optics** | TetraSpeck beads (T7279) | $280 |
| | FluoSpheres 488/520 | $250 |
| | Semrock dichroic FF497-Di01 | $395 |
| | Emission bandpass FF01-525/45 | $295 |
| **Laser** | 450 nm 500 mW CW TTL module | $60 |
| | Laser safety goggles OD4+ | $25 |
| **Camera** | ToupCam E3CMOS05000KMA mono (IMX264) | $394 |
| | Custom C-mount adapter | $56 |
| **Stage** | LGY40-C (already owned) | $50 |
| | NEMA-11 stepper + TMC2209 driver | $35 |
| | MKS SERVO42 closed-loop | $85 |
| | Flex couplers + endstops | $15 |
| **Control** | ESP32-S3 DevKit | $12 |
| | Sensors (DS18B20, BPW34, IRLZ44N, reed) | $18 |
| **Infrastructure** | ACP 3mm black + Al profile | $100 |
| | Noctua 120mm fan | $25 |
| | UPS SMT1500 (renewed) | $299 |
| | External HDD 4TB | $80 |
| **AI compute** | DeepSeek API (6mo ~500 calls/day × $0.0005) | $450 |
| | Gemini 2.5 Flash API (vision ~100 calls/day) | $180 |
| **Contingency** | 15% | $420 |
| **Total** | | **$3,524** |

## 17. Work Package structure

| WP | Task | Months | Deliverable | Dependencies |
|:--:|------|:------:|-------------|:-------------:|
| **1** | Procurement | 1 | All hardware delivered | None |
| **2** | Enclosure + LED retrofit | 2 | Light-tight box, Köhler aligned | WP1 |
| **3** | Motorized stage assembly | 2-3 | XY stage ±5 µm repeatability | WP2 |
| **4** | ESP32-S3 firmware + Physical Beacon | 3 | Interlock + beacon decoder tested | WP3 |
| **5** | Python API + CUSUM monitoring | 3-4 | Tool functions, control chart | WP4 |
| **6** | AI agent integration (Claude Code) | 4-5 | 1000-cycle dry run | WP5 |
| **7** | 6-month commissioning session | 5-10 | All §19 acceptance criteria met | WP6 |
| **8** | Analysis + reporting | 10-12 | arXiv preprint, GitHub release | WP7 |

## 18. Risk Matrix

| Risk | P | I | RPN | Mitigation |
|------|:-:|:-:|:---:|------------|
| ESP32-S3 firmware crash during 6-month run | 3 | 4 | 12 | Watchdog 500 ms + systemd auto-restart |
| AI hallucinates target on empty slide | 2 | 5 | 10 | Physical Beacon hardware block |
| DeepSeek API outage | 3 | 3 | 9 | Fallback: Llama 3.2 90B via Groq (~3s switch) |
| Stage drift >±5 µm over 6 months | 2 | 4 | 8 | Weekly cal + MKS SERVO42 closed-loop feedback |
| CUSUM false alarm (drift below 93%) | 2 | 3 | 6 | Adjustable LCL; human operator review |
| Camera failure | 1 | 4 | 4 | Redundant Pi HQ camera path |
| Physical Beacon LED burnout | 1 | 3 | 3 | Redundant LED; test at WP4 commissioning |
| Power outage > UPS runtime | 1 | 4 | 4 | Ordered shutdown; auto-resume on power return |

**P** = probability (1-5), **I** = impact (1-5), **RPN** = P × I. All RPNs < 15 — no critical risks.

## 19. Acceptance criteria (Phase A → graduation)

1. ✅ SNR ≥ 5× background on Kaede-beads
2. ✅ XY stage repeatability ±5 µm (n=50, Holm-Bonferroni)
3. ✅ **Discrimination accuracy ≥95%** old vs new beads (≥500 cycles)
4. ✅ **CUSUM:** 0 crossings of 93% boundary over 6 months
5. ✅ **Physical Beacon:** 10+ stress tests (beacon hidden → laser blocked)
6. ✅ Safety interlock FMEA: RPN < 100
7. ✅ 6-month uptime ≥99%

---

*CONCEPT.md v4.3 — 2026-05-15. TBPR cycle-11 fixes: Parrish moved to §9 (Grant context — Phase B downstream). Team §8 is PI-only. No external Co-PIs in ARGUS Phase A.*
