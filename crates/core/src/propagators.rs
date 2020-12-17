use serde::{Serialize, Deserialize};

use crate::integrators::Integrator;
use crate::potentials::Potentials;
use crate::system::System;
use crate::thermostats::Thermostat;

#[typetag::serde(tag = "type")]
pub trait Propagator: Send + Sync {
    fn setup(&mut self, _: &mut System, _: &Potentials) {}
    fn propagate(&mut self, _: &mut System, _: &Potentials) {}
}

#[derive(Serialize, Deserialize)]
pub struct MolecularDynamics {
    integrator: Box<dyn Integrator>,
    thermostat: Box<dyn Thermostat>,
}

impl MolecularDynamics {
    pub fn new(
        integrator: Box<dyn Integrator>,
        thermostat: Box<dyn Thermostat>,
    ) -> MolecularDynamics {
        MolecularDynamics {
            integrator,
            thermostat,
        }
    }
}

#[typetag::serde]
impl Propagator for MolecularDynamics {
    fn setup(&mut self, system: &mut System, potentials: &Potentials) {
        self.integrator.setup(system, potentials);
        self.thermostat.setup(system);
    }

    fn propagate(&mut self, system: &mut System, potentials: &Potentials) {
        self.thermostat.pre_integrate(system);
        self.integrator.integrate(system, potentials);
        self.thermostat.post_integrate(system);
    }
}
