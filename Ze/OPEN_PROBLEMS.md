# Ze · OPEN_PROBLEMS

**Status:** Known gaps · regenerated 2026-04-28

This file is the project's intellectual debt log. If you are tempted to "just add" something to the simulator, check that it isn't tracked here as deferred — and if it is, decide explicitly whether you are resolving it or postponing it again.

---

## §1. Theoretical / mathematical

### §1.1 Tsirelson-bound interpretation of `S_Ze > 2√2`

**Problem.** The Ze prediction `S_Ze = 2√2 + δ·1.7478` exceeds the Tsirelson bound `2√2` for any `δ > 0`. The standard reading is that this would falsify quantum mechanics. The Ze reading is that the *measured* CHSH parameter under realistic experimental conditions follows the deformed correlation function, not the QM-pure one — but a rigorous re-derivation of the bound on the deformed correlation is missing.

**Status.** Open. Affects how the simulator should *label* its `S_Ze` output.

**What the simulator does for now.** Returns both `s_qm = 2√2` and `s_ze = 2√2 + δ·1.7478`, and includes a `"warning": "exceeds_tsirelson_bound"` flag whenever `δ > 0`.

### §1.2 Choice of `α`, `β`, `Λ_Ze`

**Problem.** No experiment fixes any of `α`, `β`, `Λ_Ze`. They are all `O(1)` by convention. The theory is therefore predictive only up to ratios of these constants.

**Status.** Open. May be fixed by a fit to specific experimental data (e.g., Pearson 2021 nanoscale clock scaling), but no such fit is in scope of Phase 1.

### §1.3 Quantum density-matrix support

**Problem.** Impedance is currently implemented for classical probability distributions and for **real symmetric** density matrices. Complex Hermitian density matrices are not supported.

**Status.** Phase 2. Tracked in TODO.

### §1.4 Non-dissipative regimes

**Problem.** The equality `I = ⟨ΔS⟩_gen` holds only for dissipative Markovian processes. The simulator currently does not detect non-dissipative regimes; it computes `I` and assumes the equality.

**Status.** Open. Mitigation: simulator output includes `"regime": "dissipative_assumed"` until detection logic is added.

### §1.5 Information-gradient `(∇I)`

**Problem.** `δ ∝ (∇I)²/Λ_Ze²` connects the deformation to the impedance gradient, but the gradient is over the abstract state-space metric. Phase 1 treats `δ` as an independent input. We do not compute `δ` from a given `I(state)` field.

**Status.** Phase 2 deferred. Requires a state-space parameterization that we do not yet have.

### §1.6 Connection to MCOA / CDATA

**Problem.** Top-level CLAUDE.md positions Ze as one of several "counters" in the MCOA framework (Counter for synchronization). The mathematical bridge from the canonical Ze quantities to a population-level aging counter is missing.

**Status.** Cross-subproject coordination needed (MCOA + Ze). Out of scope of Ze Phase 1.

---

## §2. Numerical / engineering

### §2.1 CHSH grid optimizer cost

**Problem.** F3 verifies `1.7478` via grid search. At `n=64`, the search is over `64⁴ ≈ 1.7 × 10⁷` quadruples — feasible (~seconds in release), but `n=256` (`4 × 10⁹`) is not. We use Nelder–Mead in production and grid only for the F3 verification at small `n`.

**Status.** Acceptable trade-off. Documented in PARAMETERS §2.

### §2.2 `BTAU_LIMIT` boundary behavior

**Problem.** When `β·I·τ → 1`, the `(1 − β·I·τ)` factor in the QFI lower bound goes to zero. The simulator returns "extrapolation_refused" beyond `BTAU_LIMIT = 1.0`. The transition is discontinuous in the API, which can confuse users sweeping through the boundary.

**Status.** Acceptable for now. UI plot will show a clear "regime boundary" line.

### §2.3 LiveView debounce vs `qfi_sweep` latency

**Problem.** A 200-ms debounce on slider updates is fine for single-point evaluations but `qfi_sweep` at `n=50` may take longer than 200 ms on some servers, causing a visible lag and queueing of LiveView events.

**Status.** Acceptable for Phase 1; revisit if profiling shows queuing.

---

## §3. Architectural / scope

### §3.1 Reaction–diffusion demos

**Problem.** Previous Ze interpretations had a 1D `(I, x, y)` RD system as a third simulator block ("autowaves"). It is retracted as a canonical primitive (`CONCEPT §6`) but may legitimately be reintroduced as a *demo* of impedance gradients in space.

**Status.** Deferred indefinitely. If reintroduced, must live under a clearly labeled `ze-simulator/src/demo/rd.rs` and not be exposed via the canonical HTTP API.

### §3.2 Quantum Darwinism / consciousness modeling

**Problem.** Source docx §6 (Quantum Darwinism) and §7 (consciousness, active inference) are *labeling* sections. They identify `I` with KL terms in existing frameworks. The simulator could in principle implement a Friston-style active-inference loop, but this is a separate research direction.

**Status.** Deferred. Not in TODO. Re-evaluate only if a concrete user need arises.

### §3.3 Multi-language interop with MCOA

**Problem.** MCOA subproject under LongevityCommon is in a different language stack (TBD). If MCOA wants to call `ze-simulator`, the cleanest path is the existing HTTP API. No FFI layer planned.

**Status.** Deferred until MCOA explicitly needs it.

### §3.4 Public deployment of `ze-backend`

**Problem.** Backend is loopback-only. Public deployment would need TLS, auth, rate limiting, and a story for distribution-sized DoS protection.

**Status.** Out of scope. Subproject is for local exploration.

---

## §4. Documentation / governance

### §4.1 Source-document drift

**Problem.** Master `~/Desktop/Ze Theory.docx` may be edited. The current core .md files are pinned to the version with mtime `Apr 28 00:16` and md5 (to be recorded on next commit). Without a strict regeneration ritual, drift is silent.

**Status.** Mitigation: `CONCEPT §8` defines the regeneration rule. Should be enforced by a startup check (not yet implemented).

### §4.2 Self-citation policy applicability

**Problem.** Memory rule "self-citations ≤15% in articles" applies to publications. `EVIDENCE.md` is internal documentation, but if it is reused in a paper draft, the rule kicks in. Make sure to filter Tkemaladze's own publications out before reusing this list.

**Status.** Procedural; flagged here as a reminder.

---

## §5. Process

### §5.1 No CI

**Problem.** F1–F6 tests are not run in CI; they pass on the developer machine but nothing prevents a future regression.

**Status.** Phase 2. Add a GitHub Actions workflow that runs `cargo test -p ze-simulator` on push.

### §5.2 No regeneration script

**Problem.** Re-running the docx-to-md conversion and a consistency check (CONCEPT §3 ↔ THEORY §§2–5 ↔ PARAMETERS §1) is currently manual.

**Status.** Phase 2 nice-to-have.

---

## §6. How to retire an entry

When an open problem is closed:

1. Don't delete the entry — change its **Status** line to `**Resolved 2026-MM-DD: <pointer to commit / file>**`.
2. Move the resolved entry to a new section `## §∞. Resolved` at the bottom of this file.
3. If the resolution changed any of the canonical quantities, also bump CONCEPT version per CONCEPT §8.
