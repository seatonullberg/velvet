use velvet_core::prelude::*;
use velvet_external_data::prelude::*;

static UPDATE_FREQUENCY: usize = 5;

pub fn argon_system() -> System {
    Poscar.parse_system_from_file(resources_path("Ar.poscar"))
}

pub fn binary_gas_system() -> System {
    Poscar.parse_system_from_file(resources_path("ArXe.poscar"))
}

pub fn rocksalt_system() -> System {
    Poscar.parse_system_from_file(resources_path("NaCl.poscar"))
}

pub fn xenon_system() -> System {
    Poscar.parse_system_from_file(resources_path("Xe.poscar"))
}

pub fn argon_potentials() -> Potentials {
    let argon = ParticleType::from_element(Element::Ar);
    let cutoff = 8.5;
    let thickness = 1.0;
    let lj = LennardJones::new(4.184, 3.4);
    PotentialsBuilder::new()
        .pair_update_frequency(UPDATE_FREQUENCY)
        .pair(lj, (argon, argon), cutoff, thickness)
        .build()
}

pub fn binary_gas_potentials() -> Potentials {
    let argon = ParticleType::from_element(Element::Ar);
    let xenon = ParticleType::from_element(Element::Xe);
    let cutoff = 12.0;
    let thickness = 1.5;
    let lj0 = LennardJones::new(4.184, 3.4);
    let lj1 = LennardJones::new(7.824, 4.57);
    let lj2 = LennardJones::new(6.276, 4.0);
    PotentialsBuilder::new()
        .pair_update_frequency(UPDATE_FREQUENCY)
        .pair(lj0, (argon, argon), cutoff, thickness)
        .pair(lj1, (xenon, xenon), cutoff, thickness)
        .pair(lj2, (argon, xenon), cutoff, thickness)
        .build()
}

pub fn xenon_potentials() -> Potentials {
    let xenon = ParticleType::from_element(Element::Xe);
    let cutoff = 12.0;
    let thickness = 1.5;
    let lj = LennardJones::new(7.824, 4.57);
    PotentialsBuilder::new()
        .pair_update_frequency(UPDATE_FREQUENCY)
        .pair(lj, (xenon, xenon), cutoff, thickness)
        .build()
}

pub fn resources_path(filename: &str) -> String {
    format!(
        "{}/../../resources/test/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    )
}

pub fn nve_simulation(mut system: System, potentials: Potentials) -> Simulation {
    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);
    let velocity_verlet = VelocityVerlet::new(0.1);
    let md = MolecularDynamics::new(velocity_verlet, NullThermostat);
    let config = ConfigurationBuilder::new().build();
    Simulation::new(system, potentials, md, config)
}

pub fn nvt_simulation(mut system: System, potentials: Potentials) -> Simulation {
    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);
    let velocity_verlet = VelocityVerlet::new(0.1);
    let nose_hoover = NoseHoover::new(300.0, 1.25, 1.0);
    let md = MolecularDynamics::new(velocity_verlet, nose_hoover);
    let config = ConfigurationBuilder::new().build();
    Simulation::new(system, potentials, md, config)
}
