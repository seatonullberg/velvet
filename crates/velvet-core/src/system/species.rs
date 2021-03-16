use serde::{Deserialize, Serialize};

use crate::internal::Float;
use crate::system::elements::Element;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Specie {
    id: usize,
    mass: Float,
    charge: Float,
}

impl Specie {
    pub fn new(id: usize, mass: Float, charge: Float) -> Specie {
        Specie { id, mass, charge }
    }

    pub fn from_element(id: usize, element: Element) -> Specie {
        Specie {
            id,
            mass: element.mass(),
            charge: element.charge(),
        }
    }

    pub fn mass(&self) -> Float {
        self.mass
    }

    pub fn charge(&self) -> Float {
        self.charge
    }
}

impl PartialEq for Specie {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::Specie;
    use crate::system::elements::Element;

    #[test]
    fn from_element() {
        let element = Element::H;
        let specie = Specie::from_element(0, element);
        assert_eq!(specie.mass(), element.mass());
        assert_eq!(specie.charge(), element.charge());
    }

    #[test]
    fn compare_invalid() {
        let s0 = Specie::from_element(0, Element::H);
        let s1 = Specie::from_element(1, Element::H);
        assert_ne!(s0, s1);
    }

    #[test]
    fn compare_valid() {
        let s0 = Specie::from_element(0, Element::H);
        let s1 = Specie::from_element(0, Element::H);
        assert_eq!(s0, s1);
    }
}
