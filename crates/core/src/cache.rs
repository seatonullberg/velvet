// //! Cached physical properties of the system.

// use nalgebra::Vector3;

// use crate::potentials::Potentials;
// use crate::properties::{
//     Forces, KineticEnergy, PotentialEnergy, Property, Temperature, TotalEnergy,
// };
// use crate::system::System;

// /// Cached physical properties of the system.
// pub struct PropertyCache {
//     forces: Vec<Vector3<f32>>,
//     potential_energy: f32,
//     kinetic_energy: f32,
//     total_energy: f32,
//     temperature: f32,
// }

// impl PropertyCache {
//     /// Initializes a `PropertyCache` from System and Potentials.
//     pub fn new(system: &System, potentials: &Potentials) -> PropertyCache {
//         PropertyCache {
//             forces: Forces.calculate(system, potentials),
//             potential_energy: PotentialEnergy.calculate(system, potentials),
//             kinetic_energy: KineticEnergy.calculate(system, potentials),
//             total_energy: TotalEnergy.calculate(system, potentials),
//             temperature: Temperature.calculate(system, potentials),
//         }
//     }

//     /// Returns an iterator over the forces in the system.
//     pub fn forces(&self) -> impl Iterator<Item = &Vector3<f32>> {
//         self.forces.iter()
//     }

//     /// Sets the cached forces value.
//     pub fn set_forces(&mut self, forces: Vec<Vector3<f32>>) {
//         self.forces = forces;
//     }

//     /// Calculates forces, sets the cache value to a clone of the result, then returns the result.
//     pub fn calc_set_forces(
//         &mut self,
//         system: &System,
//         potentials: &Potentials,
//     ) -> Vec<Vector3<f32>> {
//         let forces = Forces.calculate(system, potentials);
//         self.forces = forces.clone();
//         forces
//     }

//     /// Returns the potential energy of the system
//     pub fn potential_energy(&self) -> f32 {
//         self.potential_energy
//     }

//     /// Sets the cached potential energy value
//     pub fn set_potential_energy(&mut self, potential_energy: f32) {
//         self.potential_energy = potential_energy
//     }

//     /// Calculates potential energy, sets the cache value to a clone of the result, then returns the result.
//     pub fn calc_set_potential_energy(&mut self, system: &System, potentials: &Potentials) -> f32 {
//         let energy = PotentialEnergy.calculate(system, potentials);
//         self.potential_energy = energy;
//         energy
//     }

//     /// Returns the kinetic energy of the system
//     pub fn kinetic_energy(&self) -> f32 {
//         self.kinetic_energy
//     }

//     /// Sets the cached kinetic energy value
//     pub fn set_kinetic_energy(&mut self, kinetic_energy: f32) {
//         self.kinetic_energy = kinetic_energy
//     }

//     /// Calculates kinetic energy, sets the cache value to a clone of the result, then returns the result.
//     pub fn calc_set_kinetic_energy(&mut self, system: &System, potentials: &Potentials) -> f32 {
//         let energy = KineticEnergy.calculate(system, potentials);
//         self.kinetic_energy = energy;
//         energy
//     }

//     /// Returns the total energy of the system
//     pub fn total_energy(&self) -> f32 {
//         self.total_energy
//     }

//     /// Sets the cached total energy value
//     pub fn set_total_energy(&mut self, total_energy: f32) {
//         self.total_energy = total_energy
//     }

//     /// Calculates total energy, sets the cache value to a clone of the result, then returns the result.
//     pub fn calc_set_total_energy(&mut self, system: &System, potentials: &Potentials) -> f32 {
//         let energy = TotalEnergy.calculate(system, potentials);
//         self.total_energy = energy;
//         energy
//     }

//     /// Returns the instantaneous temperature of the system
//     pub fn temperature(&self) -> f32 {
//         self.temperature
//     }

//     /// Sets the cached instantaneous temperature value
//     pub fn set_temperature(&mut self, temperature: f32) {
//         self.temperature = temperature
//     }

//     /// Calculates temperature, sets the cache value to a clone of the result, then returns the result.
//     pub fn calc_set_temperature(&mut self, system: &System, potentials: &Potentials) -> f32 {
//         let temperature = Temperature.calculate(system, potentials);
//         self.temperature = temperature;
//         temperature
//     }
// }
