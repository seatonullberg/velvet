use approx::*;

use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::potentials::Potentials;
use velvet_core::propagators::{MolecularDynamics, Propagator};
use velvet_core::properties::energy::{KineticEnergy, PotentialEnergy};
use velvet_core::properties::temperature::Temperature;
use velvet_core::properties::Property;
use velvet_core::system::System;
use velvet_core::thermostats::NullThermostat;
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 10000;

#[test]
fn argon() {
    let mut system = test_utils::argon_system();
    let potentials = test_utils::argon_potentials(&system);
    nve(&mut system, &potentials);

    let pe_target = -3130.0;
    assert_relative_eq!(
        PotentialEnergy.calculate(&mut system, &potentials),
        pe_target,
        epsilon = 50.0
    );

    let ke_target = 50.0;
    assert_relative_eq!(
        KineticEnergy.calculate(&mut system, &potentials),
        ke_target,
        epsilon = 5.0
    );

    let temp_target = 150.0;
    assert_relative_eq!(
        Temperature.calculate(&mut system, &potentials),
        temp_target,
        epsilon = 25.0
    );
}

#[test]
fn xenon() {
    let mut system = test_utils::xenon_system();
    let potentials = test_utils::xenon_potentials(&system);
    nve(&mut system, &potentials);

    let pe_target = -5500.0;
    assert_relative_eq!(
        PotentialEnergy.calculate(&mut system, &potentials),
        pe_target,
        epsilon = 200.0
    );

    let ke_target = 50.0;
    assert_relative_eq!(
        KineticEnergy.calculate(&mut system, &potentials),
        ke_target,
        epsilon = 10.0
    );

    let temp_target = 150.0;
    assert_relative_eq!(
        Temperature.calculate(&mut system, &potentials),
        temp_target,
        epsilon = 50.0
    );
}

fn nve(system: &mut System, potentials: &Potentials) {
    let boltz = Boltzmann::new(300.0);
    boltz.apply(system);

    let mut velocity_verlet = VelocityVerlet::new(0.1);
    velocity_verlet.setup(&system, &potentials);

    let mut md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(NullThermostat));
    md.setup(system, &potentials);

    for _ in 0..ITERATIONS {
        md.propagate(system, &potentials);
    }
}
