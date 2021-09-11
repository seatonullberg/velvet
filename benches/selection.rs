use criterion::{criterion_group, criterion_main, Criterion};

use velvet::prelude::*;
use velvet_test_utils as test_utils;

pub fn benchmark_pairs(c: &mut Criterion) {
    let system: System = test_utils::magnesium_oxide_system();
    let mut group = c.benchmark_group("pairs");

    let mut selection = Selection::new(setup_pairs_by_species, update_pairs_by_cutoff_radius);
    group.bench_function("setup-pairs-by-species", |b| {
        b.iter(|| {
            selection.setup(
                &system,
                (
                    Species::from_element(Element::Mg),
                    Species::from_element(Element::O),
                ),
            )
        })
    });

    let mut selection = Selection::new(setup_pairs_with_charge, update_pairs_by_cutoff_radius);
    group.bench_function("setup-pairs-with-charge", |b| {
        b.iter(|| selection.setup(&system, ()))
    });

    let mut selection = Selection::new(setup_pairs_by_species, update_pairs_by_cutoff_radius);
    selection.setup(
        &system,
        (
            Species::from_element(Element::Mg),
            Species::from_element(Element::O),
        ),
    );
    group.bench_function("update-pairs-by-cutoff-radius", |b| {
        b.iter(|| selection.update(&system, 10.0))
    });
}

criterion_group!(selection, benchmark_pairs);
criterion_main!(selection);
