use serde::{Serialize, Deserialize};

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
        // open HDF5 output file
        let file = hdf5::File::create(self.config.output_filename()).unwrap();

        // setup propagation
        self.propagator.setup(&mut self.system, &self.potentials);
        for i in 0..steps {
            self.propagator
                .propagate(&mut self.system, &self.potentials);
            let group = file.create_group(&format!("{}", i)).unwrap();
            for out in self.config.outputs() {
                out.output(&self.system, &self.potentials, &group);
            }
        }
    }
}
