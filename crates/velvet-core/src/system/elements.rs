//! Elemental properties.

use crate::internal::Float;

/// Every element on the periodic table.
#[derive(Clone, Copy, Debug, PartialEq, EnumString, Hash, Eq)]
pub enum Element {
    /// Hydrogen
    H,
    /// Helium
    He,
    /// Lithium
    Li,
    /// Beryllium
    Be,
    /// Boron
    B,
    /// Carbon
    C,
    /// Nitrogen
    N,
    /// Oxygen
    O,
    /// Fluorine
    F,
    /// Neon
    Ne,
    /// Sodium
    Na,
    /// Magnesium
    Mg,
    /// Aluminum
    Al,
    /// Silicon
    Si,
    /// Phosphorus
    P,
    /// Sulfur
    S,
    /// Chlorine
    Cl,
    /// Argon
    Ar,
    /// Potassium
    K,
    /// Calcium
    Ca,
    /// Scandium
    Sc,
    /// Titanium
    Ti,
    /// Vanadium
    V,
    /// Chromium
    Cr,
    /// Manganese
    Mn,
    /// Iron
    Fe,
    /// Cobalt
    Co,
    /// Nickel
    Ni,
    /// Copper
    Cu,
    /// Zinc
    Zn,
    /// Gallium
    Ga,
    /// Germanium
    Ge,
    /// Arsenic
    As,
    /// Selenium
    Se,
    /// Bromine
    Br,
    /// Krypton
    Kr,
    /// Rubidium
    Rb,
    /// Strontium
    Sr,
    /// Yttrium
    Y,
    /// Zirconium
    Zr,
    /// Niobium
    Nb,
    /// Molybdenum
    Mo,
    /// Technetium
    Tc,
    /// Ruthenium
    Ru,
    /// Rhodium
    Rh,
    /// Palladium
    Pd,
    /// Silver
    Ag,
    /// Cadmium
    Cd,
    /// Indium
    In,
    /// Tin
    Sn,
    /// Antimony
    Sb,
    /// Tellurium
    Te,
    /// Iodine
    I,
    /// Xenon
    Xe,
    /// Cesium
    Cs,
    /// Barium
    Ba,
    /// Lanthanum
    La,
    /// Cerium
    Ce,
    /// Praseodymium
    Pr,
    /// Neodymium
    Nd,
    /// Promethium
    Pm,
    /// Samarium
    Sm,
    /// Europium
    Eu,
    /// Gadolinium
    Gd,
    /// Terbium
    Tb,
    /// Dysprosium
    Dy,
    /// Holmium
    Ho,
    /// Erbium
    Er,
    /// Thulium
    Tm,
    /// Ytterbium
    Yb,
    /// Lutetium
    Lu,
    /// Hafnium
    Hf,
    /// Tantalum
    Ta,
    /// Tungsten
    W,
    /// Rhenium
    Re,
    /// Osmium
    Os,
    /// Iridium
    Ir,
    /// Platinum
    Pt,
    /// Gold
    Au,
    /// Mercury
    Hg,
    /// Thallium
    Tl,
    /// Lead
    Pb,
    /// Bismuth
    Bi,
    /// Polonium
    Po,
    /// Astatine
    At,
    /// Radon
    Rn,
    /// Francium
    Fr,
    /// Radium
    Ra,
    /// Actinium
    Ac,
    /// Thorium
    Th,
    /// Protactinium
    Pa,
    /// Uranium
    U,
}

