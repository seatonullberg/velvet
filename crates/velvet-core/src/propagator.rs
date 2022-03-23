//! Algorithms to control the progress of a simulation.

use crate::potentials::Potentials;
use velvet_system::System;

/// Shared behavior for algorithms which control the progress of a simulation.
pub trait Propagator {
    fn setup(&mut self, system: &mut System, potentials: &Potentials);
    fn propagate(&mut self, system: &mut System, potentials: &Potentials);
}
