use criterion::{criterion_group, criterion_main, Criterion};

use velvet_core::potentials::pair::{Harmonic, LennardJones, Mie, Morse, PairPotential};

#[cfg(feature = "f64")]
static DISTANCE: f64 = 1.0;
#[cfg(not(feature = "f64"))]
static DISTANCE: f32 = 1.0;

pub fn pair_potential_energy_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pair_potential_energy");

    // Lennard-Jones
    let lj = LennardJones::new(0.8, 2.0);
    group.bench_function("lennard_jones", |b| b.iter(|| lj.energy(DISTANCE)));

    // Harmonic
    let harmonic = Harmonic::new(50.0, 2.0);
    group.bench_function("harmonic", |b| b.iter(|| harmonic.energy(DISTANCE)));

    // Mie
    let mie = Mie::new(0.8, 2.0, 6.0, 12.0);
    group.bench_function("mie", |b| b.iter(|| mie.energy(DISTANCE)));

    // Morse
    let morse = Morse::new(1.3, 4.0, 2.0);
    group.bench_function("morse", |b| b.iter(|| morse.energy(DISTANCE)));

    group.finish();
}

pub fn pair_potential_force_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pair_potential_force");

    // Lennard-Jones
    let lj = LennardJones::new(0.8, 2.0);
    group.bench_function("lennard_jones", |b| b.iter(|| lj.force(DISTANCE)));

    // Harmonic
    let harmonic = Harmonic::new(50.0, 2.0);
    group.bench_function("harmonic", |b| b.iter(|| harmonic.force(DISTANCE)));

    // Mie
    let mie = Mie::new(0.8, 2.0, 6.0, 12.0);
    group.bench_function("mie", |b| b.iter(|| mie.force(DISTANCE)));

    // Morse
    let morse = Morse::new(1.3, 4.0, 2.0);
    group.bench_function("morse", |b| b.iter(|| morse.force(DISTANCE)));

    group.finish();
}

criterion_group!(
    pair_potentials,
    pair_potential_energy_benchmark,
    pair_potential_force_benchmark,
);
criterion_main!(pair_potentials);
