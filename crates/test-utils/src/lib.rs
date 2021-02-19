use std::fs::File;
use std::io::BufReader;

use velvet_core::potentials::coulomb::{CoulombMeta, Wolf};
use velvet_core::potentials::pair::{Harmonic, LennardJones, PairMeta};
use velvet_core::potentials::{Potentials, PotentialsBuilder};
use velvet_core::system::elements::Element;
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

pub fn fluorine_system() -> System {
    let file = File::open(resources_path("F.poscar")).unwrap();
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

pub fn argon_potentials(system: &System) -> Potentials {
    let lj = LennardJones::new(4.184, 3.4);
    let meta = PairMeta::new(8.5, (Element::Ar, Element::Ar), system);
    PotentialsBuilder::new()
        .add_pair(meta, Box::new(lj))
        .build()
}

pub fn binary_gas_potentials(system: &System) -> Potentials {
    let lj0 = LennardJones::new(1.0, 3.4);
    let meta0 = PairMeta::new(12.0, (Element::Ar, Element::Ar), &system);
    let lj1 = LennardJones::new(1.87, 4.57);
    let meta1 = PairMeta::new(12.0, (Element::Xe, Element::Xe), &system);
    let lj2 = LennardJones::new(1.5, 4.0);
    let meta2 = PairMeta::new(12.0, (Element::Ar, Element::Xe), &system);
    PotentialsBuilder::new()
        .add_pair(meta0, Box::new(lj0))
        .add_pair(meta1, Box::new(lj1))
        .add_pair(meta2, Box::new(lj2))
        .build()
}

pub fn fluorine_potentials(system: &System) -> Potentials {
    let harmonic = Harmonic::new(300.0, 1.2);
    let meta = PairMeta::new(5.0, (Element::F, Element::F), system);
    PotentialsBuilder::new()
        .add_pair(meta, Box::new(harmonic))
        .build()
}

pub fn rocksalt_potentials(system: &System) -> Potentials {
    let wolf = Wolf::new(8.0);
    let wolf_meta = CoulombMeta::new(8.0, &system);
    let lj0 = LennardJones::new(0.00598, 4.612);
    let lj0_meta = PairMeta::new(8.0, (Element::Cl, Element::Cl), &system);
    let lj1 = LennardJones::new(0.0187, 2.497);
    let lj1_meta = PairMeta::new(8.0, (Element::Na, Element::Na), &system);
    let lj2 = LennardJones::new(0.01058, 3.5545);
    let lj2_meta = PairMeta::new(8.0, (Element::Cl, Element::Na), &system);
    PotentialsBuilder::new()
        .add_coulomb(wolf_meta, Box::new(wolf))
        .add_pair(lj0_meta, Box::new(lj0))
        .add_pair(lj1_meta, Box::new(lj1))
        .add_pair(lj2_meta, Box::new(lj2))
        .build()
}

pub fn xenon_potentials(system: &System) -> Potentials {
    let lj = LennardJones::new(1.87, 4.57);
    let meta = PairMeta::new(12.0, (Element::Xe, Element::Xe), system);
    PotentialsBuilder::new()
        .add_pair(meta, Box::new(lj))
        .build()
}

pub fn resources_path(filename: &str) -> String {
    format!(
        "{}/../../resources/test/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    )
}
