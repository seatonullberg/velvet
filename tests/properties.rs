use approx::*;

use velvet_core::properties::{
    Forces, IntrinsicProperty, KineticEnergy, PotentialEnergy, Property, Temperature, TotalEnergy,
};

mod common;

#[test]
fn forces() {
    // load system
    // let path = test_resources_path("fluorine.sys.velvet");
    // let file = File::open(&path).unwrap();
    // let system: System = ron::de::from_reader(file).unwrap();
    let system = common::get_fluorine_system();

    // load potentials
    // let path = test_resources_path("fluorine.pot.velvet");
    // let file = File::open(&path).unwrap();
    // let potentials: Potentials = ron::de::from_reader(file).unwrap();
    let potentials = common::get_fluorine_potentials(&system);

    let forces = Forces.calculate(&system, &potentials);

    let total_force = (forces[0] + forces[1]).norm();
    assert_relative_eq!(total_force, 0.0, epsilon = 1e-4);

    let target_force = 30.0;
    assert_relative_eq!(forces[0][0], -target_force, epsilon = 1e-4);
    assert_relative_eq!(forces[1][0], target_force, epsilon = 1e-4);
}

#[test]
fn energy() {
    // load system
    // let path = test_resources_path("fluorine.sys.velvet");
    // let file = File::open(&path).unwrap();
    // let system: System = ron::de::from_reader(file).unwrap();
    let system = common::get_fluorine_system();

    // load potentials
    // let path = test_resources_path("fluorine.pot.velvet");
    // let file = File::open(&path).unwrap();
    // let potentials: Potentials = ron::de::from_reader(file).unwrap();
    let potentials = common::get_fluorine_potentials(&system);

    let ke = KineticEnergy.calculate_intrinsic(&system);
    let pe = PotentialEnergy.calculate(&system, &potentials);
    let te = TotalEnergy.calculate(&system, &potentials);

    let ke_target = 0.0007483;
    assert_relative_eq!(ke + pe, te, epsilon = 1e-5);
    assert_relative_eq!(ke, ke_target, epsilon = 1e-5);
}

#[test]
fn temperature() {
    // load system
    // let path = test_resources_path("fluorine.sys.velvet");
    // let file = File::open(&path).unwrap();
    // let system: System = ron::de::from_reader(file).unwrap();
    let system = common::get_fluorine_system();

    let temp = Temperature.calculate_intrinsic(&system);

    let target_temp = 300.0;
    assert_relative_eq!(temp, target_temp, epsilon = 1e-2);
}
