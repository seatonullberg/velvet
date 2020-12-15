//! Elemental properties.

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// Every element on the periodic table.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, EnumString)]
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
    /// Argon
    Ar,
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
            Element::Ar => 39.948,
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
}
