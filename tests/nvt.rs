use approx::*;

use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::potentials::Potentials;
use velvet_core::propagators::{MolecularDynamics, Propagator};
use velvet_core::properties::energy::{KineticEnergy, PotentialEnergy};
use velvet_core::properties::temperature::Temperature;
use velvet_core::properties::Property;
use velvet_core::system::System;
use velvet_core::thermostats::{NoseHoover, Thermostat};
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 10000;

#[test]
fn argon() {
    let mut system = test_utils::argon_system();
    let potentials = test_utils::argon_potentials(&system);
    nvt(&mut system, &potentials);

    let pe_target = -3090.0;
    assert_relative_eq!(
        PotentialEnergy.calculate(&mut system, &potentials),
        pe_target,
        epsilon = 50.0
    );

    let ke_target = 90.0;
    assert_relative_eq!(
        KineticEnergy.calculate(&mut system, &potentials),
        ke_target,
        epsilon = 25.0
    );

    let temp_target = 300.0;
    assert_relative_eq!(
        Temperature.calculate(&system, &potentials),
        temp_target,
        epsilon = 50.0
    );
}

#[test]
fn binary_gas() {
    let mut system = test_utils::binary_gas_system();
    let potentials = test_utils::binary_gas_potentials(&system);
    nvt(&mut system, &potentials);

    let pe_target = -4800.0;
    assert_relative_eq!(
        PotentialEnergy.calculate(&mut system, &potentials),
        pe_target,
        epsilon = 100.0
    );

    let ke_target = 100.0;
    assert_relative_eq!(
        KineticEnergy.calculate(&mut system, &potentials),
        ke_target,
        epsilon = 25.0
    );

    let temp_target = 300.0;
    assert_relative_eq!(
        Temperature.calculate(&system, &potentials),
        temp_target,
        epsilon = 50.0
    );
}

#[test]
fn xenon() {
    let mut system = test_utils::xenon_system();
    let potentials = test_utils::xenon_potentials(&system);
    nvt(&mut system, &potentials);

    let pe_target = -5500.0;
    assert_relative_eq!(
        PotentialEnergy.calculate(&mut system, &potentials),
        pe_target,
        epsilon = 1000.0
    );

    let ke_target = 90.0;
    assert_relative_eq!(
        KineticEnergy.calculate(&mut system, &potentials),
        ke_target,
        epsilon = 25.0
    );

    let temp_target = 300.0;
    assert_relative_eq!(
        Temperature.calculate(&system, &potentials),
        temp_target,
        epsilon = 100.0
    );
}

fn nvt(system: &mut System, potentials: &Potentials) {
    let boltz = Boltzmann::new(300.0);
    boltz.apply(system);

    let mut velocity_verlet = VelocityVerlet::new(0.5);
    velocity_verlet.setup(system, potentials);

    let mut nose_hoover = NoseHoover::new(300.0, 1.5, 1.0);
    nose_hoover.setup(system);

    let mut md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(nose_hoover));
    md.setup(system, potentials);

    for _ in 0..ITERATIONS {
        md.propagate(system, potentials);
    }
}
