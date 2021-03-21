//! User defined configuration options.

use serde::{Deserialize, Serialize};

use crate::outputs::Output;
#[cfg(feature = "hdf5-output")]
use crate::outputs::hdf5::Hdf5Output;

/// High-level configuration options.
#[derive(Serialize, Deserialize)]
pub struct Configuration {
    // default options
    outputs: Vec<Box<dyn Output>>,
    output_interval: usize,

    // hdf5 options
    #[cfg(feature = "hdf5-output")]
    hdf5_output_filename: String,
    #[cfg(feature = "hdf5-output")]
    hdf5_outputs: Vec<Box<dyn Hdf5Output>>,

    // rayon options
    #[cfg(feature = "rayon")]
    n_threads: usize,
}

impl Configuration {
    /// Returns an iterator over the outputs.
    pub fn outputs(&self) -> impl Iterator<Item = &dyn Output> {
        self.outputs.iter().map(|x| x.as_ref())
    }

    /// Returns the number of steps between each output call.
    pub fn output_interval(&self) -> usize {
        self.output_interval
    }
    
    /// Returns the filename of the HDF5 formatted output file.
    #[cfg(feature = "hdf5-output")]
    pub fn hdf5_output_filename(&self) -> String {
        self.hdf5_output_filename.clone()
    }

    /// Returns an iterator over the HDF5 outputs
    #[cfg(feature = "hdf5-output")]
    pub fn hdf5_outputs(&self) -> impl Iterator<Item = &dyn Hdf5Output> {
        self.hdf5_outputs.iter().map(|x| x.as_ref())
    }

    /// Returns the number of threads in the threadpool.
    #[cfg(feature = "rayon")]
    pub fn n_threads(&self) -> usize {
        self.n_threads
    }
}

/// Constructor for the [`Configuration`](velvet_core::config::Configuration) type.
pub struct ConfigurationBuilder {
    outputs: Vec<Box<dyn Output>>,
    output_interval: Option<usize>,

    #[cfg(feature = "hdf5-output")]
    hdf5_output_filename: Option<String>,
    #[cfg(feature = "hdf5-output")]
    hdf5_outputs: Vec<Box<dyn Hdf5Output>>,

    #[cfg(feature = "rayon")]
    n_threads: Option<usize>,
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
            outputs: Vec::new(),
            output_interval: None,

            #[cfg(feature = "hdf5-output")]
            hdf5_output_filename: None,
            #[cfg(feature = "hdf5-output")]
            hdf5_outputs: Vec::new(),
            
            #[cfg(feature = "rayon")]
            n_threads: None,
        }
    }

    /// Adds an output to the configuration.
    pub fn add_output<T: Output + 'static>(mut self, output: T) -> ConfigurationBuilder {
        self.outputs.push(Box::new(output));
        self
    }

    /// Sets the number of steps between output calls.
    pub fn with_output_interval(mut self, interval: usize) -> ConfigurationBuilder {
        self.output_interval = Some(interval);
        self
    }

    /// Sets the filename of the HDF5 formatted output file.
    #[cfg(feature = "hdf5-output")]
    pub fn with_hdf5_output_filename(mut self, filename: String) -> ConfigurationBuilder {
        self.hdf5_output_filename = Some(filename);
        self
    }

    /// Adds an HDF5 formatted output to the configuration
    #[cfg(feature = "hdf5-output")]
    pub fn add_hdf5_output<T: Hdf5Output + 'static>(mut self, output: T) -> ConfigurationBuilder {
        self.hdf5_outputs.push(Box::new(output));
        self
    }

    /// Sets the size of the threadpool.
    #[cfg(feature = "rayon")]
    pub fn with_n_threads(mut self, n_threads: usize) -> ConfigurationBuilder {
        self.n_threads = Some(n_threads);
        self
    }

    /// Returns an initialized `Configuration`.
    pub fn build(self) -> Configuration {
        let outputs = self.outputs;
        let output_interval = self.output_interval.unwrap_or(1); // TODO: this is a terrible default

        #[cfg(feature = "hdf5-output")]
        let hdf5_output_filename = self
            .hdf5_output_filename
            .unwrap_or_else(|| "velvet.h5".to_string());
        #[cfg(feature = "hdf5-output")]
        let hdf5_outputs = self.hdf5_outputs;

        #[cfg(feature = "rayon")]
        let n_threads = self.n_threads.unwrap_or(1); // TODO: use num_cpus crate to determine optimal thread count
        
        Configuration {
            outputs,
            output_interval,
            #[cfg(feature = "hdf5-output")]
            hdf5_output_filename,
            #[cfg(feature = "hdf5-output")]
            hdf5_outputs,
            #[cfg(feature = "rayon")]
            n_threads,
        }
    }
}
