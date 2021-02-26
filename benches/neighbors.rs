use criterion::{criterion_group, criterion_main, Criterion};

use velvet_core::neighbors::NeighborList;
use velvet_core::system::elements::Element;
use velvet_core::system::species::Specie;
use velvet_test_utils as test_utils;

pub fn neighbor_list_update_benchmark(c: &mut Criterion) {
    let system = test_utils::argon_system();
    let argon = Specie::from_element(0, Element::Ar);

    let mut neighbor_list = NeighborList::new(8.5, Some((argon, argon)));
    neighbor_list.setup(&system);

    c.bench_function("neighbor_list_update", |b| {
        b.iter(|| neighbor_list.update(&system))
    });
}

criterion_group!(neighbors, neighbor_list_update_benchmark,);
criterion_main!(neighbors);
