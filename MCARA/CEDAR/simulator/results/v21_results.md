# CEDAR-v2.1 reference results

Base parameter set: {'decay_scale': 2.45, 's_D': 0.006, 'q_amp': 0.0079, 'kappa': 0.85, 'r_mult_cv': 0.2, 'aurA_cv': 0.62}
Cells per run: 10000; seeds: 10

## 1. v2.0 baseline reproduction (ATM-only, no autophagy, P=0)
| metric | reference | published v2.0 | deviation |
|---|---|---|---|
| median_exit | 50.50 | 48.0 | +5.2% |
| iqr_exit | 9.50 | 8.0 | +18.8% |
| amp_pct | 49.89 | 52.0 | -4.1% |
| p53 route, % | 26.73 | 28.5 | -6.2% |

## 2. v2.1 scenarios
| scenario | Hayflick median | IQR | p53-route % | exhaustion % | PIDDosome % | silenced % | amplified at exit % | corrected % |
|---|---|---|---|---|---|---|---|---|
| v2.0 baseline (ATM only) | 50.5 ± 0.50 | 9.5 | 26.7 | 73.3 | 0.0 | 31.4 | 49.9 | 0.0 |
| v2.1 WT (P=1, autophagy on) | 49.8 ± 0.40 | 12.9 | 47.5 | 52.5 | 25.2 | 28.3 | 37.8 | 62.3 |
| v2.1 PIDD1-KO / CASP2-KO (P=0) | 51.3 ± 0.46 | 9.2 | 28.2 | 71.8 | 0.0 | 33.5 | 34.6 | 68.9 |
| v2.1 WT + Aurora A x3 (Tet-On) | 50.0 ± 0.00 | 11.6 | 20.0 | 80.0 | 19.2 | 9.5 | 50.6 | 64.0 |
| v2.1 WT + Aurora A x3 + alisertib-like (x0.35) | 50.0 ± 0.00 | 12.9 | 45.3 | 54.7 | 25.0 | 28.5 | 38.1 | 62.3 |
| v2.0 + Aurora A x3 (no PIDDosome, no autophagy) | 50.0 ± 0.00 | 10.0 | 1.0 | 99.0 | 0.0 | 10.1 | 67.0 | 0.0 |

## 3. Route partition, v2.0 vs v2.1 (WT)
- **v2.0 baseline (ATM only)**: exhaustion 41.9%, exhaustion(silenced) 31.4%, ATM-arrest 26.7%, PIDDosome-arrest 0.0%, censored 0.0%
- **v2.1 WT (P=1, autophagy on)**: exhaustion 24.2%, exhaustion(silenced) 28.3%, ATM-arrest 22.3%, PIDDosome-arrest 25.2%, censored 0.0%

## 4. Variance-based sensitivity, v2.1 WT
| parameter | S1 (median) | ST (median) | S1 (silenced %) | ST (silenced %) |
|---|---|---|---|---|
| decay_scale | +0.007 | +0.003 | +0.101 | +0.171 |
| s_D | -0.000 | +0.000 | +0.394 | +0.474 |
| kappa | +0.002 | +0.000 | -0.003 | +0.001 |
| N_crit_P | +0.942 | +0.840 | +0.386 | +0.528 |
| lam_p21 | +0.134 | +0.206 | +0.001 | +0.118 |
| k_auto | -0.014 | +0.003 | -0.011 | +0.010 |

## 5. Regularisation checks
- Held-out fold medians: [50.0, 49.0, 50.0, 50.0, 50.0] (sd 0.45, CV 0.9%)
- Wasserstein-1 to a normal surrogate with the published median/IQR: 6.17 generations

Runtime: 52.8 s
