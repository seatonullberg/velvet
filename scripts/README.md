# Scripts

[__plot-outputs.py__](./plot-outputs.py) - Plots the results of a Velvet simulation.

#### Positional Arguments:

* `src` - Path the the source file.
* `dst` - Path to the destination file.

#### Optional Arguments

* `--output-frequency` - Number of timesteps between outputs. Only required for raw output.
* `-pe` - Flag to add potential energy to the plot.
* `-ke` - Flag to add kinetic energy to the plot.
* `-etotal` - Flag to add total energy to the plot.
* `-temp` - Flag to add instantaneous temperature to the plot.