// TODO: break out the identical portions of `evaluate_energy` and `evaluate_force` into a separate function.

pub mod pair;

use crate::distance::distance;
use crate::energy::EnergyEvaluator;
use crate::force::ForceEvaluator;
use crate::potential::pair::PairPotential;
use crate::system::System;

use nalgebra::Vector3;

/// Any interatomic potential.
pub struct Potential<T> {
    pub cutoff: f32,
    pub symbols: Vec<String>,
    pub evaluator: T,
}

impl<T: PairPotential> EnergyEvaluator for Potential<T> {
    fn evaluate_energy(&self, system: &System, index: usize) -> f32 {
        let atom = &system.atoms[index];
        // NOTE: `self.symbols` is assumed to be sorted
        let defined_symbols: Vec<&str> = self.symbols.iter().map(AsRef::as_ref).collect();
        let mut energy = 0.0;
        // iterate over all atoms in the system
        for (i, atom_i) in system.atoms.iter().enumerate() {
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
            // calculate the distance
            let dist = distance(system, &atom.position, &atom_i.position);
            let mag = dist.norm();
            // skip atoms beyond the cutoff radius
            if mag > self.cutoff {
                continue;
            }
            // add to the total energy
            energy += self.evaluator.energy(mag);
        }
        energy
    }
}

impl<T: PairPotential> ForceEvaluator for Potential<T> {
    fn evaluate_force(&self, system: &System, index: usize) -> Vector3<f32> {
        let atom = &system.atoms[index];
        // NOTE: `self.symbols` is assumed to be sorted
        let defined_symbols: Vec<&str> = self.symbols.iter().map(AsRef::as_ref).collect();
        let mut force: Vector3<f32> = Vector3::zeros();
        // iterate over all atoms in the system
        for (i, atom_i) in system.atoms.iter().enumerate() {
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
            // calculate the distance
            let dist = distance(system, &atom.position, &atom_i.position);
            let mag = dist.norm();
            // skip atoms beyond the cutoff radius
            if mag > self.cutoff {
                continue;
            }
            // add to the total force
            force += (dist / mag) * self.evaluator.force(mag);
        }
        force
    }
}
