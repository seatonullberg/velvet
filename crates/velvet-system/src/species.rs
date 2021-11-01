//! Representation of a unique chemical species.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::elements::Element;
use velvet_internals::float::Float;

/// Representation of a unique chemical species.
#[derive(Clone, Copy, Debug)]
pub struct Species {
    id: u64,
    mass: Float,
    charge: Float,
}

impl Species {
    /// Returns a new [Species].
    pub fn new<S>(name: S, mass: Float, charge: Float) -> Self
    where
        S: Into<String>,
    {
        let mut hasher = DefaultHasher::new();
        name.into().hash(&mut hasher);
        let id = hasher.finish();
        Species { id, mass, charge }
    }

    /// Constructs a [Species] from an [Element].
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    ///
    /// let sodium = Element::Na;
    /// let species = Species::from_element(&sodium);
    /// assert_eq!(species.mass(), sodium.mass());
    /// assert_eq!(species.charge(), sodium.charge());
    /// ```
    pub fn from_element(element: &Element) -> Self {
        let mut hasher = DefaultHasher::new();
        element.to_string().hash(&mut hasher);
        let id = hasher.finish();
        Species {
            id,
            mass: element.mass(),
            charge: element.charge(),
        }
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

impl PartialEq for Species {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::Species;
    use crate::elements::Element;

    #[test]
    fn from_element() {
        let hydrogen = Element::H;
        let species = Species::from_element(&hydrogen);
        assert_eq!(species.mass(), hydrogen.mass());
        assert_eq!(species.charge(), hydrogen.charge());
    }

    #[test]
    fn compare_equivalent() {
        let hydrogen = Element::H;
        let species1 = Species::from_element(&hydrogen);
        let species2 = Species::from_element(&hydrogen);
        assert_eq!(species1, species2);
    }

    #[test]
    fn compare_nonequivalent() {
        let hydrogen = Element::H;
        let helium = Element::He;
        let species1 = Species::from_element(&hydrogen);
        let species2 = Species::from_element(&helium);
        assert_ne!(species1, species2);
        let species3 = Species::new("H_new", hydrogen.mass(), hydrogen.charge());
        assert_ne!(species3, species1);
    }
}
