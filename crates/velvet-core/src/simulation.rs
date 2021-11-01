//! High level abstraction for an atomistic simulation.

#[cfg(feature = "quiet")]
use indicatif::ProgressDrawTarget;
use indicatif::{ProgressBar, ProgressStyle};

use crate::potentials::Potentials;
use crate::propagators::Propagator;
use crate::system::System;

/// High level abstraction for an atomistic simulation.
pub struct Simulation {
    system: System,
    potentials: Potentials,
    propagator: Box<dyn Propagator>,
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
        // Setup the global threadpool.
        // TODO: Actually handle the possible error. Causes issues with unit testing if I unwrap().
        let _ = rayon::ThreadPoolBuilder::new()
            .num_threads(self.config.n_threads)
            .build_global();

        // Setup potentials.
        self.potentials.setup(&self.system);

        // Setup propagation method.
        self.propagator.setup(&mut self.system, &self.potentials);

        // Setup progress bar.
        let pb = ProgressBar::new(steps as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{eta_precise}] {bar:40.green} {pos:>7} /{len:>7} steps"),
        );

        // Hide the progress bar if the `quiet` feature is enabled.
        #[cfg(feature = "quiet")]
        pb.set_draw_target(ProgressDrawTarget::hidden());

        // Start iteration loop.
        for i in 0..steps {
            // Do one propagation step.
            self.propagator
                .propagate(&mut self.system, &self.potentials);

            // Update the potentials.
            self.potentials.update(&self.system, i);

            // Generate raw outputs at appropriate intervals.
            for group in self.config.raw_output_groups() {
                let should_output = i % group.interval == 0 || i == steps - 1;
                let destination = group.destination.as_mut();
                for output in group.outputs.iter() {
                    if should_output {
                        output.output_raw(&self.system, &self.potentials, destination)
                    }
                }
            }

            // Generate HDF5 formatted outputs of the `hdf5-output` feature is enabled.
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

            // Update the progress bar at the end of each step.
            pb.inc(1);
        }

        // Finish the progress bar once iteration has terminated.
        pb.finish();
    }

    /// Consume the simulation and return its [`System`] and [`Potentials`].
    pub fn consume(self) -> (System, Potentials) {
        (self.system, self.potentials)
    }
}
