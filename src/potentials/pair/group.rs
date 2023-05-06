//! Pair potential groups combine atom types which are modeled by the same potential.

use crate::errors::PotentialsInitializationError;
use crate::potentials::pair::PairPotential;
use crate::system::AtomType;

use std::collections::HashMap;

use strum::Display;
use uuid::Uuid;

/// Group of atom types which share a common pair potential.
///
/// Potential groups enable 'hybrid' simulations which apply
/// different potential types to a single system of atoms.
/// For example, when simulating a solid-liquid interface it
/// is sensible to use different potentials to describe each phase.
/// Additionally, a third group must be provided which 'links' the
/// atom types in both groups with another potential that describes
/// the cross terms.
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
        // If this group links other groups, the mixing strategy must be `Explicit`.

        // NOTE TO SELF:
        // ONLY DO THE HALF CHECK HERE. NO NEED TO CHECK FOR EXHAUSTIVE LIST
        // OF ATOM PAIRS UNTIL ALL GROUPS HAVE BEEN COMPILED.

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
#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum MixingStrategy {
    /// Define an exhaustive list of atom types.
    /// No interpolation required.
    Explicit,
    /// Arithmetic mean.
    Arithmetic,
    /// Geometric mean.
    Geometric,
}
