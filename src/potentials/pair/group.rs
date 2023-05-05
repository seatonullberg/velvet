//! Pair potential groups combine atom types which are modeled by the same potential.

use crate::errors::PotentialsInitializationError;
use crate::potentials::pair::PairPotential;
use crate::system::AtomType;

use std::collections::HashMap;

use strum::Display;
use uuid::Uuid;

pub struct PairPotentialGroup<'a, P> {
    potentials: HashMap<(AtomType, AtomType), P>,
    mixing_strategy: MixingStrategy,
    links: Option<(&'a P, &'a P)>,
    uuid: Uuid,
}

impl<'a, P> PairPotentialGroup<'a, P>
where
    P: PairPotential<'a>,
{
    pub fn new(
        potentials: HashMap<(AtomType, AtomType), P>,
        mixing_strategy: MixingStrategy,
        links: Option<(&'a P, &'a P)>,
    ) -> Result<Self, PotentialsInitializationError> {
        // Create a unique ID to accelerate graph lookup.
        let uuid = Uuid::new_v4();
        // TODO: VALIDATE ARGS
        Ok(PairPotentialGroup {
            potentials,
            mixing_strategy,
            links,
            uuid,
        })
    }
}

/// Determines how parameter values are interpolated for pairs of dissimilar atoms.
#[derive(Clone, Copy, Debug, Display)]
pub enum MixingStrategy {
    /// Define an exhaustive list of atom types, no interpolation needed.
    Explicit,
    /// Arithmetic mean.
    Arithmetic,
    /// Geometric mean.
    Geometric,
}
