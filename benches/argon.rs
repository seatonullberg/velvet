use criterion::{criterion_group, criterion_main, Criterion};

use velvet::prelude::*;
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 10_000;

pub fn benchmark(c: &mut Criterion) {
    let mut system = test_utils::argon_system();
    let potentials: Potentials = test_utils::argon_potentials();

    // prepare a NVE simulation
    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);
    let velocity_verlet = VelocityVerlet::new(0.1);
    let md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(NullThermostat));
    let config = ConfigurationBuilder::default().build();
    let mut sim = Simulation::new(system, potentials, Box::new(md), config);
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

    group.finish();
}

criterion_group!(argon, benchmark,);
criterion_main!(argon);
