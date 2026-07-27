# CEDAR — Parameters

**Version:** 1.0 | **Updated:** 2026-07-26

## Centriolar Damage Accumulation Model Parameters

### Core Model Parameters (Cell‑DT v3.0)

| Parameter | Symbol | Value | Units | Source |
|-----------|--------|-------|-------|--------|
| HSC division rate | α_HSC | 0.0082 | yr⁻¹ | Round-7 MCMC posterior |
| HSC annual divisions | ν_HSC | 1.2 | yr⁻¹ | Fitted to BrdU data |
| Baseline damage rate | β_HSC | 0.005 | damage/division | Cell‑DT calibration |
| Protection period | τ_protection | 24.3 | yr | MCMC fitted |
| Baseline epigen rate | ep_rate_base | 0.01 | ep_unit/yr | MCMC pilot |
| Epigen-coupling coefficient | k_ep | 0.8 | ep_unit/damage | Analytical coupling |
| Initial protection fraction | π_0 | 0.87 | dimensionless | MCMC posterior |
| Baseline protection fraction | π_base | 0.10 | dimensionless | Literature estimate |

### Coupling Parameters (v4.0, planned)

| Parameter | Symbol | Range | Units | Status |
|-----------|--------|-------|-------|--------|
| Epigen-coupling strength | γ_epi | [0, 0.05] | dimensionless | Default: 0 |
| Coupling k_ep range | k_ep | [0.5, 2.0] | dimensionless | To calibrate |

### Sobol Sensitivity (MCARA Counter #1)

| Parameter | First-order S₁ | Total-effect S_T | Rank |
|-----------|:--------------:|:----------------:|:----:|
| π_0 (initial protection) | 0.42 | 0.58 | 1 |
| α_HSC (division rate) | 0.23 | 0.35 | 2 |
| β_HSC (damage rate) | 0.15 | 0.22 | 3 |
| τ_protection | 0.08 | 0.12 | 4 |
| ep_rate_base | 0.05 | 0.08 | 5 |

### Model Performance

| Metric | Value | Target |
|--------|-------|--------|
| In-sample R² (MCAI) | 0.745 | >0.7 ✅ |
| LOO-CV mean | −0.093 | >0 ⚠️ |
| Calibration needed | ROS equation fix + coupling | v4.0 |

## LLPS / Centrosome Parameters (2026-07-26)

| Parameter | Symbol | Value | Units | Source |
|-----------|--------|-------|-------|--------|
| Cep63-Cep152 KD for LLPS | Kd | ~μM | M | PMID 33208041 |
| Centriole disengagement time | τ_disengage | ~1-2 | hr | PMID 20861312 |
| Ca²⁺ binding affinity (centrin) | Kd_Ca | ~10⁻⁷ | M | PMID 17694534 |
| Centriole number control | n_centrioles | 2/cell (G1) | count | Canonical |

## Evolution Parameters (Comparative Biology)

| Organism | Centrioles | MTOC | Cell wall | Habitat |
|----------|:----------:|:----:|:---------:|---------|
| H. sapiens | ✅ (2) | Centrosome | ❌ | — |
| Chlamydomonas | ✅ | Basal body | Thin (glycoprotein) | Freshwater |
| Physcomitrella (moss) | ✅ (gametes only) | Acentrosomal | ✅ (cellulose) | Terrestrial |
| Arabidopsis | ❌ | Acentrosomal (Ran-GTP) | ✅ (cellulose) | Terrestrial |
| Dictyostelium | ❌ | Centrosome-like core | ❌ | Freshwater/Soil |
| S. cerevisiae | ❌ | SPB (no centrioles) | ✅ (chitin) | — |

## Grant Parameters (CIRCBIO-07 / EIC)

| Parameter | Value |
|-----------|-------|
| Budget total | €2.0M |
| Duration | 36 months |
| Partners | INRAE + Wageningen + DTU + TSU |
| Deadline | 17 Sep 2026 |

## References

- PMID 33208041 — Cep63-Cep152 LLPS at centrosome
- PMID 17694534 — Centrin evolution and centriole duplication
- PMID 20861312 — Centriole disengagement after DNA damage
- PMID 25690512 — Chlamydomonas cell cycle (LECA model)
- PMID 22691130 — Evolution of land plant cilia
