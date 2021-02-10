//! User defined configuration options.

use serde::{Deserialize, Serialize};

use crate::outputs::Output;

/// High-level configuration options.
#[derive(Serialize, Deserialize)]
pub struct Configuration {
    threads: usize,
    outputs: Vec<Box<dyn Output>>,
    output_interval: usize,
    output_filename: String,
}

impl Configuration {
    /// Returns the number of threads in the threadpool.
    pub fn threads(&self) -> usize {
        self.threads
    }

    /// Returns an iterator over the outputs.
    pub fn outputs(&self) -> impl Iterator<Item = &Box<dyn Output>> {
        self.outputs.iter()
    }

    /// Returns the number of steps between each output call.
    pub fn output_interval(&self) -> usize {
        self.output_interval
    }

    /// Returns the filename of the HDF5 formatted output file.
    pub fn output_filename(&self) -> String {
        self.output_filename.clone()
    }
}

/// Constructor for the [`Configuration`](velvet_core::config::Configuration) type.
pub struct ConfigurationBuilder {
    threads: Option<usize>,
    outputs: Vec<Box<dyn Output>>,
    output_interval: Option<usize>,
    output_filename: Option<String>,
}

impl Default for ConfigurationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigurationBuilder {
    /// Returns a new `ConfigurationBuilder`.
    pub fn new() -> ConfigurationBuilder {
        ConfigurationBuilder {
            threads: None,
            outputs: Vec::new(),
            output_interval: None,
            output_filename: None,
        }
    }

    /// Sets the size of the threadpool.
    pub fn with_threads(mut self, threads: usize) -> ConfigurationBuilder {
        self.threads = Some(threads);
        self
    }

    /// Adds an output to the configuration.
    pub fn with_output(mut self, output: Box<dyn Output>) -> ConfigurationBuilder {
        self.outputs.push(output);
        self
    }

    /// Sets the number of steps between output calls.
    pub fn with_output_interval(mut self, interval: usize) -> ConfigurationBuilder {
        self.output_interval = Some(interval);
        self
    }

    /// Sets the filename of the HDF5 formatted output file.
    pub fn with_output_filename(mut self, filename: String) -> ConfigurationBuilder {
        self.output_filename = Some(filename);
        self
    }

    /// Returns an initialized `Configuration`.
    pub fn build(self) -> Configuration {
        let threads = match self.threads {
            Some(t) => t,
            None => 1,
        };

        let outputs = self.outputs;

        let output_interval = match self.output_interval {
            Some(interval) => interval,
            None => 1,
        };

        let output_filename = match self.output_filename {
            Some(filename) => filename,
            None => "velvet.h5".to_string(),
        };

        Configuration {
            threads,
            outputs,
            output_interval,
            output_filename,
        }
    }
}
