use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::internal::Float;
use crate::potentials::Potentials;
use crate::properties::Property;
use crate::system::System;

// /// Force acting on each atom in the system due to coulombic potentials.
// #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
// pub struct CoulombForces;

// impl Property for CoulombForces {
//     type Res = Vec<Vector3<Float>>;

//     fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
//         let mut forces: Vec<Vector3<Float>> = vec![Vector3::zeros(); system.size()];
//         for (meta, potential) in &potentials.coulombs {
//             for (i, j) in &meta.indices {
//                 let elem_i = system.elements[*i];
//                 let elem_j = system.elements[*j];
//                 let pos_i = system.positions[*i];
//                 let pos_j = system.positions[*j];
//                 let r = system.cell().distance(&pos_i, &pos_j);
//                 let force = if r < meta.cutoff {
//                     let dir = system.cell().direction(&pos_i, &pos_j);
//                     potential.force(elem_i.charge(), elem_j.charge(), r) * dir
//                 } else {
//                     Vector3::zeros()
//                 };
//                 forces[*i] += force;
//                 forces[*j] -= force;
//             }
//         }
//         forces
//     }
// }

/// Force acting on each atom in the system due to pairwise potentials.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PairForces;

impl Property for PairForces {
    type Res = Vec<Vector3<Float>>;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        potentials
            .pair_interactions()
            .iter()
            .map(|(potential, i, j)| {
                let pos_i = system.positions[*i];
                let pos_j = system.positions[*j];
                let r = system.cell.distance(&pos_i, &pos_j);
                let dir = system.cell.direction(&pos_i, &pos_j);
                (potential.force(r) * dir, i, j)
            })
            .fold(
                vec![Vector3::zeros(); system.size],
                |mut accumulator, (force, i, j)| {
                    accumulator[*i] += force;
                    accumulator[*j] -= force;
                    accumulator
                },
            )
    }
}

/// Force acting on each atom in the system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Forces;

impl Property for Forces {
    type Res = Vec<Vector3<Float>>;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let pair_forces = PairForces.calculate(system, potentials);
        pair_forces
    }
}
