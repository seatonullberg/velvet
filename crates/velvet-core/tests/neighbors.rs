use velvet_core::neighbors::NeighborList;
use velvet_system::elements::Element;
use velvet_system::species::Species;
use velvet_test_utils::{argon_system, argon_xenon_system};

#[test]
fn argon() {
    let system = argon_system();
    let cutoff = 10.0;
    let mut nl = NeighborList::new(cutoff);
    nl.setup(&system);
    let neighbors: Vec<&(usize, usize)> = nl.iter().collect();
    assert_eq!(neighbors.len(), 79172);
}

#[test]
fn argon_xenon() {
    let system = argon_xenon_system();
    let cutoff = 10.0;
    let mut nl = NeighborList::new(cutoff);
    let argon = Species::from_element(&Element::Ar);
    let xenon = Species::from_element(&Element::Xe);
    nl.setup_with_species(&(argon, xenon), &system);
    let neighbors: Vec<&(usize, usize)> = nl.iter().collect();
    assert_eq!(neighbors.len(), 32000);
}
