# TODO.md — CEDAR-v2 Simulator

## Urgent
- [x] Add DOI (Zenodo or CrossRef)
- [x] Publish on PyPI (cedar-sim)
- [x] Create documentation on ReadTheDocs

## Technical
- [x] Add Jupyter notebooks with examples
- [x] Speed up ABC-SMC (JIT/Numba?)
- [x] Add parallel processing (multiprocessing)
- [x] Integration tests
- [x] CI/CD (GitHub Actions)

## Scientific
- [x] Validation on independent data
- [x] Add calibration for 4th cell type
- [x] Article in peer-reviewed journal
- [x] Model registration in BioModels

## Documentation
- [x] API reference
- [x] Tutorial on ABC-SMC calibration
- [x] Guide on GSA interpretation

## Low priority
- [x] Web interface for running simulations
- [x] Rust port for performance
- [x] Integration with LC/sim_core

## rDNA clock (2026-08-08)
- [x] Rust AgingEngine: SenescenceTrigger::RdnDnaShortening + rdna_copy_number (v4.7) — DONE, 547 tests pass
- [ ] Python cedar-sim: mirror rDNA clock (RDNA_LOSS_PER_DIVISION etc.) — next
