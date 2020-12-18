//! This example executes a simulation of 108 Ar atoms in the NVE ensemble.
//! After sucessful completion, a figure is generated at `nve.png` which plots the total energy at each timestep.

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use plotters::prelude::*;

use std::fs::File;
use std::io::BufReader;

use velvet::prelude::*;

static TIMESTEPS: u64 = 250000;
static PLOT_INTERVAL: u64 = 50;
static FILENAME: &'static str = "assets/nve.png";

fn main() {
    // Load the Ar gas system directly from a POSCAR formatted file.
    let file = File::open("resources/test/argon.poscar").unwrap();
    let reader = BufReader::new(file);
    let mut system = load_poscar(reader);

    // Setup and apply an initial velocity distribution with a target temperature
    let boltz = Boltzmann::new(300 as f32);
    boltz.apply(&mut system);

    // Define a Lennard-Jones style pair potential.
    let lj = LennardJones::new(1.0, 3.4);
    let meta = PairPotentialMeta::new((Element::Ar, Element::Ar), 8.5, Restriction::None);

    // Initialize a collection of potentials.
    let potentials = PotentialsBuilder::new().with_pair(Box::new(lj), meta).finish();

    // Initialize a velocity Verlet style integrator.
    let velocity_verlet = VelocityVerlet::new(1.0);

    // Run MD without a thermostat to simulate the NVE ensemble.
    let md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(NullThermostat));

    // Initialize a configuration.
    let config = ConfigurationBuilder::new()
        .with_output_interval(PLOT_INTERVAL as usize)
        .with_output(Box::new(PotentialEnergy))
        .with_output_filename("nve.h5".to_string())
        .finish();

    // Run the simulation.
    let mut sim = Simulation::new(system, potentials, Box::new(md), config);
    sim.run(TIMESTEPS as usize);

    // read results file
    let file = hdf5::File::open("nve.h5").unwrap();
    let mut energies: Vec<(u64, f64)> = Vec::with_capacity((TIMESTEPS % PLOT_INTERVAL) as usize);
    for i in 0..TIMESTEPS {
        if i % PLOT_INTERVAL == 0 {
            let energy = file.dataset(&format!("{}/potential_energy", i)).unwrap();
            let energy = energy.read_1d::<f32>().unwrap();
            energies.push((i, energy[0] as f64));
        }
    }

    info!("Simulation completed successfully.");

    // Plot the results
    plot_results(energies);

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
        .build_cartesian_2d(0..TIMESTEPS, -250.59..-250.55)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Timestep")
        .x_label_style(("sans-serif", 18))
        .y_desc("Total Energy (kJ/mol)")
        .y_label_style(("sans-serif", 18))
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(data.into_iter(), &BLUE))
        .unwrap();
}
