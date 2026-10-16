# Ze_CHSH — CHSH(entropy) as a falsifiable prediction of Ze


**Type:** theorem and mathematical proof — see CONCEPT.md for details.
**Version:** 1.0

## Core Claim

Ze theory predicts a **linear decrease** of the CHSH parameter $S$ with increasing injected entropy $H$:

$$S(H) = 2\sqrt{2} - \beta H, \quad \beta = \frac{2}{\ln 2} \approx 4.082\ \text{nat}^{-1}$$

## How it is derived

The derivation combines three independent bounds:

| # | Bound | Source | Status |
|---|---------|----------|--------|
| 1 | Tsirelson bound | $S_{\max} \leq 2\sqrt{2}$ | ✅ Proven |
| 2 | LGI → QFI bound | $F_Q \geq 2\|\Delta_{\text{LGI}}\|$ | ✅ **Verified 2026-10-15: arXiv:2604.09772 — real form $F_Q \geq 8(K - \langle Q^2\rangle)$ (pure), coeff 8 ≠ 2; ⚠️ β NOT derivable as theorem (см. BETA VERDICT)** |
| 3 | Entropy inequality | $\|\Delta_{\text{LGI}}\| \leq \frac{\ln 2}{2} \cdot H$ | ❌ **Not verified** |

The constant $\beta = 2/\ln 2$ was claimed to be derived by combining (1)+(2)+(3) — **see BETA VERDICT below: NOT derivable; β is a postulate** (numerically 4.082 = 2√2/ln2).

## Key problems (updated 2026-10-15 after deep database search)

1. **Reference [11]** — Abboud, Guan, Bradlyn, Noronha (2026) "LGI–QFI bound" — ✅ **FOUND + CONTENT VERIFIED 2026-10-15**: "Leggett-Garg Inequality Violations Bound Quantum Fisher Information", arXiv:2604.09772 (2026-04-10). ⚠️ **The paper's actual bound differs from CONCEPT:** pure state $F_Q \geq 8(K - \langle s|Q^2|s\rangle)$ [Eq.8]; thermal $F_Q \geq (K - \langle Q^2\rangle)/\gamma(2\tau/\beta)$ [Eq.11], and $F_Q \geq 7(K-\langle Q^2\rangle)/(2z^2)$ for $\tau > \tau_{th}$ [Eq.12]. Coefficient is **8**, not 2, and it depends on $\langle Q^2\rangle$, not on $\|\Delta_{LGI}\|$ alone. For a dichotomic Q ($Q^2=1$): $F_Q \geq 8(K-1) = 8\Delta_{LGI}$. **→ β = 2/ln2 must be re-checked against the real chain in v3.** Note: paper concerns LGI (temporal), Ze_CHSH is CHSH (spatial) — they connect via QFI.
2. **Reference [13]** — Hall (2013) strengthened entropy inequality — ✅ **EXISTS on arXiv 2026-10-15: arXiv:1307.2697, *Correlation distance and bounds for mutual information*** (Michael J. W. Hall, 2013) — tight lower bounds on mutual information I via correlation distance C for qubits (stronger than Pinsker); relevant to the entropic bound. ⚠️ Exact form $C \leq 2\sqrt{I(2\ln 2-I)}/\ln 2$ to be verified against the full text. (Rastegin 2014 / Cao 2016 remain useful cross-references.)
3. **Reference [14]** — Honma & Vu (2025) quantum thermokinetic TUR — ✅ **FULLY VERIFIED 2026-10-15**: Honma & Van Vu, arXiv:**2510.04866** (2025, year in CONCEPT correct) = *Phys. Rev. A* (2026), DOI **10.1103/f77p-kw54**. Abstract contains verbatim: "we derive a **quantum thermokinetic uncertainty relation** for interacting multipartite systems". No remaining doubt.
4. **Reference [7]** — Burgholzer (2015) "Information loss and entropy production" — ✅ **EXISTS on arXiv 2026-10-15: arXiv:1502.00214, *Information loss and entropy production during dissipative processes*** (Peter Burgholzer, 2015) — stochastic thermodynamics, ill-posed inverse problems, entropy production; topically relevant. ⚠️ Exact identification "impedance I ≈ ⟨ΔS⟩_gen" to be verified against the full text.
5. **Reference [12]** — Seshadreesan, Kim, Dowling (2013) "CHSH–QFI bound" — ❌ **MISATTRIBUTED 2026-10-15**: arXiv:1310.1410 is about two-mode interferometry / N00N states in a homodyne setup, and its conclusion is the opposite of what v3 claims: *"the amount of violation in those nonlocality tests may not be used to quantify sub-shot-noise sensitivity"*. The formula $S_{\max}^2 \leq 8 + 4F_Q$ is NOT in that paper and contradicts its result.

