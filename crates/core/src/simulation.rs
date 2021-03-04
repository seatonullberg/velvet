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

    pub fn run(&mut self, steps: usize) {
        #[cfg(feature = "hdf5-output")]
        let file = hdf5::File::create(self.config.output_filename()).unwrap();

        // setup potentials
        self.potentials.setup(&self.system);

        // setup propagation
        self.propagator.setup(&mut self.system, &self.potentials);

        // start iteration loop
        info!("Starting iteration loop...");
        for i in 0..steps {
            // do the propagation step
            self.propagator
                .propagate(&mut self.system, &self.potentials);

            // update the itneraction groups
            self.potentials.update(&self.system, i);

            // output results
            if i % self.config.output_interval() == 0 || i == steps - 1 {
                info!("Results for timestep: {}", i);

                #[cfg(feature = "hdf5-output")]
                let group = file.create_group(&format!("{}", i)).unwrap();

                for out in self.config.outputs() {
                    #[cfg(not(feature = "hdf5-output"))]
                    out.output(&self.system, &self.potentials);

                    #[cfg(feature = "hdf5-output")]
                    out.output(&self.system, &self.potentials, &group);
                }
            }
        }
        info!("Iteration loop complete ({} iterations).", steps);
    }

    pub fn consume(self) -> (System, Potentials) {
        (self.system, self.potentials)
    }
}
