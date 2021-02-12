# Velvet
[![Crates.io](https://img.shields.io/crates/v/velvet)](https://crates.io/crates/velvet)
![Crates.io](https://img.shields.io/crates/l/velvet)
![Crates.io](https://img.shields.io/crates/d/velvet)

Velvet is a classical atomistic simulation engine with a focus on user-friendliness and extensibility. This project is largely a learning exercise, but as development continues I hope to accomplish the following goals:

* Extensibility via user-defined plugin modules
* Optimized single CPU performace with multithreading and SIMD support
* Implement a wide variety of interatomic potentials
* Molecular Dynamics, Monte Carlo, and Minimization routines
* Visualization tools to analyze simulation results
* Support importing and exporting data in popular external formats

## Getting Started

### Prerequisites

Velvet's standard output format is HDF5 so you will also need a local installation of `libhdf5`. The library can be installed with your package manager or downloaded directly from source [here](https://www.hdfgroup.org/solutions/hdf5/).

* [The HDF Group](https://www.hdfgroup.org/) - Official HDF5 organization

### Installation (from source)

To build Velvet you will need to have Rust's compiler and package manager installed on your machine. Instructions for most platforms can be found [here](https://www.rust-lang.org/tools/install).

* [rustc](https://doc.rust-lang.org/rustc/what-is-rustc.html) - Compiler for the Rust programming language
* [Cargo](https://doc.rust-lang.org/cargo/) - Package manager for the Rust programming language

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

The [`examples`](./examples) directory contains examples of how to use the [`velvet`](https://crates.io/crates/velvet) crate to configure simulations directly in code.
The [`scripts`](./scripts) directory contains Python scripts to visualize the results of each example.

## Roadmap

Refer to the [open issues](https://github.com/seatonullberg/velvet/issues), [FEATURES.md](FEATURES.md), and [CHANGELOG.md](CHANGELOG.md) to see planned or proposed features (and bug fixes).

## FAQ

**Q:** Why is it called "Velvet"?

**A:** "Velvet" is a concatenation of [Velocity Verlet](https://en.wikipedia.org/wiki/Verlet_integration#Velocity_Verlet), a foundational algorithm in molecular dynamics simulations.

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.

## Acknowledgements

* [Lumol](https://github.com/lumol-org/lumol) - Universal extensible molecular simulation engine
