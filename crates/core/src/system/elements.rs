//! Elemental properties.

use serde::{Deserialize, Serialize};

/// Every element on the periodic table.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, EnumString, Hash, Eq)]
pub enum Element {
    /// Hydrogen
    H,
    /// Helium
    He,
    /// Boron
    B,
    /// Nitrogen
    N,
    /// Fluorine
    F,
    /// Sodium
    Na,
    /// Chlorine
    Cl,
    /// Argon
    Ar,
    /// Xenon
    Xe,
}

impl Element {
    /// Returns the atomic mass of the element in amu.
    pub fn mass(&self) -> f32 {
        match self {
            Element::H => 1.008,
            Element::He => 4.0026,
            Element::B => 10.811,
            Element::N => 14.0067,
            Element::F => 18.998,
            Element::Na => 22.989,
            Element::Cl => 35.453,
            Element::Ar => 39.948,
            Element::Xe => 131.293,
        }
    }

    /// Returns the atomic mass of the element in amu.
    pub fn charge(&self) -> f32 {
        match self {
            Element::H => 1.0,
            Element::He => 0.0,
            Element::B => 3.0,
            Element::N => -3.0,
            Element::F => -1.0,
            Element::Na => 1.0,
            Element::Cl => -1.0,
            Element::Ar => 0.0,
            Element::Xe => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Element;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let hydrogen = Element::from_str("H").unwrap();
        assert_eq!(Element::H, hydrogen)
    }

    #[test]
    #[should_panic]
    fn compare_invalid() {
        let hydrogen = Element::H;
        let helium = Element::He;
        assert_eq!(hydrogen, helium);
    }

    #[test]
    fn compare_valid() {
        let hydrogen0 = Element::H;
        let hydrogen1 = Element::H;
        assert_eq!(hydrogen0, hydrogen1);
    }

    #[test]
    #[should_panic]
    fn compare_invalid_tuples() {
        let pair0 = (Element::He, Element::H);
        let pair1 = (Element::H, Element::He);
        assert_eq!(pair0, pair1);
    }

    #[test]
    fn compare_valid_tuples() {
        let pair0 = (Element::H, Element::He);
        let pair1 = (Element::H, Element::He);
        assert_eq!(pair0, pair1);
    }
}
