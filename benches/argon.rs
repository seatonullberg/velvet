use criterion::{criterion_group, criterion_main, Criterion};

use velvet::prelude::*;
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 100;

// benchmark an entire NVE loop for the argon gas system
pub fn benchmark_nve(c: &mut Criterion) {
    c.bench_function("argon-nve", |b| {
        b.iter(|| {
            let system = test_utils::argon_system();
            let potentials = test_utils::argon_potentials();
            let mut sim = test_utils::nve_simulation(system, potentials);
            sim.run(ITERATIONS);
        })
    });
}

// benchmark an entire NVT loop for the argon gas system
pub fn benchmark_nvt(c: &mut Criterion) {
    c.bench_function("argon-nvt", |b| {
        b.iter(|| {
            let system = test_utils::argon_system();
            let potentials = test_utils::argon_potentials();
            let mut sim = test_utils::nvt_simulation(system, potentials);
            sim.run(ITERATIONS);
        })
    });
}

// benchmark expensive property calculations for the argon gas system
pub fn benchmark_properties(c: &mut Criterion) {
    let system: System = test_utils::argon_system();
    let potentials: Potentials = test_utils::argon_potentials();
    let mut sim = test_utils::nve_simulation(system, potentials);
    sim.run(ITERATIONS);
    let (system, potentials) = sim.consume();

    let mut group = c.benchmark_group("argon-properties");

    group.bench_function("forces", |b| {
        b.iter(|| Forces.calculate(&system, &potentials))
    });

    group.bench_function("potential-energy", |b| {
        b.iter(|| PotentialEnergy.calculate(&system, &potentials))
    });

    group.bench_function("kinetic-energy", |b| {
        b.iter(|| KineticEnergy.calculate(&system, &potentials))
    });

    group.bench_function("temperature", |b| {
        b.iter(|| Temperature.calculate(&system, &potentials))
    });

    group.finish();
}

criterion_group!(argon, benchmark_nve, benchmark_nvt, benchmark_properties);
criterion_main!(argon);
