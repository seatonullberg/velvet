//! Types of forces that can be evaluated.

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use nalgebra::Vector3;

use crate::internal::Float;
use crate::potentials::Potentials;
use crate::properties::Property;
use crate::system::System;

/// Force acting on each atom in the system due to Coulombic potentials.
#[derive(Clone, Copy, Debug)]
pub struct CoulombicForces;

impl Property for CoulombicForces {
    type Res = Vec<Vector3<Float>>;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        match &potentials.coulomb_meta {
            None => vec![Vector3::zeros(); system.size],
            Some(meta) => meta.selection.indices().fold(
                vec![Vector3::zeros(); system.size],
                |mut accumulator: Vec<Vector3<Float>>, &[i, j]| {
                    let pos_i = system.positions[i];
                    let qi = system.particle_types[system.particle_type_map[i]].charge();
                    let pos_j = system.positions[j];
                    let qj = system.particle_types[system.particle_type_map[j]].charge();
                    let r = system.cell.distance(&pos_i, &pos_j);
                    if r < meta.cutoff {
                        let dir = system.cell.direction(&pos_i, &pos_j);
                        let force = meta.potential.force(qi, qj, r) * dir;
                        accumulator[i] += force;
                        accumulator[j] -= force;
                    }
                    accumulator
                },
            ),
        }
    }

    fn name(&self) -> String {
        "coulombic_forces".to_string()
    }
}

/// Force acting on each atom in the system due to pairwise potentials.
#[derive(Clone, Copy, Debug)]
pub struct PairForces;

impl Property for PairForces {
    type Res = Vec<Vector3<Float>>;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        potentials.pair_metas.iter().fold(
            vec![Vector3::zeros(); system.size],
            |mut accumulator, meta| {
                meta.selection.indices().for_each(|&[i, j]| {
                    let pos_i = system.positions[i];
                    let pos_j = system.positions[j];
                    let r = system.cell.distance(&pos_i, &pos_j);
                    if r < meta.cutoff {
                        let dir = system.cell.direction(&pos_i, &pos_j);
                        let force = meta.potential.force(r) * dir;
                        accumulator[i] += force;
                        accumulator[j] -= force;
                    }
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
        let coulomb_forces = CoulombicForces.calculate(system, potentials);
        let pair_forces = PairForces.calculate(system, potentials);
        coulomb_forces
            .iter()
            .zip(pair_forces.iter())
            .map(|(coul, pair)| coul + pair)
            .collect()
    }

    fn name(&self) -> String {
        "forces".to_string()
    }
}
