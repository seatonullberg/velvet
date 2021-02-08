use criterion::{criterion_group, criterion_main, Criterion};
use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::thermostats::{Berendsen, NoseHoover, Thermostat};
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_external_data::poscar::load_poscar;

use velvet_core::potentials::Potentials;

use test_utils::test_resources_path;

use std::fs::File;
use std::io::BufReader;

pub fn thermostats_group_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("thermostats");

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

    let mut berendsen = Berendsen::new(target, 2.0);
    berendsen.setup(&sys);

    let mut nose = NoseHoover::new(target, 1.01, 1.0);
    nose.setup(&sys);

    group.bench_function("berendsen", |b| {
        b.iter(|| {
            berendsen.pre_integrate(&mut sys);
            vv.integrate(&mut sys, &pots);
            berendsen.post_integrate(&mut sys);
        })
    });

    group.bench_function("nose_hoover", |b| {
        b.iter(|| {
            nose.pre_integrate(&mut sys);
            vv.integrate(&mut sys, &pots);
            nose.post_integrate(&mut sys);
        })
    });

    group.finish()
}

criterion_group!(thermostats, thermostats_group_benchmark);
criterion_main!(thermostats);
