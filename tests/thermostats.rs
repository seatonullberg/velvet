use approx::*;

use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::properties::{IntrinsicProperty, Temperature};
use velvet_core::thermostats::{Berendsen, NoseHoover, Thermostat};
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 5000;

#[test]
fn berendsen() {
    // load system
    let mut system = test_utils::argon_system();

    let target = 100 as f32;
    let boltz = Boltzmann::new(target);
    boltz.apply(&mut system);

    // load potentials
    let potentials = test_utils::argon_potentials(&system);

    let timestep = 1.0;
    let mut velocity_verlet = VelocityVerlet::new(timestep);
    velocity_verlet.setup(&system, &potentials);
    let tau = 2.0;
    let mut berendsen = Berendsen::new(target, tau);
    berendsen.setup(&system);

    for _ in 0..ITERATIONS {
        berendsen.pre_integrate(&mut system);
        velocity_verlet.integrate(&mut system, &potentials);
        berendsen.post_integrate(&mut system);
    }

    assert_relative_eq!(
        Temperature.calculate_intrinsic(&system),
        target,
        epsilon = 1e-4
    );
}

#[test]
fn nose_hoover() {
    // load system
    let mut system = test_utils::argon_system();

    let target = 100 as f32;
    let boltz = Boltzmann::new(target);
    boltz.apply(&mut system);

    // load potentials
    let potentials = test_utils::argon_potentials(&system);

    let timestep = 1.0;
    let mut velocity_verlet = VelocityVerlet::new(timestep);
    velocity_verlet.setup(&system, &potentials);
    let freq = 1.01;
    let mut nose_hoover = NoseHoover::new(target, freq, timestep);
    nose_hoover.setup(&system);

    for _ in 0..ITERATIONS {
        nose_hoover.pre_integrate(&mut system);
        velocity_verlet.integrate(&mut system, &potentials);
        nose_hoover.post_integrate(&mut system);
    }

    assert_relative_eq!(
        Temperature.calculate_intrinsic(&system),
        target,
        epsilon = 100.0
    );
}
