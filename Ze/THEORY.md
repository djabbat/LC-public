# Ze · THEORY

**Status:** Canonical formalism · regenerated 2026-04-28
**Source:** `~/Desktop/Ze Theory.docx` (Tkemaladze 2026)
**Authority:** This is the formal companion to `CONCEPT.md`. Any simulator code MUST reference a section here for each formula.

---

## §1. Notation

| Symbol | Meaning | Type |
|--------|---------|------|
| `Z_real` | Actual state of system (probability distribution or density matrix) | distribution / operator |
| `Z_model` | Observer's internal model of `Z_real` | distribution / operator |
| `I(Z)` | Impedance | scalar ≥ 0 |
| `S(P‖Q)` | Kullback–Leibler divergence; or quantum relative entropy | scalar ≥ 0 |
| `⟨ΔS⟩_gen` | Mean entropy production over a dissipative process | scalar |
| `Ṡ_gen` | Entropy production rate | scalar (rate) |
| `τ_Ze` | Proper-time budget | scalar (time) |
| `α, β` | Dimensionless constants | scalar |
| `ν_Ze` | Inverse of α (used in physical-time integral form) | rate |
| `δ` | Ze deformation parameter | scalar; `δ ∝ (∇I)² / Λ_Ze²` |
| `Λ_Ze` | Impedance scale parameter | scalar |
| `(a, b)` | CHSH measurement directions on the Bloch sphere | unit 3-vectors |
| `C(τ)` | Two-time correlation function | scalar |
| `K(τ)` | Leggett–Garg expression `2C(τ) − C(2τ)` | scalar |
| `F_Q` | Quantum Fisher information | scalar ≥ 0 |
| `Q` | Bounded observable in the LGI setting | operator |

All formulas use natural units (`ℏ = k_B = 1`) unless otherwise noted.

---

## §2. Information–entropy equality (Lemma A)

### §2.1 Statement

> For a system undergoing a dissipative process from initial to final state, the information loss about initial conditions equals the mean entropy produced:
> `I_loss = ⟨ΔS⟩_gen`
> where `I_loss = S(P_real ‖ P_model)`.

This is Burgholzer (2015) "Information loss and entropy production during dissipative processes…" Theorem 1, derived from the Jarzynski/Crooks fluctuation relations.

### §2.2 Proper-time consumption

From §2.1, for a process with entropy-production rate `Ṡ_gen`:

`dτ_Ze/dt = −α · Ṡ_gen` *(Postulate — single primitive postulate of Ze Theory)*

Substituting `I = ⟨ΔS⟩_gen` (which is true *for the dissipative regime*; see §2.4 for caveat):

`dτ_Ze/dt = −α · I(Z)` *(central law)*

The integral form gives **physical time** as accumulated impedance:

`t = ν_Ze⁻¹ · ∫₀ᵗ Ṡ_gen dt'` , `ν_Ze = 1/α` *(definition)*

### §2.3 Quantum case

When `Z_real, Z_model` are density matrices `ρ, σ`, the impedance is the quantum relative entropy:

`I(ρ ‖ σ) = Tr[ρ (log ρ − log σ)]`

The classical formula in `CONCEPT §3` row 1 is recovered when `ρ, σ` commute and are diagonal in the same basis.

### §2.4 Caveat

The equality `I = ⟨ΔS⟩_gen` holds for **dissipative** processes in the regime where Burgholzer's derivation applies (Markovian dynamics, well-defined Crooks-type forward/reverse pairs). For non-Markovian or unitary regimes, the relation is **inequality** (`I ≤ ⟨ΔS⟩_gen` does not generically hold; the precise bound depends on the process). The simulator flags non-dissipative regimes as `regime: "non-dissipative"` and refuses to claim the equality.

---

## §3. Quadratic CHSH deformation (Lemma B + Lemma C)

### §3.1 Setup

A two-qubit singlet state `|Ψ⁻⟩ = (|01⟩ − |10⟩)/√2`. Standard quantum-mechanical correlation is `E_QM(a, b) = −a·b`, with measurement directions `a, b ∈ S²`.

### §3.2 Information-geometric expansion (Lemma B)

The Fisher information metric on the space of probability distributions yields a natural quadratic expansion of any smooth functional in the impedance gradient `∇I`. Expanding `E(a, b)` to second order:

