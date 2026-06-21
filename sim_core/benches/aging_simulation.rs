use criterion::{black_box, Criterion};

fn bench_human_lifespan(c: &mut Criterion) {
    c.bench_function("human_120_years", |b| {
        b.iter(|| {
            let mut org = organismal_aging::organism::Organism::human();
            black_box(org.simulate_to_death(1.0, 120));
        })
    });
}

criterion::criterion_group!(benches, bench_human_lifespan);
criterion::criterion_main!(benches);
