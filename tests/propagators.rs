use approx::*;

use velvet_core::integrators::VelocityVerlet;
use velvet_core::propagators::{MolecularDynamics, Propagator};
use velvet_core::properties::{IntrinsicProperty, Temperature};
use velvet_core::thermostats::Berendsen;
use velvet_core::velocity_distributions::{Boltzmann, VelocityDistribution};

mod common;

static ITERATIONS: usize = 5000;

#[test]
fn molecular_dynamics() {
    // load system
    // let path = test_resources_path("argon.sys.velvet");
    // let file = File::open(&path).unwrap();
    // let mut system: System = ron::de::from_reader(file).unwrap();
    let mut system = common::get_argon_system();

    let target = 300 as f32;
    let boltz = Boltzmann::new(target);
    boltz.apply(&mut system);

    // load potentials
    // let path = test_resources_path("argon.pot.velvet");
    // let file = File::open(&path).unwrap();
    // let potentials: Potentials = ron::de::from_reader(file).unwrap();
    let potentials = common::get_argon_potentials(&system);

    let timestep = 1.0;
    let velocity_verlet = VelocityVerlet::new(timestep);
    let tau = 2.0;
    let berendsen = Berendsen::new(target, tau);
    let mut md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(berendsen));
    md.setup(&mut system, &potentials);

    for _ in 0..ITERATIONS {
        md.propagate(&mut system, &potentials);
    }

    assert_relative_eq!(
        Temperature.calculate_intrinsic(&system),
        target,
        epsilon = 1e-4,
    );
}