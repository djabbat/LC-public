# PARAMETERS — Ze_CHSH

**Version:** 1.0

> **Important note:** The constant γ_Ze = 2/ln2 ≈ 4.082 nat⁻¹ is a **calibration prediction of Ze-theory**, not a derivation from bounds. Experimental refutation of γ_Ze falsifies Ze (see CONCEPT.md, section Limitations).

## Quantum parameters

| Parameter | Symbol | Value | Description |
|----------|--------|----------|----------|
| Tsirelson bound | $S_{\max}$ | $2\sqrt{2} \approx 2.828$ | Max. quantum violation of CHSH |
| Classical limit | $S_{\text{class}}$ | $2.0$ | Local realism |
| Ze constant (calibration) | $\gamma_{\text{Ze}}$ | $2/\ln 2 \approx 4.082\ \text{nat}^{-1}$ | CHSH degradation rate — Ze prediction |
| Depolarization | $\gamma_{\text{depol}}$ | $\approx 5.7\ \text{nat}^{-1}$ (approximation for small H) | Standard QM: depolarizing channel ρ(p) = (1-p)ρ_ideal + p·I/4. S(p) = (1-p)·2√2. In the limit of small p: H≈p·(1−log₂p). γ_depol ≈ lim_{p→0} 2√2·p/H(p) = 2√2 / (1−log₂p) → ∞ — **not linear for small p**. Value 5.7 — linear approximation in the range H=0–0.3 nat. |
| Dephasing | $\gamma_{\text{deph}}$ | $\approx 2.0\ \text{nat}^{-1}$ (approximation) | Standard QM |
| No dependence | $\gamma_0$ | $0$ | Classical limit |
| Pump wavelength | $\lambda_p$ | 405 nm | BBO type I |
| Photon wavelength | $\lambda_s$ | 810 nm | Degenerate SPDC |
| Coincidence window | $\Delta t$ | 3 ns | Temporal coincidence window |

## Experimental parameters

| Parameter | Value | Description |
|----------|----------|----------|
| Measurement angles | $\{0^\circ, 45^\circ, 22.5^\circ, 67.5^\circ\}$ | Standard CHSH |
| Number of coincidences | $10^6$–$10^7$ | 24 hours of data collection [7] |
| Detector efficiency | $\eta \sim 0.3$ | Typical for BBO |
| Statistical error | $\sigma_S \approx 2/\sqrt{N}$ | For $N=10^6$: $\sigma_S \approx 0.002$ |
| Achievable significance (Ze vs depol.) | $>$100$\sigma$ | At H=0.2 nat, N=$10^6$ |

## Entropy parameters

| Parameter | Formula | Description |
|----------|---------|----------|
| Injected entropy | $H_{\text{inj}} = -p\ln p - (1-p)\ln(1-p)$ | Bernoulli |
| $p$ (EOM probability) | $p \in \{0, 0.05, 0.10, \dots, 0.50\}$ | 11 points |
| Von Neumann | $S_{\text{vN}} = -\text{Tr}(\rho\ln\rho)$ | For comparison |
| Small p regime | $S_{\text{vN}} \approx H_{\text{inj}} + \mathcal{O}(p^2)$ | For $p \lesssim 0.1$ |

## Predicted values S(H)

| H (nat) | Ze (γ=4.082) | Depol. (γ=5.657) | Deph. (γ≈2.0) | No eff. (γ=0) |
|:-------:|:------------:|:----------------:|:---------------:|:--------------:|
| 0.0 | 2.828 | 2.828 | 2.828 | 2.828 |
| 0.1 | 2.420 | 2.262 | 2.628 | 2.828 |
| 0.2 | 2.011 | 1.697 | 2.428 | 2.828 |
| 0.3 | 1.603 | 1.131 | 2.228 | 2.828 |
| 0.4 | 1.195 | 0.566 | 2.028 | 2.828 |
| 0.5 | 0.787 | 0.000 | 1.828 | 2.828 |

## Comparison with alternatives

| Framework | Prediction | Status |
|-----------|-------------|--------|
| Standard QM | $S = 2\sqrt{2}$ (no dependence) | Falsifiable |
| Depolarizing channel | $S = 2\sqrt{2} - 4\sqrt{2}p$ | Falsifiable |
| Dephasing | $S \approx 2\sqrt{2} - 2.0H$ | Falsifiable |
| **Ze theory** | $S = 2\sqrt{2} - (2/\ln 2)H$ | **Calibration prediction of Ze** |
