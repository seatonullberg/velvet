//! Algorithms to integrate classical equations of motion.

use nalgebra::Vector3;

use crate::potential::Potentials;
use crate::property::{Forces, Property};
use crate::system::System;

/// A numerical integration algorithm.
pub trait Integrator {
    /// Prepare the integrator to run.
    fn setup(&mut self, _: &System, _: &Potentials) {}
    /// Integrates one time step.
    fn integrate(&mut self, system: &mut System, potentials: &Potentials);
}

/// Velocity Verlet integration algorithm.
///
/// Include equations here.
#[derive(Clone, Debug)]
pub struct VelocityVerlet {
    timestep: f32,
    accelerations: Vec<Vector3<f32>>,
}

impl VelocityVerlet {
    /// Returns a new `VelocityVerlet`.
    pub fn new(timestep: f32) -> VelocityVerlet {
        VelocityVerlet {
            timestep,
            accelerations: Vec::new(),
        }
    }
}

impl Integrator for VelocityVerlet {
    fn setup(&mut self, system: &System, _: &Potentials) {
        self.accelerations = vec![Vector3::default(); system.size()];
    }

    fn integrate(&mut self, system: &mut System, potentials: &Potentials) {
        let dt = self.timestep;
        let sys_size = system.size();

        // update velocities at t + dt/2 and positions at t + dt
        for i in 0..sys_size {
            system.velocities[i] += 0.5 * dt * self.accelerations[i];
            system.positions[i] += system.velocities[i] * dt;
        }

        // calculate forces
        let forces = Forces;
        let forces = forces.calculate(system, potentials);

        // update accelerations at t + dt
        for i in 0..sys_size {
            self.accelerations[i] = forces[i] / system.masses[i];
        }

        // update velocities at t + dt
        for i in 0..sys_size {
            system.velocities[i] += 0.5 * dt * self.accelerations[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::integrate::{Integrator, VelocityVerlet};
    use crate::potential::pair::{LennardJones, PairPotentialMeta};
    use crate::potential::{Potentials, Restriction};
    use crate::system::{cell::Cell, element::Element, System};
    use nalgebra::Vector3;

    fn get_pair_system() -> System {
        let size = 2 as usize;
        let argon = Element::Ar;
        let mut sys = System::new(size);
        sys.cell = Cell::new(17.0, 17.0, 17.0, 90.0, 90.0, 90.0);
        sys.elements = vec![argon, argon];
        sys.molecules = vec![0 as usize, 0 as usize];
        sys.positions = vec![Vector3::new(0.0, 0.0, 0.0), Vector3::new(3.4, 3.4, 3.4)];
        sys.velocities = vec![
            Vector3::new(
                -0.007225222699367925,
                -0.002405756495275919,
                0.0026065109398392215,
            ),
            Vector3::new(
                0.001179633958023287,
                0.003525262341736351,
                -0.0004132774783154952,
            ),
        ];
        sys.masses = vec![argon.mass(), argon.mass()];
        sys.charges = vec![0.0, 0.0];
        sys
    }

    fn get_pair_potentials() -> Potentials {
        let mut pots = Potentials::new();
        let potential = Box::new(LennardJones::new(1.0, 3.4));
        let meta = PairPotentialMeta::new((Element::Ar, Element::Ar), 8.5, Restriction::None);
        pots.add_pair(potential, meta);
        pots
    }

    #[test]
    fn velocity_verlet() {
        // define the system
        let mut sys = get_pair_system();

        // define the potentials
        let pots = get_pair_potentials();

        // define the integrator
        let mut vv = VelocityVerlet::new(1.0);
        vv.setup(&sys, &pots);
        for _ in 0..5000 {
            vv.integrate(&mut sys, &pots)
        }

        // check that the simulation was stable
        assert!(sys.velocities[0].norm() < 0.1);
    }
}
