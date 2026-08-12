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

## Repair/Elimination Kinetic Parameters (2026-08-02)

> Added in response to Julia Mahamid objection. Full model: `docs/REPAIR_OBJECTION_DEFENSE.md` §3.2

| Parameter | Symbol | Value | Units | Source/Note |
|-----------|--------|-------|-------|-------------|
| Oxidative damage rate (basal) | k_damage_basal | 0.005-0.01 | D_critical/yr | Time-dependent, post-mitotic cells |
| Replication-coupled damage rate | k_damage_rep | 0.0005-0.002 | D_critical/division | per division |
| Autophagy repair rate | k_autophagy | 0.001-0.01 | fraction/yr | Only during duplication (~1-2% cell cycle) |
| UPS repair rate (PCM only) | k_UPS | 0.003-0.008 | fraction/yr | ~10-20% PCM accessible |
| Chaperone refolding rate | k_chaperone | ~0.01 | fraction/yr | PCM only, not covalent modifications |
| Elimination rate (germline) | k_elim_germ | ~1.0 | per generation | Complete reset oocyte/early embryo |
| Elimination rate (somatic) | k_elim_som | ≈ 0 | per year | No centriole elimination in adult somatic cells |
| Critical damage threshold | D_critical | 1.0 | dimensionless | Functional failure threshold |
| Time to D_critical (human) | τ_critical | 60-100 | years | D₀ ≈ 0.05-0.10 D_critical |
| Cartwheel radius | r_cartwheel | ~25 | nm | SAS-6 ring, 9-fold symmetry; valid as an age metric only in C. elegans/procentrioles (cartwheel disassembles at maturation in vertebrate mother centrioles — Guichard 2026) |
| Expected radius variation (late passage) | Δr_aged | 2-5 | nm | Carbonylation → surface distortion |
| Carbonylated tubulin (aged) | f_carbonyl | 2-5 | % of total | Stadtman ER (2006), PMID: 16756493 |
| Damaged dimers per centriole (aged) | N_damaged | ~36-90 | dimers | 1800 total × 2-5% carbonylation |

## References

- PMID 33208041 — Cep63-Cep152 LLPS at centrosome
- PMID 17694534 — Centrin evolution and centriole duplication
- PMID 20861312 — Centriole disengagement after DNA damage
- PMID 25690512 — Chlamydomonas cell cycle (LECA model)
- PMID 22691130 — Evolution of land plant cilia
