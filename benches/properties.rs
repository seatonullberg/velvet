use std::fs::File;
use std::io::BufReader;

use criterion::{criterion_group, criterion_main, Criterion};

use velvet_core::properties::energy::{KineticEnergy, PotentialEnergy, TotalEnergy};
use velvet_core::properties::forces::Forces;
use velvet_core::properties::temperature::Temperature;
use velvet_core::properties::{IntrinsicProperty, Property};
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_external_data::poscar::load_poscar;
use velvet_test_utils as test_utils;

pub fn forces_benchmark(c: &mut Criterion) {
    let file = File::open(test_utils::resources_path("Ar.poscar")).unwrap();
    let reader = BufReader::new(file);
    let system = load_poscar(reader);

    // load potentials
    let potentials = test_utils::argon_potentials(&system);

    c.bench_function("forces", |b| {
        b.iter(|| Forces.calculate(&system, &potentials))
    });
}

pub fn potential_energy_benchmark(c: &mut Criterion) {
    let file = File::open(test_utils::resources_path("Ar.poscar")).unwrap();
    let reader = BufReader::new(file);
    let system = load_poscar(reader);

    // load potentials
    let potentials = test_utils::argon_potentials(&system);

    c.bench_function("potential_energy", |b| {
        b.iter(|| PotentialEnergy.calculate(&system, &potentials))
    });
}

pub fn kinetic_energy_benchmark(c: &mut Criterion) {
    let file = File::open(test_utils::resources_path("Ar.poscar")).unwrap();
    let reader = BufReader::new(file);
    let system = load_poscar(reader);

    // load potentials
    let potentials = test_utils::argon_potentials(&system);

    c.bench_function("kinetic_energy", |b| {
        b.iter(|| KineticEnergy.calculate(&system, &potentials))
    });
}

pub fn temperature_benchmark(c: &mut Criterion) {
    let file = File::open(test_utils::resources_path("Ar.poscar")).unwrap();
    let reader = BufReader::new(file);
    let mut system = load_poscar(reader);
    let boltz = Boltzmann::new(1000 as f32);
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
