# Provenance and status of this code

**Status: reference implementation, not the original code.**

The simulation reported in Tqemaladze (2026), *A Stochastic Model of Centriole-Driven
Stem Cell Exhaustion*, Longevity Horizon 2(5), doi:10.65649/wjtcf387, was described in
the article but its update equations were never published. The original source was
lost in 2026: the editable install pointed at `LC/MCAOA/CDATA/simulator/cdata_sim`,
a directory that no longer exists, and the repository named in the article's
data-availability statement (`github.com/djabbat/CEDAR-sim`) had never been created.

This repository was written on 2026-11-16 from the published architecture and
parameter table, and then calibrated against the four numbers reported in the
article:

| Metric | This implementation | Published v2.0 | Deviation |
|---|---|---|---|
| Hayflick median, generations | 50.5 ± 0.5 | 48 ± 0.4 | +5.2% |
| Hayflick IQR | 9.5 | 8 | +18.8% |
| p53-mediated route, % | 26.7 | 28.5 | −6.2% |
| Amplification frequency, % | 49.9 | 52 | −4.1% |

Four quantities the article leaves unspecified were fitted to achieve this:
asymmetry-decay scale, damage scale inside the ATM term, centrosome-amplification
event rate, and persistence of amplified centrosomes. They are documented in
[`MODEL_SPEC_v2.1.md`](MODEL_SPEC_v2.1.md). Agreement is therefore **by construction**
for the v2.0 baseline; the v2.1 results below are indicative in magnitude and robust
only in direction.

## v2.1 additions

- `PIDDosome` p53 input (PIDD1-RAIDD-CASP2 → MDM2 → p53 → p21), independent of ATM
  (Fava et al. 2017, PMID 28130345).
- Aurora A(Ser215) transactivation gate that blocks both p53 arms — this is what
  produces the "silenced" state rather than postulating it.
- Autophagy correction edge (CEP170→ARHGEF2→RAC1→ATG16L1; Coelho & Glover 2026,
  PMID 42633651): LC3B+ cells can return to a normal centrosome count.

Headline changes (10 seeds × 10,000 cells): p53-mediated exits 26.7% → 47.5%,
PIDD1/CASP2 knockout returns to 28.2%, Aurora A × 3 collapses arrest to 20.0% and
partial Aurora A inhibition restores it to 45.3%, instantaneous amplification
49.9% → 37.8%. Full tables: [`results/v21_results.md`](results/v21_results.md).

## Known inconsistency

v2.1 predicted that centrosome number should keep rising in a PIDD1 knockout.
In this implementation the knockout raises *cumulative* amplification only
marginally (47% → 49%) because the correction edge dominates. This is an open
issue in the model, recorded rather than smoothed over.

## Reproducing

```bash
pip install -e .
python3 scripts/run_v21_results.py     # ~50 s
python3 -m pytest tests -q             # 8 tests
```
