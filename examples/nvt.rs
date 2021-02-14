use std::fs::File;
use std::io::BufReader;

use velvet::prelude::*;

static TIMESTEPS: usize = 100_000;
static OUTPUT_INTERVAL: usize = 50;
static OUTPUT_FILENAME: &str = "nvt.h5";

fn main() {
    // Load an Ar gas system from a POSCAR formatted file.
    let file = File::open("resources/test/Ar.poscar").unwrap();
    let reader = BufReader::new(file);
    let mut system = load_poscar(reader);

    // Initialize the system temperature using a Boltzmann velocity distribution.
    let boltz = Boltzmann::new(300 as f32);
    boltz.apply(&mut system);

    // Initialize a Lennard-Jones style pair potential between all Ar-Ar pairs.
    let lj = LennardJones::new(1.0, 3.4);
    let meta = PairMeta::new(8.0, (Element::Ar, Element::Ar), &system);

    // Store all of the system's potentials in a Potentials struct.
    let potentials = PotentialsBuilder::new()
        .add_pair(meta, Box::new(lj))
        .build();

    // Initialize a velocity Verlet style integrator.
    let velocity_verlet = VelocityVerlet::new(1.0);

    // Initialize a Nose-Hoover style thermostat.
    let nose_hoover = NoseHoover::new(300 as f32, 1.5, 1.0);

    // Run MD with a thermostat to simulate the NVT ensemble.
    let md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(nose_hoover));

    // Initialize a configuration.
    let config = ConfigurationBuilder::new()
        .with_output_filename(OUTPUT_FILENAME.to_string())
        .with_output_interval(OUTPUT_INTERVAL)
        .with_output(Box::new(Temperature))
        .with_output(Box::new(TotalEnergy))
        .build();

    // Run the simulation.
    let mut sim = Simulation::new(system, potentials, Box::new(md), config);
    sim.run(TIMESTEPS);
}
