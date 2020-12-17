mod common;

use criterion::{criterion_group, criterion_main, Criterion};
use velvet_convert::load_poscar;
use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::thermostats::{Berendsen, NoseHoover, Thermostat};

use velvet_core::potentials::Potentials;

use std::fs::File;
use std::io::BufReader;

pub fn berendsen_benchmark(c: &mut Criterion) {
    let file = File::open(common::test_resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let mut sys = load_poscar(reader);

    // load potentials
    let path = common::test_resources_path("argon.pot.velvet");
    let file = File::open(&path).unwrap();
    let pots: Potentials = ron::de::from_reader(file).unwrap();

    let mut vv = VelocityVerlet::new(1.0);
    vv.setup(&sys, &pots);

    let target = 1000 as f32;
    let mut berendsen = Berendsen::new(target, 2.0);
    berendsen.setup(&sys);

    c.bench_function("berendsen argon", |b| {
        b.iter(|| {
            berendsen.pre_integrate(&mut sys);
            vv.integrate(&mut sys, &pots);
            berendsen.post_integrate(&mut sys);
        })
    });
}

pub fn nose_hoover_benchmark(c: &mut Criterion) {
    let file = File::open(common::test_resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let mut sys = load_poscar(reader);

    // load potentials
    let path = common::test_resources_path("argon.pot.velvet");
    let file = File::open(&path).unwrap();
    let pots: Potentials = ron::de::from_reader(file).unwrap();

    let mut vv = VelocityVerlet::new(1.0);
    vv.setup(&sys, &pots);

    let target = 1000 as f32;
    let mut nose = NoseHoover::new(target, 1.01, 1.0);
    nose.setup(&sys);

    c.bench_function("nose hoover argon", |b| {
        b.iter(|| {
            nose.pre_integrate(&mut sys);
            vv.integrate(&mut sys, &pots);
            nose.post_integrate(&mut sys);
        })
    });
}

criterion_group!(thermostats, berendsen_benchmark, nose_hoover_benchmark);
criterion_main!(thermostats);
