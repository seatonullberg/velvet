use velvet_core::integrators::{Integrator, VelocityVerlet};
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};
use velvet_test_utils as test_utils;

static ITERATIONS: usize = 5000;

#[test]
fn velocity_verlet() {
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

    for _ in 0..ITERATIONS {
        velocity_verlet.integrate(&mut system, &potentials);
    }

    assert!(system.velocities[0].norm() < 0.1)
}
