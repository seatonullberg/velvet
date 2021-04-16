use velvet_core::selection::{
    setup_pairs_by_particle_type, setup_pairs_with_charge, update_pairs_by_cutoff_radius, Selection,
};
use velvet_core::system::elements::Element;
use velvet_core::system::particle::ParticleType;
use velvet_test_utils as test_utils;

#[test]
fn setup_pairs_by_particle_type_update_pairs_by_cutoff_radius() {
    let system = test_utils::binary_gas_system();
    let argon = ParticleType::from_element(Element::Ar);
    let xenon = ParticleType::from_element(Element::Xe);
    let particle_types = (argon, xenon);
    let cutoff = 10.0;
    let mut selection = Selection::new(setup_pairs_by_particle_type, update_pairs_by_cutoff_radius);
    selection.setup(&system, particle_types);
    selection.update(&system, cutoff);
    for [i, j] in selection.indices() {
        assert_eq!(system.particle_types[system.particle_type_map[*i]], argon);
        assert_eq!(system.particle_types[system.particle_type_map[*j]], xenon);
    }
}

#[test]
fn setup_pairs_with_charge_update_pairs_by_cutoff_radius() {
    // system with no charged particles
    let system = test_utils::argon_system();
    let cutoff = 10.0;
    let mut selection = Selection::new(setup_pairs_with_charge, update_pairs_by_cutoff_radius);
    selection.setup(&system, ());
    selection.update(&system, cutoff);
    assert_eq!(selection.indices().count(), 0);

    // system with charged particles
    let system = test_utils::magnesium_oxide_system();
    selection.setup(&system, ());
    selection.update(&system, cutoff);
    assert_ne!(selection.indices().count(), 0);
}
