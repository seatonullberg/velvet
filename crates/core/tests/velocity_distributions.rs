use approx::*;

use velvet_core::properties::temperature::Temperature;
use velvet_core::properties::IntrinsicProperty;
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_test_utils as test_utils;

#[test]
fn boltzmann() {
    let mut system = test_utils::argon_system();
    let target = 1000.0;
    let boltz = Boltzmann::new(target);
    boltz.apply(&mut system);
    assert_relative_eq!(
        Temperature.calculate_intrinsic(&system),
        target,
        epsilon = 1e-3
    );
}
