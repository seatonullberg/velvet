use crate::potentials::Potentials;
use crate::properties::{
    Forces, KineticEnergy, PotentialEnergy, Property, Temperature, TotalEnergy,
};
use crate::system::System;

pub trait Output {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group);
}

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

impl Output for Temperature {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let temperature = self.calculate(system, potentials);
        let dataset = group.new_dataset::<f32>().create("temperature", 1).unwrap();
        dataset.write(&[temperature]).unwrap();
    }
}
