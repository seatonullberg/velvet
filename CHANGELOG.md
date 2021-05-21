# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* Optional `f64` storage type.
* Neighbor list optimization.
* Custom `ParticleType`s.
* Grouped outputs for raw and HDF5 formatted data.
* Generic `Selection`s of atoms.
* `CoulombPotential` trait with a `StandardCoulombic` implementation.
* `Buckingham` pair potential.
* MgO example.
* Progress bar.

### Changed

* Improved flexibility of the example visualization script with support for command line arguments.

### Removed

* `serde` serialization and deserialization.

## [0.4.2] - 2021-02-12

### Added

* Optional support for multithreading with the `rayon` feature.

## [0.4.1] - 2021-02-12

### Changed

* Make HDF5 an optional dependency with the `hdf5-output` feature.

## [0.4.0] - 2021-02-10

### Changed

* Examples are now post-processed with separate Python scripts.
* Renamed `velvet-convert` crate to `velvet-external-data`.

## [0.3.2] - 2020-12-19

### Fixed

* Actually fixed `docs.rs` documentation build failure.

## [0.3.1] - 2020-12-19

### Fixed

* `docs.rs` documentation build failure.

## [0.3.0] - 2020-12-18

### Fixed

* Bug in loading `Cell` from POSCAR data.

### Added

* Additional benchmark tests.
* HDF5 output support.
* Round-trip serialization for `Simulation`.
* User defined `Configuration`.
* `Propagator` interface.
* `prelude` module.
* CLI tool to convert POSCAR data to the internal data format.

## [0.2.1] - 2020-12-15

### Fixed

* crates.io repository link and documentation.

## [0.2.0] - 2020-12-15

### Changed

* Cargo workspace layout.

### Added

* Benchmark test suite.
* NVE and NVT examples.
* Berendsen and Nose-Hoover thermostats.
* Boltzmann initial velocity distribution.
* LennardJones, Mie, Morse, and Harmonic interatomic pair potentials.
* Velocity Verlet integration algorithm.
* System property for instantaneous temperature.
* System properties for potential, kinetic, and total energy.
* System property for force acting on every atom.

## [0.1.0] - 2020-07-19

Initial release only available on [crates.io](https://crates.io/crates/velvet/0.1.0).
