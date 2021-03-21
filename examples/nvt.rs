extern crate pretty_env_logger;

use std::fs::File;
use std::io::BufReader;

use vasp_poscar::Poscar;
use velvet::prelude::*;

static TIMESTEPS: usize = 250_000;
static OUTPUT_INTERVAL: usize = 100;
static HDF5_OUTPUT_FILENAME: &str = "nvt.h5";

fn main() {
    pretty_env_logger::init();

    // Load an Ar gas system from a POSCAR formatted file.
    let file = File::open("resources/test/Ar.poscar").unwrap();
    let reader = BufReader::new(file);
    let poscar = Poscar::from_reader(reader).unwrap();
    let mut system = import_poscar(&poscar);

    // Initialize the system temperature using a Boltzmann velocity distribution.
    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);

    // Initialize a Lennard-Jones style pair potential between all Ar-Ar pairs.
    let lj = LennardJones::new(4.184, 3.4);
    let argon = Specie::from_element(Element::Ar);

    // Store all of the system's potentials in a Potentials struct.
    let potentials = PotentialsBuilder::new()
        .with_pair_update_frequency(3)
        .add_pair(lj, (argon, argon), 8.5, 1.0)
        .build();

    // Initialize a velocity Verlet style integrator.
    let velocity_verlet = VelocityVerlet::new(0.1);

    // Initialize a Nose-Hoover style thermostat.
    let nose_hoover = NoseHoover::new(300.0, 1.25, 1.0);

    // Run MD with a Nose-Hoover thermostat to simulate the NVT ensemble.
    let md = MolecularDynamics::new(velocity_verlet, nose_hoover);

    // Initialize a configuration.
    let mut config_builder = ConfigurationBuilder::new()
        .with_output_interval(OUTPUT_INTERVAL)
        .add_output(PotentialEnergy)
        .add_output(KineticEnergy)
        .add_output(TotalEnergy)
        .add_output(Temperature);

    // Add HDF5 outputs if the feature is enabled.
    #[cfg(feature = "hdf5-output")] {
        config_builder = config_builder
            .with_hdf5_output_filename(HDF5_OUTPUT_FILENAME.to_string())
            .add_hdf5_output(PotentialEnergy)
            .add_hdf5_output(KineticEnergy)
            .add_hdf5_output(TotalEnergy)
            .add_hdf5_output(Temperature);
    }

    // Build the configuration
    let config = config_builder.build();

    // Run the simulation.
    let mut sim = Simulation::new(system, potentials, md, config);
    sim.run(TIMESTEPS);
}
