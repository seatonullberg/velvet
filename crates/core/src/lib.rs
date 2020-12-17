//! Core feature library for the Velvet simulation engine.

#[warn(missing_docs)]
pub mod config;
pub mod constants;
pub mod distributions;
pub mod integrators;
pub mod outputs;
pub mod potentials;
pub mod propagators;
pub mod properties;
pub mod simulation;
pub mod system;
pub mod thermostats;

// use crate::potentials::Potentials;
// use crate::propagators::Propagator;
// use crate::system::System;

// pub fn run(
//     steps: usize,
//     propagator: &mut dyn Propagator,
//     system: &mut System,
//     potentials: &Potentials,
// ) {
//     propagator.setup(system, potentials);
//     for _ in 0..steps {
//         propagator.propagate(system, potentials);
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::run;
//     use crate::distributions::Boltzmann;
//     use crate::integrators::VelocityVerlet;
//     use crate::propagators::MolecularDynamics;
//     use crate::properties::{IntrinsicProperty, Temperature};
//     use crate::thermostats::Berendsen;
//     use crate::utils::{load_test_potentials, load_test_system};
//     use approx::*;

//     #[test]
//     fn test_run() {
//         let mut sys = load_test_system("argon");
//         let pots = load_test_potentials("argon");
//         let dt = 1.0;
//         let integrator = VelocityVerlet::new(dt);
//         let target_temp = 300 as f32;
//         let vdistr = Boltzmann::new(target_temp);
//         let thermostat = Berendsen::new(target_temp, 2.0);
//         let mut propagator =
//             MolecularDynamics::new(Box::new(integrator), Box::new(thermostat), Box::new(vdistr));

//         let steps = 5000;
//         run(steps, &mut propagator, &mut sys, &pots);

//         assert_relative_eq!(
//             Temperature.calculate_intrinsic(&sys),
//             target_temp,
//             epsilon = 1e-5
//         );
//     }
// }
