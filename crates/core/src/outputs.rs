use serde::{Deserialize, Serialize};

use crate::potentials::Potentials;
use crate::properties::{
    Forces, KineticEnergy, PotentialEnergy, Property, Temperature, TotalEnergy,
};
use crate::system::System;

#[typetag::serde(tag = "type")]
pub trait Output {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group);
}

#[typetag::serde]
impl Output for Forces {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let forces = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<[f32; 3]>()
            .create("forces", system.size())
            .unwrap();
        let arr: Vec<[f32; 3]> = forces.into_iter().map(|x| [x[0], x[1], x[2]]).collect();
        dataset.write(arr.as_slice()).unwrap()
    }
}

#[typetag::serde]
impl Output for KineticEnergy {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let ke = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<f32>()
            .create("kinetic_energy", 1)
            .unwrap();
        dataset.write(&[ke]).unwrap();
    }
}

#[typetag::serde]
impl Output for PotentialEnergy {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let pe = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<f32>()
            .create("potential_energy", 1)
            .unwrap();
        dataset.write(&[pe]).unwrap();
    }
}

#[typetag::serde]
impl Output for TotalEnergy {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let te = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<f32>()
            .create("total_energy", 1)
            .unwrap();
        dataset.write(&[te]).unwrap();
    }
}

#[typetag::serde]
impl Output for Temperature {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let temperature = self.calculate(system, potentials);
        let dataset = group.new_dataset::<f32>().create("temperature", 1).unwrap();
        dataset.write(&[temperature]).unwrap();
    }
}
