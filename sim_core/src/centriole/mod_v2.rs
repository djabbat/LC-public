// Центриоль v2.0 — расширенная модель с CCI, CML-дифференциацией и PCM-трафиком
//
// v1.0 → v2.0 изменения:
//   + CCI (Ciliary-Centriolar Index) = L_centriole / L_cilium
//   + CML_structural vs CML_signaling
//   + Динамика удлинения центриоли (CPAP-зависимое)
//   + Динамика цилии (inflammaging-зависимое укорочение)
//   + CCP-активность с возрастным спадом
//   + PCM-трафик (сателлиты → центросома)
//   + Тканеспецифичные параметры
//   + Различение материнской и дочерней центриоли
//   + Порог сенесценции CML_crit

pub mod entropy;
pub mod replication;
pub mod division;
pub mod polyglutamylation;

use crate::{Fraction, Time, Divisions};

/// Тип центриоли
#[derive(Debug, Clone, PartialEq)]
pub enum CentrioleType {
    Mother,     // Материнская (>3 циклов)
    Daughter,   // Дочерняя (1-2 цикла)
    Procentriole, // Процентриоль (<1 цикла)
}

/// Кумулятивная модификационная нагрузка
#[derive(Debug, Clone)]
pub struct CML {
    /// Структурный CML: полиглутамилирование + детиrosинирование + Δ2-тубулин
    pub structural: Fraction,
    /// Сигнальный CML: ацетилирование (протективное)
    pub signaling: Fraction,
}

impl CML {
    pub fn new() -> Self {
        Self { structural: 0.0, signaling: 0.0 }
    }

    /// Интегральный CML: structural минус protective_signaling
    pub fn net(&self) -> Fraction {
        (self.structural - 0.3 * self.signaling).max(0.0)
    }
}

/// Состояние центриоли v2.0
#[derive(Debug, Clone)]
pub struct CentrioleState {
    /// Тип центриоли
    pub centriole_type: CentrioleType,
    /// Кумулятивная модификационная нагрузка
    pub cml: CML,
    /// Накопленное число делений
    pub divisions: Divisions,
    /// Уровень полиглутамилирования [0, 1]
    pub polyglu: Fraction,
    /// Уровень ацетилирования [0, 1]
    pub acetylation: Fraction,
    /// Длина центриоли (нм)
    pub length_centriole: f64,
    /// Длина первичной цилии (мкм) — измеряется в G0/G1
    pub length_cilium: f64,
    /// Ciliary-Centriolar Index
    pub cci: f64,
    /// Активность деглутамилаз CCP [0, 1]
    pub ccp_activity: Fraction,
    /// Эффективность PCM-трафика [0, 1]
    pub pcm_traffic: Fraction,
    /// Возраст в годах
    pub age_years: f64,
}

/// Параметры ткани
#[derive(Debug, Clone)]
pub struct TissueParams {
    /// Референсное число делений (предел Хейфлика для данной ткани)
    pub hayflick_limit: f64,
    /// Базовая скорость полиглутамилирования (TTLL-активность)
    pub ttll_rate: Fraction,
    /// Базовая скорость деглутамилирования (CCP-активность)
    pub ccp_rate_initial: Fraction,
    /// Тau спада CCP-активности (годы)
    pub tau_ccp: f64,
    /// Скорость удлинения центриоли (нм/деление, CPAP-зависимая)
    pub elongation_rate: f64,
    /// Скорость укорочения цилии (мкм/год, inflammaging)
    pub cilium_shortening: f64,
    /// Чувствительность к ROS
    pub ros_sensitivity: Fraction,
    /// Эффективность дорифагии
    pub doryphagy_efficiency: Fraction,
}

impl Default for TissueParams {
    fn default() -> Self {
        Self {
            hayflick_limit: 50.0,       // Фибробласты
            ttll_rate: 0.02,
            ccp_rate_initial: 0.025,     // На старте CCP > TTLL → медленное накопление
            tau_ccp: 20.0,              // Спад CCP за ~20 лет
            elongation_rate: 1.5,        // +1.5 нм/деление
            cilium_shortening: 0.03,     // -0.03 мкм/год
            ros_sensitivity: 0.01,
            doryphagy_efficiency: 0.8,
        }
    }
}

