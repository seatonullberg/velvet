use nalgebra::Vector3;

use crate::potential::{pair::PairPotential, Potentials, Restriction};
use crate::system::System;

/// Calculates a system-wide property.
pub trait Property {
    fn calculate(&self, system: &System, potentials: &Potentials) -> Output;
}

pub enum PropertyEnum {
    Forces(Forces),
    External(Box<dyn Property>),
}

impl Property for PropertyEnum {
    fn calculate(&self, system: &System, potentials: &Potentials) -> Output {
        match self {
            PropertyEnum::Forces(f) => f.calculate(system, potentials),
            PropertyEnum::External(e) => e.calculate(system, potentials),
        }
    }
}

/// Valid output types for system properties.
#[derive(Clone, Debug)]
pub enum Output {
    Scalar(f32),
    Vector(Vec<f32>),
    Matrix(Vec<Vector3<f32>>),
}

#[derive(Clone, Debug)]
pub struct Forces;

impl Property for Forces {
    fn calculate(&self, system: &System, potentials: &Potentials) -> Output {
        let sys_size = system.size();
        let mut forces: Vec<Vector3<f32>> = vec![Vector3::new(0.0, 0.0, 0.0); sys_size];
        // iterate over all pairs of atoms
        for i in 0..sys_size {
            for j in 0..sys_size {
                // skip duplicate or identical pairs
                if j <= i {
                    continue;
                }

                // calculate distance between the pair
                let pos1 = &system.positions[i];
                let pos2 = &system.positions[j];
                let r: f32 = system.cell.distance(pos1, pos2);

                // iterate over the pair potentials
                for (potential, meta) in potentials.pairs() {
                    // check cutoff radius
                    if meta.cutoff < r {
                        continue;
                    }

                    // check element pair
                    let elem1 = &system.elements[i];
                    let elem2 = &system.elements[j];
                    if (*elem1, *elem2) != meta.elements {
                        continue;
                    }

                    // check restricton
                    let ok = match meta.restriction {
                        Restriction::None => true,
                        Restriction::Intermolecular => &system.molecules[i] != &system.molecules[j],
                        Restriction::Intramolecular => &system.molecules[i] == &system.molecules[j],
                    };
                    if ok {
                        let dir = &system.cell.direction(pos1, pos2);
                        let force = dir * potential.force(r);
                        forces[i] += force;
                        forces[j] -= force;
                    }
                }
            }
        }
        Output::Matrix(forces)
    }
}

#[cfg(test)]
mod tests {
    use crate::potential::pair::{LennardJones, PairPotentialEnum, PairPotentialMeta};
    use crate::potential::{Potentials, Restriction};
    use crate::property::{Forces, Output, Property};
    use crate::system::{cell::Cell, element::Element, System};
    use nalgebra::Vector3;

    #[test]
    fn forces() {
        // define the system
        let size = 2 as usize;
        let mut sys = System::new(size);
        sys.cell = Cell::new(5.0, 5.0, 5.0, 90.0, 90.0, 90.0);
        sys.elements = vec![Element::H, Element::H];
        sys.molecules = vec![0 as usize, 0 as usize];
        sys.positions = vec![Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0)];
        sys.velocities = vec![Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)];
        sys.masses = vec![1.01, 1.01];
        sys.charges = vec![0.0, 0.0];

        // define the potentials
        let lj = LennardJones::new(3.0, 1.5);
        let lj = PairPotentialEnum::LennardJones(lj);
        let meta = PairPotentialMeta::new((Element::H, Element::H), 10.0, Restriction::None);
        let mut pots = Potentials::new();
        pots.add_pair(lj, meta);

        // calculate the forces
        let forces = Forces;
        let forces = forces.calculate(&sys, &pots);
        let res = match forces {
            Output::Scalar(_) => panic!("wrong type"),
            Output::Vector(_) => panic!("wrong type"),
            Output::Matrix(m) => m,
        };
        assert_eq!(res[0].norm(), 2.740163);
    }
}
