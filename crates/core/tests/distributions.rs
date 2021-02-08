use approx::*;

use std::fs::File;

use test_utils::test_resources_path;
use velvet_core::distributions::{Boltzmann, VelocityDistribution};
use velvet_core::properties::{IntrinsicProperty, Temperature};
use velvet_core::system::System;

#[test]
fn boltzmann() {
    // load system
    let path = test_resources_path("argon.sys.velvet");
    let file = File::open(&path).unwrap();
    let mut system: System = ron::de::from_reader(file).unwrap();

    let target = 1000 as f32;
    let boltz = Boltzmann::new(target);
    boltz.apply(&mut system);
    let res = Temperature.calculate_intrinsic(&system);
    assert_relative_eq!(res, target, epsilon = 1e-3);
}
