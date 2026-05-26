/// GET /api/disclosures/v5_changes
///
/// Public, no-auth endpoint returning the full v5 (2026-04-28) change-set:
/// what was retracted, what was added as honest-disclosure, what is
/// hypothesis-stage, and where the upstream LC CONCEPT lives.
///
/// Mirrors umbrella `~/Desktop/LC/CONCEPT.md` v5.6 §2 + STATE.md §5.
/// Clients SHOULD surface the `summary` and `headline_blockers` fields prominently.
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct V5Disclosures {
    pub longevitycommon_version: &'static str,
    pub effective_date: &'static str,
    pub status: &'static str,
    pub summary: &'static str,
    pub retracted_or_deprecated: Vec<&'static str>,
    pub hypothesis_stage_caveats: Vec<&'static str>,
    pub headline_blockers: Vec<&'static str>,
    pub authority: &'static str,
    pub upstream_concept_path: &'static str,
}

pub async fn get_v5_changes() -> Json<V5Disclosures> {
    Json(V5Disclosures {
        longevitycommon_version: "v5.6",
        effective_date: "2026-04-28",
        status: "hypothesis-stage research framework; not a validated medical product",
        summary: "LC is a hypothesis-stage integrative ecosystem. \
                  All χ_Ze values, AUC scores, and aging-activity estimates surfaced by this API \
                  are exploratory (hypothesis-generating), NOT confirmatory. Pre-registered tests \
                  of an earlier univariate χ_Ze formulation gave NULL results (deprecated/superseded). \
                  Current multimodal χ_Ze is a post-hoc reformulation, not yet validated on a \
                  pre-registered N≥2000 cohort. p-hacking risk per Ioannidis 2005 (PMID 16060722) \
                  applies. Not medical advice.",
        retracted_or_deprecated: vec![
            "v1 univariate χ_Ze formulation: pre-registered NULL results on Cuban EEG, Dortmund Vital, MPI-LEMON cohorts (deprecated; superseded by v2 multimodal)",
            "Schultz & Sinclair 2019 PMID 30982602 reference: misciation (PMID 30982602 = Kucab 2019, not Schultz/Sinclair) — removed from all docs",
            "Miller 2025 PpgAge reference: not verifiable via PubMed/arXiv — removed",
            "Shim & Onnela 2025 NHATS reference: not verifiable — removed",
            "Rando, Brunet & Goodell 2025: pending DOI — removed pending verification",
            "Old `R² < 0.5` arbitrary M4 threshold: superseded by community-standard `partial r² < 0.05` on N≥2000, α=0.001",
            "Old FCLC `privacy-by-design without caveats`: replaced by explicit semi-honest-only threat model",
            "Old `dτ/dt = -α·I derived from Burgholzer/Pearson`: relabeled to POSTULATED ansatz",
            "Old multimodal weights `(0.30, 0.30, 0.20, 0.20)` framed as theory-fixed: now disclosed as post-hoc pilot fit",
            "Old `v* identifiability impossible due to χ_Ze normalisation`: incorrect; swept-v* search on All-of-Us N=500 gave v* = 0.451 (95% CI 0.443-0.459), consistent with theoretical 0.45631",
            "Old realtime port 4000: conflicted with Ze Phoenix; moved to 4500",
        ],
        hypothesis_stage_caveats: vec![
            "All χ_Ze readings exploratory; no clinical action implied",
            "AUC values reported anywhere are subject to multiple-testing inflation (no Bonferroni / BH correction applied)",
            "CDATA centriolar damage hypothesis: status INCONCLUSIVE; Sobol p=0.12 after correction; full S2/ST decomposition deferred to Cell-DT v4.0 on real GTEx data N=948",
            "Ze Theory bridge to biological aging: POSTULATED, not derived; underlying Burgholzer/Pearson results apply to PHYSICAL clocks only",
            "Bridge to CDATA (5 free parameters on N=196): underpowered (39 obs/param < Harrell 10/param); moved to Supplementary",
            "FCLC v13.4 PASS milestone is semi-honest-only secure; NOT secure against active server collusion or malicious server; GDPR Article 9 blocker until FCLC v14 (planned Q1 2027)",
            "MCAOA submitted to Nature Aging (NATAGING-P13741) but NOT peer-reviewed at the time of this disclosure",
        ],
        headline_blockers: vec![
            "Confirmatory validation: pending pre-registered N≥2000 cohort (UK Biobank wearable subset / All-of-Us / Aqtivirebuli pilot)",
            "EIC Pathfinder Challenges 2026: 0 signed EU LoIs as of 2026-04-21 (Geiger Ulm confirmed 2026-04-23 = 1; Janke Curie pending; Miguel Angel González Ballester UPF in active discussion)",
            "FCLC v14 malicious-secure migration: planned Q1 2027",
            "Nature Aging review of MCAOA: pending decision",
        ],
        authority: "LC CONCEPT.md > <subproject>/CONCEPT.md > <subproject>/THEORY.md > simulator code",
        upstream_concept_path: "~/Desktop/LC/CONCEPT.md (umbrella v5.6, 2026-04-28)",
    })
}
