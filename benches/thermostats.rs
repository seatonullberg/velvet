use std::fs::File;
use std::io::BufReader;

use criterion::{criterion_group, criterion_main, Criterion};

use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::thermostats::{Berendsen, NoseHoover, Thermostat};
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_external_data::poscar::load_poscar;
use velvet_test_utils as test_utils;

pub fn thermostats_group_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("thermostats");

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

    let mut berendsen = Berendsen::new(target, 2.0);
    berendsen.setup(&system);

    let mut nose = NoseHoover::new(target, 1.01, 1.0);
    nose.setup(&system);

    group.bench_function("berendsen", |b| {
        b.iter(|| {
            berendsen.pre_integrate(&mut system);
            vv.integrate(&mut system, &potentials);
            berendsen.post_integrate(&mut system);
        })
    });

    group.bench_function("nose_hoover", |b| {
        b.iter(|| {
            nose.pre_integrate(&mut system);
            vv.integrate(&mut system, &potentials);
            nose.post_integrate(&mut system);
        })
    });

    group.finish()
}

criterion_group!(thermostats, thermostats_group_benchmark);
criterion_main!(thermostats);
