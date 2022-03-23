pub mod integrators;
pub mod thermostats;
pub mod velocity_distributions;

use velvet_core::potentials::Potentials;
use velvet_core::propagator::Propagator;
use velvet_system::System;

use crate::integrators::Integrator;
use crate::thermostats::Thermostat;

pub struct MolecularDynamics {
    integrator: Box<dyn Integrator>,
    thermostat: Option<Box<dyn Thermostat>>,
    // barostat: Option<Box<>dyn Barostat>,
}

impl Propagator for MolecularDynamics {
    fn setup(&mut self, system: &mut System, potentials: &Potentials) {
        self.integrator.setup(system, potentials);
        match &mut self.thermostat {
            Some(thermostat) => thermostat.setup(system),
            None => {}
        }
    }

    fn propagate(&mut self, system: &mut System, potentials: &Potentials) {
        // TODO: this match will get ugly when barostat is added
        match &mut self.thermostat {
            Some(thermostat) => {
                thermostat.pre_integrate(system, potentials);
                self.integrator.integrate(system, potentials);
                thermostat.post_integrate(system, potentials);
            }
            None => self.integrator.integrate(system, potentials),
        }
    }
}

pub struct MolecularDynamicsBuilder {
    integrator: Box<dyn Integrator>,
    thermostat: Option<Box<dyn Thermostat>>,
    // barostat: Option<Box<dyn Barostat>>,
}

impl MolecularDynamicsBuilder {
    pub fn new<I>(integrator: I) -> Self
    where
        I: Integrator + 'static,
    {
        MolecularDynamicsBuilder {
            integrator: Box::new(integrator),
            thermostat: None,
        }
    }

    pub fn thermostat<T>(mut self, thermostat: T) -> Self
    where
        T: Thermostat + 'static,
    {
        self.thermostat = Some(Box::new(thermostat));
        self
    }

    pub fn build(self) -> MolecularDynamics {
        MolecularDynamics {
            integrator: self.integrator,
            thermostat: self.thermostat,
        }
    }
}
