// Интеграция: целый организм
// Развитие с зиготы, кривая старения, смертность,
// Frailty Index, заболевания, травмы, интервенции.

pub mod development;
pub mod aging_curve;
pub mod mortality;
pub mod frailty;
pub mod disease;
pub mod trauma;

use crate::{Fraction, Time, Divisions, SimulationStep};
use crate::centriole::{CentrioleState, EntropyRates};
use crate::counters::{CounterState, CounterParams, CounterType};
use crate::tissue::{TissueState, ZeConflict, human_tissue_configs};
use crate::macrobiome::{DietConfig, DigestionResult};
use crate::learning::BayesianLoop;

/// Главная структура — организм
#[derive(Debug, Clone)]
pub struct Organism {
    pub name: String,
    pub age: Time,
    pub max_lifespan: Time,

    // Уровень #1
    pub centriole: CentrioleState,
    pub entropy_rates: EntropyRates,
    pub n_ref: Divisions,

    // Уровень #2
    pub counters: Vec<CounterState>,

    // Уровень #3
    pub tissues: Vec<TissueState>,
    pub connectivity: Vec<Vec<Fraction>>, // C_ij — матрица связности тканей
    pub z_crit: Fraction,

    // Интеграция
    pub frailty_index: Fraction,
    pub is_alive: bool,
    pub events: Vec<String>,
    pub step_count: u64,

    // Внешние возмущения
    pub ros_level: Fraction,
    pub division_rate: Fraction, // средняя по организму

    // Макробиом
    pub diet: DietConfig,
    pub digestion: Option<DigestionResult>,

    // Самообучение
    pub learning_loop: BayesianLoop,
}

impl Organism {
    /// Создать организм человека с параметрами по умолчанию
    pub fn human() -> Self {
        let tissue_configs = human_tissue_configs();
        let n_tissues = tissue_configs.len();

        let counters = vec![
            CounterState::new(CounterType::Telomere, CounterParams::telomere()),
            CounterState::new(CounterType::Mitochondrial, CounterParams::mitochondrial()),
            CounterState::new(CounterType::Epigenetic, CounterParams::epigenetic()),
            CounterState::new(CounterType::Proteostatic, CounterParams::proteostatic()),
        ];

        // Веса счётчиков — хранятся в TissueConfig каждой ткани
        // (используются в TissueState::update())

        // Матрица связности (по умолчанию — единичная)
        let connectivity = vec![vec![1.0; n_tissues]; n_tissues];

        let tissues: Vec<TissueState> = tissue_configs
            .into_iter()
            .map(TissueState::new)
            .collect();

        let mut org = Self {
            name: "Homo sapiens".into(),
            age: 0.0,
            max_lifespan: 120.0,
            centriole: CentrioleState::new(),
            entropy_rates: EntropyRates::default(),
            n_ref: 50.0,
            counters,
            tissues,
            connectivity,
            z_crit: 0.30,
            frailty_index: 0.0,
            is_alive: true,
            events: vec!["Рождение".into()],
            step_count: 0,
            ros_level: 0.1,
            division_rate: 1.0,
            diet: DietConfig::default(),
            digestion: None,
            learning_loop: BayesianLoop::new(),
        };

        // Инициализация байесовского контура
        org.init_learning_parameters();

        org
    }

    /// Инициализировать параметры байесовского контура
    fn init_learning_parameters(&mut self) {
        self.learning_loop.register_parameter("eta_time", 0.010, 0.005,
            crate::provenance::sources::CENTRIOLE_ENTROPY_POSTULATE);
        self.learning_loop.register_parameter("beta_epi", 0.030, 0.010,
            crate::provenance::sources::EPIGENETIC_CLOCK);
    }

    /// Применить диету — пересчитать ROS и протеостазные множители
    pub fn apply_diet(&mut self, diet: DietConfig) {
        self.diet = diet.clone();
        self.digestion = Some(DigestionResult::simulate(&diet));
        if let Some(ref dig) = self.digestion {
            self.ros_level = (0.1 * dig.ros_impact(&diet)).min(1.0);
            // Влияние на протеостаз: модифицируем beta счётчика #5
            for counter in &mut self.counters {
                if matches!(counter.counter_type, CounterType::Proteostatic) {
                    counter.params.beta *= dig.proteo_impact(&diet);
                }
            }
        }
    }

