mod common;

use approx::assert_relative_eq;
use velvet_core::properties::energy::PairEnergy;
use velvet_core::properties::Property;
use velvet_internals::float::Float;
use velvet_test_utils::argon_system;

#[test]
fn pair_energy() {
    let system = argon_system();
    let mut potentials = common::argon_potentials();
    potentials.setup(&system);
    let pair_energy = PairEnergy.calculate(&system, &potentials);
    let target = 1_483_408 as Float;
    let epsilon = 1_200 as Float;
    assert_relative_eq!(pair_energy, target, epsilon = epsilon);
}
