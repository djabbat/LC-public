// Пример: полная симуляция жизни человека (120 лет)
// Вывод в CSV-формате для визуализации.

use organismal_aging::organism::Organism;

fn main() {
    let mut org = Organism::human();
    let dt = 1.0; // шаг 1 год
    let max_years = 120;

    // Заголовок CSV
    println!("age,S_centriole,telomere_D,mito_D,epigen_D,proteo_D,\
              epidermis_L,gut_L,liver_L,neurons_L,hsc_L,heart_L,\
              endothelium_L,bone_L,max_L,fi,alive");

    for _ in 0..max_years {
        if !org.is_alive {
            break;
        }
        let step = org.step(dt);

        let max_L = step.tissue_burdens.iter().cloned().fold(0.0_f64, f64::max);

        print!("{:.1},{:.4}", step.time, step.centriole_entropy);

        // Повреждения счётчиков
        for c in &org.counters {
            print!(",{:.4}", c.damage);
        }

        // Бремя тканей
        for &b in &step.tissue_burdens {
            print!(",{:.4}", b);
        }

        // Интегральные метрики
        println!(",{:.4},{:.4},{}", max_L, step.frailty_index, step.is_alive);
    }

    // События
    eprintln!("\n=== СОБЫТИЯ ===");
    for event in &org.events {
        eprintln!("  {}", event);
    }

    // Аудит источников
    eprintln!("\n=== АУДИТ ИСТОЧНИКОВ ===");
    for line in org.audit_provenance() {
        eprintln!("  {}", line);
    }

    eprintln!("\nФинальный возраст: {:.1} лет", org.age);
    eprintln!("Энтропия центриоли: {:.4}", org.centriole.entropy);
    eprintln!("Frailty Index: {:.4}", org.frailty_index);
    eprintln!("Число шагов: {}", org.step_count);
}
