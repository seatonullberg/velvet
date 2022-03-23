use crate::outputs::Output;
use crate::potentials::Potentials;
use crate::properties::Property;
use crate::properties::temperature::Temperature;
use velvet_system::System;

impl Output for Temperature {
    fn output(&self, system: &System, potentials: &Potentials, timestep: usize) -> String {
        let result = self.calculate(system, potentials);
        format!("{} {}", timestep, result)
    }
}
