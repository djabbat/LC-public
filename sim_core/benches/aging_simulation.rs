/// Бенчмарк: симуляция 120 лет жизни человека
/// Цель: < 10 минут для 120-летней симуляции с шагом 1 год

use criterion::{black_box, Criterion, criterion_group, criterion_main};
use organismal_aging::organism::Organism;

fn bench_full_lifespan(c: &mut Criterion) {
    let mut group = c.benchmark_group("organismal_aging");
    group.sample_size(50);

    group.bench_function("human_120_years_dt_1year", |b| {
        b.iter(|| {
            let mut org = Organism::human();
            black_box(org.simulate_to_death(1.0, 120));
        })
    });

    group.bench_function("human_120_years_dt_1month", |b| {
        b.iter(|| {
            let mut org = Organism::human();
            black_box(org.simulate_to_death(1.0 / 12.0, 120 * 12));
        })
    });

    group.finish();
}

criterion_group!(benches, bench_full_lifespan);
criterion_main!(benches);
