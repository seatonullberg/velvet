use criterion::{criterion_group, criterion_main, Criterion};
use velvet_convert::load_poscar;
use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::thermostats::{Berendsen, Thermostat};
use velvet_core::utils::{load_test_potentials, test_path};

pub fn berendsen_benchmark(c: &mut Criterion) {
    let mut sys = load_poscar(test_path("argon.poscar"));
    let pots = load_test_potentials("argon");

    let mut vv = VelocityVerlet::new(1.0);
    vv.setup(&sys, &pots);

    let target = 1000 as f32;
    let mut berendsen = Berendsen::new(target, 2.0);

    c.bench_function("berendsen argon", |b| {
        b.iter(|| {
            berendsen.post_integrate(&mut sys);
        })
    });
}

criterion_group!(thermostats, berendsen_benchmark);
criterion_main!(thermostats);
