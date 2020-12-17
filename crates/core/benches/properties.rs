mod common;

use criterion::{criterion_group, criterion_main, Criterion};
use velvet_convert::load_poscar;
use velvet_core::distributions::{Boltzmann, VelocityDistribution};
use velvet_core::properties::{
    Forces, IntrinsicProperty, KineticEnergy, PotentialEnergy, Property, Temperature,
};
use velvet_core::potentials::Potentials;
use std::fs::File;
use std::io::BufReader;

pub fn forces_benchmark(c: &mut Criterion) {
    let file = File::open(common::test_resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let sys = load_poscar(reader);
    
    // load potentials
    let path = common::test_resources_path("argon.pot.velvet");
    let file = File::open(&path).unwrap();
    let pots: Potentials = ron::de::from_reader(file).unwrap();
    
    c.bench_function("forces argon", |b| b.iter(|| Forces.calculate(&sys, &pots)));
}

pub fn potential_energy_benchmark(c: &mut Criterion) {
    let file = File::open(common::test_resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let sys = load_poscar(reader);

    // load potentials
    let path = common::test_resources_path("argon.pot.velvet");
    let file = File::open(&path).unwrap();
    let pots: Potentials = ron::de::from_reader(file).unwrap();
    
    c.bench_function("potential energy argon", |b| {
        b.iter(|| PotentialEnergy.calculate(&sys, &pots))
    });
}

pub fn kinetic_energy_benchmark(c: &mut Criterion) {
    let file = File::open(common::test_resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let sys = load_poscar(reader);

    // load potentials
    let path = common::test_resources_path("argon.pot.velvet");
    let file = File::open(&path).unwrap();
    let pots: Potentials = ron::de::from_reader(file).unwrap();
    
    c.bench_function("kinetic energy argon", |b| {
        b.iter(|| KineticEnergy.calculate(&sys, &pots))
    });
}

pub fn temperature_benchmark(c: &mut Criterion) {
    let file = File::open(common::test_resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let mut sys = load_poscar(reader);
    let boltz = Boltzmann::new(1000 as f32);
    boltz.apply(&mut sys);
    c.bench_function("temperature argon", |b| {
        b.iter(|| Temperature.calculate_intrinsic(&sys))
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
