mod common;

use approx::*;

use std::fs::File;

use velvet_core::thermostats::{Thermostat, Berendsen, NoseHoover};
use velvet_core::properties::{IntrinsicProperty, Temperature};
use velvet_core::distributions::{VelocityDistribution, Boltzmann};
use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::system::System;
use velvet_core::potentials::Potentials;

#[test]
fn berendsen() {
    // load system
    let path = common::test_resources_path("argon.sys.velvet");
    let file = File::open(&path).unwrap();
    let mut system: System = ron::de::from_reader(file).unwrap();

    let target = 100 as f32;
    let boltz = Boltzmann::new(target);
    boltz.apply(&mut system);

    // load potentials
    let path = common::test_resources_path("argon.pot.velvet");
    let file = File::open(&path).unwrap();
    let potentials: Potentials = ron::de::from_reader(file).unwrap();
    
    let timestep = 1.0;
    let mut velocity_verlet = VelocityVerlet::new(timestep);
    velocity_verlet.setup(&system, &potentials);
    let tau = 2.0;
    let mut berendsen = Berendsen::new(target, tau);
    berendsen.setup(&system);

    for _ in 0..common::ITERATIONS {
        berendsen.pre_integrate(&mut system);
        velocity_verlet.integrate(&mut system, &potentials);
        berendsen.post_integrate(&mut system);
    }

    assert_relative_eq!(Temperature.calculate_intrinsic(&system), target, epsilon = 1e-4);
}

#[test]
fn nose_hoover() {
    // load system
    let path = common::test_resources_path("argon.sys.velvet");
    let file = File::open(&path).unwrap();
    let mut system: System = ron::de::from_reader(file).unwrap();

    let target = 100 as f32;
    let boltz = Boltzmann::new(target);
    boltz.apply(&mut system);
    
    // load potentials
    let path = common::test_resources_path("argon.pot.velvet");
    let file = File::open(&path).unwrap();
    let potentials: Potentials = ron::de::from_reader(file).unwrap();

    let timestep = 1.0;
    let mut velocity_verlet = VelocityVerlet::new(timestep);
    velocity_verlet.setup(&system, &potentials);
    let freq = 1.01;
    let mut nose_hoover = NoseHoover::new(target, freq, timestep);
    nose_hoover.setup(&system);

    for _ in 0..common::ITERATIONS {
        nose_hoover.pre_integrate(&mut system);
        velocity_verlet.integrate(&mut system, &potentials);
        nose_hoover.post_integrate(&mut system);
    }

    assert_relative_eq!(Temperature.calculate_intrinsic(&system), target, epsilon = 100.0);
}
