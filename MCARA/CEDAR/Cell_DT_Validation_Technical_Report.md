# Cell-DT / CEDAR — Validation Status and Technical Report

**Date:** 2026-10-26
**Author:** Jaba Tqemaladze (Georgia Longevity Alliance)
**Repositories:** github.com/djabbat/LC-public (platform, MIT) · github.com/djabbat/CASCADE (counter model, Apache-2.0)
**Purpose:** To make the public record accurate — what is implemented, what is calibrated against documented anchors, and which quantitative claims are not yet documented in open materials.

---

## 1. Scope

This report accompanies the Cell-DT computational aging model (multi-tissue simulator of organismal aging). It separates three categories that are often conflated in academic summaries: (i) implemented open-source functionality, (ii) calibration that is documented with verifiable anchors, and (iii) quantitative results that have been stated in CV material but for which no source dataset or analysis is present in the public repositories.

## 2. What is implemented and open-source

- **Multi-tissue modular simulator (Rust):** tissue / counter / organism layers; five molecular aging counters (MCARA); inter-tissue Ze-conflicts; Bayesian learning loop (MCMC/NUTS); sensitivity analysis; CLI (`oa`), provenance audit, species and intervention modules. — `github.com/djabbat/LC-public`
- **Centriole counter model (CEDAR/CASCADE):** centriolar division-counter hypothesis, audit pipeline with calibration membrane. — `github.com/djabbat/CASCADE` (Apache-2.0)
- **Validation framework:** `cell_dt_validation` crate — `ValidationResult` (r_squared, rmse, mae, n_points), `ValidationSuite` (mean_r2), acceptance thresholds (strictly > 0.75), per-biomarker evaluation. Synthetic reference datasets are derived from published literature (ages 20–80). — path in LC-public docs (CASCADE/CALIBRATION.md; cell_dt_validation).

## 3. Calibration to date (documented, literature-anchor based)

Source: `CASCADE/CALIBRATION.md` (2026-10-17). Parameters are anchored to human data, with explicit ranges where an anchor constrains only a window:

| Parameter | Value | Human anchor | PMID |
|---|---|---|---|
| c_tm (mtDNA/ROS → telomere) | 4.5 | telomere erosion 90 bp/PD → >500 bp/PD at 40% O₂ (≥5.5×) | 7664835 (von Zglinicki 1995) |
| k_c (centriole threshold) | 31 | audit-fitted within S_c(50)=30 < 31 < S_c(200)=120 | model/audit |
| HSC pool erosion | ~0.001/yr | ~7% pool loss by age 75 | MCARA anchor |
| mtDNA deletions (d_m, k_m) | 0.5, 25 | ~98× rise 20→80 yr in human skeletal muscle | 32965609 (Herbst 2021) |
| Epigenetic mitotic-age counter (e_drift, k_e) | 0.4, 25 | E = e·n linear in divisions, normal/pre-cancerous human tissues | 38760334 (Teschendorff 2024); 39289103 (Zhou & Reizel 2025) |
| Proteostasis (d_p, r_p, k_p) | 0.4, 0.05, 8 | proteasome subunit and proteolysis decline with age; mitochondrial+proteasomal dysfunction | 17317272 (Ponnappan 2007); 29652618 (Cho 2018) |
| Coupling coefficients (c_mt, c_et, c_pe, c_mp) | 0.1, 0.1, 0.02, 0.05 | directions confirmed | 21307849 (Sahin 2011); 23746838 (López-Otín 2013) |

**ABC / grid fit (2026-10-17):** 19,683 parameter combinations against five human anchors; 100% accepted (mean n* = 32.3), i.e. the literature-derived parameter ranges are consistent with the anchors without point-fitting. A full point-fit requires fixed units and divisions-per-year per tissue.

## 4. Claims NOT documented in the public repositories (status: author-reported)

The following figures have appeared in academic CV material but, as of 2026-10-26, **no source dataset, analysis script, or result file corresponding to them is present in the public repositories** (LC-public, CASCADE) or in the accompanying documentation:

- Calibration on **62,000 patient records** (R² ≈ 0.89);
- Validation on independently assembled cohorts aged 60–100 (R² ≈ 0.84 for frailty, mortality and epigenetic-clock outcomes);
- Blind prediction of the mean lifespan of a cohort of Italian centenarians within 1.6 years.

The public record contains a validation *framework* and *synthetic* datasets, but no 62,000-record calibration run, no independent-cohort analysis, and no centenarian-prediction result.

**Status of these three claims: claimed by the author; not independently verifiable from open materials.**

**Required action (either):** (a) publish the underlying dataset and analysis so the figures become reproducible, or (b) withdraw the figures from public claims until they are documented. This report exists precisely to keep the public record consistent with verifiable evidence.

## 5. Honest summary table

| Claim | Status | Where documented |
|---|---|---|
| Multi-tissue architecture (5 counters, Ze-conflicts) | Implemented, open-source | LC-public |
| Literature-anchor calibration | Documented | CASCADE/CALIBRATION.md |
| Validation framework + synthetic datasets | Implemented, open-source | cell_dt_validation (LC-public docs) |
| Calibration on 62,000 records (R² ≈ 0.89) | Author-reported; no public source | NOT documented |
| Independent cohorts aged 60–100 (R² ≈ 0.84) | Author-reported; no public source | NOT documented |
| Centenarian lifespan prediction (within 1.6 yr) | Author-reported; no public source | NOT documented |

## 6. Next steps

1. Publish the dataset and analysis behind the claimed figures, or remove them from CV/grant material.
2. Complete the large-scale GTEx/UK Biobank calibration (currently Phase 4, planned) and publish its report.
3. Keep the evidence base (30+ PMIDs) current and DOI-verifiable.