    /// Один шаг симуляции (dt в годах)
    pub fn step(&mut self, dt: Time) -> SimulationStep {
        self.step_count += 1;
        self.age += dt;

        // Замедление деления с возрастом
        let age_factor = 1.0 / (1.0 + 0.05 * self.age);
        let effective_division_rate = self.division_rate * age_factor;

        // === Уровень #1: Центриоль ===
        self.centriole.update(
            dt, self.n_ref, self.ros_level,
            effective_division_rate, &self.entropy_rates,
        );

        // === Уровень #2: Счётчики ===
        for counter in &mut self.counters {
            counter.update(dt, effective_division_rate);
        }

        // === Уровень #3: Ткани ===
        for (_i, tissue) in self.tissues.iter_mut().enumerate() {
            tissue.update(dt, &self.counters, self.centriole.entropy);

            if tissue.is_critical() && !self.events.iter().any(|e| e.contains(&format!("Болезнь: {}", tissue.config.name))) {
                self.events.push(format!(
                    "Болезнь: {} (L={:.3} > L_crit={:.3}, возраст {:.1} лет)",
                    tissue.config.name, tissue.burden, tissue.config.critical_burden, self.age
                ));
            }
        }

        // === Ze-конфликты ===
        let mut ze_conflicts = vec![vec![0.0_f64; self.tissues.len()]; self.tissues.len()];
        for i in 0..self.tissues.len() {
            for j in (i+1)..self.tissues.len() {
                let z = ZeConflict::compute(
                    &self.tissues[i], &self.tissues[j],
                    self.connectivity[i][j],
                );
                ze_conflicts[i][j] = z.value;
                ze_conflicts[j][i] = z.value;

                if z.is_critical(self.z_crit) {
                    self.events.push(format!(
                        "Ze-конфликт: {} ↔ {} (Z={:.3})",
                        self.tissues[i].config.name,
                        self.tissues[j].config.name,
                        z.value
                    ));
                }
            }
        }

        // === Frailty Index ===
        let max_burden = self.tissues.iter()
            .map(|t| t.burden)
            .fold(0.0_f64, f64::max);
        self.frailty_index = 0.7 * max_burden; // FI = 0.7 * L_max (Rockwood 2005)

        // === Проверка смерти ===
        // Смерть при L_max > 0.90 (практически — полиорганная недостаточность)
        // или при превышении max_lifespan
        let l_max = max_burden;
        if l_max >= 0.99 || self.age > self.max_lifespan {
            self.is_alive = false;
            self.events.push(format!("Смерть в возрасте {:.1} лет (L_max={:.3})", self.age, l_max));
        }

        SimulationStep {
            time: self.age,
            centriole_entropy: self.centriole.entropy,
            tissue_burdens: self.tissues.iter().map(|t| t.burden).collect(),
            ze_conflicts,
            frailty_index: self.frailty_index,
            is_alive: self.is_alive,
            events: self.events.clone(),
        }
    }

    /// Симулировать до смерти или max_steps
    pub fn simulate_to_death(&mut self, dt: Time, max_steps: u64) -> Vec<SimulationStep> {
        let mut history = Vec::new();
        for _ in 0..max_steps {
            if !self.is_alive {
                break;
            }
            let s = self.step(dt);
            history.push(s);
        }
        history
    }

    /// Аудит источников — вывести все provenance в человекочитаемом виде
    pub fn audit_provenance(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push("=== АУДИТ ИСТОЧНИКОВ ===".into());
        lines.push(format!("Центриоль: {}", self.entropy_rates.source.cite()));
        for c in &self.counters {
            lines.push(format!("  Счётчик {:?}: {}", c.counter_type, c.params.source.cite()));
        }
        for t in &self.tissues {
            lines.push(format!("  Ткань '{}': τ={:.0} дн, [ОЦЕНКА] веса счётчиков",
                t.config.name, t.config.renewal_period_days));
        }
        lines.push(format!("Z_crit={:.2}: [{}]", self.z_crit,
            crate::provenance::sources::Z_CRIT_ESTIMATE.cite()));
        lines.push(format!("Frailty Index: {}", crate::provenance::sources::FRAILTY_INDEX.cite()));
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_organism_creation() {
        let org = Organism::human();
        assert_eq!(org.tissues.len(), 8);
        assert_eq!(org.counters.len(), 4);
        assert!(org.is_alive);
        assert!((org.age - 0.0).abs() < 1e-10);
    }

    #[test]
    fn organism_ages() {
        let mut org = Organism::human();
        let step = org.step(1.0);
        assert!((step.time - 1.0).abs() < 1e-10);
        assert!(step.centriole_entropy > 0.01); // энтропия выросла
    }

    #[test]
    fn organism_dies() {
        let mut org = Organism::human();
        org.max_lifespan = 5.0; // ускоряем для теста
        let history = org.simulate_to_death(1.0, 100);
        assert!(!org.is_alive || org.age >= org.max_lifespan);
        assert!(history.len() <= 100);
    }

    #[test]
    fn frailty_index_in_range() {
        let mut org = Organism::human();
        for _ in 0..120 {
            if !org.is_alive { break; }
            org.step(1.0);
            assert!(org.frailty_index >= 0.0 && org.frailty_index <= 0.7);
        }
    }

    #[test]
    fn diet_changes_ros() {
        let mut org = Organism::human();
        let ros_before = org.ros_level;
        org.apply_diet(DietConfig::high_fat());
        assert!(org.ros_level > ros_before, "High-fat diet must increase ROS");
    }

    #[test]
    fn mediterranean_diet_lowers_ros() {
        let mut org = Organism::human();
        org.apply_diet(DietConfig::mediterranean());
        assert!(org.ros_level < 0.1, "Mediterranean diet must lower ROS below baseline");
    }
}
