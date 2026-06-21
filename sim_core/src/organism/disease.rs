/// Болезни как следствие L_tissue > L_crit
/// В v1.0: детекция в organism/mod.rs (step)
///
/// Принцип: L_tissue(t) → L_critical → disease onset (гипотеза)
/// Каждое заболевание связано с конкретными тканями и механизмами.
/// См. CONCEPT.md §8 для таблицы заболеваний с PMID.
pub struct DiseaseOnset {
    pub tissue_name: &'static str,
    pub l_crit: f64,
    pub disease_name: &'static str,
}

impl DiseaseOnset {
    pub fn is_triggered(&self, l_tissue: f64) -> bool {
        l_tissue > self.l_crit
    }
}
