use crate::potentials::pair::PairPotential;
use crate::potentials::Potentials;
use crate::properties::Property;
use nalgebra::Vector3;
use velvet_internals::float::Float;
use velvet_system::System;

/// Potential energy due to nonbonded pairwise interactions.
#[derive(Clone, Copy, Debug)]
pub struct PairEnergy;

impl Property for PairEnergy {
    type Res = Float;

    fn name(&self) -> String {
        "PairEnergy".to_string()
    }

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        // initialize loop variables
        let mut pos_i: Vector3<Float> = Vector3::zeros();
        let mut pos_j: Vector3<Float> = Vector3::zeros();
        let mut r: Float = 0 as Float;

        // evaluate all pair interactions
        let pair_metas = &potentials.pair_metas;
        match pair_metas {
            Some(pair_metas) => pair_metas
                .iter()
                .map(|meta| -> Float {
                    meta.iter()
                        .map(|(i, j)| -> Float {
                            pos_i = system.positions[*i];
                            pos_j = system.positions[*j];
                            r = system.cell.distance(&pos_i, &pos_j);
                            meta.energy(r)
                        })
                        .sum()
                })
                .sum(),
            None => 0 as Float,
        }
    }
}

/// Total potential energy.
#[derive(Clone, Copy, Debug)]
pub struct PotentialEnergy;

impl Property for PotentialEnergy {
    type Res = Float;

    fn name(&self) -> String {
        "PotentialEnergy".to_string()
    }

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let pair_energy = PairEnergy.calculate(system, potentials);
        // TODO: include other potential energy contributions
        pair_energy
    }
}

/// Total kinetic energy.
#[derive(Clone, Copy, Debug)]
pub struct KineticEnergy;

impl Property for KineticEnergy {
    type Res = Float;

    fn name(&self) -> String {
        "KineticEnergy".to_string()
    }

    fn calculate(&self, system: &System, _: &Potentials) -> Self::Res {
        system
            .species
            .iter()
            .zip(system.velocities.iter())
            .map(|(species, velocity)| 0.5 * species.mass() * velocity.norm_squared())
            .sum()
    }
}

/// Sum of potential and kinetic energy
#[derive(Clone, Copy, Debug)]
pub struct TotalEnergy;

impl Property for TotalEnergy {
    type Res = Float;

    fn name(&self) -> String {
        "TotalEnergy".to_string()
    }

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let kinetic = KineticEnergy.calculate(system, potentials);
        let potential = PotentialEnergy.calculate(system, potentials);
        kinetic + potential
    }
}
