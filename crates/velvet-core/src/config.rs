//! User defined configuration options.

use num_cpus;

#[cfg(feature = "hdf5-output")]
use crate::outputs::hdf5::Hdf5OutputGroup;
use crate::outputs::raw::RawOutputGroup;

/// High-level configuration options.
pub struct Configuration {
    pub n_threads: usize,
    raw_output_groups: Vec<RawOutputGroup>,
    #[cfg(feature = "hdf5-output")]
    hdf5_output_groups: Vec<Hdf5OutputGroup>,
}

impl Configuration {
    /// Returns an iterator over the raw output groups.
    pub fn raw_output_groups(&mut self) -> impl Iterator<Item = &mut RawOutputGroup> {
        self.raw_output_groups.iter_mut()
    }

    /// Returns an iterator over the HDF5 output groups.
    #[cfg(feature = "hdf5-output")]
    pub fn hdf5_output_groups(&mut self) -> impl Iterator<Item = &mut Hdf5OutputGroup> {
        self.hdf5_output_groups.iter_mut()
    }
}

/// Constructor for the [`Configuration`](velvet_core::config::Configuration) type.
pub struct ConfigurationBuilder {
    n_threads: usize,
    raw_output_groups: Vec<RawOutputGroup>,
    #[cfg(feature = "hdf5-output")]
    hdf5_output_groups: Vec<Hdf5OutputGroup>,
}

impl ConfigurationBuilder {
    /// Returns a new `ConfigurationBuilder`.
    pub fn new() -> ConfigurationBuilder {
        ConfigurationBuilder {
            n_threads: num_cpus::get(),
            raw_output_groups: Vec::new(),
            #[cfg(feature = "hdf5-output")]
            hdf5_output_groups: Vec::new(),
        }
    }

    /// Sets the number of threads in the global threadpool.
    pub fn n_threads(mut self, n_threads: usize) -> ConfigurationBuilder {
        self.n_threads = n_threads;
        self
    }

    /// Adds a raw output group to the configuration.
    pub fn raw_output_group(mut self, group: RawOutputGroup) -> ConfigurationBuilder {
        self.raw_output_groups.push(group);
        self
    }

    #[cfg(feature = "hdf5-output")]
    /// Adds an HDF5 output group to the configuration.
    pub fn hdf5_output_group(mut self, group: Hdf5OutputGroup) -> ConfigurationBuilder {
        self.hdf5_output_groups.push(group);
        self
    }

    /// Returns an initialized [`Configuration`].
    pub fn build(self) -> Configuration {
        Configuration {
            n_threads: self.n_threads,
            raw_output_groups: self.raw_output_groups,
            #[cfg(feature = "hdf5-output")]
            hdf5_output_groups: self.hdf5_output_groups,
        }
    }
}
