use std::fs::File;

use test_utils::test_resources_path;
use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::potentials::Potentials;
use velvet_core::system::System;
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};

static ITERATIONS: usize = 5000;

#[test]
fn velocity_verlet() {
    // load system
    let path = test_resources_path("argon.sys.velvet");
    let file = File::open(&path).unwrap();
    let mut system: System = ron::de::from_reader(file).unwrap();

    let target = 100 as f32;
    let boltz = Boltzmann::new(target);
    boltz.apply(&mut system);

    // load potentials
    let path = test_resources_path("argon.pot.velvet");
    let file = File::open(&path).unwrap();
    let potentials: Potentials = ron::de::from_reader(file).unwrap();

    let timestep = 1.0;
    let mut velocity_verlet = VelocityVerlet::new(timestep);
    velocity_verlet.setup(&system, &potentials);

    for _ in 0..ITERATIONS {
        velocity_verlet.integrate(&mut system, &potentials);
    }

    assert!(system.velocities[0].norm() < 0.1)
}
