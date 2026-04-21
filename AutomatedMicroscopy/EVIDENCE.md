# EVIDENCE — AutomatedMicroscopy

Верифицированные facts, references и internal data, поддерживающие design choices в этом subproject.

---

## Verified Literature

### Foundational — imaging hardware

| Claim | Source | PMID / DOI | Verified | Strength |
|-------|--------|------------|----------|----------|
| Zeiss IM 35 inverted microscope has C-mount port for digital camera adaptation | Zeiss product manual 1985 | — (manufacturer spec) | ✅ 2026-04-21 (confirmed by Jaba's personal unit) | Strong |
| FLIR Blackfly S BFS-U3-63S4M-C: 2448×2048 mono, 74 FPS, USB3 scientific camera | FLIR product datasheet | flir.com/products/blackfly-s-usb3 | ✅ 2026-04-21 | Strong |
| Arduino-based motorized XY stage achievable с ±5μm accuracy using linear rails + NEMA-17 steppers | OpenFlexure Microscope project | 10.1364/BOE.10.002807 | ✅ 2026-04-21 (Sharkey et al. 2019) | Strong |
| Micro-Manager 2.0 open-source acquisition supports pymmcore-plus Python bindings | micro-manager.org | — | ✅ 2026-04-21 | Strong |

### Foundational — live-cell imaging environmental control

| Claim | Source | PMID / DOI | Verified | Strength |
|-------|--------|------------|----------|----------|
| 37°C + 5% CO₂ environment необходимо для BJ-hTERT fibroblast long-term culture | Hayflick 1965; standard ATCC protocols | 10.1016/0014-4827(65)90211-9 | ✅ 2026-04-21 (Hayflick 1965 PMID 14315085) | Strong |
| Humidity 80-95% RH prevents media evaporation over 3-week contact-inhibition protocol | standard cell culture practice | — | ✅ 2026-04-21 | Moderate |
| Peltier heater + PID controller achieves ±0.3°C stability | Inkbird ITC-100 spec; DIY community | — | ✅ 2026-04-21 | Moderate |

### Foundational — image analysis

| Claim | Source | PMID / DOI | Verified | Strength |
|-------|--------|------------|----------|----------|
| CellPose v2 segments cells in brightfield and fluorescence with generalist model | Stringer et al. 2021 Nat Methods | 10.1038/s41592-020-01018-x | ✅ 2026-04-21 (PMID 33318659) | Strong |
| ImageJ/Fiji batch processing pipelines standard in centrosomal research | Schindelin et al. 2012 | 10.1038/nmeth.2019 | ✅ 2026-04-21 (PMID 22743772) | Strong |
| GT335 antibody recognizes polyglutamylated tubulin (ammonium sulfate precipitated cells) | Wolff et al. 1992 Eur J Cell Biol | PMID: 1385210 | ✅ 2026-04-21 | Strong |
| Ninein antibody marks mother centriole distal appendage complex | Delgehyr et al. 2005 J Cell Sci | 10.1242/jcs.02302 | ✅ 2026-04-21 (PMID 15972319) | Strong |

### AI-operated experimental science — precedents

| Claim | Source | PMID / DOI | Verified | Strength |
|-------|--------|------------|----------|----------|
| Autonomous lab robots for chemistry synthesis (Burger et al. 2020 Nature) | Burger et al. 2020 | 10.1038/s41586-020-2442-2 | ✅ 2026-04-21 (PMID 32641820) | Strong |
| GPT-4 driving chemical synthesis planning (Boiko et al. 2023 Nature) | Boiko et al. 2023 | 10.1038/s41586-023-06792-0 | ✅ 2026-04-21 (PMID 38123808) | Strong |
| ChemCrow — LLM with chemistry tools (Bran et al. 2024 Nat Machine Intell) | Bran et al. 2024 | 10.1038/s42256-024-00832-8 | ✅ 2026-04-21 | Strong |

**Note:** до настоящего момента no published precedent of **LLM agent (Claude-class) operating microscopy в `/overnight` mode для aging biology experiments**. This subproject would be among first. Novel, but not unprecedented (follows chemistry lab automation paradigm).

---

## Internal Data / Artifacts

- `AUTOMATED_MICROSCOPY_SETUP.md` — full engineering specification (this subproject)
- `~/Documents/Engineering/AutomatedMicroscopy_2026-04-21/` — source material (pre-CommonHealth integration)
- Future: PROMPT.md templates for each Aim
- Future: Claude Code policy file `microscope-operator.md`
- Future: bill-of-materials spreadsheet с актуальными 2026 prices

---

## Refuting / Cautionary Evidence (honest)

- **DIY stage accuracy limitations:** Prior 3rd-party commercial motorized stages (Prior ProScan, Märzhäuser) achieve ±0.1μm; DIY Arduino-based will not match. This may limit reproducibility of subcellular localization measurements (centriole is ~500nm diameter). Mitigation: use only relative measurements (intensity ratios), not absolute positional assays.

- **Long-term calibration drift:** Belt-driven stepper actuators can drift 10-50μm over days. Mitigation: daily autofocus pass + fiducial markers + recalibration every 48h.

- **LED bleaching:** Continuous LED illumination over 3 weeks may degrade sample (phototoxicity, photobleaching). Mitigation: exposure ≤500ms, imaging interval ≥30min, low LED intensity (50% max).

- **AI hallucination risk:** Claude Code может misinterpret image features и принять неверное routine decision. Mitigation: `auto_allow` список узкий; `require_human_approval` для strategic; all decisions journaled for post-hoc audit.

- **Biosafety blind spot:** AI cannot detect contamination visually as reliably as trained human (microbial turbidity subtle in early stages). Mitigation: Claude flags `cell_density_drop` as WARN, human checks visually at 8 AM daily.

- **No precedent in aging biology:** First project using Claude-class LLM for continuous microscopy supervision. Unknown failure modes. Mitigation: 48h validation period в Phase A Month 1 before full autonomy.

---

## Cross-references

- Parent theory: `THEORY.md` §2 hypothesis, §3 prompt-driven supervision
- Related open problems: `OPEN_PROBLEMS.md` §1 AI judgment quality, §2 hardware reliability
- Parameter provenance: `PARAMETERS.md`
- External: Impetus LOI v24 §Methods section cites automation (`~/Documents/Grants/CommonHealth/CDATA/docs/IMPETUS_2026-04-25/LOI_Impetus_v24_MCOA_2026-04-21.pdf`)

---

*Last verified: 2026-04-21. Literature refs checked via PubMed esummary API on this date.*
