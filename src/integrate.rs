use crate::force::ForceEvaluator;
use crate::system::{Atom, System};

use nalgebra::Vector3;

pub trait Integrator {
    fn integrate(&self, system: &System) -> Vec<Atom>;
}

// pub struct Beeman;

// impl Integrator for Beeman {
//     fn integrate(&self, system: &mut System) {}
// }

pub struct VelocityVerlet {
    forces: Vec<Box<dyn ForceEvaluator>>,
}

impl Integrator for VelocityVerlet {
    fn integrate(&self, system: &System) -> Vec<Atom> {
        // use existing atoms as the starting point
        let mut new_atoms = system.atoms.clone();
        // iterate over all atoms
        for (i, atom_i) in system.atoms.iter().enumerate() {
            // calculate acceleration from force and mass
            let acceleration = atom_i.force / atom_i.mass;
            // calculate position at next time step
            let position = atom_i.position
                + (atom_i.velocity * system.timestep)
                + (acceleration * (system.timestep.powi(2) / 2.0));
            // evaluate all forces
            let force = self
                .forces
                .iter()
                .fold(Vector3::zeros(), |acc, x| acc + x.evaluate_force(system, i));
            // calculate velocity at next timestep
            let velocity =
                atom_i.velocity + (acceleration + (force / atom_i.mass)) * (system.timestep / 2.0);
            // assign new properties
            new_atoms[i].position = position;
            new_atoms[i].velocity = velocity;
            new_atoms[i].force = force;
        }
        // return the updated atoms
        new_atoms
    }
}