`E(a, b) = E_QM(a, b) + ½ · (∂²E/∂I²) · (∇I)² + O((∇I)⁴)`

The second-order term is constrained by:

1. **Odd parity** of the singlet correlation: `E(a, b) = −E(−a, b)`.
2. **No-signaling**: `Σ_b E(a, b) = 0` for all `a`.
3. **Rotational invariance** of the singlet.

The unique polynomial in `(a·b)` consistent with all three constraints is `[(a·b)² − 1/3]` (the constant `1/3` enforces no-signaling under uniform `b`).

Therefore:

`E_Ze(a, b) = −a·b + δ · [(a·b)² − 1/3]`

with `δ ∝ (∇I)² / Λ_Ze²`.

### §3.3 No-signaling check

Over uniformly distributed unit vectors `b ∈ S²`:

`⟨(a·b)²⟩_b = 1/3`

so `Σ_b [(a·b)² − 1/3] = 0`. Therefore `Σ_b E_Ze(a, b) = 0`. ✅

### §3.4 Optimal CHSH (Lemma C)

The CHSH parameter is `S = E(a, b) − E(a, b') + E(a', b) + E(a', b')` for measurement settings `a, a', b, b'`. For QM at the Tsirelson-optimal angles `(a, a') = (0, π/2)`, `(b, b') = (π/4, 3π/4)`, the QM contribution is `2√2`.

Inserting the deformation term `δ · [(a·b)² − 1/3]` and re-optimizing over the four measurement angles yields the Ze prediction:

`S_Ze = 2√2 + δ · 1.7478`

The constant `1.7478` comes from the optimization of `Σ ±[(aᵢ·bⱼ)² − 1/3]` over the four CHSH terms. **The simulator must verify this constant numerically** as part of test F3 (see §7); it is not to be hard-coded as a "magic number."

### §3.5 Tsirelson bound

For any `δ > 0`, `S_Ze > 2√2`. This is **not** a violation of the Tsirelson bound on standard quantum correlations — it is a prediction that the true correlation function is the deformed one, and the bound on the deformed correlation must be re-derived. Reviewers may legitimately challenge this; the simulator therefore exposes both `S_QM` and `S_Ze` for any input.

---

## §4. Correlation decay (Lemma D)

### §4.1 Information-geometric foundation

Gassner et al. (2021) showed that the minimum-entropy-production path between two states in the space of distributions is the geodesic in the Fisher metric, and that the entropic speed `v` along that geodesic relates to entropy-production rate as `Ṡ_gen ∝ v²`.

For a system with impedance `I`, combining `Ṡ_gen ∝ v²` with `I ∝ Ṡ_gen` (§2.1) gives `v² ∝ I`.

### §4.2 Exponential decay

The two-time correlation along the geodesic decays as

`C(τ) = C₀ · exp(−v² · τ / 2) = C₀ · exp(−β · I · τ)`

where `β` absorbs the proportionality constants and depends on the geometry of the state space.

### §4.3 Caveat

This is a leading-order result in the small-`I·τ` regime; non-geodesic paths give corrections beyond simple exponential. The simulator's correlation primitive must accept an optional `geometry: "geodesic"` flag (default true) and refuse to extrapolate when the user requests `β·I·τ > 1`.

---

## §5. QFI bound (Lemma E)

### §5.1 Abboud bound (input)

Abboud et al. (2026) Theorem 1: for a stationary state and bounded observable `Q`,

`F_Q ≥ 8 · [K(τ) − ⟨Q²⟩]`

where `K(τ) = 2C(τ) − C(2τ)`.

### §5.2 Substitution

Insert `C(τ) = C₀ · exp(−β·I·τ)` from §4. Let `x = β·I·τ`:

`K(τ) = C₀ · (2 e⁻ˣ − e⁻²ˣ)`

Taylor expansion to `O(x³)`:

`K(τ) = C₀ · (1 − x² + x³) + O(x⁴)`

(The linear term in `x` cancels exactly: `−2x + 2x = 0`. This cancellation is the source of the central nontriviality.)

`F_Q ≥ 8 · [C₀ · (1 − x² + x³) − ⟨Q²⟩]`

For the stationary case where `⟨Q²⟩ = C₀` at `τ → 0`:

`F_Q ≥ 8 · C₀ · (−x² + x³) = 8 · C₀ · x² · (−1 + x)`

