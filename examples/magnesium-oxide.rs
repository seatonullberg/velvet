// Molecular dynamics simulation of MgO in the NVT ensemble.

use velvet::prelude::*;

fn main() {
    // Load the MgO system from a POSCAR formatted file.
    let mut system = Poscar.parse_system_from_file("resources/test/MgO.poscar");

    // Initialize the system temperature using a Boltzmann velocity distribution.
    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);

    // Initialize Buckingham style pair potentials between each pair of chemical species.
    let magnesium = Species::from_element(Element::Mg);
    let oxygen = Species::from_element(Element::O);
    let buck_mg_o = Buckingham::new(18946.9176, 0.32, 0.0);
    let buck_o_o = Buckingham::new(524960.604, 0.149, 642.94068);

    // Initialize a DSF potential to evaluate electrostatic interactions.
    let dsf = DampedShiftedForce::new(0.1, 10.0);
    // let coul = StandardCoulombic::new(1.0);

    // Store all of the system's potentials in a Potentials struct.
    let potentials = PotentialsBuilder::new()
        .update_frequency(1)
        .coulomb(dsf, 10.0, 3.0)
        .build();

    // Initialize a velocity Verlet style integrator.
    let velocity_verlet = VelocityVerlet::new(0.001);

    // Initialize a Nose-Hoover style thermostat.
    let nose_hoover = NoseHoover::new(300.0, 1.25, 0.1);

    // Run MD with a Nose-Hoover thermostat to simulate the NVT ensemble.
    let md = MolecularDynamics::new(velocity_verlet, nose_hoover);

    // Create an output group which writes scalar properties to a file for post-processing.
    let file_group = RawOutputGroupBuilder::new()
        .destination(std::fs::File::create("magnesium-oxide.txt").unwrap())
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
    sim.run(50_000);
}
