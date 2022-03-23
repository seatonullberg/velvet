// Molecular dynamics simulation of argon gas in the NVE ensemble.

use velvet::chemfiles::*;
use velvet::core::*;
use velvet::core::outputs::Output;
use velvet::core::potentials::PotentialsBuilder;
use velvet::core::potentials::types::LennardJones;
use velvet::core::properties::energy::TotalEnergy;
use velvet::core::simulation::SimulationBuilder;
use velvet::md::*;
use velvet::md::integrators::VelocityVerlet;
use velvet::md::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet::system::*;
use velvet::system::species::Species;
use velvet::system::elements::Element;

fn main() {
    // Load the argon gas system from a POSCAR formatted file.
    let mut system = System::from_chemfile("resources/test/argon.xyz");

    // Initialize a Lennard-Jones style pair potential between all Ar-Ar pairs.
    let argon = Species::from_element(&Element::Ar);
    let lj = LennardJones::new(4.184, 3.4);

    // Store all of the system's potentials in a Potentials struct.
    let potentials = PotentialsBuilder::new()
        .pair(lj, argon, argon, 8.5)
        .build();

    // Initialize the system temperature using a Boltzmann velocity distribution.
    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system, &potentials);

    // Initialize a Velocity Verlet style integrator.
    let velocity_verlet = VelocityVerlet::new(1.0);

    // Run MD with no thermostat to simulate the NVE ensemble.
    let md = MolecularDynamicsBuilder::new(velocity_verlet).build();

    // Run the simulation.
    let energy_file = std::fs::File::create("total_energy.dat").unwrap();
    let mut simulation = SimulationBuilder::new(system, potentials, md)
        .output(TotalEnergy, energy_file, 1000)
        .build();
    simulation.run(100_000);
}
