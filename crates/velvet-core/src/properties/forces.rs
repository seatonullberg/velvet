use crate::potentials::pair::PairPotential;
use crate::potentials::Potentials;
use crate::properties::Property;
use nalgebra::Vector3;
use velvet_internals::float::Float;
use velvet_system::System;

/// Force acting on each atom due to nonbonded pairwise interactions.
#[derive(Clone, Copy, Debug)]
pub struct PairForces;

impl Property for PairForces {
    type Res = Vec<Vector3<Float>>;

    fn name(&self) -> String {
        "PairForces".to_string()
    }

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        // initialize loop variables
        let mut pos_i: Vector3<Float> = Vector3::zeros();
        let mut pos_j: Vector3<Float> = Vector3::zeros();
        let mut dir: Vector3<Float> = Vector3::zeros();
        let mut force: Vector3<Float> = Vector3::zeros();
        let mut r: Float = 0 as Float;

        // very slow to accumulate forces this way
        let mut forces = vec![Vector3::zeros(); system.n_atoms];

        // evaluate all pair interactions
        let pair_metas = &potentials.pair_metas;
        match pair_metas {
            Some(pair_metas) => pair_metas.iter().for_each(|meta| {
                meta.iter().for_each(|(i, j)| {
                    pos_i = system.positions[*i];
                    pos_j = system.positions[*j];
                    r = system.cell.distance(&pos_i, &pos_j);
                    dir = system.cell.direction(&pos_i, &pos_j);
                    force = meta.force(r) * dir;
                    forces[*i] += force;
                    forces[*j] -= force;
                })
            }),
            None => {}
        }
        forces
    }
}

/// Total force acting on each atom
#[derive(Clone, Copy, Debug)]
pub struct Forces;

impl Property for Forces {
    type Res = Vec<Vector3<Float>>;

    fn name(&self) -> String {
        "Forces".to_string()
    }

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let pair_forces = PairForces.calculate(system, potentials);
        // TODO: include other force contributions
        pair_forces
    }
}