## 🔴 BETA VERDICT (2026-10-15, after full-text verification)
**β is NOT derivable as a theorem from bounds (1)+(2)+(3).** Five independent defects in the v3 chain:
1. **Ref [12] misattributed** — real Seshadreesan et al. say CHSH-violation does NOT quantify QFI; the link $S_{\max}^2 \leq 8 + 4F_Q$ has no verified source.
2. **Ref [11] coefficient wrong** — real Abboud et al. bound is $F_Q \geq 8\Delta_{LGI}$ (dichotomic, pure), not $F_Q \geq 2\Delta_{LGI}$; and it is a *lower* bound, not the equality v3 claims.
3. **Wrong inequality direction** — from $S_{\max}^2 \leq 8 + 4F_Q$ (upper bound on S via F_Q) and $F_Q \geq 8\Delta$ (lower bound on F_Q) one CANNOT derive an upper bound on S via Δ. One would need an upper bound $F_Q \leq f(\Delta)$, which [11] does not provide.
4. **Wrong sign** — even taken at face value, the chain gives $S_{\max}^2 \leq 8 + 4\ln2\cdot H$, an upper bound that *grows* with H; the minus sign in $S = 2\sqrt{2} - \beta H$ is postulated, not derived (v3's own note admits this).
5. **Numerical mismatch** — the stated value 4.082 nat⁻¹ equals $2\sqrt{2}/\ln 2$, NOT $2/\ln 2 \approx 2.886$. The formula and the number contradict each other. (The chain's own expansion would give $\beta \approx (\sqrt{2}/2)\ln 2 \approx 0.490$ with the minus sign.)

**Honest status:** β is a **phenomenological postulate** of Ze — linear entropy-driven degradation $S = 2\sqrt{2} - \beta H$, with the sign and rate fixed empirically. It is *motivated* (not derived) by TUR/LGI/entropic bounds. Falsifiability is preserved and is the scientific core: the CHSH(entropy) experiment determines β, sign, and slope. The article must be reworded: remove "β is a theorem" (§3.2) and reclassify β as postulate + experimental prediction.

## Prediction

| $H$ (nat) | $S(H)$ | Status |
|:---------:|:------:|:------:|
| 0.0 | 2.828 | Tsirelson bound |
| 0.1 | 2.420 | Above classical limit |
| 0.2 | 2.011 | At classical boundary |
| 0.3 | 1.603 | Below classical |
| 0.4 | 1.195 | Classical regime |
| 0.5 | 0.787 | Strong degradation |

## Falsifiability

Experiment: BBO source + EOM on one arm → injection of entropy $H$ → measurement of $S$.
- Statistical significance: **> 5σ within 24 hours** with existing setups
- If slope $\beta \neq 4.082$ — Ze theory is falsified
- If $\beta = 0$ (no dependence on entropy) — Ze theory is falsified

## Experimental protocol

Details: `docs/Ze_CHSH_Entropy_Quantum_v3_ru.md`, §4.

| Component | Specification |
|-----------|--------------|
| Source | BBO, type I, 405 nm → 810 nm |
| State | Singlet $|\Psi^-\rangle$ |
| Entropy injection | EOM + RNG on one arm |
| Measurements | PBS + single-photon detectors |
| Coincidence window | 3 ns |
| Settings | $\{0^\circ, 45^\circ, 22.5^\circ, 67.5^\circ\}$ |

## Connection to other Ze subprojects

- **Ze (parent):** CHSH prediction — direct consequence of Ze impedance and entropy production
- **D_Ze:** CHSH degradation as quantum analog of Ze friction between subsystems
- **Ze-Hierarchy:** General principle — entropy destroys correlations (CHSH / HI)
- **Poincare:** Mathematical apparatus for analyzing Ze flows

## Consumables (annual)

| **Cloud/API services** (AI model API keys, hosting) | **$1,000** |
| **Office consumables** (printing, stationery) | **$300** |


## Hypothesis

*To be specified — see CONCEPT.md §1 for project rationale.*


## References

*See project MEMORY.md for reference history.*


## Budget

Theory — no budget needed. Detailed budget in PARAMETERS.md.
