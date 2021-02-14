use approx::*;

use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::potentials::Potentials;
use velvet_core::propagators::{MolecularDynamics, Propagator};
use velvet_core::properties::energy::PotentialEnergy;
use velvet_core::properties::Property;
use velvet_core::system::System;
use velvet_core::thermostats::NullThermostat;
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 100;

#[test]
fn argon() {
    let system = test_utils::argon_system();
    let potentials = test_utils::argon_potentials(&system);
    nve(system, potentials, -763.0)
}

// #[test]
// fn binary_gas() {
//     let system = test_utils::binary_gas_system();
//     let potentials = test_utils::binary_gas_potentials(&system);
//     nve(system, potentials, -1090.0)
// }

#[test]
fn xenon() {
    let system = test_utils::xenon_system();
    let potentials = test_utils::xenon_potentials(&system);
    nve(system, potentials, -1300.0)
}

fn nve(mut system: System, potentials: Potentials, target: f32) {
    let boltz = Boltzmann::new(300 as f32);
    boltz.apply(&mut system);

    let mut velocity_verlet = VelocityVerlet::new(1.0);
    velocity_verlet.setup(&system, &potentials);

    let mut md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(NullThermostat));
    md.setup(&mut system, &potentials);

    for _ in 0..ITERATIONS {
        md.propagate(&mut system, &potentials);
    }

    assert_relative_eq!(
        PotentialEnergy.calculate(&system, &potentials),
        target,
        epsilon = 10.0
    );
}
