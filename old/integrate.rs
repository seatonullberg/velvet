use crate::force::ForceEvaluator;
use crate::simcell::{Atom, SimulationCell};

use nalgebra::Vector3;

pub trait Integrator {
    fn integrate(&self, cell: &SimulationCell, forces: Vec<Box<dyn ForceEvaluator>>, timestep: f32) -> Vec<Atom>;
}

pub struct VelocityVerlet;

impl Integrator for VelocityVerlet {
    fn integrate(&self, cell: &SimulationCell, forces: Vec<Box<dyn ForceEvaluator>>, timestep: f32) -> Vec<Atom> {
        // use existing atoms as the starting point
        let mut new_atoms = cell.atoms.clone();
        // iterate over all atoms
        for (i, atom_i) in cell.atoms.iter().enumerate() {
            // calculate acceleration from force and mass
            let acceleration = atom_i.force / atom_i.mass;
            // calculate position at next time step
            let position = atom_i.position
                + (atom_i.velocity * timestep)
                + (acceleration * (timestep.powi(2) / 2.0));
            // evaluate all forces
            let force = forces
                .iter()
                .fold(Vector3::zeros(), |acc, x| acc + x.evaluate_force(cell, i));
            // calculate velocity at next timestep
            let velocity =
                atom_i.velocity + (acceleration + (force / atom_i.mass)) * (timestep / 2.0);
            // assign new properties
            new_atoms[i].position = position;
            new_atoms[i].velocity = velocity;
            new_atoms[i].force = force;
        }
        // return the updated atoms
        new_atoms
    }
}
