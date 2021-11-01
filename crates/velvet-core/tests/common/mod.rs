use velvet_core::potentials::types::LennardJones;
use velvet_core::potentials::{Potentials, PotentialsBuilder};
use velvet_system::elements::Element;
use velvet_system::species::Species;

pub fn argon_potentials() -> Potentials {
    let argon = Species::from_element(&Element::Ar);
    let cutoff = 10.0;
    let lj = LennardJones::new(4.184, 3.4);
    PotentialsBuilder::new()
        .pair((argon, argon), cutoff, lj)
        .build()
}
