use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::BufReader;
use velvet_convert::load_poscar;
use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::utils::{load_test_potentials, test_path};

pub fn velocity_verlet_benchmark(c: &mut Criterion) {
    let file = File::open(test_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let mut sys = load_poscar(reader);
    let pots = load_test_potentials("argon");

    let mut vv = VelocityVerlet::new(1.0);
    vv.setup(&sys, &pots);

    c.bench_function("velocity verlet argon", |b| {
        b.iter(|| vv.integrate(&mut sys, &pots))
    });
}

criterion_group!(integrators, velocity_verlet_benchmark);
criterion_main!(integrators);
