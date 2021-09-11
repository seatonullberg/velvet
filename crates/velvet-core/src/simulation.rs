//! High level abstraction for an atomistic simulation.

#[cfg(feature = "quiet")]
use indicatif::ProgressDrawTarget;
use indicatif::{ProgressBar, ProgressStyle};

use crate::config::Configuration;
use crate::potentials::Potentials;
use crate::propagators::Propagator;
use crate::system::System;

/// High level abstraction for an atomistic simulation.
pub struct Simulation {
    system: System,
    potentials: Potentials,
    propagator: Box<dyn Propagator>,
    config: Configuration,
}

impl<'a> Simulation {
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
        // setup global threadpool
        rayon::ThreadPoolBuilder::new()
            .num_threads(self.config.n_threads)
            .build_global();

        // setup potentials
        self.potentials.setup(&self.system);

        // setup propagation
        self.propagator.setup(&mut self.system, &self.potentials);

        // setup progress bar
        let pb = ProgressBar::new(steps as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{eta_precise}] {bar:40.green} {pos:>7} /{len:>7} steps"),
        );

        #[cfg(feature = "quiet")]
        pb.set_draw_target(ProgressDrawTarget::hidden());

        // start iteration loop
        for i in 0..steps {
            // do one propagation step
            self.propagator
                .propagate(&mut self.system, &self.potentials);

            // update the potentials
            self.potentials.update(&self.system, i);

            // raw outputs
            for group in self.config.raw_output_groups() {
                let should_output = i % group.interval == 0 || i == steps - 1;
                let destination = group.destination.as_mut();
                for output in group.outputs.iter() {
                    if should_output {
                        output.output_raw(&self.system, &self.potentials, destination)
                    }
                }
            }

            // HDF5 outputs
            #[cfg(feature = "hdf5-output")]
            {
                for group in self.config.hdf5_output_groups() {
                    let should_output = i % group.interval == 0 || i == steps - 1;
                    let g = group.file_handle.create_group(&format!("{}", i)).unwrap();
                    for output in group.outputs.iter() {
                        if should_output {
                            output.output_hdf5(&self.system, &self.potentials, &g)
                        }
                    }
                }
            }
            pb.inc(1);
        }
        pb.finish();
    }

    /// Consume the simulation and return its [`System`] and [`Potentials`].
    pub fn consume(self) -> (System, Potentials) {
        (self.system, self.potentials)
    }
}