impl Element {
    /// Returns the atomic mass of the element in amu.
    pub const fn mass(&self) -> Float {
        match self {
            Element::H => 1.008,
            Element::He => 4.0026,
            Element::Li => 6.94,
            Element::Be => 9.0122,
            Element::B => 10.81,
            Element::C => 12.011,
            Element::N => 14.007,
            Element::O => 15.999,
            Element::F => 18.998,
            Element::Ne => 20.180,
            Element::Na => 22.990,
            Element::Mg => 24.305,
            Element::Al => 26.982,
            Element::Si => 28.085,
            Element::P => 30.974,
            Element::S => 32.06,
            Element::Cl => 35.45,
            Element::Ar => 39.948,
            Element::K => 39.098,
            Element::Ca => 40.08,
            Element::Sc => 44.956,
            Element::Ti => 47.867,
            Element::V => 50.942,
            Element::Cr => 52.0,
            Element::Mn => 54.938,
            Element::Fe => 55.845,
            Element::Co => 58.933,
            Element::Ni => 58.693,
            Element::Cu => 63.546,
            Element::Zn => 65.38,
            Element::Ga => 69.723,
            Element::Ge => 72.630,
            Element::As => 74.922,
            Element::Se => 78.971,
            Element::Br => 79.904,
            Element::Kr => 83.798,
            Element::Rb => 85.468,
            Element::Sr => 87.62,
            Element::Y => 88.906,
            Element::Zr => 91.224,
            Element::Nb => 92.906,
            Element::Mo => 95.95,
            Element::Tc => 98.0,
            Element::Ru => 101.07,
            Element::Rh => 102.91,
            Element::Pd => 106.42,
            Element::Ag => 107.87,
            Element::Cd => 112.41,
            Element::In => 114.82,
            Element::Sn => 118.71,
            Element::Sb => 121.76,
            Element::Te => 127.60,
            Element::I => 126.90,
            Element::Xe => 131.29,
            Element::Cs => 132.905,
            Element::Ba => 137.327,
            Element::La => 138.905,
            Element::Ce => 140.116,
            Element::Pr => 140.907,
            Element::Nd => 144.242,
            Element::Pm => 145.000,
            Element::Sm => 150.36,
            Element::Eu => 151.964,
            Element::Gd => 157.25,
            Element::Tb => 158.925,
            Element::Dy => 162.500,
            Element::Ho => 164.930,
            Element::Er => 167.259,
            Element::Tm => 168.934,
            Element::Yb => 173.045,
            Element::Lu => 174.966,
            Element::Hf => 178.49,
            Element::Ta => 180.948,
            Element::W => 183.84,
            Element::Re => 186.207,
            Element::Os => 190.23,
            Element::Ir => 192.217,
            Element::Pt => 195.085,
            Element::Au => 196.967,
            Element::Hg => 200.592,
            Element::Tl => 204.383,
            Element::Pb => 207.2,
            Element::Bi => 208.980,
            Element::Po => 209.000,
            Element::At => 210.000,
            Element::Rn => 222.000,
            Element::Fr => 223.000,
            Element::Ra => 226.000,
            Element::Ac => 227.000,
            Element::Th => 232.038,
            Element::Pa => 231.036,
            Element::U => 238.02891,
        }
    }

    /// Returns the electronic charge of the element as a multiple of electron charge.
    pub const fn charge(&self) -> Float {
        match self {
            Element::H => 1.0,
            Element::He => 0.0,
            Element::Li => 1.0,
            Element::Be => 2.0,
            Element::B => 3.0,
            Element::C => 4.0,
            Element::N => -3.0,
            Element::O => -2.0,
            Element::F => -1.0,
            Element::Ne => 0.0,
            Element::Na => 1.0,
            Element::Mg => 2.0,
            Element::Al => 3.0,
            Element::Si => 4.0,
            Element::P => -3.0,
            Element::S => -2.0,
            Element::Cl => -1.0,
            Element::Ar => 0.0,
            Element::K => 1.0,
            Element::Ca => 2.0,
            Element::Sc => 3.0,
            Element::Ti => 4.0,
            Element::V => 5.0,
            Element::Cr => 6.0,
            Element::Mn => 7.0,
            Element::Fe => 8.0,
            Element::Co => 9.0,
            Element::Ni => 10.0,
            Element::Cu => 11.0,
            Element::Zn => 12.0,
            Element::Ga => 3.0,
            Element::Ge => 4.0,
            Element::As => -3.0,
            Element::Se => -2.0,
            Element::Br => -1.0,
            Element::Kr => 0.0,
            Element::Rb => 1.0,
            Element::Sr => 2.0,
            Element::Y => 3.0,
            Element::Zr => 4.0,
            Element::Nb => 5.0,
            Element::Mo => 6.0,
            Element::Tc => 7.0,
            Element::Ru => 8.0,
            Element::Rh => 9.0,
            Element::Pd => 10.0,
            Element::Ag => 11.0,
            Element::Cd => 12.0,
            Element::In => 3.0,
            Element::Sn => 4.0,
            Element::Sb => -3.0,
            Element::Te => -2.0,
            Element::I => -1.0,
            Element::Xe => 0.0,
            Element::Cs => 1.0,
            Element::Ba => 2.0,
            Element::La => 3.0,
            Element::Ce => 4.0,
            Element::Pr => 3.0,
            Element::Nd => 3.0,
            Element::Pm => 3.0,
            Element::Sm => 3.0,
            Element::Eu => 3.0,
            Element::Gd => 3.0,
            Element::Tb => 3.0,
            Element::Dy => 3.0,
            Element::Ho => 3.0,
            Element::Er => 3.0,
            Element::Tm => 3.0,
            Element::Yb => 3.0,
            Element::Lu => 3.0,
            Element::Hf => 4.0,
            Element::Ta => 5.0,
            Element::W => 6.0,
            Element::Re => 7.0,
            Element::Os => 8.0,
            Element::Ir => 9.0,
            Element::Pt => 10.0,
            Element::Au => 11.0,
            Element::Hg => 12.0,
            Element::Tl => 3.0,
            Element::Pb => 4.0,
            Element::Bi => 3.0,
            Element::Po => -2.0,
            Element::At => -1.0,
            Element::Rn => 0.0,
            Element::Fr => 1.0,
            Element::Ra => 2.0,
            Element::Ac => 3.0,
            Element::Th => 4.0,
            Element::Pa => 5.0,
            Element::U => 6.0,
        }
    }

