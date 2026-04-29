# BioSense

A reference simulator + wearable-platform companion for **BioSense Theory** — aging conceptualised as a Total Chronic Disease, monitored via the χ_Ze index derived from EEG, HRV, respiration, and sleep.

The canonical exposition lives in [`CONCEPT.md`](./CONCEPT.md), with formal derivations in [`THEORY.md`](./THEORY.md). The source paper is `BioSense.docx` on the author's desktop (Tkemaladze 2026, *Longevity Horizon* 2(5), DOI 10.65649/23ba5z09).

## What this project is

A Rust workspace + a Phoenix LiveView UI that compute and visualise five quantities:

1. **Ze velocity** `v` from a binary symbolised physiological signal.
2. **Predictive information** `I_pred` (closed form for symmetric Markov; numerical estimator for arbitrary signals).
3. **χ_Ze index** — composite over EEG/HRV/respiration/sleep modalities, normalised against the theoretical fixed point `v* = 0.45631`.
4. **CDATA bridge** — maps centriolar damage `D(t)` to disease activity `A(t)` and to χ_Ze.
5. **Exacerbation risk** — 30-day binary risk classifier on rolling χ_Ze.

Plus a software 5-layer privacy stack (DP noise, k-anonymity, secure aggregation) wrapping every released aggregate.

## Run it

```bash
# Rust simulator + HTTP backend on :4101
cargo build --release
cargo run -p biosense-backend

# Phoenix LiveView on :4100 (separate shell)
cd biosense-web && mix phx.server
```

Then open `http://127.0.0.1:4100`.

## Datasets (Phase 2)

The `datasets/` crate (planned) exposes loaders for LEMON, Cuban, NHATS, All-of-Us Fitbit, and UK Biobank wearable subsets behind a single `Dataset` trait. See [`TODO §2`](./TODO.md).

## License

MIT (see `LICENSE`). Research project; correctness claims are bounded by the B1–B6 tests in `biosense-simulator/tests/`.
