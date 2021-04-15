# Velvet
[![Crates.io](https://img.shields.io/crates/v/velvet)](https://crates.io/crates/velvet)
![Crates.io](https://img.shields.io/crates/l/velvet)
![Crates.io](https://img.shields.io/crates/d/velvet)

Velvet is a classical atomistic simulation engine with a focus on user-friendliness and extensibility. This project is largely a learning exercise, but as development continues I hope to accomplish the following goals:

* Extensibility via user-defined plugin modules
* Optimized single CPU performace with multithreading and SIMD support
* Implement a wide variety of interatomic potentials
* Molecular dynamics, Monte Carlo, and energy minimization routines
* Visualization tools to analyze simulation results
* Support importing and exporting data in popular external formats

## Getting Started

### Prerequisites

To build Velvet you will need to have Rust's compiler and package manager installed on your machine. Instructions for most platforms can be found [here](https://www.rust-lang.org/tools/install).

* [rustc](https://doc.rust-lang.org/rustc/what-is-rustc.html) - Compiler for the Rust programming language
* [Cargo](https://doc.rust-lang.org/cargo/) - Package manager for the Rust programming language

Velvet has optional support to write HDF5 formatted results. If this is your preferred format, you will need a local installation  of `libhdf5`. The library can be installed with your package manager of choice or downloaded directly from source [here](https://www.hdfgroup.org/solutions/hdf5/).

* [The HDF Group](https://www.hdfgroup.org/) - Official HDF5 organization

### Installation

If you're interested in contributing or modifying the code for personal use you can install a local copy with the following instructions. Users who don't need to modify the code can skip ahead to the [usage](#usage) section.

1. Clone the repo.

```bash
$ git clone https://github.com/seatonullberg/velvet && cd velvet
```

2. Build in release mode.

```bash
$ cargo build --release --workspace
```

3. Check that all tests pass. Release mode is required to run the integration tests efficiently.

```bash
$ cargo test --release --workspace
```

#### Optional Features

Velvet supports a number of compile time options that can be opted into by using the `--features` flag when building with Cargo.

* `f64` - Sets the underlying storage type to a 64 bit floating point number. Default is 32 bit.
* `hdf5-output` - Enables HDF5 formatted output. Requires a local installation of `libhdf5`.
* `rayon` - Enables multithreading with [rayon](https://github.com/rayon-rs/rayon) parallel iterators.

## Usage

Velvet is designed to be easy for developers to hack on and extend. With this goal in mind, Velvet forgoes support for static configuration files or input scripts, which can limit flexibility and complicate backend logic, in favor of defining simulations directly in code using the high-level [`velvet`](https://crates.io/crates/velvet) crate. While this may sound daunting to researchers who are more familiar with mainstream atomistic simulation software, the samples in the [`examples`](./examples) directory show that this can be a rather elegant solution.

## Roadmap

Refer to the [open issues](https://github.com/seatonullberg/velvet/issues), [FEATURES.md](FEATURES.md), and [CHANGELOG.md](CHANGELOG.md) to see planned or proposed features (and bug fixes).

## FAQ

* Why is it called "Velvet"?

    * "Velvet" is a concatenation of [Velocity Verlet](https://en.wikipedia.org/wiki/Verlet_integration#Velocity_Verlet) which is a foundational algorithm in the field of molecular dynamics.

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.

## Acknowledgements

* [LAMMPS](https://github.com/lammps/lammps) - Large-scale Atomic/Molecular Massively Parallel Simulator
* [Lumol](https://github.com/lumol-org/lumol) - Universal extensible molecular simulation engine
