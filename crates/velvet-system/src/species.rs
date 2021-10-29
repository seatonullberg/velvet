//! Representation of a unique chemical species.

use std::hash::{Hash, Hasher};

use crate::elements::Element;
use velvet_internals::float::Float;

/// Representation of a unique chemical species.
#[derive(Clone, Debug)]
pub struct Species {
    name: String,
    mass: Float,
    charge: Float,
}

impl Species {
    /// Returns a new [`Species`].
    pub fn new<S>(name: S, mass: Float, charge: Float) -> Species
    where
        S: Into<String>,
    {
        Species {
            name: name.into(),
            mass,
            charge,
        }
    }

    /// Constructs a [`Species`] from an [`Element`].
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    ///
    /// let species = Species::from_element(Element::Na);
    /// assert_eq!(species.mass(), Element::Na.mass());
    /// assert_eq!(species.charge(), Element::Na.charge());
    /// ```
    pub fn from_element(element: Element) -> Species {
        Species {
            name: element.to_string(),
            mass: element.mass(),
            charge: element.charge(),
        }
    }

    /// Returns the species' name.
    pub fn name(&self) -> &str {
        &self.name
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
        self.name.hash(state);
    }
}

impl PartialEq for Species {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use super::Species;
    use crate::elements::Element;

    #[test]
    fn from_element() {
        let element = Element::H;
        let species = Species::from_element(element);
        assert_eq!(species.mass(), element.mass());
        assert_eq!(species.charge(), element.charge());
        assert_eq!(species.name(), element.symbol());
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
        let new_hydrogen = Species::new("H_new", hydrogen.mass(), hydrogen.charge());
        assert_ne!(new_hydrogen, hydrogen);
    }
}