    /// Returns the atomic number of the element.
    pub const fn number(&self) -> u8 {
        match self {
            Element::H => 1,
            Element::He => 2,
            Element::Li => 3,
            Element::Be => 4,
            Element::B => 5,
            Element::C => 6,
            Element::N => 7,
            Element::O => 8,
            Element::F => 9,
            Element::Ne => 10,
            Element::Na => 11,
            Element::Mg => 12,
            Element::Al => 13,
            Element::Si => 14,
            Element::P => 15,
            Element::S => 16,
            Element::Cl => 17,
            Element::Ar => 18,
            Element::K => 19,
            Element::Ca => 20,
            Element::Sc => 21,
            Element::Ti => 22,
            Element::V => 23,
            Element::Cr => 24,
            Element::Mn => 25,
            Element::Fe => 26,
            Element::Co => 27,
            Element::Ni => 28,
            Element::Cu => 29,
            Element::Zn => 30,
            Element::Ga => 31,
            Element::Ge => 32,
            Element::As => 33,
            Element::Se => 34,
            Element::Br => 35,
            Element::Kr => 36,
            Element::Rb => 37,
            Element::Sr => 38,
            Element::Y => 39,
            Element::Zr => 40,
            Element::Nb => 41,
            Element::Mo => 42,
            Element::Tc => 43,
            Element::Ru => 44,
            Element::Rh => 45,
            Element::Pd => 46,
            Element::Ag => 47,
            Element::Cd => 48,
            Element::In => 49,
            Element::Sn => 50,
            Element::Sb => 51,
            Element::Te => 52,
            Element::I => 53,
            Element::Xe => 54,
            Element::Cs => 55,
            Element::Ba => 56,
            Element::La => 57,
            Element::Ce => 58,
            Element::Pr => 59,
            Element::Nd => 60,
            Element::Pm => 61,
            Element::Sm => 62,
            Element::Eu => 63,
            Element::Gd => 64,
            Element::Tb => 65,
            Element::Dy => 66,
            Element::Ho => 67,
            Element::Er => 68,
            Element::Tm => 69,
            Element::Yb => 70,
            Element::Lu => 71,
            Element::Hf => 72,
            Element::Ta => 73,
            Element::W => 74,
            Element::Re => 75,
            Element::Os => 76,
            Element::Ir => 77,
            Element::Pt => 78,
            Element::Au => 79,
            Element::Hg => 80,
            Element::Tl => 81,
            Element::Pb => 82,
            Element::Bi => 83,
            Element::Po => 84,
            Element::At => 85,
            Element::Rn => 86,
            Element::Fr => 87,
            Element::Ra => 88,
            Element::Ac => 89,
            Element::Th => 90,
            Element::Pa => 91,
            Element::U => 92,
        }
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
