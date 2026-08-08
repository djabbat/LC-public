# PARAMETERS — D_Ze

**Version:** 1.0

## Main Parameters

| Parameter | Symbol | Formula | Value / Range |
|----------|--------|---------|:-------------------:|
| Ze-discordance | D_Ze | (|Δv| + |Δτ| + |ΔZ|) / 3 | 0.0 – 1.0 |
| Ze-friction | F_Ze | D_Ze · (v₁ + v₂) / 2 | 0.0 – 0.5 |
| Friction/conflict threshold | — | — | D_Ze > 0.3 |
| Desynchronosis threshold | — | — | D_Ze > 0.5 |
| Reference Ze-velocity | v* | 1 − ln2 | 0.3069 |
| Reference Ze-index | Z* | 1/(1+e⁻¹) | 0.731 |

## D_Ze between Tissues

| Tissue pair | v₁ | v₂ | D_Ze | Interpretation |
|-------------|:--:|:--:|:----:|---------------|
| Hippocampus – neocortex | 0.32 | 0.10 | **0.22** | Internal brain conflict = learning |
| Intestinal epithelium – bone | 0.40 | 0.25 | **0.25** | Maximum conflict = inflammation zone |
| Intestinal epithelium – skin | 0.40 | 0.35 | 0.08 | Rapid renewal, low conflict |
| Epithelium – connective tissue | 0.40 | 0.28 | **0.20** | Chronic inflammation |
| Blood – neocortex | 0.33 | 0.10 | **0.22** | BBB = barrier between Ze-worlds |
| Hippocampus – cerebellum | 0.32 | 0.08 | **0.24** | Conflict of plasticity and automatisms |

## Half-Recovery Periods of Cell Types

| Cell type | T_½ | v | τ | Cytogenetic status |
|---------------|:---:|:--:|:--:|------------------------|
| Stem (totipotent) | ~2–5 days | 0.42 | 0.24 | Undifferentiated |
| Progenitor (multipotent) | ~1–4 weeks | 0.38 | 0.27 | Partially differentiated |
| Precursor (oligopotent) | ~1–6 months | 0.34 | 0.30 | Committed |
| Mature (differentiated) | ~6–24 months | 0.30 | 0.33 | Terminally differentiated |
| Terminal (postmitotic) | not renewed | 0.10–0.05 | 0.40–0.48 | Maximum diff. |

## Evolutionary Levels of D_Ze

| Level | Examples | Irreversible diff. | Characteristic D_Ze |
|:-------:|---------|:------------------:|:----------------:|
| 0 | Bacteria, archaea | No | 0 |
| 1 | Dictyostelium, Volvox | Only germ + somatic | < 0.05 |
| 2 | Plants (Plantae) | Only germ | 0.05–0.10 |
| 3 | Invertebrates | Partial somatic | 0.10–0.20 |
| 4 | Vertebrates | Complete somatic | 0.15–0.30 |
| 5 | Mammals + brain | Maximum + neuronal heterogeneity | 0.20–0.30 |
