use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

use velvet_core::neighbors::NeighborList;
use velvet_core::potentials::pair::LennardJones;
use velvet_core::potentials::{Potentials, PotentialsBuilder};
use velvet_core::system::elements::Element;
use velvet_core::system::species::Specie;
use velvet_core::system::System;
use velvet_external_data::poscar::load_poscar;

static UPDATE_FREQUENCY: usize = 3;

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
    let nl = NeighborList::new(8.5, Some((argon, argon)), UPDATE_FREQUENCY);
    PotentialsBuilder::new().with_pair(Rc::new(lj), nl).build()
}

pub fn binary_gas_potentials() -> Potentials {
    let argon = Specie::from_element(0, Element::Ar);
    let xenon = Specie::from_element(1, Element::Xe);
    let lj0 = LennardJones::new(4.184, 3.4);
    let nl0 = NeighborList::new(12.0, Some((argon, argon)), UPDATE_FREQUENCY);
    let lj1 = LennardJones::new(7.824, 4.57);
    let nl1 = NeighborList::new(12.0, Some((xenon, xenon)), UPDATE_FREQUENCY);
    let lj2 = LennardJones::new(6.276, 4.0);
    let nl2 = NeighborList::new(12.0, Some((argon, xenon)), UPDATE_FREQUENCY);
    PotentialsBuilder::new()
        .with_pair(Rc::new(lj0), nl0)
        .with_pair(Rc::new(lj1), nl1)
        .with_pair(Rc::new(lj2), nl2)
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
    let nl = NeighborList::new(12.0, Some((xenon, xenon)), UPDATE_FREQUENCY);
    PotentialsBuilder::new().with_pair(Rc::new(lj), nl).build()
}

pub fn resources_path(filename: &str) -> String {
    format!(
        "{}/../../resources/test/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    )
}
