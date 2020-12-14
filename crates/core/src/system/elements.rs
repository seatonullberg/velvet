//! Elemental properties.

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// Every element on the periodic table.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[derive(EnumString)]
pub enum Element {
    /// Hydrogen
    H,
    /// Helium
    He,
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
            Element::F => 18.998,
            Element::Ar => 39.948,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::Element;

    #[test]
    fn from_str() {
        let hydrogen = Element::from_str("H").unwrap();
        assert_eq!(Element::H, hydrogen)
    }
}