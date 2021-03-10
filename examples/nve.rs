extern crate pretty_env_logger;

use std::fs::File;
use std::io::BufReader;

use vasp_poscar::Poscar;
use velvet::prelude::*;

static TIMESTEPS: usize = 250_000;
static OUTPUT_INTERVAL: usize = 100;
static OUTPUT_FILENAME: &str = "nve.h5";

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
    let argon = Specie::from_element(0, Element::Ar);

    // Store all of the system's potentials in a Potentials struct.
    let potentials = PotentialsBuilder::new()
        .with_pair_update_frequency(3)
        .add_pair(Box::new(lj), (argon, argon), 8.5, 1.0)
        .build();

    // Initialize a velocity Verlet style integrator.
    let velocity_verlet = VelocityVerlet::new(0.1);

    // Run MD with no thermostat to simulate the NVE ensemble.
    let md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(NullThermostat));

    // Initialize a configuration.
    let config = ConfigurationBuilder::new()
        .with_output_filename(OUTPUT_FILENAME.to_string())
        .with_output_interval(OUTPUT_INTERVAL)
        .with_output(Box::new(PotentialEnergy))
        .with_output(Box::new(KineticEnergy))
        .with_output(Box::new(TotalEnergy))
        .with_output(Box::new(Temperature))
        .build();

    // Run the simulation.
    let mut sim = Simulation::new(system, potentials, Box::new(md), config);
    sim.run(TIMESTEPS);
}
