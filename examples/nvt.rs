//! This example executes a simulation of 108 Ar atoms in the NVT ensemble.
//! After sucessful completion, a figure is generated at `nvt.png` which plots the total energy at each timestep.

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use plotters::prelude::*;

use std::fs::File;
use std::io::BufReader;

use velvet::convert::load_poscar;
use velvet::core::distributions::{Boltzmann, VelocityDistribution};
use velvet::core::integrators::VelocityVerlet;
use velvet::core::potentials::pair::{LennardJones, PairPotentialMeta};
use velvet::core::potentials::{Potentials, Restriction};
use velvet::core::properties::Temperature;
use velvet::core::system::elements::Element;
use velvet::core::thermostats::NoseHoover;
use velvet_core::config::ConfigurationBuilder;
use velvet_core::propagators::MolecularDynamics;
use velvet_core::simulation::Simulation;

static TIMESTEPS: u64 = 250000;
static PLOT_INTERVAL: u64 = 50;
static FILENAME: &'static str = "assets/nvt.png";

fn main() {
    pretty_env_logger::init();
    info!("Starting a NVT simulation of Ar gas...");

    // Load the Ar gas system directly from a POSCAR formatted file.
    let file = File::open("resources/test/argon.poscar").unwrap();
    let reader = BufReader::new(file);
    let mut system = load_poscar(reader);

    // Setup an initial velocity distribution with a target temperature
    let boltz = Boltzmann::new(300 as f32);

    // Apply the initialized velocity distribution to the system.
    boltz.apply(&mut system);

    // Define a Lennard-Jones style pair potential.
    let lj = LennardJones::new(1.0, 3.4);

    // Define some metadata about the potential.
    // - The element pair which it applies to.
    // - The cutoff radius.
    // - Any additional restrictions (intermolecular/intramolecular...)
    let meta = PairPotentialMeta::new((Element::Ar, Element::Ar), 8.5, Restriction::None);

    // Initialize a collection of potentials and add the previously defined pair potential with metadata.
    let mut potentials = Potentials::new();
    potentials.add_pair(Box::new(lj), meta);

    // Define a velocity Verlet style integrator.
    let velocity_verlet = VelocityVerlet::new(1.0);

    // Define a Nose-Hoover thermostat
    let nose_hoover = NoseHoover::new(300 as f32, 1.5, 1.0);

    // Build molecular dynamics propagator from components.
    // Use a Nose-Hoover style thermostat to simulate the NVT ensemble.
    let md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(nose_hoover));

    // Default configuration
    let mut builder = ConfigurationBuilder::new();
    builder.with_output_interval(PLOT_INTERVAL as usize);
    builder.with_output(Box::new(Temperature));
    builder.with_output_filename("nvt.h5");
    let config = builder.finish();

    let mut sim = Simulation::new(system, potentials, Box::new(md), config);

    sim.run(TIMESTEPS as usize);

    // read results file
    let file = hdf5::File::open("nvt.h5").unwrap();
    let mut temperatures: Vec<(u64, f64)> =
        Vec::with_capacity((TIMESTEPS % PLOT_INTERVAL) as usize);
    for i in 0..TIMESTEPS {
        if i % PLOT_INTERVAL == 0 {
            let temp = file.dataset(&format!("{}/temperature", i)).unwrap();
            let temp = temp.read_1d::<f32>().unwrap();
            temperatures.push((i, temp[0] as f64));
        }
    }

    info!("Simulation completed successfully.");

    // Plot the energy results
    plot_results(temperatures);

    info!("Generated summary figure: `{}`", FILENAME);
}

fn plot_results(data: Vec<(u64, f64)>) {
    let root_area = BitMapBackend::new(FILENAME, (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 100)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .margin(10)
        .margin_right(30)
        .build_cartesian_2d(0..TIMESTEPS, 285.0..320.0)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Timestep")
        .x_label_style(("sans-serif", 18))
        .y_desc("Temperature (K)")
        .y_label_style(("sans-serif", 18))
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(data.into_iter(), &BLUE))
        .unwrap();
}
