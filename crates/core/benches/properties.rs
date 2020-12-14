use criterion::{criterion_group, criterion_main, Criterion};
use velvet_convert::poscar::load_poscar;
use velvet_core::distributions::{Boltzmann, VelocityDistribution};
use velvet_core::properties::{
    Forces, IntrinsicProperty, KineticEnergy, PotentialEnergy, Property, Temperature,
};
use velvet_core::utils::{load_test_potentials, test_path};

pub fn forces_benchmark(c: &mut Criterion) {
    let sys = load_poscar(test_path("argon.poscar"));
    let pots = load_test_potentials("argon");
    c.bench_function("forces argon", |b| b.iter(|| Forces.calculate(&sys, &pots)));
}

pub fn potential_energy_benchmark(c: &mut Criterion) {
    let sys = load_poscar(test_path("argon.poscar"));
    let pots = load_test_potentials("argon");
    c.bench_function("potential energy argon", |b| {
        b.iter(|| PotentialEnergy.calculate(&sys, &pots))
    });
}

pub fn kinetic_energy_benchmark(c: &mut Criterion) {
    let sys = load_poscar(test_path("argon.poscar"));
    let pots = load_test_potentials("argon");
    c.bench_function("kinetic energy argon", |b| {
        b.iter(|| KineticEnergy.calculate(&sys, &pots))
    });
}

pub fn temperature_benchmark(c: &mut Criterion) {
    let mut sys = load_poscar(test_path("argon.poscar"));
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
