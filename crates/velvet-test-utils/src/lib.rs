use velvet_chemfiles::FromChemfile;
use velvet_system::System;

pub fn argon_system() -> System {
    System::from_chemfile(resources_path("argon.xyz"))
}

pub fn argon_xenon_system() -> System {
    System::from_chemfile(resources_path("argon-xenon.xyz"))
}

pub fn zirconia_system() -> System {
    System::from_chemfile(resources_path("zirconia.cif"))
}

// pub fn argon_potentials() -> Potentials {
//     let argon = Species::from_element(Element::Ar);
//     let cutoff = 8.5;
//     let thickness = 1.0;
//     let lj = LennardJones::new(4.184, 3.4);
//     PotentialsBuilder::new()
//         .update_frequency(UPDATE_FREQUENCY)
//         .pair(lj, (argon, argon), cutoff, thickness)
//         .build()
// }

// pub fn binary_gas_potentials() -> Potentials {
//     let argon = Species::from_element(Element::Ar);
//     let xenon = Species::from_element(Element::Xe);
//     let cutoff = 12.0;
//     let thickness = 1.5;
//     let lj0 = LennardJones::new(4.184, 3.4);
//     let lj1 = LennardJones::new(7.824, 4.57);
//     let lj2 = LennardJones::new(6.276, 4.0);
//     PotentialsBuilder::new()
//         .update_frequency(UPDATE_FREQUENCY)
//         .pair(lj0, (argon, argon), cutoff, thickness)
//         .pair(lj1, (xenon, xenon), cutoff, thickness)
//         .pair(lj2, (argon, xenon), cutoff, thickness)
//         .build()
// }

// pub fn magnesium_oxide_potentials() -> Potentials {
//     let magnesium = Species::from_element(Element::Mg);
//     let oxygen = Species::from_element(Element::O);
//     let cutoff = 10.0;
//     let thickness = 1.5;
//     let buck_mg_o = Buckingham::new(18947.0, 0.3242, 0.0);
//     let buck_o_o = Buckingham::new(524960.0, 0.1490, 643.0);
//     let wolf = Wolf::new(0.1, 10.0);
//     PotentialsBuilder::new()
//         .update_frequency(UPDATE_FREQUENCY)
//         .pair(buck_mg_o, (magnesium, oxygen), cutoff, thickness)
//         .pair(buck_o_o, (oxygen, oxygen), cutoff, thickness)
//         .coulomb(wolf, cutoff, thickness)
//         .build()
// }

// pub fn xenon_potentials() -> Potentials {
//     let xenon = Species::from_element(Element::Xe);
//     let cutoff = 12.0;
//     let thickness = 1.5;
//     let lj = LennardJones::new(7.824, 4.57);
//     PotentialsBuilder::new()
//         .update_frequency(UPDATE_FREQUENCY)
//         .pair(lj, (xenon, xenon), cutoff, thickness)
//         .build()
// }

pub fn resources_path(filename: &str) -> String {
    format!(
        "{}/../../resources/test/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    )
}

// pub fn nve_simulation(mut system: System, potentials: Potentials) -> Simulation {
//     let boltz = Boltzmann::new(300.0);
//     boltz.apply(&mut system);
//     let velocity_verlet = VelocityVerlet::new(0.1);
//     let md = MolecularDynamics::new(velocity_verlet, NullThermostat);
//     let config = ConfigurationBuilder::new().build();
//     Simulation::new(system, potentials, md, config)
// }

// pub fn nvt_simulation(mut system: System, potentials: Potentials) -> Simulation {
//     let boltz = Boltzmann::new(300.0);
//     boltz.apply(&mut system);
//     let velocity_verlet = VelocityVerlet::new(0.1);
//     let nose_hoover = NoseHoover::new(300.0, 1.25, 1.0);
//     let md = MolecularDynamics::new(velocity_verlet, nose_hoover);
//     let config = ConfigurationBuilder::new().build();
//     Simulation::new(system, potentials, md, config)
// }
