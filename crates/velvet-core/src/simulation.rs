//! High level abstraction for an atomistic simulation.

use crate::outputs::{Output, OutputMeta};
use crate::potentials::Potentials;
use crate::propagator::Propagator;
use velvet_system::System;

/// High level abstraction for an atomistic simulation.
pub struct Simulation {
    system: System,
    potentials: Potentials,
    propagator: Box<dyn Propagator>,
    outputs: Option<Vec<OutputMeta>>,
}

pub struct SimulationBuilder {
    system: System,
    potentials: Potentials,
    propagator: Box<dyn Propagator>,
    outputs: Option<Vec<OutputMeta>>
}

impl SimulationBuilder {
    pub fn new<P>(system: System, potentials: Potentials, propagator: P) -> Self 
    where
        P: Propagator + 'static,
    {
        let propagator = Box::new(propagator);
        let outputs = None;
        SimulationBuilder {
            system,
            potentials,
            propagator,
            outputs,
        }
    }

    pub fn output<O, W>(mut self, output: O, writer: W, interval: usize) -> Self 
    where
        O: Output + 'static,
        W: std::io::Write + 'static,
    {
        let output_meta = OutputMeta::new(output, writer, interval);
        let output_metas = &mut self.outputs;
        match output_metas {
            Some(output_metas) => {
                output_metas.push(output_meta);
            }
            None => {
                let mut output_metas = Vec::new();
                output_metas.push(output_meta);
                self.outputs = Some(output_metas);
            }
        }
        self
    }

    pub fn build(self) -> Simulation {
        let system = self.system;
        let potentials = self.potentials;
        let propagator = self.propagator;
        let outputs = self.outputs;
        Simulation {
            system,
            potentials,
            propagator,
            outputs,
        }
    }
}

impl Simulation {
    pub fn run(&mut self, timesteps: usize) {
        // Setup potentials.
        self.potentials.setup(&self.system);
        // Setup propagator.
        self.propagator.setup(&mut self.system, &self.potentials);
        // Start iteration loop.
        for i in 0..timesteps {
            // Propagate 1 timestep.
            self.propagator.propagate(&mut self.system, &self.potentials);
            // TODO: updates should NOT be done every timestep.
            self.potentials.update(&self.system);
            // Log outputs if any exist.
            match &mut self.outputs {
                Some(outputs) => {
                    outputs
                        .iter_mut()
                        .for_each(|meta| {
                            if i % meta.interval == 0 {
                                meta.output(&self.system, &self.potentials, i);
                            }
                        });
                }
                None => {}
            }
        }
    }

    pub fn consume(self) -> (System, Potentials) {
        (self.system, self.potentials)
    }
}
