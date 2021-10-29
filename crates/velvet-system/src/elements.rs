//! Elemental properties.

use velvet_internals::float::Float;

/// Every element on the periodic table.
#[derive(Clone, Copy, Debug, Display, EnumString, Eq, Hash, PartialEq)]
pub enum Element {
    /// Hydrogen
    H,
    /// Helium
    He,
    /// Boron
    B,
    /// Nitrogen
    N,
    /// Oxygen
    O,
    /// Fluorine
    F,
    /// Sodium
    Na,
    /// Magnesium
    Mg,
    /// Chlorine
    Cl,
    /// Argon
    Ar,
    /// Xenon
    Xe,
}

impl Element {
    /// Returns the atomic mass of the element in amu.
    pub const fn mass(&self) -> Float {
        match self {
            Element::H => 1.008,
            Element::He => 4.0026,
            Element::B => 10.811,
            Element::N => 14.0067,
            Element::O => 15.999,
            Element::F => 18.998,
            Element::Na => 22.989,
            Element::Mg => 24.305,
            Element::Cl => 35.453,
            Element::Ar => 39.948,
            Element::Xe => 131.293,
        }
    }

    /// Returns the electronic charge of the element as a multiple of electron charge.
    pub const fn charge(&self) -> Float {
        match self {
            Element::H => 1.0,
            Element::He => 0.0,
            Element::B => 3.0,
            Element::N => -3.0,
            Element::O => -2.0,
            Element::F => -1.0,
            Element::Na => 1.0,
            Element::Mg => 2.0,
            Element::Cl => -1.0,
            Element::Ar => 0.0,
            Element::Xe => 0.0,
        }
    }

    /// Returns the atomic number of the element.
    pub const fn number(&self) -> u8 {
        match self {
            Element::H => 1,
            Element::He => 2,
            Element::B => 5,
            Element::N => 7,
            Element::O => 8,
            Element::F => 9,
            Element::Na => 11,
            Element::Mg => 12,
            Element::Cl => 17,
            Element::Ar => 18,
            Element::Xe => 54,
        }
    }

    /// Returns the chemical symbol of the element.
    pub fn symbol(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Element;
    use std::str::FromStr;

    #[test]
    fn from_str_valid() {
        let hydrogen = Element::from_str("H").unwrap();
        assert_eq!(Element::H, hydrogen)
    }

    #[test]
    #[should_panic]
    fn from_str_invalid() {
        let _ = Element::from_str("not a valid symbol").unwrap();
    }
}
