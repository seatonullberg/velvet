use criterion::{criterion_group, criterion_main, Criterion};
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_external_data::poscar::load_poscar;

use test_utils;

use std::fs::File;
use std::io::BufReader;

pub fn velocity_distributions_group_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("velocity_distributions");

    let file = File::open(test_utils::test_resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let mut system = load_poscar(reader);

    let target = 100 as f32;
    let boltz = Boltzmann::new(target);

    group.bench_function("boltzmann", |b| {
        b.iter(|| {
            boltz.apply(&mut system);
        })
    });

    group.finish()
}

criterion_group!(
    velocity_distributions,
    velocity_distributions_group_benchmark
);
criterion_main!(velocity_distributions);
