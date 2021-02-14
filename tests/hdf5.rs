#[cfg(feature = "hdf5-output")]
use velvet_core::config::ConfigurationBuilder;
#[cfg(feature = "hdf5-output")]
use velvet_core::integrators::VelocityVerlet;
#[cfg(feature = "hdf5-output")]
use velvet_core::propagators::MolecularDynamics;
#[cfg(feature = "hdf5-output")]
use velvet_core::properties::energy::{KineticEnergy, PotentialEnergy, TotalEnergy};
#[cfg(feature = "hdf5-output")]
use velvet_core::properties::forces::Forces;
#[cfg(feature = "hdf5-output")]
use velvet_core::properties::temperature::Temperature;
#[cfg(feature = "hdf5-output")]
use velvet_core::simulation::Simulation;
#[cfg(feature = "hdf5-output")]
use velvet_core::thermostats::NullThermostat;
#[cfg(feature = "hdf5-output")]
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
#[cfg(feature = "hdf5-output")]
use velvet_test_utils as test_utils;

#[cfg(feature = "hdf5-output")]
static ITERATIONS: usize = 100;
#[cfg(feature = "hdf5-output")]
static OUTPUT_FILENAME: &str = "test.h5";
#[cfg(feature = "hdf5-output")]
static OUTPUT_INTERVAL: usize = 10;

#[cfg(feature = "hdf5-output")]
#[test]
fn hdf5_output() {
    let mut system = test_utils::argon_system();
    let size = system.size();
    let potentials = test_utils::argon_potentials(&system);

    let boltz = Boltzmann::new(300 as f32);
    boltz.apply(&mut system);

    let velocity_verlet = VelocityVerlet::new(1.0);

    let md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(NullThermostat));

    let config = ConfigurationBuilder::new()
        .with_output_filename(OUTPUT_FILENAME.to_string())
        .with_output_interval(OUTPUT_INTERVAL)
        .with_output(Box::new(Forces))
        .with_output(Box::new(KineticEnergy))
        .with_output(Box::new(PotentialEnergy))
        .with_output(Box::new(TotalEnergy))
        .with_output(Box::new(Temperature))
        .build();

    // Run the simulation.
    let mut sim = Simulation::new(system, potentials, Box::new(md), config);
    sim.run(ITERATIONS);

    let file = hdf5::File::open(OUTPUT_FILENAME).unwrap();
    let forces = file
        .dataset(format!("{:?}/forces", ITERATIONS - 1).as_ref())
        .unwrap()
        .read_1d::<[f32; 3]>()
        .unwrap();
    assert_eq!(forces.len(), size);

    let kinetic_energy = file
        .dataset(format!("{:?}/kinetic_energy", ITERATIONS - 1).as_ref())
        .unwrap()
        .read_1d::<f32>()
        .unwrap()[0];
    assert!(kinetic_energy > 0.0);

    let potential_energy = file
        .dataset(format!("{:?}/potential_energy", ITERATIONS - 1).as_ref())
        .unwrap()
        .read_1d::<f32>()
        .unwrap()[0];
    assert!(potential_energy < 0.0);

    let total_energy = file
        .dataset(format!("{:?}/total_energy", ITERATIONS - 1).as_ref())
        .unwrap()
        .read_1d::<f32>()
        .unwrap()[0];
    assert!(total_energy < 0.0);

    let temperature = file
        .dataset(format!("{:?}/temperature", ITERATIONS - 1).as_ref())
        .unwrap()
        .read_1d::<f32>()
        .unwrap()[0];
    assert!(temperature > 0.0);

    std::fs::remove_file(OUTPUT_FILENAME).unwrap();
}