/// Тканеспецифичные пресеты
impl TissueParams {
    /// Фибробласты (предел ~50)
    pub fn fibroblast() -> Self {
        Self { hayflick_limit: 50.0, ..Default::default() }
    }
    /// Кератиноциты (предел ~20)
    pub fn keratinocyte() -> Self {
        Self {
            hayflick_limit: 20.0,
            ttll_rate: 0.035,            // Выше TTLL → быстрее накопление
            ccp_rate_initial: 0.02,      // Ниже CCP → быстрее накопление
            tau_ccp: 12.0,               // Быстрее спад
            ..Default::default()
        }
    }
    /// ГСК (предел ~30-50, теломераза активна)
    pub fn hsc() -> Self {
        Self {
            hayflick_limit: 40.0,
            tau_ccp: 18.0,
            doryphagy_efficiency: 0.6,   // Сниженная дорифагия в старых ГСК
            ..Default::default()
        }
    }
    /// ESC (условно бессмертны)
    pub fn esc() -> Self {
        Self {
            hayflick_limit: 500.0,       // Очень высокий предел
            ttll_rate: 0.005,            // Очень низкая TTLL
            ccp_rate_initial: 0.04,      // Очень высокая CCP
            tau_ccp: 100.0,              // Практически не падает
            elongation_rate: 0.2,        // Минимальное удлинение
            ..Default::default()
        }
    }
}

/// Скорости накопления
#[derive(Debug, Clone)]
pub struct EntropyRates {
    pub eta_div: Fraction,
    pub eta_time: Fraction,
    pub eta_ros: Fraction,
    pub eta_rep: Fraction,
    pub eta_elongation: Fraction,  // Скорость удлинения центриоли
    pub source: crate::provenance::Source,
}

impl Default for EntropyRates {
    fn default() -> Self {
        Self {
            eta_div: 0.02 / 50.0,
            eta_time: 0.010,
            eta_ros: 0.01,
            eta_rep: 0.001,
            eta_elongation: 0.005,
            source: crate::provenance::sources::CENTRIOLE_ENTROPY_POSTULATE,
        }
    }
}

impl CentrioleState {
    /// Создать новую центриоль (зигота / de novo сборка)
    pub fn new(centriole_type: CentrioleType) -> Self {
        Self {
            centriole_type,
            cml: CML::new(),
            divisions: 0.0,
            polyglu: 0.0,
            acetylation: 0.0,
            length_centriole: 400.0,  // Базовая длина ~400 нм
            length_cilium: 5.0,        // Базовая длина цилии ~5 мкм
            cci: 400.0 / 5000.0,       // CCI = 400 нм / 5000 нм = 0.08
            ccp_activity: 1.0,
            pcm_traffic: 1.0,
            age_years: 0.0,
        }
    }

    /// Обновить состояние центриоли за шаг dt (в годах)
    pub fn update(
        &mut self,
        dt: Time,
        ros: Fraction,
        division_rate: Fraction,
        rates: &EntropyRates,
        tissue: &TissueParams,
    ) {
        // Возраст
        self.age_years += dt;

        // Число делений за шаг
        let dn = division_rate * dt;
        self.divisions += dn;

        // --- CCP-активность падает с возрастом ---
        self.ccp_activity = (tissue.ccp_rate_initial
            * (-self.age_years / tissue.tau_ccp).exp())
            .max(0.05); // Минимальная остаточная активность 5%

        // --- Накопление CML ---
        // Структурный CML (полиглу + детиrosинирование)
        let dcml_struct = tissue.ttll_rate * dn
            + rates.eta_ros * ros * dt
            - self.ccp_activity * dt * tissue.doryphagy_efficiency;

        self.cml.structural = (self.cml.structural + dcml_struct).max(0.0).min(1.0);

        // Сигнальный CML (ацетилирование — протективное)
        let dcml_signal = rates.eta_time * 0.5 * dt  // Медленнее структурного
            + rates.eta_rep * 0.2 * dn;
        self.cml.signaling = (self.cml.signaling + dcml_signal).min(1.0);

        // polyGlu трекается отдельно
        self.polyglu = self.cml.structural;
        self.acetylation = self.cml.signaling;

        // --- Удлинение центриоли (CPAP-зависимое) ---
        // Удлиняется пропорционально числу делений и структурному CML
        self.length_centriole += tissue.elongation_rate * dn
            * (1.0 + self.cml.structural);  // CML ускоряет удлинение
        self.length_centriole = self.length_centriole.min(2000.0); // Макс ~2 мкм

        // --- Укорочение цилии (inflammaging + CML) ---
        let cilium_loss = tissue.cilium_shortening * dt
            * (1.0 + ros * tissue.ros_sensitivity)
            * (1.0 + 0.5 * self.cml.structural);
        self.length_cilium = (self.length_cilium - cilium_loss).max(1.0); // Мин ~1 мкм

        // --- CCI ---
        self.cci = self.length_centriole / (self.length_cilium * 1000.0); // мкм → нм

        // --- PCM-трафик падает с CML ---
        self.pcm_traffic = (1.0 - 0.7 * self.cml.structural).max(0.1);

        // --- Обратная связь: плохой PCM-трафик → ещё больше CML ---
        let feedback = 0.1 * (1.0 - self.pcm_traffic) * dt;
        self.cml.structural = (self.cml.structural + feedback).min(1.0);
    }

