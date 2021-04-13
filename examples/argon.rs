// Molecular dynamics simulation of argon gas in the NVE ensemble.

use std::fs::File;
use std::io::BufReader;

use vasp_poscar::Poscar;
use velvet::prelude::*;

fn main() {
    // Load the argon gas system from a POSCAR formatted file.
    let file = File::open("resources/test/Ar.poscar").unwrap();
    let reader = BufReader::new(file);
    let poscar = Poscar::from_reader(reader).unwrap();
    let mut system = import_poscar(&poscar);

    // Initialize the system temperature using a Boltzmann velocity distribution.
    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);

    // Initialize a Lennard-Jones style pair potential between all Ar-Ar pairs.
    let argon = Specie::from_element(Element::Ar);
    let lj = LennardJones::new(4.184, 3.4);

    // Store all of the system's potentials in a Potentials struct.
    let potentials = PotentialsBuilder::new()
        .with_pair_update_frequency(3)
        .add_pair(lj, (argon, argon), 8.5, 1.0)
        .build();

    // Initialize a Velocity Verlet style integrator.
    let velocity_verlet = VelocityVerlet::new(0.1);

    // Run MD with no thermostat to simulate the NVE ensemble.
    let md = MolecularDynamics::new(velocity_verlet, NullThermostat);

    // Create an output group which writes scalar properties to stderr (the default destination).
    let stderr_group = RawOutputGroupBuilder::new()
        .interval(100)
        .output(PotentialEnergy)
        .output(KineticEnergy)
        .output(TotalEnergy)
        .output(Temperature)
        .build();

    // Create an output group which writes the forces acting on each atom to a text file.
    let file_group = RawOutputGroupBuilder::new()
        .destination(File::create("argon-forces.txt").unwrap())
        .interval(500)
        .output(Forces)
        .build();

    // Build the configuration.
    let config = ConfigurationBuilder::new()
        .raw_output_group(stderr_group)
        .raw_output_group(file_group)
        .build();

    // Run the simulation.
    let mut sim = Simulation::new(system, potentials, md, config);
    sim.run(250_000);
}
