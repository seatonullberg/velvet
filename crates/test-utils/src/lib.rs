use nalgebra::{Matrix3, Vector3};

use velvet_core::potentials::pair::{Harmonic, LennardJones, PairDescriptor, PairMeta};
use velvet_core::potentials::{Potentials, PotentialsBuilder};
use velvet_core::system::cell::Cell;
use velvet_core::system::elements::Element;
use velvet_core::system::{System, SystemBuilder};

pub fn argon_system() -> System {
    let system_builder = SystemBuilder::new(2);
    system_builder
        .with_cell(Cell::from_matrix(Matrix3::new(
            17.0,
            0.0,
            0.0,
            -0.0000007430936,
            17.0,
            0.0,
            -0.0000007430936,
            -0.0000007430936,
            17.0,
        )))
        .with_elements(vec![Element::Ar, Element::Ar])
        .with_positions(vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(3.4, 3.4, 3.4),
        ])
        .with_velocities(vec![
            Vector3::new(-0.007225223, -0.0024057564, 0.0026065109),
            Vector3::new(0.001179634, 0.0035252622, -0.00041327748),
        ])
        .build()
}

pub fn fluorine_system() -> System {
    let system_builder = SystemBuilder::new(2);
    system_builder
        .with_cell(Cell::from_matrix(Matrix3::new(
            10.0,
            0.0,
            0.0,
            -0.0000004371139,
            10.0,
            0.0,
            -0.0000004371139,
            -0.0000004371139,
            10.0,
        )))
        .with_elements(vec![Element::F, Element::F])
        .with_positions(vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.3, 0.0, 0.0),
        ])
        .with_velocities(vec![
            Vector3::new(-0.007225223, -0.0024057564, 0.0026065109),
            Vector3::new(0.001179634, 0.0035252622, -0.00041327748),
        ])
        .build()
}

pub fn argon_potentials(system: &System) -> Potentials {
    let potentials_builder = PotentialsBuilder::new();
    let lj = LennardJones::new(1.0, 3.4);
    let meta = PairMeta::new(8.5, (Element::Ar, Element::Ar));
    let descr = PairDescriptor::new(Box::new(lj), meta, system);
    potentials_builder.add_pair(descr).build()
}

pub fn fluorine_potentials(system: &System) -> Potentials {
    let potentials_builder = PotentialsBuilder::new();
    let harmonic = Harmonic::new(300.0, 1.2);
    let meta = PairMeta::new(5.0, (Element::F, Element::F));
    let descr = PairDescriptor::new(Box::new(harmonic), meta, system);
    potentials_builder.add_pair(descr).build()
}

pub fn resources_path(filename: &str) -> String {
    format!(
        "{}/../../resources/test/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    )
}
