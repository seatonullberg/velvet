//! Representation of a unique particle type.

use uuid::Uuid;

use crate::internal::Float;
use crate::system::elements::Element;

/// Representation of a unique particle type.
#[derive(Clone, Copy, Debug)]
pub struct ParticleType {
    id: u128,
    mass: Float,
    charge: Float,
}

impl ParticleType {
    /// Returns a new [`ParticleType`].
    pub fn new(mass: Float, charge: Float) -> ParticleType {
        ParticleType {
            id: Uuid::new_v4().as_u128(),
            mass,
            charge,
        }
    }

    /// Constructs a [`ParticleType`] from an [`Element`].
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_core::prelude::*;
    ///
    /// let pt = ParticleType::from_element(Element::Na);
    /// assert_eq!(pt.mass(), Element::Na.mass());
    /// assert_eq!(pt.charge(), Element::Na.charge());
    /// ```
    pub fn from_element(element: Element) -> ParticleType {
        ParticleType {
            id: element.number() as u128,
            mass: element.mass(),
            charge: element.charge(),
        }
    }

    /// Returns the particle's unique ID.
    pub fn id(&self) -> u128 {
        self.id
    }

    /// Returns the particle's mass.
    pub fn mass(&self) -> Float {
        self.mass
    }

    /// Returns the particle's electronic charge.
    pub fn charge(&self) -> Float {
        self.charge
    }
}

impl PartialEq for ParticleType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::ParticleType;
    use crate::system::elements::Element;

    #[test]
    fn from_element() {
        let element = Element::H;
        let pt = ParticleType::from_element(element);
        assert_eq!(pt.mass(), element.mass());
        assert_eq!(pt.charge(), element.charge());
        assert_eq!(pt.id(), element.number() as u128);
    }

    #[test]
    fn compare_equivalent() {
        let hydrogen1 = ParticleType::from_element(Element::H);
        let hydrogen2 = ParticleType::from_element(Element::H);
        assert_eq!(hydrogen1, hydrogen2);
    }

    #[test]
    fn compare_nonequivalent() {
        let hydrogen = ParticleType::from_element(Element::H);
        let helium = ParticleType::from_element(Element::He);
        assert_ne!(hydrogen, helium);
        let pt = ParticleType::new(hydrogen.mass(), hydrogen.charge());
        assert_ne!(pt, hydrogen);
    }
}
