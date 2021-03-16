use serde::{Deserialize, Serialize};

use crate::internal::{self, Float};
use crate::potentials::coulomb::CoulombPotential;
use crate::potentials::pair::PairPotential;

/// Metadata describing pairwise interactions.
#[derive(Serialize, Deserialize)]
pub struct PairInteraction {
    pub potential: internal::Rc<dyn PairPotential>,
    pub cutoff: Float,
    pub index_i: usize,
    pub index_j: usize,
}

/// Metadata describing electrostatic coulomb interactions.
#[derive(Serialize, Deserialize)]
pub struct CoulombInteraction {
    pub potential: internal::Rc<dyn CoulombPotential>,
    pub cutoff: Float,
    pub index_i: usize,
    pub index_j: usize,
}
