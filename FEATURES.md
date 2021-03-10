# Features

All of the project's completed and proposed features will be documented in this file.

## Table of Contents

* [Computed Properties](#computed-properties)
* [Data Formats](#data-formats)
  * [Inputs](#data-formats-inputs)
  * [Outputs](#data-formats-outputs)
* [Integration Algorithms](#integration-algorithms)
* [Potentials](#potentials)
* [Propogation Techniques](#propogation-techniques)
* [Runtime Performance](#runtime-performance)
* [Temperature Initialization](#temperature-initialization)
* [Thermostats](#thermostats)


## Computed Properties <a name="computed-properties">

- [x] **Forces** - Force acting on each atom in the system.
- [x] **Potential Energy** - Total potential energy of the system.
- [x] **Kinetic Energy** - Total kinetic energy in the system.
- [x] **Total Energy** - Summation of potential and kinetic energy in the system.
- [x] **Temperature** - Instantaneous temperature of the system.
- [ ] **Stress Tensor** - 3x3 tensor defining the system's stress state.


## Data Formats <a name="data-formats">

### Inputs <a name="data-formats-inputs">

- [x] **POSCAR** - Load internal system representation from [VASP](https://www.vasp.at/wiki/index.php/POSCAR)'s structure file format.
- [ ] **CIF** - Load internal system representation from a [crystallographic information file](https://en.wikipedia.org/wiki/Crystallographic_Information_File).
- [ ] **LAMMPS** - Load internal system representation from [LAMMPS](https://lammps.sandia.gov/doc/2001/data_format.html)'s data file format.
- [ ] **PDB** - Load internal system representation from a [protein data bank file](https://www.cgl.ucsf.edu/chimera/docs/UsersGuide/tutorials/pdbintro.html).

### Outputs <a name="data-formats-outputs">

- [x] **HDF5** - Write results in [HDF5](https://www.hdfgroup.org/solutions/hdf5/) format with (optional) `libhdf5` dependency.
- [ ] **CBOR** - Serialize simulation configurations as binary [CBOR](https://cbor.io/) data.

## Integration Algorithms <a name="integration-algorithms">

- [x] **Velocity Verlet** - [Velocity Verlet](https://en.wikipedia.org/wiki/Verlet_integration#Velocity_Verlet) style integration algorithm.
- [ ] **Leapfrog** - [Leapfrog](https://en.wikipedia.org/wiki/Leapfrog_integration) numerical integration technique.
- [ ] **Verlet** - [Verlet](https://en.wikipedia.org/wiki/Verlet_integration) (without velocity) style integration algorithm.

## Potentials <a name="potentials">

- [x] **Lennard-Jones** - [Lennard-Jones](https://en.wikipedia.org/wiki/Lennard-Jones_potential) (12,6) style pairwise interatomic potential.
    - [x] Pair interactions 
- [x] **Harmonic** - [Harmonic](https://en.wikipedia.org/wiki/Harmonic_oscillator) oscillator style pairwise interatomic potential.
    - [x] Pair interactions
    - [ ] Angle interactions
    - [ ] Bond interactions
    - [ ] Dihedral interactions
- [x] **Mie** - [Mie](https://lammps.sandia.gov/doc/pair_mie.html) (1903) style pairwise interatomic potential.
    - [x] Pair interactions 
- [x] **Morse** - [Morse](https://en.wikipedia.org/wiki/Morse_potential) (1929) style pairwise interatomic potential.
    - [x] Pair interactions
- [ ] **Wolf Summation** - [Wolf](https://en.wikipedia.org/wiki/Wolf_summation) (1999) computationally efficient summation method for electroatatic interactions. 
    - [ ] Coulomb interactions
- [ ] **Cosine** - [Cosine](https://lammps.sandia.gov/doc/angle_cosine.html) angle potential.
    - [ ] Angle interactions 

## Propogation Techniques <a name="propogation-techniques">

- [x] **Molecular Dynamics** - Timestep integration based propagation.
- [ ] **Monte Carlo** - Stochastic movement based propagation.
- [ ] **Energy Minimization** - Numerical minimization of the system's energy to optimize positions and/or system size.

## Runtime Performance <a name="runtime-performance">

- [x] **Neighbor Lists** - [Neighbor list](https://en.wikipedia.org/wiki/Verlet_list) buffering of nonbonded interactions.
- [x] **Multithreading** - Thread parallelism via [`rayon`](https://github.com/rayon-rs/rayon) parallel iterators (optional).
- [ ] **SIMD** - Multiple dispatch of single instructions.

## Temperature Initialization <a name="temperature-initialization">

- [x] **Boltzmann Distribution** - Initialize the system's velocities to fit a [Boltzmann distribution](https://en.wikipedia.org/wiki/Boltzmann_distribution).
- [ ] **Uniform Distribution** - Initialize the system's velocities to fit a [uniform distribution](https://en.wikipedia.org/wiki/Continuous_uniform_distribution).

## Thermostats <a name="thermostats">

- [ ] **Andersen** - [Andersen](http://www.sklogwiki.org/SklogWiki/index.php/Andersen_thermostat) (1980) Boltzmann statistics based velocity reassignment thermostat.
- [x] **Berendsen** - [Berendsen](https://en.wikipedia.org/wiki/Berendsen_thermostat) (1984) velocity rescale thermostat.
- [x] **Nose-Hoover** - [Nose-Hoover](https://en.wikipedia.org/wiki/Nos%C3%A9%E2%80%93Hoover_thermostat) (1984) deterministic thermostat.
