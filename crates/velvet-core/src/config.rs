//! User defined configuration options.

#[cfg(feature = "hdf5-output")]
use crate::outputs::hdf5::Hdf5OutputGroup;
use crate::outputs::raw::RawOutputGroup;

/// High-level configuration options.
pub struct Configuration {
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
    raw_output_groups: Vec<RawOutputGroup>,
    #[cfg(feature = "hdf5-output")]
    hdf5_output_groups: Vec<Hdf5OutputGroup>,
}

impl ConfigurationBuilder {
    /// Returns a new `ConfigurationBuilder`.
    pub fn new() -> ConfigurationBuilder {
        ConfigurationBuilder {
            raw_output_groups: Vec::new(),
            #[cfg(feature = "hdf5-output")]
            hdf5_output_groups: Vec::new(),
        }
    }

    pub fn raw_output_group(mut self, group: RawOutputGroup) -> ConfigurationBuilder {
        self.raw_output_groups.push(group);
        self
    }

    #[cfg(feature = "hdf5-output")]
    pub fn hdf5_output_group(mut self, group: Hdf5OutputGroup) -> ConfigurationBuilder {
        self.hdf5_output_groups.push(group);
        self
    }

    /// Returns an initialized `Configuration`.
    pub fn build(self) -> Configuration {
        Configuration {
            raw_output_groups: self.raw_output_groups,
            #[cfg(feature = "hdf5-output")]
            hdf5_output_groups: self.hdf5_output_groups,
        }
    }
}
