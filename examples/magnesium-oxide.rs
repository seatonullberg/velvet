// Molecular dynamics simulation of MgO in the NVT ensemble.

use velvet::prelude::*;

fn main() {
    // Load the MgO system from a POSCAR formatted file.
    let mut system = Poscar.parse_system_from_file("resources/test/MgO.poscar");

    // Initialize the system temperature using a Boltzmann velocity distribution.
    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);

    // Initialize Buckingham style pair potentials between each pair of particle types.
    let magnesium = ParticleType::from_element(Element::Mg);
    let oxygen = ParticleType::from_element(Element::O);
    let buck_mg_o = Buckingham::new(18946.9176, 0.32, 0.0);
    let buck_o_o = Buckingham::new(524960.604, 0.149, 642.94068);

    // Initialize standard Coulombic potentials between all charged particles.
    let coul = StandardCoulombic::new(1.0);

    // Store all of the system's potentials in a Potentials struct.
    let cutoff_buck = 5.0;
    let cutoff_coul = 10.0;
    let thickness = 3.0;
    let potentials = PotentialsBuilder::new()
        .pair_update_frequency(3)
        .pair(buck_mg_o, (magnesium, oxygen), cutoff_buck, thickness)
        .pair(buck_o_o, (oxygen, oxygen), cutoff_buck, thickness)
        .coulomb(coul, cutoff_coul, thickness)
        .build();

    // Initialize a velocity Verlet style integrator.
    let velocity_verlet = VelocityVerlet::new(1e-6);

    // Initialize a Nose-Hoover style thermostat.
    let nose_hoover = NoseHoover::new(300.0, 1.25, 1e-4);

    // Run MD with a Nose-Hoover thermostat to simulate the NVT ensemble.
    let md = MolecularDynamics::new(velocity_verlet, nose_hoover);

    // Create an output group which writes scalar properties to stderr (the default destination).
    let stderr_group = RawOutputGroupBuilder::new()
        .interval(100)
        .output(PotentialEnergy)
        .output(KineticEnergy)
        .output(TotalEnergy)
        .output(Temperature)
        .build();

    // Write the same outputs to a file for post-processing.
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
        .raw_output_group(stderr_group)
        .raw_output_group(file_group)
        .build();

    // Run the simulation.
    let mut sim = Simulation::new(system, potentials, md, config);
    sim.run(250_000);
}
