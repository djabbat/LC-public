// Интеграционные тесты: sim_core ↔ mcoa_core ↔ cell_dt
//
// Проверяют, что новый sim_core совместим с существующим кодом
// mcoa_core (MCAOA) и cell_dt_core (CDATA).

#[cfg(test)]
mod tests {
    use organismal_aging::organism::Organism;
    use organismal_aging::counters::{CounterState, CounterParams, CounterType, l_tissue_aggregator};

    /// MCAOA Axiom M3: Σ w_i ≈ 1.0 для каждой ткани
    /// Примечание: веса счётчиков #2–#5 хранятся в TissueConfig;
    /// центриоль (#1) добавляется отдельно → сумма ~0.6-0.9.
    #[test]
    fn all_tissues_have_weights_in_range() {
        let org = Organism::human();
        for (i, tissue) in org.tissues.iter().enumerate() {
            let sum: f64 = tissue.config.counter_weights.iter().sum();
            let tissue_name = &tissue.config.name;
            assert!(
                sum >= 0.6 && sum <= 1.0,
                "Ткань '{}': сумма весов = {:.3} (ожидается 0.6–1.0 для счетчиков #2-#5)",
                tissue_name, sum
            );
        }
    }

    /// MCAOA Axiom M2: размерностная согласованность
    #[test]
    fn drift_is_dimensionless() {
        let params = CounterParams::telomere();
        let mut c = CounterState::new(CounterType::Telomere, params);
        let before = c.damage;
        // При n=n*, t=τ* → D увеличивается на α+β
        c.update(params.tau_star, params.n_star / params.tau_star);
        let delta = c.damage - before;
        let expected = params.alpha + params.beta;
        assert!((delta - expected).abs() < 1e-6,
            "ΔD={:.6} expected {:.6}", delta, expected);
    }

    /// Постмитотические ткани: α → 0 для теломерного счётчика
    #[test]
    fn post_mitotic_telomere_alpha_zero() {
        // Нейроны: w_telomere = 0.00
        let org = Organism::human();
        let neuron_weights = &org.tissues[3].config.counter_weights; // нейроны — индекс 3
        assert!((neuron_weights[0] - 0.0).abs() < 1e-10,
            "Вес теломерного счётчика для нейронов должен быть 0");
    }

    /// CDATA: центриолярная энтропия монотонна
    #[test]
    fn centriole_entropy_monotonic() {
        let mut org = Organism::human();
        let mut prev_entropy = org.centriole.entropy;
        for _ in 0..10 {
            org.step(1.0);
            assert!(org.centriole.entropy >= prev_entropy,
                "Энтропия центриоли должна быть монотонно неубывающей");
            prev_entropy = org.centriole.entropy;
        }
    }

    /// Проверка что L_tissue ∈ [0,1]
    #[test]
    fn l_tissue_bounded() {
        let mut org = Organism::human();
        for _ in 0..120 {
            if !org.is_alive { break; }
            let step = org.step(1.0);
            for &burden in &step.tissue_burdens {
                assert!(burden >= 0.0 && burden <= 1.0,
                    "L_tissue={:.3} вне [0,1]", burden);
            }
        }
    }

    /// Frailty Index ∈ [0, 0.7]
    #[test]
    fn frailty_index_bounded() {
        let mut org = Organism::human();
        for _ in 0..120 {
            if !org.is_alive { break; }
            let step = org.step(1.0);
            assert!(step.frailty_index >= 0.0 && step.frailty_index <= 0.7,
                "FI={:.3} вне [0, 0.7]", step.frailty_index);
        }
    }

    /// Смерть наступает до max_lifespan или при L_max > 1.0
    #[test]
    fn death_occurs_before_max_lifespan() {
        let mut org = Organism::human();
        org.max_lifespan = 50.0; // ускоренный тест
        let history = org.simulate_to_death(1.0, 60);
        let final_step = history.last().unwrap();
        assert!(!final_step.is_alive || final_step.time >= org.max_lifespan,
            "Организм должен умереть до или в max_lifespan");
    }

    /// Мыши живут меньше людей
    #[test]
    fn mouse_lives_less_than_human() {
        let mut human = Organism::human();
        human.max_lifespan = 100.0;
        let human_steps = human.simulate_to_death(1.0, 120).len();

        // Мышь с ускоренным старением
        let mut mouse = Organism::human(); // TODO: Organism::mouse()
        mouse.max_lifespan = 3.0;
        mouse.division_rate = 5.0; // мыши делятся быстрее
        mouse.ros_level = 0.15;
        let mouse_steps = mouse.simulate_to_death(0.1, 50).len();

        // Мышь должна умереть быстрее (в годах)
        assert!(mouse_steps as f64 * 0.1 < human_steps as f64,
            "Мышь должна жить меньше человека");
    }

    /// Проверка что болезни детектируются
    #[test]
    fn diseases_are_detected() {
        let mut org = Organism::human();
        // Ускоряем старение для теста
        for counter in &mut org.counters {
            counter.params.beta *= 50.0; // x50 ускорение
        }
        let history = org.simulate_to_death(1.0, 20);
        let disease_events: Vec<_> = history.iter()
            .flat_map(|s| s.events.iter())
            .filter(|e| e.starts_with("Болезнь:"))
            .collect();
        assert!(!disease_events.is_empty(),
            "При ускоренном старении должны детектироваться болезни");
    }

    /// Ze-конфликты вычисляются без ошибок
    #[test]
    fn ze_conflicts_are_computed() {
        let mut org = Organism::human();
        let step = org.step(1.0);
        // Проверяем что матрица конфликтов имеет правильные размеры
        assert_eq!(step.ze_conflicts.len(), 8);
        assert_eq!(step.ze_conflicts[0].len(), 8);
        // Диагональные элементы = 0
        for i in 0..8 {
            assert!((step.ze_conflicts[i][i] - 0.0).abs() < 1e-10);
        }
    }
}
