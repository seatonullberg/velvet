use nalgebra::Vector3;

use crate::potential::{ForceEvaluator, Scope};
use crate::system::System;

/// Required behaviors for a pairwise interatomic potential.
pub trait PairPotential {
    /// Returns the potential energy of an atom separated from another by a distance `r`.
    fn energy(&self, r: f32) -> f32;
    /// Returns the magnitude of the force acting on an atom by another separated by a distance `r`.
    fn force(&self, r: f32) -> f32;
}

/// Pair potential with required metadata.
pub struct PairPotentialData {
    /// Heap allocated interatomic pair potential.
    potential: Box<dyn PairPotential>,
    /// Pair of species this potential applies to.
    species: (usize, usize),
    /// Applicability of the potential.
    scope: Scope,
    /// Cutoff radius within which the potential is applied.
    cutoff: f32,
}

impl ForceEvaluator for PairPotentialData {
    fn evaluate_forces(&self, system: &System) -> Vec<Vector3<f32>> {
        // start with all pairs that are in scope
        let mut indices = system.pairs.get(&self.species).unwrap().clone();
        indices.retain(|(i, j)| match self.scope {
            Scope::Global => true,
            Scope::Intermolecular => system.molecule_ids[*i] != system.molecule_ids[*j],
            Scope::Intramolecular => system.molecule_ids[*i] == system.molecule_ids[*j],
        });

        // calculate distances between each pair
        let distances: Vec<f32> = indices
            .iter()
            .map(|(i, j)| {
                system
                    .cell
                    .distance(&system.positions[*i], &system.positions[*j])
            })
            .collect();

        // calculate force magnitudes for each pair
        let force_mags: Vec<f32> = distances
            .iter()
            .map(|r| {
                if *r > self.cutoff {
                    0 as f32
                } else {
                    self.potential.force(*r)
                }
            })
            .collect();

        // calculate unit vector directions between each pair
        let directions: Vec<Vector3<f32>> = indices
            .iter()
            .map(|(i, j)| {
                system
                    .cell
                    .direction(&system.positions[*i], &system.positions[*j])
            })
            .collect();

        // calculate the force vector of each atom in the system
        let mut forces: Vec<Vector3<f32>> = vec![Vector3::default(); system.size()];
        indices
            .iter()
            .zip(force_mags.iter())
            .zip(directions.iter())
            .for_each(|(((i, j), &f), &dir)| {
                forces[*i] += f * dir;
                forces[*j] -= f * dir;
            });

        forces
    }
}
