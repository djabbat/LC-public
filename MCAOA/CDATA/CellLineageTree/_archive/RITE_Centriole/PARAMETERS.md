# RITE_Centriole — PARAMETERS

## Molecular design

| Element | Spec |
|---|---|
| Backbone | pLenti-CMV (or pLV-EF1α) |
| Selection | Puromycin 2 µg/mL, 7 d |
| Recombinase | Cre-ER^T2 (mutant ligand-binding domain) |
| Sites | loxP (34 bp) / lox2272 (heterospecific) |
| Pre-pulse tag | mCherry (ex 587 / em 610) |
| Post-pulse tag | EGFP (ex 488 / em 507) |
| Scaffold candidates | Centrin-1 (172 aa), SAS-6 (657 aa), CEP152 (1654 aa) |
| Linker | GS flexible, 3× (GGGGS) |

## Induction

| Parameter | Value |
|---|---|
| 4-OHT concentration | 500 nM (standard) – 2 µM (max) |
| Pulse duration | 1 h → 4 h (to be optimized) |
| Washout | 3× PBS, complete medium replacement |
| Expected swap efficiency | ≥ 80 % (target) |

## Imaging

| Parameter | Value |
|---|---|
| Red channel | 561 nm laser, 10 % power, 200 ms exposure |
| Green channel | 488 nm laser, 10 % power, 200 ms exposure |
| Frame interval | 10 min for lineage; 30 s for mitosis |
| Total duration | up to 72 h |

## Budget (Phase A)

| Item | EUR |
|---|---|
| Gene synthesis (Twist, 3 constructs) | 1,800 |
| Lentivirus packaging reagents | 1,200 |
| Cell lines (HEK293T, BJ-hTERT) + QC | 600 |
| 4-OHT + puromycin + consumables | 400 |
| Plasmid deposition (Addgene × 3) | 195 |
| **Total** | **~4,195** |

## Risk matrix

| # | Risk | Probability | Impact | Mitigation |
|---|---|---|---|---|
| 1 | Low tag-swap efficiency (Cre-mediated recombination may be incomplete, leading to mixed red/green centriole populations) of target centriolar gene | Medium | High | Use additional loxP sites (e.g., lox2272) to increase recombination efficiency; validate by Western blot before imaging |
| 2 | High rate of Cre-independent recombination (leakiness) | Low | High | Use inducible Cre-ER^T2 with tight Dox/tamoxifen control; include no-induction control |
| 3 | Phototoxicity from prolonged fluorescence imaging | Medium | Medium | Limit excitation intensity and exposure time; use spinning-disk confocal |
| 4 | High variability in transfection/transduction efficiency | High | Medium | Normalise by co-expressing constitutive fluorescent marker; use FACS sorting |
| 5 | Erroneous centriole segregation due to mis-segregation of labelled proteins | Medium | High | Validate by co-staining with centriole markers (e.g., Centrin-1 antibody) |
