<!-- AUTO-TRANSLATED via DeepSeek 2026-05-13. Source language: mixed. Original preserved at OPEN_PROBLEMS.x.md. -->

# LC · OPEN_PROBLEMS

**Status:** Cross-cutting open issues · 2026-04-28 (CONCEPT v5.6)
**Authority:** Subproject-level open problems remain in `<subproject>/OPEN_PROBLEMS.md`; cross-cutting only here.

---

## §1. Scientific (cross-subproject)

### §1.1 Pre-registered confirmatory cohort (N ≥ 2000)
**Problem:** All pilots underpowered (N=150 for BioSense, N=196 Cuban, N=2222 All-of-Us without pre-registration). M4 falsifiability requires N≥2000 at α=0.001 for partial r²<0.05 mortality test.
**Status:** open. **Largest blocker for Nature Aging-tier submission.**
**Path forward:** UK Biobank wearable subset DUA + cost (years), All-of-Us Researcher Workbench DUA (months), or new prospective cohort via Aqtivirebuli clinical pilot.

### §1.2 CDATA inconclusive (Sobol nested CV)
**Problem:** ABL-2 paradox (R²_no_α=0.833 vs full=0.778) appears in synthetic Sobol — not statistically significant after nested CV (p=0.12). Full S1+S2+ST decomposition on **real GTEx data N=948** required to determine if CDATA contributes uniquely.
**Status:** open. Tracked in `CDATA/OPEN_PROBLEMS.md`; depends on Cell-DT v4.0 build.
**Path forward:** Cell-DT v4.0 (planned, not started); GTEx access already available.

### §1.3 Ze Theory → biology bridge (formal)
**Problem:** `dτ_Ze/dt = −α·I(Z)` postulated by analogy with Burgholzer 2015 + Pearson 2021 (physical clocks). Formal derivation for biological systems is absent. v5 honest relabel to "ansatz" not "derivation" applied — but hypothesis-stage.
**Status:** open theoretical problem. Empirical validation `v* = 0.451 ± 0.008` consistent with theory but does not prove bridge.
**Path forward:** swept-v* on extended cohort + cross-modality replication.

### §1.4 v* full sensitivity & identifiability
**Problem:** v5 identifiability claim "impossible due to χ_Ze normalisation" was reversed. Swept-v* search IS feasible. Done on All-of-Us N=500 with `v*_optimal = 0.451 (95% CI 0.443-0.459)` — consistent.
**Status:** partially resolved (v5). Full pre-registered swept-v* protocol on N≥2000 cohort pending.
**Path forward:** specify exact protocol (cohort, N, power analysis, primary outcome, falsification criterion) — currently outline only.

### §1.5 Multimodal weights post-hoc
**Problem:** `(0.30, 0.30, 0.20, 0.20)` for (EEG/HRV/resp/sleep) — fitted on N=150 underpowered pilot; not theory-fixed.
**Status:** open. Weights need re-fit on pre-registered cohort.
**Path forward:** lock weights via blinded fit on hold-out portion of N≥500 cohort before any unblinding.

### §1.6 Bridge to CDATA (5 free params)
**Problem:** `A(D) = a + bD + cD²; χ_Ze = g₀ − g₁A` has 5 free params on N=196 → 39 obs/param; below Harrell standard 10/param for stable fit.
**Status:** moved to Supplementary in article v5; remains open.
**Path forward:** simpler 2-param linear bridge OR theory-derived constraints.

## §2. Engineering / infrastructure

### §2.1 FCLC malicious-secure migration (v14)
**Problem:** v13.4 PASS milestone is **semi-honest server only** + Byzantine-robust (Krum ≤25%); NOT secure against active server collusion or malicious server. **GDPR Article 9 blocker** for medical data.
**Status:** open. v14 planned Q1 2027.
**Path forward:** integrate cryptographic protocol (verifiable secret sharing + zero-knowledge proofs) — significant engineering work.

### §2.2 ε double-counting fix verification
**Problem:** v4 had double-counting between BioSense ε_local=2.0 (on-device) and FCLC ε_total=0.43 (federated). v5 corrected via formal Rényi α=2 composition (Mironov 2017) → ε_total ≈ 2.4 combined.
**Status:** addressed in article v5 §6; cross-check with FCLC `dp/recalibration_v13.rs` to ensure code matches doc.
**Path forward:** audit FCLC code; if mismatched, bump composition module.

