use serde::{Deserialize, Serialize};

use crate::config::Configuration;
use crate::potentials::Potentials;
use crate::propagators::Propagator;
use crate::system::System;

#[derive(Serialize, Deserialize)]
pub struct Simulation {
    system: System,
    potentials: Potentials,
    propagator: Box<dyn Propagator>,
    config: Configuration,
}

impl Simulation {
    pub fn new(
        system: System,
        potentials: Potentials,
        propagator: Box<dyn Propagator>,
        config: Configuration,
    ) -> Simulation {
        Simulation {
            system,
            potentials,
            propagator,
            config,
        }
    }

    #[cfg(not(feature = "hdf5-output"))]
    pub fn run(&mut self, steps: usize) {
        // Initialize the logger.
        pretty_env_logger::init();
        info!("Starting simulation...");

        // setup propagation
        self.propagator.setup(&mut self.system, &self.potentials);
        for i in 0..steps {
            self.propagator
                .propagate(&mut self.system, &self.potentials);

            if i == 0 || i % self.config.output_interval() == 0 || i == steps - 1 {
                info!("Logging results for timestep: {}", i);
                for out in self.config.outputs() {
                    out.output(&self.system, &self.potentials);
                }
            }
        }
        info!("Simulation complete.")
    }

    #[cfg(feature = "hdf5-output")]
    pub fn run(&mut self, steps: usize) {
        // Initialize the logger.
        pretty_env_logger::init();
        info!("Starting simulation...");

        // open HDF5 output file
        let file = hdf5::File::create(self.config.output_filename()).unwrap();

        // setup propagation
        self.propagator.setup(&mut self.system, &self.potentials);
        for i in 0..steps {
            self.propagator
                .propagate(&mut self.system, &self.potentials);

            if i == 0 || i % self.config.output_interval() == 0 || i == steps - 1 {
                info!("Logging results for timestep: {}", i);
                let group = file.create_group(&format!("{}", i)).unwrap();
                for out in self.config.outputs() {
                    out.output(&self.system, &self.potentials, &group);
                }
            }
        }
        info!("Simulation complete.")
    }
}
