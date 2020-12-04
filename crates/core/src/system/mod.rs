pub mod cell;
pub mod element;

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::system::cell::Cell;
use crate::system::element::Element;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct System {
    /// Number of atoms in the system.
    size: usize,

    /// Simulation cell with periodic boundary conditions.
    pub cell: Cell,

    /// Element type for each atom in the system.
    pub elements: Vec<Element>,
    /// Molecule type for each atom in the system.
    pub molecules: Vec<usize>,
    /// Position of each atom in the system.
    pub positions: Vec<Vector3<f32>>,
    /// Velocity of each atom in the system.
    pub velocities: Vec<Vector3<f32>>,
    /// Mass of each atom in the system.
    pub masses: Vec<f32>,
    /// Electronic charge of each atom in the system.
    pub charges: Vec<f32>,

    /// Collection of bond indices grouped by bond type.
    pub bonds: Vec<Vec<(usize, usize)>>,
    /// Collection of angle triplet indices grouped by angle type.
    pub angles: Vec<Vec<(usize, usize, usize)>>,
    /// Collection of dihedral quadruplet indices grouped by dihedral type.
    pub dihedrals: Vec<Vec<(usize, usize, usize, usize)>>,
}

impl System {
    /// Returns the number of atoms in the system.
    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use crate::system::cell::Cell;
    use crate::system::element::Element;
    use crate::system::System;
    use nalgebra::Vector3;
    use ron::de::from_str;
    use ron::ser::{to_string_pretty, PrettyConfig};

    #[test]
    fn serde() {
        let sys = System {
            size: 2,
            cell: Cell::new(5.0, 5.0, 5.0, 90.0, 90.0, 90.0),
            elements: vec![Element::H, Element::H],
            molecules: vec![1, 1],
            positions: vec![Vector3::default(), Vector3::new(1.0, 1.0, 1.0)],
            velocities: vec![Vector3::default(), Vector3::default()],
            masses: vec![1.01, 1.01],
            charges: vec![0.0, 0.0],
            bonds: vec![vec![(0, 1)]],
            angles: Vec::new(),
            dihedrals: Vec::new(),
        };
        let pretty = PrettyConfig::new().with_depth_limit(2);
        let s = to_string_pretty(&sys, pretty).unwrap();
        let _: System = match from_str(&s) {
            Ok(x) => x,
            Err(e) => {
                println!("failed to load system from string: {}", e);
                std::process::exit(0);
            }
        };
    }
}
