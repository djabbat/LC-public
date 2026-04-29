# Ze

A reference simulator for **Ze Theory** — the proposal that time is a thermodynamic resource consumed by prediction error, with quantitative consequences for CHSH correlations and quantum Fisher information.

The canonical exposition lives in [`CONCEPT.md`](./CONCEPT.md), with formal derivations in [`THEORY.md`](./THEORY.md). The source paper is `Ze Theory.docx` on the author's desktop.

## What this project is

A Rust workspace + a Phoenix LiveView UI that compute and visualize five quantities:

1. **Impedance** — `I(Z) = S(Z_real ‖ Z_model)`, the Kullback–Leibler divergence between actual and modeled state.
2. **Proper-time consumption** — `dτ_Ze/dt = −α·I(Z)`.
3. **CHSH deformation** — `E_Ze(a, b) = −a·b + δ·[(a·b)² − 1/3]`, giving `S_Ze = 2√2 + δ·1.7478`.
4. **Correlation decay** — `C(τ) = C₀·exp(−β·I·τ)`.
5. **QFI bound** — `F_Q ≥ 8·C₀·(β·I·τ)²·(1 − β·I·τ)` and, optimized, `F_Q,max ∝ |dτ_Ze/dt|`.

## Run it

```bash
# Rust simulator + HTTP backend on :4001
cargo build --release
cargo run -p ze-backend

# Phoenix LiveView on :4000 (separate shell)
cd ze-web && mix phx.server
```

Then open `http://127.0.0.1:4000`.

## License

MIT (see `LICENSE`). This is a research project; correctness claims are bounded by the F1–F6 tests in `ze-simulator/tests/`.
