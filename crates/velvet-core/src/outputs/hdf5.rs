//! HDF5 formatted outputs.

use crate::internal::Float;

use crate::potentials::collections::Potentials;
use crate::properties::energy::{KineticEnergy, PairEnergy, PotentialEnergy, TotalEnergy};
use crate::properties::forces::Forces;
use crate::properties::temperature::Temperature;
use crate::properties::Property;
use crate::system::System;

#[typetag::serde(tag = "type")]
pub trait Hdf5Output {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group);
}

#[typetag::serde]
impl Hdf5Output for Forces {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let forces = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<[Float; 3]>()
            .create("forces", system.size)
            .unwrap();
        let arr: Vec<[Float; 3]> = forces.iter().map(|x| [x[0], x[1], x[2]]).collect();
        dataset.write(arr.as_slice()).unwrap()
    }
}

#[typetag::serde]
impl Hdf5Output for KineticEnergy {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<Float>()
            .create("kinetic_energy", 1)
            .unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

#[typetag::serde]
impl Hdf5Output for PotentialEnergy {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<Float>()
            .create("potential_energy", 1)
            .unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

#[typetag::serde]
impl Hdf5Output for TotalEnergy {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<Float>()
            .create("total_energy", 1)
            .unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

#[typetag::serde]
impl Hdf5Output for PairEnergy {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<Float>()
            .create("pair_energy", 1)
            .unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

#[typetag::serde]
impl Hdf5Output for Temperature {
    fn output_hdf5(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let temperature = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<Float>()
            .create("temperature", 1)
            .unwrap();
        dataset.write(&[temperature]).unwrap();
    }
}
