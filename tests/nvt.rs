use approx::*;

use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::potentials::Potentials;
use velvet_core::propagators::{MolecularDynamics, Propagator};
use velvet_core::properties::temperature::Temperature;
use velvet_core::properties::Property;
use velvet_core::system::System;
use velvet_core::thermostats::{NoseHoover, Thermostat};
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 100;

#[test]
fn argon() {
    let system = test_utils::argon_system();
    let potentials = test_utils::argon_potentials(&system);
    nvt(system, potentials)
}

// #[test]
// fn binary_gas() {
//     let system = test_utils::binary_gas_system();
//     let potentials = test_utils::binary_gas_potentials(&system);
//     nvt(system, potentials)
// }

#[test]
fn xenon() {
    let system = test_utils::xenon_system();
    let potentials = test_utils::xenon_potentials(&system);
    nvt(system, potentials)
}

fn nvt(mut system: System, potentials: Potentials) {
    let boltz = Boltzmann::new(300 as f32);
    boltz.apply(&mut system);

    let mut velocity_verlet = VelocityVerlet::new(1.0);
    velocity_verlet.setup(&system, &potentials);

    let mut nose_hoover = NoseHoover::new(300.0, 1.5, 1.0);
    nose_hoover.setup(&system);

    let mut md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(nose_hoover));
    md.setup(&mut system, &potentials);

    for _ in 0..ITERATIONS {
        md.propagate(&mut system, &potentials);
    }

    assert_relative_eq!(
        Temperature.calculate(&system, &potentials),
        300.0,
        epsilon = 100.0
    );
}
