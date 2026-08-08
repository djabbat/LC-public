# Ze Model — Formal Axiomatics


**Type:** mathematical model only — see CONCEPT.md for details.
**Version:** 1.0

## Concept

**Ze Model** is a fundamental theory in which spacetime and quantum correlations emerge from a more primitive notion: **global knowledge**.

Knowledge differs from passive memory in that it is an **active predictive resource** — the state of a system represents not merely a record of past events, but an informational basis for generating expectations about future outcomes.

## Five axioms

### A1: Time as a fundamental parameter
One-dimensional parameter t ∈ ℝ, ordering events. Time is postulated, not derived.

### A2: Discrete local dynamics
Evolution at a point x depends only on its causal past:
$$S(x, t+1) = \Phi(\{S(y, t) : y \in J^-(x, t)\})$$

### A3: Knowledge as a code of the past
The current state fully encodes the past: S(t) = K(past). Historical information is not lost.

### A4: Prediction error
$$\eta(t) = d(\hat{S}(t+1), S(t+1))$$
where d is a metric. Evolution minimizes cumulative error. η(t) is the fundamental "cost" of prediction.

### A5: Nonlocal knowledge (global connectivity)
Knowledge is nonlocal. Global knowledge states connect spatially separated regions without local exchange. Evolution remains local (A2), but the knowledge state is nonlocally connected.

## Key results

1. **State space** — Hilbert space H, basis |i⟩ (elementary units of knowledge)
2. **Unitary evolution** — from A3 (preservation of distinguishability)
3. **Born rule** — postulated: p_i = |⟨i|Ψ⟩|², Σ|c_i|² = 1
4. **Tsirelson bound** — S = 2√2 for CHSH (from projector structure)
5. **Barrett loophole** — not applicable (Ze: P(a|A) = |⟨a|Π_A|Ψ⟩|², independent of past outcomes)
6. **Yang's trichotomy** — Ze violates ODL (A5), preserves MI, rejects classical ontology

## Falsifiable predictions

- In non-equilibrium (Δt < τ_knowledge): |S_Ze(N) − S_QM| ≥ γ/√N
- Knowledge relaxation: τ_knowledge ≥ ℏ/ΔE
- Temporal CHSH correlations: ⟨S(t)·S(t+Δt)⟩_Ze − ⟨S(t)·S(t+Δt)⟩_QM > 0

## Connection with subprojects

| Subproject | Connection with Ze Model |
|-----------|------------------|
| Ze_CHSH | S(H) = 2√2 − βH — direct consequence of Ze Model |
| D_Ze | Multilayer age — extension of A3, D_Ze — consequence of A5 |
| Ze-Hierarchy | Experimental verification on bristlebot swarms |

## Consumables (annual)

| **Office consumables** (printing, stationery, toner) | **$300** |

## Hypothesis

*To be specified — see CONCEPT.md §1 for project rationale.*

## References

*See project MEMORY.md for reference history.*


## Budget

Theory — no budget needed. Detailed budget in PARAMETERS.md.
