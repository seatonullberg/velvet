use std::fs::File;
use std::io::BufReader;

use velvet_core::potentials::pair::LennardJones;
use velvet_core::potentials::{Potentials, PotentialsBuilder};
use velvet_core::system::elements::Element;
use velvet_core::system::species::Specie;
use velvet_core::system::System;
use velvet_external_data::poscar::load_poscar;

pub fn argon_system() -> System {
    let file = File::open(resources_path("Ar.poscar")).unwrap();
    let reader = BufReader::new(file);
    load_poscar(reader)
}

pub fn binary_gas_system() -> System {
    let file = File::open(resources_path("ArXe.poscar")).unwrap();
    let reader = BufReader::new(file);
    load_poscar(reader)
}

pub fn rocksalt_system() -> System {
    let file = File::open(resources_path("NaCl.poscar")).unwrap();
    let reader = BufReader::new(file);
    load_poscar(reader)
}

pub fn xenon_system() -> System {
    let file = File::open(resources_path("Xe.poscar")).unwrap();
    let reader = BufReader::new(file);
    load_poscar(reader)
}

pub fn argon_potentials() -> Potentials {
    let argon = Specie::from_element(0, Element::Ar);
    let lj = LennardJones::new(4.184, 3.4);
    PotentialsBuilder::new()
        .with_pair(Box::new(lj), 8.5, (argon, argon))
        .build()
}

pub fn binary_gas_potentials() -> Potentials {
    let argon = Specie::from_element(0, Element::Ar);
    let xenon = Specie::from_element(1, Element::Xe);
    let lj0 = LennardJones::new(4.184, 3.4);
    let lj1 = LennardJones::new(7.824, 4.57);
    let lj2 = LennardJones::new(6.276, 4.0);
    PotentialsBuilder::new()
        .with_pair(Box::new(lj0), 12.0, (argon, argon))
        .with_pair(Box::new(lj1), 12.0, (xenon, xenon))
        .with_pair(Box::new(lj2), 12.0, (argon, xenon))
        .build()
}

// pub fn rocksalt_potentials(system: &System) -> Potentials {
//     let wolf = Wolf::new(8.0);
//     let wolf_meta = CoulombMeta::new(8.0, &system);
//     let lj0 = LennardJones::new(0.00598, 4.612);
//     let lj0_meta = PairMeta::new(8.0, (Element::Cl, Element::Cl), &system);
//     let lj1 = LennardJones::new(0.0187, 2.497);
//     let lj1_meta = PairMeta::new(8.0, (Element::Na, Element::Na), &system);
//     let lj2 = LennardJones::new(0.01058, 3.5545);
//     let lj2_meta = PairMeta::new(8.0, (Element::Cl, Element::Na), &system);
//     PotentialsBuilder::new()
//         .add_coulomb(wolf_meta, Box::new(wolf))
//         .add_pair(lj0_meta, Box::new(lj0))
//         .add_pair(lj1_meta, Box::new(lj1))
//         .add_pair(lj2_meta, Box::new(lj2))
//         .build()
// }

pub fn xenon_potentials() -> Potentials {
    let xenon = Specie::from_element(0, Element::Xe);
    let lj = LennardJones::new(7.824, 4.57);
    PotentialsBuilder::new()
        .with_pair(Box::new(lj), 12.0, (xenon, xenon))
        .build()
}

pub fn resources_path(filename: &str) -> String {
    format!(
        "{}/../../resources/test/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    )
}
