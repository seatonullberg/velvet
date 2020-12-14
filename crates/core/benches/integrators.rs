use criterion::{criterion_group, criterion_main, Criterion};
use velvet_convert::poscar::load_poscar;
use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::utils::{load_test_potentials, test_path};

pub fn velocity_verlet_benchmark(c: &mut Criterion) {
    let mut sys = load_poscar(test_path("argon.poscar"));
    let pots = load_test_potentials("argon");

    let mut vv = VelocityVerlet::new(1.0);
    vv.setup(&sys, &pots);

    c.bench_function("velocity verlet argon", |b| {
        b.iter(|| vv.integrate(&mut sys, &pots))
    });
}

criterion_group!(integrators, velocity_verlet_benchmark);
criterion_main!(integrators);
