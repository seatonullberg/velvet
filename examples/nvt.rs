//! This example executes a simulation of 108 Ar atoms in the NVT ensemble.
//! After sucessful completion, a figure is generated at `nvt.png` which plots the total energy at each timestep.

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use plotters::prelude::*;

use std::fs::File;
use std::io::BufReader;

use velvet::prelude::*;

static TIMESTEPS: u64 = 250000;
static PLOT_INTERVAL: u64 = 50;
static FILENAME: &'static str = "assets/nvt.png";

fn main() {
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
    let meta = PairPotentialMeta::new((Element::Ar, Element::Ar), 8.5, Restriction::None);

    // Initialize a collection of potentials.
    let potentials = PotentialsBuilder::new().with_pair(Box::new(lj), meta).finish();

    // Initialize a velocity Verlet style integrator.
    let velocity_verlet = VelocityVerlet::new(1.0);

    // Initialize a Nose-Hoover thermostat.
    let nose_hoover = NoseHoover::new(300 as f32, 1.5, 1.0);

    // Run MD with a thermostst to simulate the NVT ensemble.
    let md = MolecularDynamics::new(Box::new(velocity_verlet), Box::new(nose_hoover));

    // Initialize a configuration.
    let config = ConfigurationBuilder::new()
        .with_output_interval(PLOT_INTERVAL as usize)
        .with_output(Box::new(Temperature))
        .with_output_filename("nvt.h5".to_string())
        .finish();

    // Run the simulation
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
