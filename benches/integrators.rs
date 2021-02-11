use std::fs::File;
use std::io::BufReader;

use criterion::{criterion_group, criterion_main, Criterion};

use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_external_data::poscar::load_poscar;
use velvet_test_utils as test_utils;

pub fn velocity_verlet_benchmark(c: &mut Criterion) {
    let file = File::open(test_utils::resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let mut system = load_poscar(reader);

    let target = 100 as f32;
    let boltz = Boltzmann::new(target);
    boltz.apply(&mut system);

    // load potentials
    let potentials = test_utils::argon_potentials(&system);

    let mut vv = VelocityVerlet::new(1.0);
    vv.setup(&system, &potentials);

    c.bench_function("velocity_verlet", |b| {
        b.iter(|| vv.integrate(&mut system, &potentials))
    });
}

criterion_group!(integrators, velocity_verlet_benchmark);
criterion_main!(integrators);
