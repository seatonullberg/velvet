// TODO: break out the identical portions of `evaluate_energy` and `evaluate_force` into a separate function.

pub mod pair;

use crate::energy::EnergyEvaluator;
use crate::force::ForceEvaluator;
use crate::potential::pair::PairPotential;
use crate::simcell::SimulationCell;

use nalgebra::Vector3;

/// Any interatomic potential.
pub struct Potential<T> {
    pub cutoff: f32,
    pub symbols: Vec<String>,
    pub evaluator: T,
}

impl<T: PairPotential> EnergyEvaluator for Potential<T> {
    fn evaluate_energy(&self, cell: &SimulationCell, index: usize) -> f32 {
        let atom = &cell.atoms[index];
        // NOTE: `self.symbols` is assumed to be sorted
        let defined_symbols: Vec<&str> = self.symbols.iter().map(AsRef::as_ref).collect();
        let mut energy = 0.0;
        // iterate over all atoms in the cell
        for (i, atom_i) in cell.atoms.iter().enumerate() {
            // skip the atom of interest
            if i == index {
                continue;
            }
            // skip undefined symbol pairs
            let mut current_symbols = vec![&atom.symbol, &atom_i.symbol];
            current_symbols.sort();
            if current_symbols != defined_symbols {
                continue;
            }
            // calculate the distance between atoms
            let dist = cell.distance_between(i, index);
            // skip atoms beyond the cutoff radius
            if dist > self.cutoff {
                continue;
            }
            // add to the total energy
            energy += self.evaluator.energy(dist);
        }
        energy
    }
}

impl<T: PairPotential> ForceEvaluator for Potential<T> {
    fn evaluate_force(&self, cell: &SimulationCell, index: usize) -> Vector3<f32> {
        let atom = &cell.atoms[index];
        // NOTE: `self.symbols` is assumed to be sorted
        let defined_symbols: Vec<&str> = self.symbols.iter().map(AsRef::as_ref).collect();
        let mut force: Vector3<f32> = Vector3::zeros();
        // iterate over all atoms in the system
        for (i, atom_i) in cell.atoms.iter().enumerate() {
            // skip the atom of interest
            if i == index {
                continue;
            }
            // skip undefined symbol pairs
            let mut current_symbols = vec![&atom.symbol, &atom_i.symbol];
            current_symbols.sort();
            if current_symbols != defined_symbols {
                continue;
            }
            // calculate the distance vector between atoms
            let direction = cell.direction_between(i, index);
            let distance = cell.distance_between(i, index);
            // skip atoms beyond the cutoff radius
            if distance > self.cutoff {
                continue;
            }
            // add to the total force
            force += direction * self.evaluator.force(distance);
        }
        force
    }
}

#[cfg(test)]
mod tests {
    use crate::energy::EnergyEvaluator;
    use crate::potential::pair::LennardJones;
    use crate::potential::Potential;
    use crate::simcell::{Atom, Bounds, SimulationCell};

    use nalgebra::{Matrix3, Vector3};

    #[test]
    fn pair_potential_evaluate_energy() {
        let atoms = vec![
            Atom {
                symbol: String::from("Ar"),
                charge: 0.0,
                mass: 39.948,
                force: Vector3::zeros(),
                position: Vector3::zeros(),
                velocity: Vector3::zeros(),
            },
            Atom {
                symbol: String::from("Ar"),
                charge: 0.0,
                mass: 39.948,
                force: Vector3::zeros(),
                position: Vector3::new(2.5, 0.0, 0.0),
                velocity: Vector3::zeros(),
            },
        ];
        let bounds = Bounds {
            matrix: Matrix3::identity(),
            periodicity: Vector3::new(false, false, false),
        };
        let mut cell = SimulationCell {atoms, bounds};
        cell.bounds.matrix *= 5.0;        
        let lj = LennardJones::new(0.8, 2.0);
        let potential = Potential {
            cutoff: 5.0,
            symbols: vec![String::from("Ar"), String::from("Ar")],
            evaluator: lj,
        };
        let energy = potential.evaluate_energy(&cell, 0);
        assert_eq!(energy, -0.6189586)
    }
}
