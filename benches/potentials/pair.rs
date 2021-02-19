use criterion::{criterion_group, criterion_main, Criterion};

use velvet_core::potentials::pair::{Harmonic, LennardJones, Mie, Morse, PairPotential};

static DISTANCE: f64 = 1.0;

pub fn lennard_jones_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("lennard_jones");
    let lj = LennardJones::new(0.8, 2.0);
    group.bench_function("energy", |b| b.iter(|| lj.energy(DISTANCE)));
    group.bench_function("force", |b| b.iter(|| lj.force(DISTANCE)));
    group.finish();
}

pub fn harmonic_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("harmonic");
    let harmonic = Harmonic::new(50.0, 2.0);
    group.bench_function("energy", |b| b.iter(|| harmonic.energy(DISTANCE)));
    group.bench_function("force", |b| b.iter(|| harmonic.force(DISTANCE)));
    group.finish();
}

pub fn mie_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("mie");
    let mie = Mie::new(0.8, 2.0, 6.0, 12.0);
    group.bench_function("energy", |b| b.iter(|| mie.energy(DISTANCE)));
    group.bench_function("force", |b| b.iter(|| mie.force(DISTANCE)));
    group.finish();
}

pub fn morse_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("morse");
    let morse = Morse::new(1.3, 4.0, 2.0);
    group.bench_function("energy", |b| b.iter(|| morse.energy(DISTANCE)));
    group.bench_function("force", |b| b.iter(|| morse.force(DISTANCE)));
    group.finish();
}

criterion_group!(
    pair_potentials,
    lennard_jones_benchmark,
    harmonic_benchmark,
    mie_benchmark,
    morse_benchmark,
);
criterion_main!(pair_potentials);
