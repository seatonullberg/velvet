//! Representation of a unique chemical species.

use std::hash::{Hash, Hasher};

use uuid::Uuid;

use crate::internal::Float;
use crate::system::elements::Element;

/// Representation of a unique chemical species.
#[derive(Clone, Copy, Debug)]
pub struct Species {
    id: u128,
    mass: Float,
    charge: Float,
}

impl Species {
    /// Returns a new [`Species`].
    pub fn new(mass: Float, charge: Float) -> Species {
        Species {
            id: Uuid::new_v4().as_u128(),
            mass,
            charge,
        }
    }

    /// Constructs a [`Species`] from an [`Element`].
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_core::prelude::*;
    ///
    /// let species = Species::from_element(Element::Na);
    /// assert_eq!(species.mass(), Element::Na.mass());
    /// assert_eq!(species.charge(), Element::Na.charge());
    /// ```
    pub fn from_element(element: Element) -> Species {
        Species {
            id: element.number() as u128,
            mass: element.mass(),
            charge: element.charge(),
        }
    }

    /// Returns the species' unique ID.
    pub fn id(&self) -> u128 {
        self.id
    }

    /// Returns the species' mass.
    pub fn mass(&self) -> Float {
        self.mass
    }

    /// Returns the species' electronic charge.
    pub fn charge(&self) -> Float {
        self.charge
    }
}

impl Hash for Species {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Species {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::Species;
    use crate::system::elements::Element;

    #[test]
    fn from_element() {
        let element = Element::H;
        let species = Species::from_element(element);
        assert_eq!(species.mass(), element.mass());
        assert_eq!(species.charge(), element.charge());
        assert_eq!(species.id(), element.number() as u128);
    }

    #[test]
    fn compare_equivalent() {
        let hydrogen1 = Species::from_element(Element::H);
        let hydrogen2 = Species::from_element(Element::H);
        assert_eq!(hydrogen1, hydrogen2);
    }

    #[test]
    fn compare_nonequivalent() {
        let hydrogen = Species::from_element(Element::H);
        let helium = Species::from_element(Element::He);
        assert_ne!(hydrogen, helium);
        let species = Species::new(hydrogen.mass(), hydrogen.charge());
        assert_ne!(species, hydrogen);
    }
}
