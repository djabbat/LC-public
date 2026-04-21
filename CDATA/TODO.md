# CDATA — TODO

**Last updated:** 2026-04-21

## CONCEPT↔CODE MISMATCHES (2026-04-21 audit)

Source: `CONCEPT_CODE_AUDIT_2026-04-21.md`.
All items are FIX LATER (non-trivial, require user decision or multi-file refactor).

### L1. Parameter value reconciliation (P0, CRITICAL)
`FixedParameters::default()` in `crates/cell_dt_core/src/parameters/fixed_params.rs` uses numeric defaults that **do not match** `PARAMETERS.md` CORRECTIONS-2026-04-22 canon. Example drifts:
- α_HSC: docs 0.028, code 0.0082
- ν_HSC: docs 1.2/yr, code 12.0 (10×)
- β_HSC: docs 0.005, code 1.0 (200×)
- τ_protection: docs 15 yr, code 24.3
- π_base: docs 0.65, code (as `pi_baseline`) 0.10
- π_0: docs 0.20, code 0.87

**Action required:** user decision which is canonical. If PARAMETERS.md is truth → update `Default` impl + MCMC posteriors + retests. If code is truth → regenerate PARAMETERS.md. Ref user rule `feedback_cdata_docs_sync`.

### L2. Rename `pi_baseline` → `pi_base`
Docs (THEORY.md §3.2, PARAMETERS.md) use `π_base` / `pi_base`. Code uses `pi_baseline`. Cross-crate rename (~30 refs including tests).

### L3. Document two damage equations
`cell_dt_cli::compute_damage()` implements canonical CONCEPT/THEORY.md additive form. `cell_dt_modules/aging_engine::AgingEngine::step()` implements a multiplicative rate form (called "article v3.2.3"). Write derivation (or mapping) document, or deprecate one. Currently silent correspondence.

### L4. P1..P10 prediction test harness
THEORY.md §4 defines 10 falsifiable predictions. `crates/cell_dt_validation/examples/` has scattered tests (hTERT, O2, circadian, centenarian, not-R) but no `predictions_P1_to_P10.rs`. Create harness with explicit stubs for predictions that require wet-lab data (P6 CCP1 KO, P7 TTLL6 inhibition, P9 CCP1 OE) so untested status is visible at build time.

### L5. Generate 7 missing core files
Per `feedback_project_core`, every project needs 10 core files. CDATA has 7 documents, missing: **CLAUDE.md, UPGRADE.md, KNOWLEDGE.md, MAP.md, MEMORY.md, LINKS.md** (TODO.md now exists). README also advertises AGENTS.md, JOURNAL.md, ROADMAP.md — none exist. Generate from CONCEPT.md.

### L6. `cdata_coupling` range in Python Sobol
`scripts/cdata_ablation_sobol.py` L56 samples `cdata_coupling ∈ [0.05, 0.30]`. CORRECTIONS-2026-04-22 + PARAMETERS.md require γ_i default 0 (null hypothesis, range [0, 0.05]). Either narrow the sampling range or add docstring justifying the wider exploration.

### L7. Python ↔ Rust parameter name map
Python: `nu_Muscle`, `nu_Neural`, `beta_HSC`, `pi_base`, `epigenetic_rate`. Rust: `muscle_nu`, `neural_nu`, `hsc_beta`, `pi_baseline`, (epigenetic_rate absent as named field — EPI_STRESS_COEFF=0.15 is a hardcoded constant). Create an explicit name map or unify.

### L8. Verify ABL-2 disclosure in CONCEPT.md body
CONCEPT.md is ~200 KB; audit could not fully read it. Grep found no "ABL-2" match in CONCEPT.md/THEORY.md/README.md. CORRECTIONS-2026-04-22 §1.6 + §2.2 require honest disclosure in `CDATA/CONCEPT.md Appendix B`. Verify whether Appendix B exists and contains Sobol paradox text, or add it.

### L9. Counter numbering self-consistency
README.md: "Counter #1" (L3) and "Counter #2" (L12). THEORY.md L4 says "Counter #2", L9 says "Counter #1". Code `COUNTER_NUMBER = 1`. Pick one.

