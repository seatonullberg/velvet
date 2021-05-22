// Molecular dynamics simulation of a gaseous mixture in the NVT ensemble.

use velvet::prelude::*;

fn main() {
    // Load the argon/xenon gas system from a POSCAR formatted file.
    let mut system = Poscar.parse_system_from_file("resources/test/ArXe.poscar");

    // Initialize the system temperature using a Boltzmann velocity distribution.
    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);

    // Initialize Lennard-Jones style pair potentials between each pair of chemical species.
    let argon = Species::from_element(Element::Ar);
    let xenon = Species::from_element(Element::Xe);
    let lj_argon_argon = LennardJones::new(4.184, 3.4);
    let lj_xenon_xenon = LennardJones::new(7.824, 4.57);
    let lj_argon_xenon = LennardJones::new(6.276, 4.0);

    // Store all of the system's potentials in a Potentials struct.
    let cutoff = 12.0;
    let thickness = 1.5;
    let potentials = PotentialsBuilder::new()
        .update_frequency(3)
        .pair(lj_argon_argon, (argon, argon), cutoff, thickness)
        .pair(lj_xenon_xenon, (xenon, xenon), cutoff, thickness)
        .pair(lj_argon_xenon, (argon, xenon), cutoff, thickness)
        .build();

    // Initialize a velocity Verlet style integrator.
    let velocity_verlet = VelocityVerlet::new(0.1);

    // Initialize a Nose-Hoover style thermostat.
    let nose_hoover = NoseHoover::new(300.0, 1.25, 1.0);

    // Run MD with a Nose-Hoover thermostat to simulate the NVT ensemble.
    let md = MolecularDynamics::new(velocity_verlet, nose_hoover);

    // Create an output group which writes scalar properties to a file for post-processing.
    let file_group = RawOutputGroupBuilder::new()
        .destination(std::fs::File::create("binary-gas.txt").unwrap())
        .interval(100)
        .output(PotentialEnergy)
        .output(KineticEnergy)
        .output(TotalEnergy)
        .output(Temperature)
        .build();

    // Build the configuration.
    let config = ConfigurationBuilder::new()
        .raw_output_group(file_group)
        .build();

    // Run the simulation.
    let mut sim = Simulation::new(system, potentials, md, config);
    sim.run(250_000);
}
