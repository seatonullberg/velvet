//! High level abstraction for an atomistic simulation.

use serde::{Deserialize, Serialize};

use crate::config::Configuration;
use crate::potentials::collections::Potentials;
use crate::propagators::Propagator;
use crate::system::System;

/// High level abstraction for an atomistic simulation.
#[derive(Serialize, Deserialize)]
pub struct Simulation {
    system: System,
    potentials: Potentials,
    propagator: Box<dyn Propagator>,
    config: Configuration,
}

impl Simulation {
    /// Returns a new [`Simulation`].
    pub fn new<P>(
        system: System,
        potentials: Potentials,
        propagator: P,
        config: Configuration,
    ) -> Simulation 
    where
        P: Propagator + 'static,
    {
        Simulation {
            system,
            potentials,
            propagator: Box::new(propagator),
            config,
        }
    }

    /// Runs the full iteration loop of the simulation.
    pub fn run(&mut self, steps: usize) {
        // setup potentials
        self.potentials.setup(&self.system);

        // setup propagation
        self.propagator.setup(&mut self.system, &self.potentials);

        // setup HDF5 output file
        #[cfg(feature = "hdf5-output")]
        let file = hdf5::File::create(self.config.hdf5_output_filename()).unwrap();

        // start iteration loop
        info!("Starting iteration loop...");
        for i in 0..steps {
            // do one propagation step
            self.propagator
                .propagate(&mut self.system, &self.potentials);

            // update the potentials
            self.potentials.update(&self.system, i);

            // output results
            if i % self.config.output_interval() == 0 || i == steps - 1 {
                info!("Results for timestep: {}", i);

                // log the standard outputs
                for out in self.config.outputs() {
                    out.output(&self.system, &self.potentials);
                }

                // write the HDF5 outputs to file
                #[cfg(feature = "hdf5-output")]
                let group = file.create_group(&format!("{}", i)).unwrap();
                #[cfg(feature = "hdf5-output")]
                for out in self.config.hdf5_outputs() {
                    out.output_hdf5(&self.system, &self.potentials, &group);
                }
            }
        }
        info!("Iteration loop complete ({} iterations).", steps);
    }

    /// Consume the simulation and return its [`System`] and [`Potentials`].
    pub fn consume(self) -> (System, Potentials) {
        (self.system, self.potentials)
    }
}
