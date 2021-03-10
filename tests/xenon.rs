use approx::*;

use velvet::prelude::*;
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 10_000;

#[test]
fn nve() {
    // load xenon data
    let mut system = test_utils::xenon_system();
    let potentials = test_utils::xenon_potentials();

    // prepare a NVE simulation
    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);
    let velocity_verlet = VelocityVerlet::new(0.1);
    let md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(NullThermostat));
    let config = ConfigurationBuilder::default().build();
    let mut sim = Simulation::new(system, potentials, Box::new(md), config);

    // run the simulation and return its components
    sim.run(ITERATIONS);
    let (mut system, potentials) = sim.consume();

    // compare results to values from LAMMPS

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

#[test]
fn nvt() {
    // load xenon data
    let mut system = test_utils::xenon_system();
    let potentials = test_utils::xenon_potentials();

    // prepare a NVT simulation
    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);
    let velocity_verlet = VelocityVerlet::new(0.1);
    let nose_hoover = NoseHoover::new(300.0, 1.5, 1.0);
    let md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(nose_hoover));
    let config = ConfigurationBuilder::default().build();

    let mut sim = Simulation::new(system, potentials, Box::new(md), config);
    sim.run(ITERATIONS);
    let (mut system, potentials) = sim.consume();

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