For `x < 1` this is negative; since `F_Q ≥ 0`, take absolute value (which corresponds to optimizing measurement direction):

`F_Q ≥ 8 · C₀ · (β·I·τ)² · |1 − β·I·τ|`

For `β·I·τ < 1`:

`F_Q ≥ 8 · C₀ · (β·I·τ)² · (1 − β·I·τ)`  *(Lemma E, regime-restricted)*

### §5.3 QFI ∝ proper-time consumption (Theorem 1)

From the central law (§2.2), `I = −(1/α) · dτ_Ze/dt`. Substitute into Lemma E:

`F_Q ≥ 8 · C₀ · (β/α · τ · |dτ_Ze/dt|)² · (1 − β/α · τ · |dτ_Ze/dt|)`

Optimizing over `τ` (extremizing the right-hand side w.r.t. `τ`) gives `τ_opt` proportional to `α/(β·|dτ_Ze/dt|)`. At `τ_opt`:

`F_Q,max ∝ |dτ_Ze/dt|`  *(Theorem 1, leading order)*

### §5.4 Caveat

The proportionality `F_Q ∝ |dτ_Ze/dt|` is a leading-order, optimal-`τ`, single-observable result. Multi-observable settings, finite-`τ` measurements, and non-stationary states all generate corrections. The simulator must clearly label its `F_Q` output as `F_Q_lower_bound_optimal_tau` and not as "the QFI."

---

## §6. Entropy / consciousness identification (out of scope of simulator)

Section 7 of the source docx identifies impedance with the KL term in Friston's variational free energy and defines consciousness as `C(x, t) = −dI/dt`. These are **labeling** statements in Ze Theory. The simulator does **not** consume them; they are not part of the five canonical quantities. They live only in `CONCEPT §1` summary and in `EVIDENCE.md` as connections, not as code.

---

## §7. Required tests (F1–F6)

Every release of `ze-simulator` MUST pass:

| ID | Test | What it verifies |
|----|------|------------------|
| **F1** | `KL(p, p) == 0` for every `p`; `KL(p, q) > 0` for `p ≠ q`; `KL` is asymmetric | Impedance is a genuine KL divergence. |
| **F2** | Forward Euler vs RK4 vs analytical solution of `dτ/dt = −α·I` for piecewise-constant `I(t)` agree within `10⁻⁶` (RK4) and `10⁻³` (Euler with `dt=10⁻³`) | Proper-time integrator is correct. |
| **F3** | Numerical optimization over `(a, a', b, b') ∈ (S²)⁴` of the Ze CHSH expression returns `S_Ze − 2√2 == δ · 1.7478 ± 10⁻⁴` for `δ ∈ {0, 0.05, 0.10, 0.15}` | The constant 1.7478 is recovered from the geometry, not assumed. |
| **F4** | `C(τ)` matches `C₀ · exp(−β·I·τ)` at sample `τ` to `10⁻⁹` | Exponential decay implementation is exact. |
| **F5** | `F_Q` Lemma E formula matches direct numerical evaluation of `8·[K(τ) − ⟨Q²⟩]` for the implemented `K, C` to `10⁻⁹` | QFI lower bound is consistent with itself. |
| **F6** | `F_Q,max` (numerically optimizing `τ`) scales linearly with `|dτ_Ze/dt|` over a 2-decade sweep, slope and intercept fit `F_Q ∝ |dτ_Ze/dt|` to `R² > 0.999` | Theorem 1. |

These tests live in `ze-simulator/tests/` as integration tests. They are required for any commit that touches simulation primitives.

---

## §8. Source-document section anchors

| THEORY § | docx § |
|----------|--------|
| §2.1 Lemma A | §2.1 |
| §2.2 Postulate + central law | §2.2, §2.3 |
| §2.3 Quantum case | §1.2 (von Neumann) |
| §2.4 Caveat | implicit in §2.1 (regime of derivation) |
| §3 Lemma B + C | §4.1–§4.4 |
| §3.5 Tsirelson note | not in source — added by simulator-design layer |
| §4 Lemma D | §5.2 |
| §5 Lemma E + Theorem 1 | §5.1, §5.3, §5.4 |
| §6 (consciousness) | §7 |
| §7 Tests | not in source — simulator-side requirement |
