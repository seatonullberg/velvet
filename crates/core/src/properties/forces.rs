use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::potentials::Potentials;
use crate::properties::Property;
use crate::system::System;

/// Force acting on each atom in the system due to coulombic potentials.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CoulombForces;

impl Property for CoulombForces {
    type Res = Vec<Vector3<f32>>;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let mut forces: Vec<Vector3<f32>> = vec![Vector3::zeros(); system.size()];
        for (meta, potential) in &potentials.coulombs {
            for (i, j) in &meta.indices {
                let elem_i = system.elements[*i];
                let elem_j = system.elements[*j];
                let pos_i = system.positions[*i];
                let pos_j = system.positions[*j];
                let r = system.cell().distance(&pos_i, &pos_j);
                let force = if meta.cutoff > r {
                    let dir = system.cell().direction(&pos_i, &pos_j);
                    potential.force(elem_i.charge(), elem_j.charge(), r) * dir
                } else {
                    Vector3::zeros()
                };
                forces[*i] += force;
                forces[*j] -= force;
            }
        }
        forces
    }

    // fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
    //     let pairwise_forces: Vec<Vec<((usize, usize), Vector3<f32>)>> = potentials
    //         .coulombs
    //         .iter()
    //         .map(|(meta, potential)| {
    //             meta.indices
    //                 .iter()
    //                 .map(|(i, j)| {
    //                     // TODO: filter out 0 charge elements
    //                     // maybe do it in indices selection
    //                     let pos_i = system.positions[*i];
    //                     let pos_j = system.positions[*j];
    //                     let r = system.cell().distance(&pos_i, &pos_j);
    //                     let indices = (*i, *j);
    //                     let mut force = Vector3::zeros();
    //                     if meta.cutoff > r {
    //                         let dir = system.cell().direction(&pos_i, &pos_j);
    //                         let elem_i = system.elements[*i];
    //                         let elem_j = system.elements[*j];
    //                         force = potential.force(elem_i.charge(), elem_j.charge(), r) * dir;
    //                     }
    //                     (indices, force)
    //                 })
    //                 .collect()
    //         })
    //         .collect();
    //     let mut forces: Vec<Vector3<f32>> = vec![Vector3::zeros(); system.size()];
    //     pairwise_forces.iter().for_each(|pair_group| {
    //         pair_group.iter().for_each(|((i, j), force)| {
    //             forces[*i] += force;
    //             forces[*j] -= force;
    //         })
    //     });
    //     forces
    // }
}

/// Force acting on each atom in the system due to pairwise potentials.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PairForces;

impl Property for PairForces {
    type Res = Vec<Vector3<f32>>;

    // fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
    //     let pairwise_forces: Vec<Vec<((usize, usize), Vector3<f32>)>> = potentials
    //         .pairs
    //         .iter()
    //         .map(|(meta, potential)| {
    //             meta.indices
    //                 .iter()
    //                 .map(|(i, j)| {
    //                     let pos_i = system.positions[*i];
    //                     let pos_j = system.positions[*j];
    //                     let r = system.cell().distance(&pos_i, &pos_j);
    //                     let indices = (*i, *j);
    //                     let mut force = Vector3::zeros();
    //                     if meta.cutoff > r {
    //                         let dir = system.cell().direction(&pos_i, &pos_j);
    //                         force = potential.force(r) * dir;
    //                     }
    //                     (indices, force)
    //                 })
    //                 .collect()
    //         })
    //         .collect();
    //     let mut forces: Vec<Vector3<f32>> = vec![Vector3::zeros(); system.size()];
    //     pairwise_forces.iter().for_each(|pair_group| {
    //         pair_group.iter().for_each(|((i, j), force)| {
    //             forces[*i] += force;
    //             forces[*j] -= force;
    //         })
    //     });
    //     forces
    // }

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let mut forces: Vec<Vector3<f32>> = vec![Vector3::zeros(); system.size()];
        for (meta, potential) in &potentials.pairs {
            for (i, j) in &meta.indices {
                let pos_i = system.positions[*i];
                let pos_j = system.positions[*j];
                let r = system.cell().distance(&pos_i, &pos_j);
                let force = if meta.cutoff > r {
                    let dir = system.cell().direction(&pos_i, &pos_j);
                    potential.force(r) * dir
                } else {
                    Vector3::zeros()
                };
                forces[*i] += force;
                forces[*j] -= force;
            }
        }
        forces
    }
}

/// Force acting on each atom in the system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Forces;

impl Property for Forces {
    type Res = Vec<Vector3<f32>>;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let pair_forces = PairForces.calculate(system, potentials);
        let coulomb_forces = CoulombForces.calculate(system, potentials);
        pair_forces
            .iter()
            .zip(coulomb_forces.iter())
            .map(|(p, c)| p + c)
            .collect()
    }
}
