//! Elemental properties.

use serde::{Deserialize, Serialize};

/// Every element on the periodic table.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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
