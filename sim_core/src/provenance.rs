// Источники (provenance) — откуда взята каждая формула, параметр, алгоритм.
//
// Система трейсинга: каждый элемент симулятора имеет поле source: Source.
// Никакое число в симуляторе не должно появляться без указания источника.

use serde::Serialize;

/// Источник данных/формулы/алгоритма
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Source {
    /// PubMed-индексированная статья
    PMID(u64, &'static str),           // номер, краткое описание
    /// DOI (Zenodo, Crossref)
    DOI(&'static str, &'static str),   // doi, описание
    /// Оценка (экспертная, без прямой публикации)
    Estimate(&'static str),            // что оценено и кем
    /// Теоретический постулат (первый принцип)
    Postulate(&'static str),           // формулировка постулата
    /// Выведено из других данных
    Derived(&'static str),             // цепочка вывода
    /// Экспериментальные данные (ещё не опубликованы)
    Experiment(&'static str),          // какой эксперимент
    /// Общеизвестный факт (не требует ссылки)
    KnownFact(&'static str),           // описание факта
}

impl Source {
    pub fn pmid(id: u64, desc: &'static str) -> Self {
        Source::PMID(id, desc)
    }

    pub fn estimate(desc: &'static str) -> Self {
        Source::Estimate(desc)
    }

    pub fn postulate(desc: &'static str) -> Self {
        Source::Postulate(desc)
    }

    pub fn derived(desc: &'static str) -> Self {
        Source::Derived(desc)
    }

    /// Человекочитаемая строка для логирования/аудита
    pub fn cite(&self) -> String {
        match self {
            Source::PMID(id, desc) => format!("PMID {} — {}", id, desc),
            Source::DOI(doi, desc) => format!("DOI {} — {}", doi, desc),
            Source::Estimate(desc) => format!("[ОЦЕНКА] {}", desc),
            Source::Postulate(desc) => format!("[ПОСТУЛАТ] {}", desc),
            Source::Derived(desc) => format!("[ВЫВОД] {}", desc),
            Source::Experiment(desc) => format!("[ЭКСПЕРИМЕНТ] {}", desc),
            Source::KnownFact(desc) => format!("[ФАКТ] {}", desc),
        }
    }

    /// Уровень достоверности источника (0.0 — постулат, 1.0 — PMID)
    pub fn confidence(&self) -> f64 {
        match self {
            Source::PMID(_, _) => 1.0,
            Source::DOI(_, _) => 0.9,
            Source::Experiment(_) => 0.8,
            Source::KnownFact(_) => 0.7,
            Source::Derived(_) => 0.5,
            Source::Estimate(_) => 0.3,
            Source::Postulate(_) => 0.1,
        }
    }
}

/// Типаж для всего, что имеет источник
pub trait Provenanced {
    fn source(&self) -> &Source;
    fn confidence(&self) -> f64 { self.source().confidence() }
}

/// Параметр с источником
#[derive(Debug, Clone, Serialize)]
pub struct ProvenancedValue<T> {
    pub value: T,
    pub source: Source,
}

impl<T> ProvenancedValue<T> {
    pub fn new(value: T, source: Source) -> Self {
        Self { value, source }
    }
}

impl<T> Provenanced for ProvenancedValue<T> {
    fn source(&self) -> &Source { &self.source }
}

/// Стандартные источники для Organismal Aging
pub mod sources {
    use super::Source;

    // Уровень #1: Центриоль + Энтропия
    pub const CENTRIOLE_ENTROPY_POSTULATE: Source =
        Source::Postulate("Гипотеза: центриоль имеет ограниченное self-renewal → потенциальный накопитель энтропии (Tqemaladze 2023, PMID 36583780 — гипотетическая модель). Прямых измерений нет.");
    pub const ASYMMETRIC_INHERITANCE: Source =
        Source::PMID(17255513, "Yamashita 2007 — асимметричное наследование центросомы в стволовых клетках Drosophila");
    pub const ASYMMETRIC_NEOCORTEX: Source =
        Source::PMID(19829375, "Wang 2009 — асимметричное наследование центросомы в нейральных прогениторах неокортекса");
    pub const MUSCLE_ENTROPY: Source =
        Source::PMID(41724675, "Hong 2026 — энтропия мышечных волокон предсказывает мобильность у пожилых");
    pub const DNA_ENTROPY_BIOMARKER: Source =
        Source::PMID(40096548, "Chan 2025 — энтропия метилирования ДНК как биомаркер старения");
    pub const DNA_ENTROPY_STEM: Source =
        Source::PMID(36797759, "Vaidya 2023 — энтропия метилирования ДНК как мера репликации стволовых клеток");
    pub const DISSIPATIVE_SCALING: Source =
        Source::PMID(38367762, "Kriete 2024 — диссипативное масштабирование развития и старения");
    pub const GENETICS_VS_ENTROPY: Source =
        Source::PMID(19903538, "Salminen 2010 — longevity факторы подавляют NF-κB-энтропийный процесс");
    pub const IRREPARABLE_DAMAGE: Source =
        Source::PMID(15935593, "Yin 2005 — необратимое накопление повреждений → рост энтропии");
    pub const CENTRIOLE_DAMAGE_MARKER: Source =
        Source::PMID(36583780, "Tqemaladze 2023 — гипотетическая модель: центриоль как потенциальный маркер старения (НЕ эмпирическое подтверждение)");
    pub const POLYGLU_TTLL_CCP: Source =
        Source::DOI("10.1038/s41556-024-01387-x", "Maneix 2024 — PPIA шаперон, TTLL/CCP баланс");

    // Уровень #2: Счётчики
    pub const HAYFLICK_LIMIT: Source =
        Source::KnownFact("Лимит Хейфлика ~50 делений для человеческих фибробластов (Hayflick 1965)");
    pub const TELOMERE_SHORTENING: Source =
        Source::PMID(24138928, "Horvath 2013 — эпигенетические часы и теломеры");
    pub const MITOCHONDRIAL_ROS: Source =
        Source::PMID(40410559, "Yang 2025 — митохондриальная дисфункция в старении гемопоэтической системы");
    pub const EPIGENETIC_DRIFT: Source =
        Source::PMID(22560076, "Florian 2012 — Cdc42 активность регулирует старение HSC");
    pub const EPIGENETIC_CLOCK: Source =
        Source::PMID(35029144, "Belsky 2022 — DunedinPACE, темп старения");
    pub const PROTEOSTASIS_HSC: Source =
        Source::PMID(40738832, "Catic 2026 — протеостаз в HSC, PPIA шаперон");

    // Уровень #3: Ткани
    pub const TISSUE_RENEWAL_RATES: Source =
        Source::KnownFact("Периоды обновления тканей — стандартные данные клеточной биологии");
    pub const FRAILTY_INDEX: Source =
        Source::PMID(16129869, "Rockwood 2005 — Frailty Index, FI = Σ дефицитов / Σ измеренных");
    pub const FRAILTY_PROTOCOL: Source =
        Source::PMID(18671847, "Searle 2008 — стандартный протокол Frailty Index");
    pub const FI_MORTALITY: Source =
        Source::PMID(12456714, "Mitnitski 2002 — FI и смертность: Gompertz hazard doubles per 0.07 FI");
    pub const ARTERIAL_AGING: Source =
        Source::PMID(12515756, "Lakatta & Levy 2003, Circulation — arterial and cardiac aging as major shareholders in CVD");
    pub const EXPOSOME_VS_GENOME: Source =
        Source::PMID(39972219, "Argentieri 2025, Nature Medicine — exposome explains 17% mortality vs <2% genetics, n=492,567");
    pub const MITOPHAGY_TURNOVER: Source =
        Source::PMID(22252130, "Youle 2012 — митофагия, turnover mtDNA ~30 дней");

    // Интеграция
    pub const GOMPERTZ_MORTALITY: Source =
        Source::KnownFact("Гомпертцовская модель смертности: h(t) = h₀·exp(γ·t)");
    pub const ZE_CONFLICT_POSTULATE: Source =
        Source::Postulate("Межтканевые конфликты из-за разных периодов самообновления → Z_conflict (Tqemaladze 2026, Organismal Aging)");
    pub const MCAOA_AXIOMS: Source =
        Source::DOI("10.5281/zenodo.20055806", "Tqemaladze 2026 — MCAOA preprint, аксиомы M1-M4");

    // Параметры — оценки
    pub const ETA_DIV_ESTIMATE: Source =
        Source::Estimate("η_div = 0.02/50 — оценка на основе n*=50 и максимального вклада делений 0.02");
    pub const ETA_TIME_ESTIMATE: Source =
        Source::Estimate("η_time = 0.005/год — оценка хронологического накопления");
    pub const WEIGHTS_ESTIMATE: Source =
        Source::Estimate("Веса счётчиков w_i(tissue) — предварительные оценки, калибруются на GTEx");
    pub const Z_CRIT_ESTIMATE: Source =
        Source::Estimate("Z_crit = 0.30 — предварительная оценка порога конфликта");
    pub const COUNTER_PARAMS_ESTIMATE: Source =
        Source::Estimate("α, β, k, x_crit — оценки, требуют калибровки (MCAOA Test 1)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pmid_has_max_confidence() {
        let s = Source::pmid(36583780, "Test");
        assert!((s.confidence() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn postulate_has_low_confidence() {
        let s = Source::postulate("Test postulate");
        assert!(s.confidence() < 0.2);
    }

    #[test]
    fn provenanced_value_works() {
        let pv: ProvenancedValue<f64> = ProvenancedValue::new(42.0, Source::estimate("Тестовая оценка"));
        assert!((pv.value - 42.0).abs() < 1e-10);
        assert_eq!(pv.source(), &Source::Estimate("Тестовая оценка"));
    }

    #[test]
    fn cite_produces_human_readable() {
        let s = Source::pmid(36583780, "CDATA theory");
        let cite = s.cite();
        assert!(cite.contains("PMID 36583780"));
        assert!(cite.contains("CDATA theory"));
    }

    #[test]
    fn all_standard_sources_are_valid() {
        // Проверяем что все источники из sources создаются без паники
        let _ = vec![
            sources::CENTRIOLE_ENTROPY_POSTULATE,
            sources::ASYMMETRIC_INHERITANCE,
            sources::HAYFLICK_LIMIT,
            sources::FRAILTY_INDEX,
            sources::MCAOA_AXIOMS,
        ];
    }
}
