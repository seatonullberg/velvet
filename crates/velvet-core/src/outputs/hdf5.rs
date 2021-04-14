//! HDF5 formatted outputs.

use crate::internal::Float;

use crate::potentials::collections::Potentials;
use crate::properties::energy::{KineticEnergy, PairEnergy, PotentialEnergy, TotalEnergy};
use crate::properties::forces::Forces;
use crate::properties::temperature::Temperature;
use crate::properties::Property;
use crate::system::System;

/// Shared behavior to write a simulation result to an HDF5 file.
pub trait Hdf5Output {
    /// Writes the HDF5 formatted output.
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group);
}

pub struct Hdf5OutputGroup {
    pub file_handle: hdf5::File,
    pub interval: usize,
    pub outputs: Vec<Box<dyn Hdf5Output>>,
}

pub struct Hdf5OutputGroupBuilder {
    filename: String,
    interval: usize,
    outputs: Vec<Box<dyn Hdf5Output>>,
}

impl Hdf5OutputGroupBuilder {
    pub fn new() -> Hdf5OutputGroupBuilder {
        Hdf5OutputGroupBuilder {
            filename: "velvet.h5".to_string(),
            interval: 1,
            outputs: Vec::new(),
        }
    }

    pub fn filename(&mut self, filename: Into<String>) -> Hdf5OutputGroupBuilder {
        self.filename = filename.into();
        self
    }

    pub fn interval(&mut self, interval: usize) -> Hdf5OutputGroupBuilder {
        self.interval = interval;
        self
    }

    pub fn output(&mut self, output: T) -> Hdf5OutputGroupBuilder {
        self.outputs.push(Box::new(output));
        self
    }

    pub fn build(self) -> Hdf5OutputGroup {
        Hdf5OutputGroup {
            file_handle: hdf5::File::create(self.filename).unwrap(),
            interval: self.interval,
            outputs: self.outputs,
        }
    }
}

// This issue: https://github.com/rust-lang/rust/issues/20400
// prevents me from specializing the impl block by the trait's associated type.
// Ideally I will have separate impl blocks for Property<Res=Float> and Property<Res=Vector3<Float>>
// in order to make the formatting more appropriate.

impl Hdf5Output for Forces {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let forces = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<[Float; 3]>()
            .create(self.name(), system.size)
            .unwrap();
        let arr: Vec<[Float; 3]> = forces.iter().map(|x| [x[0], x[1], x[2]]).collect();
        dataset.write(arr.as_slice()).unwrap()
    }
}

impl Hdf5Output for KineticEnergy {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group.new_dataset::<Float>().create(self.name(), 1).unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

impl Hdf5Output for PotentialEnergy {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group.new_dataset::<Float>().create(self.name(), 1).unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

impl Hdf5Output for TotalEnergy {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group.new_dataset::<Float>().create(self.name(), 1).unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

impl Hdf5Output for PairEnergy {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group.new_dataset::<Float>().create(self.name(), 1).unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

impl Hdf5Output for Temperature {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let temperature = self.calculate(system, potentials);
        let dataset = group.new_dataset::<Float>().create(self.name(), 1).unwrap();
        dataset.write(&[temperature]).unwrap();
    }
}
