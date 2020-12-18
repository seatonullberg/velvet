use indicatif::{ProgressBar, ProgressStyle};
use log::Level::Info;
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
        // Initialize the logger.
        pretty_env_logger::init();

        // Initialize the progressbar
        let progress: Option<ProgressBar> = if log_enabled!(Info) {
            let pbar = ProgressBar::new(steps as u64);
            let style = ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.green} {pos}/{len} steps");
            pbar.set_style(style);
            Some(pbar)
        } else {
            None
        };

        // open HDF5 output file
        let file = hdf5::File::create(self.config.output_filename()).unwrap();

        // setup propagation
        self.propagator.setup(&mut self.system, &self.potentials);
        for i in 0..steps {
            self.propagator
                .propagate(&mut self.system, &self.potentials);

            if i % self.config.output_interval() == 0 {
                let group = file.create_group(&format!("{}", i)).unwrap();
                for out in self.config.outputs() {
                    out.output(&self.system, &self.potentials, &group);
                }
            }

            match &progress {
                Some(pbar) => pbar.inc(1),
                None => (),
            }
        }
    }
}