### §2.3 Real EEGLAB / EDF readers in Rust
**Problem:** Datasets crate (BioSense Phase 2) has skeleton loaders for LEMON/Cuban/Dortmund but actual `.set` (EEGLAB) and `.edf` parsers not implemented in Rust (Python pipelines used MNE; Rust has no mature equivalent).
**Status:** open. Phase 2 backlog in `BioSense/datasets/MIGRATION_NOTES.md`.
**Path forward:** either port MNE EEGLAB reader to Rust, OR shell out to Python sidecar, OR pre-convert datasets to a common format.

### §2.4 Subproject CONCEPT.md alignment
**Problem:** After umbrella CONCEPT.md regeneration to v5.6, subproject CONCEPTs may contain stale cross-cutting language (e.g., older "DERIVE" wording in Ze, older "validated" wording in BioSense, older threat model in FCLC).
**Status:** open. Audit and patch needed.
**Path forward:** diff each subproject CONCEPT against new umbrella CONCEPT §3 + §5 + §7; pull any contradictions back to umbrella authority.

### §2.5 Realtime port conflict (4001)
**Problem:** `realtime/config/dev.exs` previously assumed port 4001, conflicts with `Ze/ze-backend` on 4001.
**Status:** flagged in `DESIGN.md §5.3`; not yet fixed in actual config file.
**Path forward:** edit `realtime/config/dev.exs` → port 4500 + update `deploy/docker-compose-all.yml`.

## §3. Process / governance

### §3.1 No CI for full umbrella stack
**Problem:** Subproject tests (Ze cargo, BioSense cargo, FCLC tests) run autonomously but no umbrella-level integration test (subprojects + social server + web).
**Status:** open. Phase 3 of TODO.
**Path forward:** GitHub Actions umbrella workflow.

### §3.2 No regen script for core .md
**Problem:** Regenerating CONCEPT/THEORY/DESIGN/PARAMETERS/MAP/EVIDENCE/OPEN_PROBLEMS/STATE from article is manual. Drift can accumulate silently.
**Status:** open. Phase 3 nice-to-have.
**Path forward:** `scripts/regen_umbrella_core_from_article.sh` (placeholder).

### §3.3 Cross-subproject API mock
**Problem:** Social server delegates χ_Ze computation to BioSense backend; integration tests on social server require BioSense up. No mock currently.
**Status:** open. Engineering quality issue.
**Path forward:** add mock layer in social server tests.

## §4. Strategic / external

### §4.1 0 signed EU LoIs (EIC blocker)
**Problem:** EIC Pathfinder Challenges 2026 deadline 2026-10-28 requires consortium with ≥1 EU-MS + ≥2 different MS/AC. Geiger (Ulm DE) ✅ confirmed; Janke (Curie FR) pending; Miguel Angel González Ballester (UPF ES) — meeting today 2026-04-28 12:30.
**Status:** open. **Largest non-scientific blocker.**
**Path forward:** today's meeting outcome is gating item.

### §4.2 PhD enrollment (Lezhava single-point-of-failure)
**Problem:** Lezhava 76+ years; PhD timeline until 2027-04. Risk of supervisor unavailability over 3-year cycle.
**Status:** open. Backup co-supervisor search at TSU (Trapaidze, Dzidziguri).
**Path forward:** initiate co-supervisor approach by 2026-08.

### §4.3 ICD-11 MG2A overclaim (article §1.1)
**Problem:** Article phrase implies WHO recognized aging as a disease — MG2A actually defines decline in intrinsic capacity, not aging itself.
**Status:** flagged in v3 review; cosmetic fix needed in §1.1.
**Path forward:** rephrase in next iteration.

### §4.4 Abstract vs §5.6 terminology contradiction
**Problem:** Article Abstract uses "retracted"; §5.6 uses "deprecated/superseded". Should unify.
**Status:** flagged in v3 review.
**Path forward:** unify to "deprecated/superseded" in the entire document at the next revision.

## §5. Retirement procedure

When a problem is closed:
1. Don't delete — set status to `**Resolved 2026-MM-DD: <pointer>**`.
2. Move to `## §∞. Resolved` at bottom.
3. If resolution changes any cross-cutting item → bump CONCEPT version per `CONCEPT §10`.