# PARAMETERS — HAP Project

**Version:** 1.0

## Simulation Model

### State variables
| Symbol | Variable | Default | Range | Unit |
|--------|----------|---------|-------|------|
| L | Hepatic steroid output | 0.1 | [0, 10] | nM |
| B | Brain steroid sensitivity | 0.1 | [0, 1] | a.u. |
| A | Affective circuit integrity | 0.0 | [0, 1] | a.u. |
| I | Inflammatory state | 0.1 | [0, ∞) | a.u. |
| S | HPA / stress activity | 0.2 | [0, ∞) | nM |
| M | Metabolic state | 1.0 | [0.3, ∞) | mM |

### Key Parameters
| Parameter | Value | Description |
|----------|---------|----------|
| τ_crit | 72 hpf | End of critical developmental window |
| L_basal | 1.0 nM | Basal hepatic steroid output |
| k_A_L | 0.3 | Dependence of affective circuits on L |
| k_A_B | 0.4 | Dependence on B |
| I_suppress_L | 0.3 | Suppression of L by inflammation |
| S_enhance_L | 0.2 | Enhancement of L by stress (allostasis) |

## Project Parameters

### Budget
- No external funding (at the moment)
- Time: free (collaboration with Afaf)

### Contacts
- Jaba Tqemaladze: jaba@longevity.ge
- Afaf Elfet: via email

### Tools
- Python 3.10 + SciPy/NumPy/Matplotlib (simulation)
- Rust (Cargo) — if performance is required
- GitHub: djabbat/HAP (TODO)

### License
- Publication: open access (Longevity Horizon, Gold OA)
- Code: MIT / CC-BY (TODO)