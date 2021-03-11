use criterion::{criterion_group, criterion_main, Criterion};

use velvet::prelude::*;
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 1000;

// benchmark an entire NVE loop for the argon gas system
pub fn benchmark_nve(c: &mut Criterion) {
    let mut system = test_utils::argon_system();
    let potentials = test_utils::argon_potentials();

    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);
    let velocity_verlet = VelocityVerlet::new(0.1);
    let md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(NullThermostat));
    let config = ConfigurationBuilder::default().build();
    let mut sim = Simulation::new(system, potentials, Box::new(md), config);

    c.bench_function("argon-nve", |b| b.iter(|| sim.run(ITERATIONS)));
}

// benchmark an entire NVT loop for the argon gas system
pub fn benchmark_nvt(c: &mut Criterion) {
    let mut system = test_utils::argon_system();
    let potentials = test_utils::argon_potentials();

    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);
    let velocity_verlet = VelocityVerlet::new(0.1);
    let nose_hoover = NoseHoover::new(300.0, 1.25, 1.0);
    let md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(nose_hoover));
    let config = ConfigurationBuilder::default().build();
    let mut sim = Simulation::new(system, potentials, Box::new(md), config);

    c.bench_function("argon-nvt", |b| b.iter(|| sim.run(ITERATIONS)));
}

// benchmark expensive property calculations for the argon gas system
pub fn benchmark_properties(c: &mut Criterion) {
    let system: System = test_utils::argon_system();
    let mut potentials: Potentials = test_utils::argon_potentials();
    potentials.setup(&system);
    potentials.update(&system, 0);

    let mut group = c.benchmark_group("argon-properties");

    group.bench_function("forces", |b| {
        b.iter(|| Forces.calculate(&system, &potentials))
    });

    group.bench_function("potential-energy", |b| {
        b.iter(|| PotentialEnergy.calculate(&system, &potentials))
    });

    group.finish();
}

criterion_group!(argon, benchmark_nve, benchmark_nvt, benchmark_properties);
criterion_main!(argon);
