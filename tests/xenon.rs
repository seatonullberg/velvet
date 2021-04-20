use approx::*;
use serial_test::serial;

use velvet::prelude::*;
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 10_000;

#[test]
#[serial]
fn nve() {
    let system = test_utils::xenon_system();
    let potentials = test_utils::xenon_potentials();
    let mut sim = test_utils::nve_simulation(system, potentials);

    sim.run(ITERATIONS);
    let (mut system, potentials) = sim.consume();

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
#[serial]
fn nvt() {
    let system = test_utils::xenon_system();
    let potentials = test_utils::xenon_potentials();
    let mut sim = test_utils::nvt_simulation(system, potentials);

    sim.run(ITERATIONS);
    let (mut system, potentials) = sim.consume();

    let pe_target = -5450.0;
    assert_relative_eq!(
        PotentialEnergy.calculate(&mut system, &potentials),
        pe_target,
        epsilon = 200.0
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
