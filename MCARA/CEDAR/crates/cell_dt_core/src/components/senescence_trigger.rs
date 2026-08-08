/// Triple-clock senescence trigger (CEDAR v3.5; updated v4.6; rDNA clock v4.7).
///
/// Replicative senescence can be triggered by any molecular clock independently:
///   1. Centriolar damage: D(t) ≥ D_crit (the primary CEDAR clock)
///   2. Telomere shortening: TL(n) ≤ TL_crit (classical Hayflick mechanism)
///   3. rDNA copy loss: R(n) ≤ R_crit (TRCS model, Huang 2026; Kobayashi 2014;
///      Defossez 1999 — 45S rDNA arrays as a second nuclear countdown substance,
///      shortening elevates p53 and drives senescence)
///
/// ## Scientific basis (CEDAR v4.6)
///
/// **C3 — Centriolar dysfunction directly induces p16-senescence:**
/// - SVBP/VASH pathway: biallelic SVBP variant (p.Leu49Pro) → centrosome cohesion
///   abnormalities → p16^INK4a ×3.4 in patient PBMCs → premature senescence
///   (Launay et al. 2025, Aging Cell 24:e14355, PMID 39412222).
/// - PLK4-inhibition pathway: prolonged PLK4 inhibition → centriole loss →
///   senescence, polyploidy, defective cytokinesis
///   (Dang et al. 2023, Blood 142:2002; Hamzah et al. 2025, Cytoskeleton, PMID 40257113).
/// - Stem cells in telomerase-active niches still exhibit finite replicative
///   lifespan (Peters-Hall et al. 2020: >200 PD at 2% O₂) — centriolar clock.
/// - Somatic differentiated cells: all clocks active; whichever reaches
///   threshold first triggers p16/p21 pathway and permanent cell cycle arrest.
///
/// ## rDNA clock (v4.7, TRCS — Telomere DNA and Ribosomal DNA Co-Regulation Model)
///
/// Huang (2026, Ageing Longev. Res. 2(1):2) proposes 45S rDNA arrays as a second
/// nuclear countdown substance: shortening of telomere and/or rDNA arrays elevates
/// p53 and triggers senescence. Knockdown of 45S rDNA copy number upregulates
/// p53/p21/p16/SA-β-GAL. rDNA copy number is significantly decreased in senescent
/// cells, increased in hESCs/iPSCs (rejuvenation = rDNA/telomere lengthening).
/// Supporting: Kobayashi (2014, Proc Jpn Acad B 90:119) — rDNA stability and
/// senescence; Defossez (1999, MCB 19:3848) — DNA repair mutations → rDNA circles
/// → shorter lifespan in yeast.
///
/// Reference: Tqemaladze & Chichinadze (2005) proposed centrosome-driven
/// replicative aging. CEDAR v3.5 formalises the dual-clock model; v4.7 adds the
/// rDNA clock from TRCS (Huang 2026).
use serde::{Deserialize, Serialize};

/// Which molecular clock triggered senescence onset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SenescenceTrigger {
    /// D(t) reached D_crit (1000 a.u.).
    /// Primary mechanism in stem cells / telomerase-positive niches.
    CentriolarDamage,
    /// Telomere length fell below TL_crit (≈5 kb for human fibroblasts).
    /// Primary mechanism in differentiated somatic cells.
    TelomereShortening,
    /// 45S rDNA copy number fell below R_crit (TRCS: telomere + rDNA co-regulation
    /// of cell senescence; rDNA contributes heavily to senescence — Huang 2026).
    RdnDnaShortening,
    /// Both thresholds reached simultaneously (rare; seen in very old cells).
    Both,
    /// Cell has not yet reached senescence.
    None,
}

impl SenescenceTrigger {
    /// Determine which clock triggered given current damage, telomere and rDNA values.
    ///
    /// Thresholds:
    /// - `d_crit`: default 1000.0 a.u. (normalised)
    /// - `tl_crit`: default 5.0 kb (human fibroblast; adjust per cell type)
    /// - `rdna_crit`: default 0.5 (normalised 45S rDNA copy number; half of the
    ///   young-adult ~300–400 copies — TRCS threshold, Huang 2026)
    ///
    /// Priority when multiple clocks fire simultaneously:
    /// centriolar > telomere > rDNA (CEDAR keeps the centriolar clock as primary;
    /// `Both` is preserved for the historical centriolar+telomere coincidence).
    pub fn evaluate(
        damage_normalized: f64,
        d_crit: f64,
        telomere_kb: f64,
        tl_crit: f64,
        rdna_normalized: f64,
        rdna_crit: f64,
    ) -> Self {
        let centriolar = damage_normalized >= d_crit;
        let telomere = telomere_kb <= tl_crit;
        let rdna = rdna_normalized <= rdna_crit;
        match (centriolar, telomere, rdna) {
            (true, true, _)   => SenescenceTrigger::Both,
            (true, false, _)  => SenescenceTrigger::CentriolarDamage,
            (false, true, _)  => SenescenceTrigger::TelomereShortening,
            (false, false, true) => SenescenceTrigger::RdnDnaShortening,
            (false, false, false) => SenescenceTrigger::None,
        }
    }

    /// Returns true if any senescence has been triggered.
    pub fn is_senescent(&self) -> bool {
        !matches!(self, SenescenceTrigger::None)
    }

