/// Кишечный микробиом
/// 3 отдела: тонкий кишечник, толстый кишечник, прямая кишка
/// Влияет на L_intestinal, L_immune через SCFA, LPS, цитокины
/// INFOGEST-совместимость: Brodkorb 2019 (DOI 10.1038/s41596-018-0119-1)
/// В v1.0: заглушка. Реализация — Phase 3.
pub struct GutMicrobiome {
    pub diversity_index: f64,
    pub scfa_production: f64,
    pub pathogens_ratio: f64,
}
