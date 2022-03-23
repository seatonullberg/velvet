use crate::outputs::Output;
use crate::potentials::Potentials;
use crate::properties::Property;
use crate::properties::energy::{PairEnergy, PotentialEnergy, KineticEnergy, TotalEnergy};
use velvet_system::System;

// TODO: Find a way to do blanket implementations.

impl Output for PairEnergy {
    fn output(&self, system: &System, potentials: &Potentials, timestep: usize) -> String {
        let result = self.calculate(system, potentials);
        format!("{} {}\n", timestep, result)
    }
}

impl Output for PotentialEnergy {
    fn output(&self, system: &System, potentials: &Potentials, timestep: usize) -> String {
        let result = self.calculate(system, potentials);
        format!("{} {}\n", timestep, result)
    }
}

impl Output for KineticEnergy {
    fn output(&self, system: &System, potentials: &Potentials, timestep: usize) -> String {
        let result = self.calculate(system, potentials);
        format!("{} {}\n", timestep, result)
    }
}

impl Output for TotalEnergy {
    fn output(&self, system: &System, potentials: &Potentials, timestep: usize) -> String {
        let result = self.calculate(system, potentials);
        format!("{} {}\n", timestep, result)
    }
}
