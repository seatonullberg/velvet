use criterion::{criterion_group, criterion_main, Criterion};

use velvet_core::properties::energy::{KineticEnergy, PotentialEnergy};
use velvet_core::properties::forces::Forces;
use velvet_core::properties::temperature::Temperature;
use velvet_core::properties::{IntrinsicProperty, Property};
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_test_utils as test_utils;

pub fn forces_benchmark(c: &mut Criterion) {
    // load system
    let system = test_utils::argon_system();

    // load potentials
    let mut potentials = test_utils::argon_potentials();
    potentials.setup(&system);

    c.bench_function("forces", |b| {
        b.iter(|| Forces.calculate(&system, &potentials))
    });
}

pub fn potential_energy_benchmark(c: &mut Criterion) {
    // load system
    let system = test_utils::argon_system();

    // load potentials
    let mut potentials = test_utils::argon_potentials();
    potentials.setup(&system);

    c.bench_function("potential_energy", |b| {
        b.iter(|| PotentialEnergy.calculate(&system, &potentials))
    });
}

pub fn kinetic_energy_benchmark(c: &mut Criterion) {
    //load system
    let system = test_utils::argon_system();

    // load potentials
    let mut potentials = test_utils::argon_potentials();
    potentials.setup(&system);

    c.bench_function("kinetic_energy", |b| {
        b.iter(|| KineticEnergy.calculate(&system, &potentials))
    });
}

pub fn temperature_benchmark(c: &mut Criterion) {
    // load system
    let mut system = test_utils::argon_system();

    let boltz = Boltzmann::new(1000.0);
    boltz.apply(&mut system);

    c.bench_function("temperature", |b| {
        b.iter(|| Temperature.calculate_intrinsic(&system))
    });
}

criterion_group!(
    properties,
    forces_benchmark,
    potential_energy_benchmark,
    kinetic_energy_benchmark,
    temperature_benchmark,
);
criterion_main!(properties);
