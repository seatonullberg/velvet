use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::BufReader;
use test_utils::test_resources_path;
use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::potentials::Potentials;
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_external_data::poscar::load_poscar;

pub fn velocity_verlet_benchmark(c: &mut Criterion) {
    let file = File::open(test_resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let mut sys = load_poscar(reader);

    let target = 100 as f32;
    let boltz = Boltzmann::new(target);
    boltz.apply(&mut sys);

    // load potentials
    let path = test_resources_path("argon.pot.velvet");
    let file = File::open(&path).unwrap();
    let pots: Potentials = ron::de::from_reader(file).unwrap();

    let mut vv = VelocityVerlet::new(1.0);
    vv.setup(&sys, &pots);

    c.bench_function("velocity_verlet", |b| {
        b.iter(|| vv.integrate(&mut sys, &pots))
    });
}

criterion_group!(integrators, velocity_verlet_benchmark);
criterion_main!(integrators);
