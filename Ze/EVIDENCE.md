# Ze · EVIDENCE

**Status:** Empirical-support inventory · regenerated 2026-04-28
**Source:** `~/Desktop/Ze Theory.docx` §3 + §9.1 references list.

This file lists the **published evidence** that grounds each derivation in `THEORY.md`. The simulator does not depend on these references at runtime, but every claim in `CONCEPT.md` and `THEORY.md` must be traceable to one or more of them.

---

## §1. Pillars and supporting publications

| THEORY § | Claim | Supporting publication | Status of evidence |
|----------|-------|------------------------|--------------------|
| §2.1 (Lemma A) | `I = ⟨ΔS⟩_gen` for dissipative processes | Burgholzer P. (2015), *Information loss and entropy production during dissipative processes in a macroscopic system kicked out of equilibrium*, **arXiv:1502.00214**. | Theorem; rigorous derivation from Jarzynski/Crooks. |
| §2.2 (central law experimental basis) | Clock accuracy ∝ entropy produced per tick | Pearson A. N., Guryanova Y., Erker P., Laird E. A., Briggs G. A. D., Huber M., Ares N. (2021), *Measuring the thermodynamic cost of timekeeping*, **Phys. Rev. X 11(2), 021029**. | Experimental; nanoscale resonating-membrane clock, classical and quantum regimes. Quantum bound `N = ΔS_tick / 2k_B`. |
| §3 (Lemma B information-geometric expansion) | Fisher metric provides natural quadratic expansion; geodesic minimizes entropy | Gassner S., Cafaro C., Ali S. A., Alsing P. M. (2021), *Information geometric aspects of probability paths with minimum entropy production for quantum state evolution*, **Int. J. Geom. Methods Mod. Phys. 18(8), 2150127**. | Mathematical; provides the metric structure used in §3 and §4. |
| §3.4 (CHSH protocol) | Asymmetric CHSH inequalities are tight and robust | Woodhead E., Acín A., Pironio S. (2021), *Device-independent quantum key distribution based on asymmetric CHSH inequalities*, **Quantum 5, 443**. | Theoretical + experimental; tolerable error rate 7.15% → 7.42% for depolarizing channel. Provides protocol to extract `δ`. |
| §4.1 (entropic speed) | Entropic speed `v` along geodesic relates to entropy production | Gassner et al. (2021) *(same)* | "A faster transfer (higher entropic speed) is associated with a higher rate of entropy production." |
| §5.1 (Abboud bound) | LGI violations bound QFI: `F_Q ≥ 8[K(τ) − ⟨Q²⟩]` | Abboud H., Plávala M., Kleinmann M., Gühne O. (2026), *Leggett–Garg inequality violations bound quantum Fisher information*, **arXiv:2604.09772**. | Theorem; for stationary pure states and thermal states. |
| §6 (Quantum Darwinism, retracted from canonical sim but kept as labeling) | Redundant encoding → classical objectivity | Zurek W. H. (2003), **Rev. Mod. Phys. 75(3), 715–775**; Ollivier H., Poulin D., Zurek W. H. (2005), **Phys. Rev. A 72(4), 042113**. | Theoretical framework. Used in CONCEPT §1 narrative only. |
| §6 (metrological objectivity emergence) | QFI provides metric for objectivity emergence rate | Kiely A., Chisholm D. A., Touil A., Deffner S., Landi G., Campbell S. (2026), **Phys. Rev. A 113(2), 022403**. | Theoretical; supports `F_Q ∝ proper time consumption` interpretation. |
| §6 (active inference labeling) | Variational free-energy decomposition has KL term | Friston K. (2017), *The free-energy principle: A unified brain theory?*, in *The Routledge Companion to Philosophy of Psychology*, 2nd ed. | Conceptual; identifies `I` with the KL term in active inference. Not used in simulator. |
| §6 (psychedelics labeling) | Psychedelics relax priors → increase prediction error | Carhart-Harris R. L., Friston K. J. (2019), *REBUS and the anarchic brain*, **Pharmacological Reviews 71(3), 316–344**. | Phenomenological; informs the consciousness-as-error-reduction framing. Not used in simulator. |
| §3.5 (observer-dependent reality, narrative only) | Bell-type test in extended Wigner's friend setup | Proietti M., Pickston A., Graffitti F., et al. (2019), *Experimental test of local observer independence*, **Sci. Adv. 5(9), eaaw9832**. | Experimental; 5σ violation. |

---

## §2. Confirmation status

| Claim | Status |
|-------|--------|
| `dτ_Ze/dt ∝ −Ṡ_gen` (central law, classical regime) | **Confirmed** by Pearson et al. (2021). Nanoscale clock quantum bound `N = ΔS_tick / 2k_B` directly validates linear scaling. |
| `I = ⟨ΔS⟩_gen` (information–entropy equality) | **Proven** as theorem (Burgholzer 2015), within stated regime (Markovian dissipative, Jarzynski/Crooks-applicable). |
| `S_Ze = 2√2 + δ·1.7478` (CHSH deformation) | **Predicted** — not confirmed. Detection requires asymmetric CHSH protocol (Woodhead 2021) at `~10⁹` coincidence count for 5σ. |
| `F_Q ∝ |dτ_Ze/dt|` (Theorem 1) | **Predicted** — not confirmed. Requires LGI–QFI protocol (Abboud 2026) on systems with controlled decoherence. |
| `C(τ) = C₀ exp(−β·I·τ)` exact form | **Leading-order** result. Empirically robust for many systems but not a universal law. |

---

## §3. What this list deliberately omits

- **Biomedical / clinical claims.** None. The previous interpretations of `χ_Ze` as EEG biomarker, plasma SASP synchronization index, etc., are retracted (CONCEPT §6) and have no place in EVIDENCE.
- **Speculative cosmology.** "Time accelerates inside horizon" (CLAUDE.md flagged this). Not in canonical THEORY.md, not in EVIDENCE.
- **Self-citation.** This is a theoretical/mathematical project; the per-paper self-citation rule (≤15%, see auto-memory) does not apply to the EVIDENCE inventory of an internal docs file. Tkemaladze's own publications referencing Ze Theory are cited only in CONCEPT §1 (the source-document citation block).

---

## §4. Update procedure

When a new publication relevant to Ze appears:

1. Add a row to §1 with `THEORY §` of the claim it supports, full citation, and a one-line "status of evidence."
2. If it changes a derivation: open `OPEN_PROBLEMS.md` to track the consistency check.
3. If it confirms a prediction in §2: change "Predicted" → "Confirmed" with citation.
4. Never delete an entry — mark superseded entries with `~~strikethrough~~` and a forwarding pointer.
