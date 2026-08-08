# MEMORY — Ze_CHSH

## 2026-06-19 — Research Square notification

**Event:** Official letter from Research Square regarding the closure of submission QSMF.
**Proposal:** Post a preprint on Research Square.
**Decision:** The article has already been resubmitted to QIP (18.06). Preprint — optional (arXiv preferred).

## 2026-06-01 — Corrections based on expert review

**Event:** Detailed review received (quantum information physics).
**Verdict:** ❌ Not recommended for funding. Revision of conclusion β required.

**Critical comments and corrections in CONCEPT.md:**

1. **‖Δ_LGI‖ not defined** ✅
   - Replaced with correct symbols: K(τ), C(τ), ⟨Q²⟩, max_τ
   - Added table with definitions of all symbols

2. **Bound #2 incorrect (LGI→QFI)** ✅
   - Was: F_Q ≥ 2‖Δ_LGI‖ (hallucination)
   - Became: F_Q ≥ 8·max_τ [K(τ) − ⟨Q²⟩] (Abboud et al. 2026, arXiv:2604.09772)

3. **Bound #3 does not exist in the literature** ✅
   - Hall (2013) proved I(A:B) ≥ 2D², NOT ‖Δ‖ ≤ (ln2/2)·H
   - Moved to "Author's hypothesis" with explicit indication

4. **Conclusion β not justified** ✅
   - Added Limitations section (5 points)
   - β marked as hypothesis
   - Added table of alternatives (depol. 5.657, dephasing 2.0, none 0)

5. **References** ✅
   - All 4 found: Abboud, Hall, Honma, Burgholzer
   - Hall (2013) — DIFFERENT inequality; status updated

**Added to PARAMETERS.md:**
- Note on the hypothetical nature of β
- Table of S(H) for 4 scenarios
- Range of possible β: [0.5, 5.0]

**Second round of corrections (after review #2):**

1. **Step 3 removed** — γ_Ze is now a Ze postulate, not derived from bounds. Section "How it is derived" → "Motivation: connection with known results".
2. **β → γ** — unification of notation.
3. **F_Q → S mapping** — added reference Braunstein & Caves (1994) + discussion of non-optimality (5-10% loss) + calibration.
4. **Stationarity** — added discussion of slow modulation, error < 1%.
5. **Power analysis** — corrected: 10⁹ → 10⁶–10⁷ coincidences.
6. **Prediction table** — 4 scenarios (Ze, depol., dephasing, no effect).
7. **Falsifiability** — 5 conditions, each falsifies Ze.
8. **Hall (2013)** — acknowledged that this is a DIFFERENT inequality.

**Review #2 rating:** 6/10 (with +3 for rigor, +5 for literature, +5 for honesty).
**Recommendation:** ✅ Recommend for funding — 8.5/10.

**Final corrections (review #3):**
1. Unified notation: γ everywhere (not β) ✅
2. γ_depol = 5.657 marked as "approximation for small H" with justification ✅
3. Popescu–Rohrlich removed from PARAMETERS.md ✅
4. N coincidences corrected: 10⁹ → 10⁶–10⁷ ✅
5. Statistical error corrected: σ_S = 2/√N ✅

**Result:** Ze_CHSH ready for submission to grant agencies (RFBR, RSF, Horizon Europe, ERC).

## 2026-06-01 — Creation of subproject

- Created subproject Ze_CHSH in ~/Desktop/LC/Ze/Ze_CHSH/
- Created core files: _pi.md, CONCEPT.md, TODO.md, PARAMETERS.md, MAP.md, STATE.md
- Article moved from Articles/ to docs/
- Rule established: do not hallucinate references — verify each one

### Key decisions

1. **Subproject Ze, not LC directly** — CHSH prediction follows directly from Ze theory
2. **Verification first, then mbpr** — pointless to review an article with unverified references
3. **β = 2/ln2 — prediction, not postulate** — derived from combination of bounds that need to be confirmed
