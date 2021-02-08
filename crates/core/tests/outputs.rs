use approx::*;

use std::fs::File;

use test_utils::test_resources_path;
use velvet_core::outputs::Output;
use velvet_core::potentials::Potentials;
use velvet_core::properties::{Forces, KineticEnergy, PotentialEnergy, Temperature, TotalEnergy};
use velvet_core::system::System;

#[test]
fn forces() {
    // load system
    let path = test_resources_path("fluorine.sys.velvet");
    let file = File::open(&path).unwrap();
    let system: System = ron::de::from_reader(file).unwrap();

    // load potentials
    let path = test_resources_path("fluorine.pot.velvet");
    let file = File::open(&path).unwrap();
    let potentials: Potentials = ron::de::from_reader(file).unwrap();

    // write file
    let file = hdf5::File::create("forces.h5").unwrap();
    let group = file.create_group("test").unwrap();
    Forces.output(&system, &potentials, &group);

    // read file
    let file = hdf5::File::open("forces.h5").unwrap();
    let forces = file.dataset("test/forces").unwrap();
    let res = forces.read_1d::<[f32; 3]>().unwrap();

    // delete the file
    std::fs::remove_file("forces.h5").unwrap();

    let target = 30.0;
    assert_relative_eq!(res[0][0], -target, epsilon = 1e-3);
    assert_relative_eq!(res[1][0], target, epsilon = 1e-3);
}

#[test]
fn energy() {
    // load system
    let path = test_resources_path("fluorine.sys.velvet");
    let file = File::open(&path).unwrap();
    let system: System = ron::de::from_reader(file).unwrap();

    // load potentials
    let path = test_resources_path("fluorine.pot.velvet");
    let file = File::open(&path).unwrap();
    let potentials: Potentials = ron::de::from_reader(file).unwrap();

    // write file
    let file = hdf5::File::create("energy.h5").unwrap();
    let group = file.create_group("test").unwrap();
    KineticEnergy.output(&system, &potentials, &group);
    PotentialEnergy.output(&system, &potentials, &group);
    TotalEnergy.output(&system, &potentials, &group);

    // read file
    let file = hdf5::File::open("energy.h5").unwrap();
    let ke = file.dataset("test/kinetic_energy").unwrap();
    let pe = file.dataset("test/potential_energy").unwrap();
    let te = file.dataset("test/total_energy").unwrap();
    let ke_res = ke.read_1d::<f32>().unwrap();
    let pe_res = pe.read_1d::<f32>().unwrap();
    let te_res = te.read_1d::<f32>().unwrap();

    // delete the file
    std::fs::remove_file("energy.h5").unwrap();

    let ke_target = 0.0007483;
    assert_relative_eq!(ke_res[0], ke_target, epsilon = 1e-5);
    assert_relative_eq!(ke_res[0] + pe_res[0], te_res[0], epsilon = 1e-5);
}

#[test]
fn temperature() {
    // load system
    let path = test_resources_path("fluorine.sys.velvet");
    let file = File::open(&path).unwrap();
    let system: System = ron::de::from_reader(file).unwrap();

    // load potentials
    let path = test_resources_path("fluorine.pot.velvet");
    let file = File::open(&path).unwrap();
    let potentials: Potentials = ron::de::from_reader(file).unwrap();

    // write file
    let file = hdf5::File::create("temperature.h5").unwrap();
    let group = file.create_group("test").unwrap();
    Temperature.output(&system, &potentials, &group);

    // read file
    let file = hdf5::File::open("temperature.h5").unwrap();
    let temperature = file.dataset("test/temperature").unwrap();
    let res = temperature.read_1d::<f32>().unwrap();

    // delete the file
    std::fs::remove_file("temperature.h5").unwrap();

    let target = 300.0;
    assert_relative_eq!(res[0], target, epsilon = 1e-2);
}
