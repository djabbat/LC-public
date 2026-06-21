/// Organismal Aging CLI — симулятор командной строки
///
/// Примеры:
///   organismal_aging simulate --species human --years 120 --dt 1.0
///   organismal_aging simulate --species mouse --years 3 --dt 0.1
///   organismal_aging audit                     — аудит источников
///   organismal_aging compare --diet mediterranean --diet western

use organismal_aging::organism::Organism;
use organismal_aging::macrobiome::{DietConfig, DigestionResult};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Organismal Aging CLI v{}", organismal_aging::VERSION);
        eprintln!("Использование:");
        eprintln!("  simulate  — симуляция жизни");
        eprintln!("  audit     — аудит источников (provenance)");
        eprintln!("  compare   — сравнение диет/интервенций");
        eprintln!("  species   — список доступных видов");
        return;
    }

    match args[1].as_str() {
        "simulate" => cmd_simulate(&args),
        "audit" => cmd_audit(),
        "compare" => cmd_compare(&args),
        "species" => cmd_species(),
        _ => eprintln!("Неизвестная команда: {}", args[1]),
    }
}

fn cmd_simulate(args: &[String]) {
    let mut org = Organism::human();
    let mut dt = 1.0_f64;
    let mut max_years = 120;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--dt" => { dt = args.get(i+1).and_then(|s| s.parse().ok()).unwrap_or(1.0); i += 2; }
            "--years" => { max_years = args.get(i+1).and_then(|s| s.parse().ok()).unwrap_or(120); i += 2; }
            "--species" => {
                let s = args.get(i+1).map(|s| s.as_str()).unwrap_or("human");
                if s == "mouse" {
                    org.max_lifespan = 3.0;
                    org.division_rate = 5.0;
                    org.ros_level = 0.15;
                }
                i += 2;
            }
            _ => { i += 1; }
        }
    }

    eprintln!("Симуляция: {} лет, dt={} год", max_years, dt);
    let steps = (max_years as f64 / dt) as u64;

    let history = org.simulate_to_death(dt, steps);

    // CSV вывод
    println!("age,S_centriole,FI,alive");
    for step in &history {
        println!("{:.2},{:.4},{:.4},{}",
            step.time, step.centriole_entropy, step.frailty_index, step.is_alive);
    }

    // События в stderr
    eprintln!("\n=== СОБЫТИЯ ===");
    for event in &org.events {
        if event.contains("Болезнь") || event.contains("Смерть") || event.contains("Ze-конфликт") {
            eprintln!("  {}", event);
        }
    }
    eprintln!("Шагов: {}, возраст: {:.1}", history.len(), org.age);
}

fn cmd_audit() {
    let org = Organism::human();
    println!("=== AUDIT: Organismal Aging v{} ===\n", organismal_aging::VERSION);
    for line in org.audit_provenance() {
        println!("{}", line);
    }
}

fn cmd_compare(_args: &[String]) {
    println!("Сравнение диет (влияние на ROS и протеостаз):\n");
    println!("{:<25} {:>8} {:>8} {:>8}",
        "Диета", "ROS×", "Prot×", "SCFA(мМ)");

    let diets = vec![
        DietConfig::default(),
        DietConfig::mediterranean(),
        DietConfig::caloric_restriction(),
        DietConfig::high_fat(),
    ];

    for diet in &diets {
        let dig = DigestionResult::simulate(diet);
        println!("{:<25} {:>8.2} {:>8.2} {:>8.1}",
            diet.name,
            dig.ros_impact(diet),
            dig.proteo_impact(diet),
            dig.scfa_production,
        );
    }
}

fn cmd_species() {
    println!("Доступные виды:\n");
    println!("  human       — Homo sapiens (120 лет, 8 тканей, центриоли ✅)");
    println!("  mouse       — Mus musculus (3 года, 8 тканей, центриоли ✅)");
    println!("  celegans    — C. elegans (3 недели, 3 ткани, центриоли ✅)");
    println!("  unicellular — Одноклеточные (часы, 1 ткань, центриоли ❌)");
    println!("\n  Все параметры калибруются. Добавление видов через SpeciesConfig.");
}
