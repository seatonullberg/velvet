use criterion::{criterion_group, criterion_main, Criterion};

use velvet::prelude::*;
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 10_000;

pub fn benchmark_argon(c: &mut Criterion) {
    let system: System = test_utils::argon_system();
    let mut potentials: Potentials = test_utils::argon_potentials();
    potentials.setup(&system);
    let mut sim = test_utils::nve_simulation(system, potentials);
    sim.run(ITERATIONS);
    let (system, potentials) = sim.consume();

    let mut group = c.benchmark_group("argon");

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
}

pub fn benchmark_binary_gas(c: &mut Criterion) {
    let system: System = test_utils::binary_gas_system();
    let mut potentials: Potentials = test_utils::binary_gas_potentials();
    potentials.setup(&system);
    let mut sim = test_utils::nve_simulation(system, potentials);
    sim.run(ITERATIONS);
    let (system, potentials) = sim.consume();

    let mut group = c.benchmark_group("binary-gas");

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
}

pub fn benchmark_xenon(c: &mut Criterion) {
    let system: System = test_utils::xenon_system();
    let mut potentials: Potentials = test_utils::xenon_potentials();
    potentials.setup(&system);
    let mut sim = test_utils::nve_simulation(system, potentials);
    sim.run(ITERATIONS);
    let (system, potentials) = sim.consume();

    let mut group = c.benchmark_group("xenon");

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
}

criterion_group!(
    properties,
    benchmark_argon,
    benchmark_binary_gas,
    benchmark_xenon
);
criterion_main!(properties);
