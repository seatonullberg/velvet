//! Types of forces that can be evaluated.

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use nalgebra::Vector3;

use crate::internal::Float;
use crate::potentials::Potentials;
use crate::properties::Property;
use crate::system::System;

/// Force acting on each atom in the system due to pairwise potentials.
#[derive(Clone, Copy, Debug)]
pub struct PairForces;

impl Property for PairForces {
    type Res = Vec<Vector3<Float>>;

    // TODO: implement rayon version
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let pair_potentials = &potentials.pair_potentials.potentials;
        let neighbor_lists = &potentials.pair_potentials.neighbor_lists;
        let cutoffs = &potentials.pair_potentials.cutoffs;

        pair_potentials
            .iter()
            .zip(neighbor_lists.iter())
            .zip(cutoffs.iter())
            .map(|((pot, nl), &cut)| -> Vec<(Vector3<Float>, usize, usize)> {
                nl.indices()
                    .iter()
                    .map(move |(i, j)| {
                        let pos_i = &system.positions[*i];
                        let pos_j = &system.positions[*j];
                        let r = system.cell.distance(&pos_i, &pos_j);
                        if r < cut {
                            let dir = system.cell.direction(&pos_i, &pos_j);
                            (pot.force(r) * dir, *i, *j)
                        } else {
                            (Vector3::zeros(), *i, *j)
                        }
                    })
                    .collect()
            })
            .fold(
                vec![Vector3::zeros(); system.size],
                |mut accumulator, forces_metadata| {
                    forces_metadata.iter().for_each(|(force, i, j)| {
                        accumulator[*i] += force;
                        accumulator[*j] -= force;
                    });
                    accumulator
                },
            )
    }

    fn name(&self) -> String {
        "pair_forces".to_string()
    }
}

/// Force acting on each atom in the system.
#[derive(Clone, Copy, Debug)]
pub struct Forces;

impl Property for Forces {
    type Res = Vec<Vector3<Float>>;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let pair_forces = PairForces.calculate(system, potentials);
        pair_forces
    }

    fn name(&self) -> String {
        "forces".to_string()
    }
}