    /// Достигла ли центриоль критического CML?
    pub fn is_critical(&self, tissue: &TissueParams) -> bool {
        // Net CML > 0.7 ИЛИ CCI > критического
        let cml_crit_reached = self.cml.net() > 0.7;
        let cci_crit_reached = self.cci > 0.25; // Порог CCI_crit
        let divisions_exceeded = self.divisions >= tissue.hayflick_limit;
        let traffic_failed = self.pcm_traffic < 0.2;

        cml_crit_reached || cci_crit_reached || (divisions_exceeded && traffic_failed)
    }

    /// Переход к сенесценции: p53/p21 активация
    pub fn senescence_probability(&self, tissue: &TissueParams) -> Fraction {
        let cml_factor = self.cml.net();
        let traffic_factor = 1.0 - self.pcm_traffic;
        let cci_factor = (self.cci / 0.25).min(1.0);

        // Комбинированная вероятность
        (0.4 * cml_factor + 0.3 * traffic_factor + 0.3 * cci_factor).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_centriole_has_minimal_cml() {
        let c = CentrioleState::new(CentrioleType::Mother);
        assert!(c.cml.net() < 0.01);
        assert!((c.length_centriole - 400.0).abs() < 1.0);
        assert!((c.length_cilium - 5.0).abs() < 0.1);
    }

    #[test]
    fn cci_grows_with_age() {
        let mut c = CentrioleState::new(CentrioleType::Mother);
        let rates = EntropyRates::default();
        let tissue = TissueParams::fibroblast();
        let before = c.cci;
        // Моделируем 30 лет с 1 делением/год
        for _ in 0..30 {
            c.update(1.0, 0.05, 1.0, &rates, &tissue);
        }
        assert!(c.cci > before, "CCI должен расти с возрастом");
        assert!(c.length_centriole > 400.0, "Центриоль должна удлиняться");
    }

    #[test]
    fn keratinocytes_senesce_faster() {
        let mut k = CentrioleState::new(CentrioleType::Mother);
        let mut f = CentrioleState::new(CentrioleType::Mother);
        let rates = EntropyRates::default();
        let tk = TissueParams::keratinocyte();
        let tf = TissueParams::fibroblast();

        for _ in 0..25 {
            k.update(1.0, 0.05, 1.0, &rates, &tk);
            f.update(1.0, 0.05, 1.0, &rates, &tf);
        }

        assert!(k.cml.net() > f.cml.net(),
            "Кератиноциты должны накапливать CML быстрее фибробластов");
    }

    #[test]
    fn esc_barely_accumulates_cml() {
        let mut esc = CentrioleState::new(CentrioleType::Mother);
        let rates = EntropyRates::default();
        let tesc = TissueParams::esc();

        for _ in 0..50 {
            esc.update(1.0, 0.05, 1.0, &rates, &tesc);
        }

        assert!(esc.cml.net() < 0.3,
            "ESC должны накапливать CML очень медленно");
    }

    #[test]
    fn pcm_traffic_declines_with_cml() {
        let mut c = CentrioleState::new(CentrioleType::Mother);
        let rates = EntropyRates::default();
        let tissue = TissueParams::fibroblast();
        assert!((c.pcm_traffic - 1.0).abs() < 0.01);

        for _ in 0..40 {
            c.update(1.0, 0.1, 2.0, &rates, &tissue);
        }

        assert!(c.pcm_traffic < 0.8,
            "PCM-трафик должен падать с накоплением CML");
    }
}
