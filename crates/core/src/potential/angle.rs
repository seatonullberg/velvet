use nalgebra::Vector3;

use crate::potential::ForceEvaluator;
use crate::system::System;

/// Required behaviors for an angle style interatomic potential.
pub trait AnglePotential {
    /// Returns the potential energy of an atom in a triplet which forms the angle `theta`.
    fn energy(&self, theta: f32) -> f32;
    /// Returns the magnitude of the force acting on an atom due to the angle formed between itself an two neighbors.
    fn force(&self, theta: f32) -> f32;
}

/// Bond potential with required metadata.
pub struct AnglePotentialData {
    /// Heap allocated interatomic bond potential.
    potential: Box<dyn AnglePotential>,
    /// Angle type this potential applies to.
    angle: usize,
}

impl ForceEvaluator for AnglePotentialData {
    fn evaluate_forces(&self, system: &System) -> Vec<Vector3<f32>> {
        unimplemented!()
    }
}
