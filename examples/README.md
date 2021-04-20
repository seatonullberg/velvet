# Examples

[__argon.rs__](./argon.rs) - Molecular dynamics simulation of argon gas in the NVE ensemble. This example uses a Lennard-Jones style pair potential to simulate the pairwise interactions between pairs of argon atoms. The Velocity Verlet algorithm is employed to integrate the equations of motion in the system. The total energy of the system over the course of the simulation is plotted in the figure below using the following commands:

```bash
$ cargo run --release --example argon
$ python scripts/plot-outputs.py argon.txt argon.png --output-frequency=100 -etotal
```

<p align="center"><img src="../assets/argon.png"></p>

[__binary-gas.rs__](./binary-gas.rs) - Molecular dynamics simulation of an argon/xenon gaseous mixture in the NVT ensemble. This example uses a Lennard-Jones style pair potential to simulate the pairwise interactions between each pair of chemical species. The Velocity Verlet algorithm is employed to integrate the equations of motion in the system. A Nose-Hoover thermostat controls the temperature of the system. The temperature of the system over the course of the simulation is plotted in the figure below using the following commands:

```bash
$ cargo run --release --example binary-gas
$ python scripts/plot-outputs.py binary-gas.txt binary-gas.png --output-frequency=100 -temp
```

<p align="center"><img src="../assets/binary-gas.png"></p>
