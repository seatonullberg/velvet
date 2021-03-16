use approx::*;

use velvet::prelude::*;
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 10_000;

#[test]
fn nve() {
    let system = test_utils::binary_gas_system();
    let potentials = test_utils::binary_gas_potentials();
    let mut sim = test_utils::nve_simulation(system, potentials);

    sim.run(ITERATIONS);
    let (mut system, potentials) = sim.consume();

    let pe_target = -4550.0;
    assert_relative_eq!(
        PotentialEnergy.calculate(&mut system, &potentials),
        pe_target,
        epsilon = 150.0
    );

    let ke_target = 425.0;
    assert_relative_eq!(
        KineticEnergy.calculate(&mut system, &potentials),
        ke_target,
        epsilon = 100.0
    );

    let temp_target = 1300.0;
    assert_relative_eq!(
        Temperature.calculate(&mut system, &potentials),
        temp_target,
        epsilon = 250.0
    );
}

#[test]
fn nvt() {
    let system = test_utils::binary_gas_system();
    let potentials = test_utils::binary_gas_potentials();
    let mut sim = test_utils::nvt_simulation(system, potentials);

    sim.run(ITERATIONS);
    let (mut system, potentials) = sim.consume();

    let pe_target = -4850.0;
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
