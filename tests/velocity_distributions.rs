use approx::*;

use velvet_core::properties::{IntrinsicProperty, Temperature};
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_test_utils as test_utils;

#[test]
fn boltzmann() {
    // load system
    let mut system = test_utils::argon_system();

    let target = 1000 as f32;
    let boltz = Boltzmann::new(target);
    boltz.apply(&mut system);
    let res = Temperature.calculate_intrinsic(&system);
    assert_relative_eq!(res, target, epsilon = 1e-3);
}
