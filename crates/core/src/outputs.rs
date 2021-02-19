use crate::potentials::Potentials;
use crate::properties::energy::{
    CoulombEnergy, KineticEnergy, PairEnergy, PotentialEnergy, TotalEnergy,
};
use crate::properties::forces::Forces;
use crate::properties::temperature::Temperature;
use crate::properties::Property;
use crate::system::System;
use crate::internal::Float;

#[cfg(not(feature = "hdf5-output"))]
#[typetag::serde(tag = "type")]
pub trait Output {
    fn output(&self, system: &System, potentials: &Potentials);
}

#[cfg(feature = "hdf5-output")]
#[typetag::serde(tag = "type")]
pub trait Output {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group);
}

#[cfg(not(feature = "hdf5-output"))]
#[typetag::serde]
impl Output for Forces {
    fn output(&self, system: &System, potentials: &Potentials) {
        let forces = self.calculate(system, potentials);
        info!("Forces: {:#?}", forces);
    }
}

#[cfg(feature = "hdf5-output")]
#[typetag::serde]
impl Output for Forces {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let forces = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<[Float; 3]>()
            .create("forces", system.size())
            .unwrap();
        let arr: Vec<[Float; 3]> = forces.iter().map(|x| [x[0], x[1], x[2]]).collect();
        dataset.write(arr.as_slice()).unwrap()
    }
}

#[cfg(not(feature = "hdf5-output"))]
#[typetag::serde]
impl Output for KineticEnergy {
    fn output(&self, system: &System, potentials: &Potentials) {
        let energy = self.calculate(system, potentials);
        info!("Kinetic Energy: {:?}", energy);
    }
}

#[cfg(feature = "hdf5-output")]
#[typetag::serde]
impl Output for KineticEnergy {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<Float>()
            .create("kinetic_energy", 1)
            .unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

#[cfg(not(feature = "hdf5-output"))]
#[typetag::serde]
impl Output for PotentialEnergy {
    fn output(&self, system: &System, potentials: &Potentials) {
        let energy = self.calculate(system, potentials);
        info!("Potential Energy: {:?}", energy);
    }
}

#[cfg(feature = "hdf5-output")]
#[typetag::serde]
impl Output for PotentialEnergy {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<Float>()
            .create("potential_energy", 1)
            .unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

#[cfg(not(feature = "hdf5-output"))]
#[typetag::serde]
impl Output for TotalEnergy {
    fn output(&self, system: &System, potentials: &Potentials) {
        let energy = self.calculate(system, potentials);
        info!("Total Energy: {:?}", energy);
    }
}

#[cfg(feature = "hdf5-output")]
#[typetag::serde]
impl Output for TotalEnergy {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<Float>()
            .create("total_energy", 1)
            .unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

#[cfg(not(feature = "hdf5-output"))]
#[typetag::serde]
impl Output for CoulombEnergy {
    fn output(&self, system: &System, potentials: &Potentials) {
        let energy = self.calculate(system, potentials);
        info!("Coulomb Energy: {:?}", energy);
    }
}

#[cfg(feature = "hdf5-output")]
#[typetag::serde]
impl Output for CoulombEnergy {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group
            .new_dataset::<Float>()
            .create("coulomb_energy", 1)
            .unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

#[cfg(not(feature = "hdf5-output"))]
#[typetag::serde]
impl Output for PairEnergy {
    fn output(&self, system: &System, potentials: &Potentials) {
        let energy = self.calculate(system, potentials);
        info!("Pair Energy: {:?}", energy);
    }
}

#[cfg(feature = "hdf5-output")]
#[typetag::serde]
impl Output for PairEnergy {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let energy = self.calculate(system, potentials);
        let dataset = group.new_dataset::<Float>().create("pair_energy", 1).unwrap();
        dataset.write(&[energy]).unwrap();
    }
}

#[cfg(not(feature = "hdf5-output"))]
#[typetag::serde]
impl Output for Temperature {
    fn output(&self, system: &System, potentials: &Potentials) {
        let temp = self.calculate(system, potentials);
        info!("Temperature: {:?}", temp);
    }
}

#[cfg(feature = "hdf5-output")]
#[typetag::serde]
impl Output for Temperature {
    fn output(&self, system: &System, potentials: &Potentials, group: &hdf5::Group) {
        let temperature = self.calculate(system, potentials);
        let dataset = group.new_dataset::<Float>().create("temperature", 1).unwrap();
        dataset.write(&[temperature]).unwrap();
    }
}
