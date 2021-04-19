import argparse
import matplotlib.pyplot as plt
from matplotlib.ticker import FormatStrFormatter
import numpy as np


def generate_plots(filepath, output_format, output_interval, properties):
    if output_format == "raw":
        _generate_plots_raw(filepath, output_interval, properties)
    elif output_format == "hdf5":
        raise NotImplementedError("HDF5 format is not yet implemented")
    else:
        raise ValueError("unsupported output format: {}".format(output_format))


def _generate_plots_raw(filepath, output_interval, properties):
    # validate output interval
    if output_interval is None:
        raise ValueError("output interval is required to parse raw format")
    else:
        output_interval = int(output_interval)

    # read raw text file
    with open(filepath, "r") as f:
        lines = [line.strip() for line in f.readlines()]

    # parse desired properties
    data = {k: [] for k in properties}
    for line in lines:
        key, value = line.split()
        key = key.replace('"', "")
        key = key.replace(':', "")
        if key in data:
            data[key].append(float(value))

    # correct timestep for output interval
    corrected_data = {k: [] for k in data}
    for key in data:
        corrected_values = {"x": [], "y": []}
        for i, value in enumerate(data[key]):
            corrected_values["x"].append(i * output_interval)
            corrected_values["y"].append(value)
        corrected_data[key] = corrected_values
    
    # actually generate plots
    _generate_plots_inner(corrected_data)


def _generate_plots_hdf5(filepath, properties):
    pass


def _generate_plots_inner(data):
    for key in data:
        fig, ax = plt.subplots(tight_layout=True)
        x = data[key]["x"]
        y = data[key]["y"]
        ax.plot(x, y, linewidth=0.5)
        ax.set_xlabel("timesteps")
        ax.set_ylabel(key)
        ax.grid(True)
        ax.yaxis.set_major_formatter(FormatStrFormatter("%.4f"))
        fig.savefig("{}.png".format(key), dpi=300)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Plots the results of a Velvet simulation.")
    parser.add_argument("filepath", help="Path to the results file.")
    parser.add_argument("output_interval", default=None, help="Number of timesteps between outputs. Only required for raw output.")
    parser.add_argument("-pe", action="store_true", help="Plot potential energy.")
    parser.add_argument("-ke", action="store_true", help="Plot kinetic energy.")
    parser.add_argument("-etotal", action="store_true", help="Plot total energy.")
    parser.add_argument("-temp", action="store_true", help="Plot instantaneous temperature.")
    args = parser.parse_args()

    # determine output format from file extension
    output_format = "raw"
    if args.filepath.endswith("h5") or args.filepath.endswith("hdf5"):
        output_format = "hdf5"
    
    # determine desired plots from flags
    properties = []
    if args.pe:
        properties.append("potential_energy")
    if args.ke:
        properties.append("kinetic_energy")
    if args.etotal:
        properties.append("total_energy")
    if args.temp:
        properties.append("temperature")

    generate_plots(args.filepath, output_format, args.output_interval, properties)    
