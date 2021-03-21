use std::fs::File;
use std::io::BufReader;

use vasp_poscar::Poscar;
use velvet_core::prelude::*;
use velvet_external_data::poscar::import_poscar;

static UPDATE_FREQUENCY: usize = 5;

pub fn argon_system() -> System {
    let file = File::open(resources_path("Ar.poscar")).unwrap();
    let reader = BufReader::new(file);
    let poscar = Poscar::from_reader(reader).unwrap();
    import_poscar(&poscar)
}

pub fn binary_gas_system() -> System {
    let file = File::open(resources_path("ArXe.poscar")).unwrap();
    let reader = BufReader::new(file);
    let poscar = Poscar::from_reader(reader).unwrap();
    import_poscar(&poscar)
}

pub fn rocksalt_system() -> System {
    let file = File::open(resources_path("NaCl.poscar")).unwrap();
    let reader = BufReader::new(file);
    let poscar = Poscar::from_reader(reader).unwrap();
    import_poscar(&poscar)
}

pub fn xenon_system() -> System {
    let file = File::open(resources_path("Xe.poscar")).unwrap();
    let reader = BufReader::new(file);
    let poscar = Poscar::from_reader(reader).unwrap();
    import_poscar(&poscar)
}

pub fn argon_potentials() -> Potentials {
    let argon = Specie::from_element(Element::Ar);
    let cutoff = 8.5;
    let thickness = 1.0;
    let lj = LennardJones::new(4.184, 3.4);
    PotentialsBuilder::new()
        .with_pair_update_frequency(UPDATE_FREQUENCY)
        .add_pair(Box::new(lj), (argon, argon), cutoff, thickness)
        .build()
}

pub fn binary_gas_potentials() -> Potentials {
    let argon = Specie::from_element(Element::Ar);
    let xenon = Specie::from_element(Element::Xe);
    let cutoff = 12.0;
    let thickness = 1.5;
    let lj0 = LennardJones::new(4.184, 3.4);
    let lj1 = LennardJones::new(7.824, 4.57);
    let lj2 = LennardJones::new(6.276, 4.0);
    PotentialsBuilder::new()
        .with_pair_update_frequency(UPDATE_FREQUENCY)
        .add_pair(Box::new(lj0), (argon, argon), cutoff, thickness)
        .add_pair(Box::new(lj1), (xenon, xenon), cutoff, thickness)
        .add_pair(Box::new(lj2), (argon, xenon), cutoff, thickness)
        .build()
}

pub fn xenon_potentials() -> Potentials {
    let xenon = Specie::from_element(Element::Xe);
    let cutoff = 12.0;
    let thickness = 1.5;
    let lj = LennardJones::new(7.824, 4.57);
    PotentialsBuilder::new()
        .with_pair_update_frequency(UPDATE_FREQUENCY)
        .add_pair(Box::new(lj), (xenon, xenon), cutoff, thickness)
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
    let config = ConfigurationBuilder::default().build();
    Simulation::new(system, potentials, md, config)
}

pub fn nvt_simulation(mut system: System, potentials: Potentials) -> Simulation {
    let boltz = Boltzmann::new(300.0);
    boltz.apply(&mut system);
    let velocity_verlet = VelocityVerlet::new(0.1);
    let nose_hoover = NoseHoover::new(300.0, 1.25, 1.0);
    let md = MolecularDynamics::new(velocity_verlet, nose_hoover);
    let config = ConfigurationBuilder::default().build();
    Simulation::new(system, potentials, md, config)
}