    /// Returns the dominant clock (for logging/reporting).
    /// CentriolarDamage is returned for Both (to preserve CEDAR primary narrative).
    pub fn dominant_clock(&self) -> Option<&'static str> {
        match self {
            SenescenceTrigger::CentriolarDamage => Some("centriolar"),
            SenescenceTrigger::TelomereShortening => Some("telomere"),
            SenescenceTrigger::RdnDnaShortening => Some("rdna"),
            SenescenceTrigger::Both => Some("centriolar"), // CEDAR dominant
            SenescenceTrigger::None => None,
        }
    }
}

impl std::fmt::Display for SenescenceTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CentriolarDamage    => write!(f, "CentriolarDamage"),
            Self::TelomereShortening  => write!(f, "TelomereShortening"),
            Self::RdnDnaShortening    => write!(f, "RdnDnaShortening"),
            Self::Both                => write!(f, "Both"),
            Self::None                => write!(f, "None"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const D_CRIT: f64 = 1000.0;
    const TL_CRIT: f64 = 5.0;
    const RDNA_CRIT: f64 = 0.5;
    const RDNA_FULL: f64 = 1.0; // молодой уровень 45S rDNA

    #[test]
    fn test_no_senescence_below_both_thresholds() {
        let t = SenescenceTrigger::evaluate(500.0, D_CRIT, 8.0, TL_CRIT, RDNA_FULL, RDNA_CRIT);
        assert_eq!(t, SenescenceTrigger::None);
        assert!(!t.is_senescent());
    }

    #[test]
    fn test_centriolar_trigger_only() {
        let t = SenescenceTrigger::evaluate(1000.0, D_CRIT, 8.0, TL_CRIT, RDNA_FULL, RDNA_CRIT);
        assert_eq!(t, SenescenceTrigger::CentriolarDamage);
        assert!(t.is_senescent());
        assert_eq!(t.dominant_clock(), Some("centriolar"));
    }

    #[test]
    fn test_telomere_trigger_only() {
        let t = SenescenceTrigger::evaluate(500.0, D_CRIT, 4.9, TL_CRIT, RDNA_FULL, RDNA_CRIT);
        assert_eq!(t, SenescenceTrigger::TelomereShortening);
        assert!(t.is_senescent());
        assert_eq!(t.dominant_clock(), Some("telomere"));
    }

    #[test]
    fn test_rdna_trigger_only() {
        // TRCS: падение 45S rDNA ниже порога (0.5) вызывает сенесценцию
        let t = SenescenceTrigger::evaluate(500.0, D_CRIT, 8.0, TL_CRIT, 0.49, RDNA_CRIT);
        assert_eq!(t, SenescenceTrigger::RdnDnaShortening);
        assert!(t.is_senescent());
        assert_eq!(t.dominant_clock(), Some("rdna"));
    }

    #[test]
    fn test_both_triggered() {
        let t = SenescenceTrigger::evaluate(1001.0, D_CRIT, 4.0, TL_CRIT, RDNA_FULL, RDNA_CRIT);
        assert_eq!(t, SenescenceTrigger::Both);
        assert!(t.is_senescent());
        assert_eq!(t.dominant_clock(), Some("centriolar")); // CEDAR dominant
    }

    #[test]
    fn test_boundary_damage_exactly_at_crit() {
        // d == d_crit triggers (>=)
        let t = SenescenceTrigger::evaluate(D_CRIT, D_CRIT, 8.0, TL_CRIT, RDNA_FULL, RDNA_CRIT);
        assert_eq!(t, SenescenceTrigger::CentriolarDamage);
    }

    #[test]
    fn test_boundary_telomere_exactly_at_crit() {
        // tl == tl_crit triggers (<=)
        let t = SenescenceTrigger::evaluate(500.0, D_CRIT, TL_CRIT, TL_CRIT, RDNA_FULL, RDNA_CRIT);
        assert_eq!(t, SenescenceTrigger::TelomereShortening);
    }

    #[test]
    fn test_boundary_rdna_exactly_at_crit() {
        // r == r_crit triggers (<=)
        let t = SenescenceTrigger::evaluate(500.0, D_CRIT, 8.0, TL_CRIT, RDNA_CRIT, RDNA_CRIT);
        assert_eq!(t, SenescenceTrigger::RdnDnaShortening);
    }

    #[test]
    fn test_rdna_full_no_trigger() {
        let t = SenescenceTrigger::evaluate(500.0, D_CRIT, 8.0, TL_CRIT, 1.0, RDNA_CRIT);
        assert_eq!(t, SenescenceTrigger::None);
    }

    #[test]
    fn test_rdna_secondary_when_centriolar_fired() {
        // Приоритет: centriolar > rDNA (CEDAR — первичные часы)
        let t = SenescenceTrigger::evaluate(1000.0, D_CRIT, 8.0, TL_CRIT, 0.2, RDNA_CRIT);
        assert_eq!(t, SenescenceTrigger::CentriolarDamage);
    }

    #[test]
    fn test_rdna_secondary_when_telomere_fired() {
        // Приоритет: telomere > rDNA
        let t = SenescenceTrigger::evaluate(500.0, D_CRIT, 4.0, TL_CRIT, 0.2, RDNA_CRIT);
        assert_eq!(t, SenescenceTrigger::TelomereShortening);
    }

    #[test]
    fn test_display_none() {
        assert_eq!(format!("{}", SenescenceTrigger::None), "None");
    }

    #[test]
    fn test_display_both() {
        assert_eq!(format!("{}", SenescenceTrigger::Both), "Both");
    }

    #[test]
    fn test_display_rdna() {
        assert_eq!(format!("{}", SenescenceTrigger::RdnDnaShortening), "RdnDnaShortening");
    }

    #[test]
    fn test_clone_eq() {
        let t = SenescenceTrigger::CentriolarDamage;
        assert_eq!(t, t.clone());
    }
}
