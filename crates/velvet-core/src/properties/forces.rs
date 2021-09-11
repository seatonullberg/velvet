//! Types of forces that can be evaluated.

use nalgebra::Vector3;
use rayon::prelude::*;

use crate::internal::Float;
use crate::potentials::pair::PairPotentialMeta;
use crate::potentials::Potentials;
use crate::properties::Property;
use crate::system::System;

/// Force acting on each atom in the system due to pairwise potentials.
#[derive(Clone, Copy, Debug)]
pub struct PairForces;

impl PairForces {
    fn calculate_inner(
        &self,
        meta: &PairPotentialMeta,
        system: &System,
        indices: &[[usize; 2]],
    ) -> Vec<Vector3<Float>> {
        let mut total: Vec<Vector3<Float>> = vec![Vector3::zeros(); system.size];
        let mut pos_i: Vector3<Float> = Vector3::zeros();
        let mut pos_j: Vector3<Float> = Vector3::zeros();
        let mut dir: Vector3<Float> = Vector3::zeros();
        let mut force: Vector3<Float> = Vector3::zeros();
        let mut r: Float = 0 as Float;
        indices.iter().for_each(|&[i, j]| {
            pos_i = system.positions[i];
            pos_j = system.positions[j];
            r = system.cell.distance(&pos_i, &pos_j);
            if r < meta.cutoff {
                dir = system.cell.direction(&pos_i, &pos_j);
                force = meta.potential.force(r) * dir;
                total[i] += force;
                total[j] -= force;
            }
        });
        total
    }
}

impl Property for PairForces {
    type Res = Vec<Vector3<Float>>;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        potentials.pair_metas.iter().fold(
            vec![Vector3::zeros(); system.size],
            |accumulator, meta| {
                meta.selection
                    .par_iter_chunks()
                    .fold(
                        || vec![Vector3::zeros(); system.size],
                        |inner_accumulator, chunk| {
                            self.calculate_inner(meta, system, chunk)
                                .iter()
                                .zip(inner_accumulator.iter())
                                .map(|(a, b)| a + b)
                                .collect()
                        },
                    )
                    .reduce(
                        || vec![Vector3::zeros(); system.size],
                        |left, right| left.iter().zip(right.iter()).map(|(a, b)| a + b).collect(),
                    )
                    .iter()
                    .zip(accumulator.iter())
                    .map(|(a, b)| a + b)
                    .collect()
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
