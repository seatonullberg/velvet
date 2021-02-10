use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::BufReader;
use test_utils::test_resources_path;
use velvet_core::properties::{
    Forces, IntrinsicProperty, KineticEnergy, PotentialEnergy, Property, Temperature,
};
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_external_data::poscar::load_poscar;

use test_utils;

pub fn forces_benchmark(c: &mut Criterion) {
    let file = File::open(test_resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let system = load_poscar(reader);

    // load potentials
    // let path = test_resources_path("argon.pot.velvet");
    // let file = File::open(&path).unwrap();
    // let pots: Potentials = ron::de::from_reader(file).unwrap();
    let potentials = test_utils::get_argon_potentials(&system);

    c.bench_function("forces", |b| {
        b.iter(|| Forces.calculate(&system, &potentials))
    });
}

pub fn potential_energy_benchmark(c: &mut Criterion) {
    let file = File::open(test_resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let system = load_poscar(reader);

    // load potentials
    // let path = test_resources_path("argon.pot.velvet");
    // let file = File::open(&path).unwrap();
    // let pots: Potentials = ron::de::from_reader(file).unwrap();
    let potentials = test_utils::get_argon_potentials(&system);

    c.bench_function("potential_energy", |b| {
        b.iter(|| PotentialEnergy.calculate(&system, &potentials))
    });
}

pub fn kinetic_energy_benchmark(c: &mut Criterion) {
    let file = File::open(test_resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let system = load_poscar(reader);

    // load potentials
    // let path = test_resources_path("argon.pot.velvet");
    // let file = File::open(&path).unwrap();
    // let pots: Potentials = ron::de::from_reader(file).unwrap();
    let potentials = test_utils::get_argon_potentials(&system);

    c.bench_function("kinetic_energy", |b| {
        b.iter(|| KineticEnergy.calculate(&system, &potentials))
    });
}

pub fn temperature_benchmark(c: &mut Criterion) {
    let file = File::open(test_resources_path("argon.poscar")).unwrap();
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
