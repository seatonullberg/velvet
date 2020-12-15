# :alembic: Velvet
[![Crates.io](https://img.shields.io/crates/v/velvet)](https://crates.io/crates/velvet)
![Crates.io](https://img.shields.io/crates/l/velvet)
![Crates.io](https://img.shields.io/crates/d/velvet)

Velvet is a classical atomistic simulation engine with a focus on user-friendliness and portability - two features that I believe are not well represented in the current scientific software environment. This project is largely a learning exercise, but as development continues I hope to accomplish the following goals:

* Extensibility via user-defined plugin modules
* Optimized single CPU performace with multithreading and SIMD support
* Implement a wide variety of interatomic potentials
* Molecular Dynamics, Monte Carlo, and Minimization routines
* Visualization tools to analyze simulation results

## Getting Started

### Prerequisites

To build Velvet you will need to have Rust's compiler and package manager installed on your machine. Instructions for most platforms can be found [here](https://www.rust-lang.org/tools/install).

* [rustc](https://doc.rust-lang.org/rustc/what-is-rustc.html) - Compiler for the Rust programming language
* [Cargo](https://doc.rust-lang.org/cargo/) - Package manager for the Rust programming language

### Installation

1. Clone the repo
```bash
$ git clone https://github.com/seatonullberg/velvet && cd velvet
```
2. Check that all tests pass
```bash
$ cargo test --workspace
```
3. Build in release mode
```bash
$ cargo build --release --workspace
```

## Usage

The [`examples`](./examples) directory contains examples of how to use the `velvet-core` API to configure simulations directly in code.

[__nve.rs__](./examples/nve.rs) - Simulation of Ar gas in the NVE ensemble. This example uses a Lennard-Jones style pair potential to simulate the pairwise interactions between Ar atoms. The velocity Verlet algorithm is employed to integrate the equations of motion in the system. The total energy of this system is plotted at each timestep in the figure below.
<p align="center"><img src="./assets/nve.png" width="720"></p>

[__nvt.rs__](./examples/nvt.rs) - Simulation of Ar gas in the NVT ensemble. This example expands upon the NVE example by adding a Nose-Hoover style thermostat to regulate the temperature of the system. The temperature of this system is plotted at each timestep in the figure below.
<p align="center"><img src="./assets/nvt.png" width="720"></p>


## Roadmap

Refer to the [open issues](https://github.com/seatonullberg/velvet/issues) and [CHANGELOG.md](CHANGELOG.md) to see planned or proposed features (and bug fixes).

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.

## Acknowledgements

* [Lumol](https://github.com/lumol-org/lumol)
