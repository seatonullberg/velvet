use crate::outputs::Output;
use crate::potentials::Potentials;
use crate::properties::Property;
use crate::properties::forces::{Forces, PairForces};
use velvet_system::System;

// TODO: Find a way to do blanket implementations.

impl Output for Forces {
    fn output(&self, system: &System, potentials: &Potentials, timestep: usize) -> String {
        let result = self.calculate(system, potentials);
        format!("{} {} {} {}\n", timestep, result[0], result[1], result[2])
    }
}

impl Output for PairForces {
    fn output(&self, system: &System, potentials: &Potentials, timestep: usize) -> String {
        let result = self.calculate(system, potentials);
        format!("{} {} {} {}\n", timestep, result[0], result[1], result[2])
    }
}
