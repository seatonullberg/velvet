import argparse

import matplotlib.pyplot as plt
from matplotlib.ticker import FormatStrFormatter
import numpy as np

LABELS = {
    "potential_energy": "Potential Energy (kcal/mol)",
    "kinetic_energy": "Kinetic Energy (kcal/mol)",
    "total_energy": "Total Energy (kcal/mol)",
    "temperature": "Temperature (Kelvin)"
}

def generate_plots(args, fmt, properties):
    if fmt == "raw":
        _generate_plots_raw(args, properties)
    elif fmt == "hdf5":
        raise NotImplementedError("HDF5 format is not yet implemented")
    else:
        raise ValueError("unsupported format: {}".format(fmt))


def _generate_plots_raw(args, properties):
    # validate output frequency
    freq = args.output_frequency
    if freq is None:
        raise ValueError("output frequency is required to parse 'raw' formatted data")
    else:
        freq = int(freq)

    # read raw text file
    with open(args.src, "r") as f:
        lines = [line.strip() for line in f.readlines()]

    # parse desired properties
    data = {k: [] for k in properties}
    for line in lines:
        key, value = line.split()
        key = key.replace('"', "")
        key = key.replace(':', "")
        if key in data:
            data[key].append(float(value))

    # format data for plotting
    formatted_data = {k: [] for k in data}
    for key in data:
        formatted_values = {"x": [], "y": []}
        for i, value in enumerate(data[key]):
            formatted_values["x"].append(i * freq)
            formatted_values["y"].append(value)
        formatted_data[key] = formatted_values

    # generate plots
    _generate_plots_inner(args, formatted_data)


def _generate_plots_hdf5(args, properties):
    pass


def _generate_plots_inner(args, data):
    fig, ax = plt.subplots(nrows=len(data), tight_layout=True, sharex=True)
    for i, key in enumerate(data):
        if len(data) > 1:
            _ax = ax[i]
        else:
            _ax = ax
        _ax.plot(data[key]["x"], data[key]["y"], linewidth=0.5)
        _ax.set_ylabel(LABELS[key])
        _ax.grid(True)
        _ax.yaxis.set_major_formatter(FormatStrFormatter("%.1f"))
    plt.xlabel("Iteration")
    plt.savefig(args.dst, dpi=300)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Plots the results of a Velvet simulation.")
    parser.add_argument("src", help="Path to the source file.")
    parser.add_argument("dst", help="Path to the destination file.")
    parser.add_argument("--output-frequency", default=None, help="Number of timesteps between outputs. Only required for raw output.")
    parser.add_argument("-pe", action="store_true", help="Plot potential energy.")
    parser.add_argument("-ke", action="store_true", help="Plot kinetic energy.")
    parser.add_argument("-etotal", action="store_true", help="Plot total energy.")
    parser.add_argument("-temp", action="store_true", help="Plot instantaneous temperature.")
    args = parser.parse_args()

    # determine source format from file extension
    fmt = "raw"
    if args.src.endswith("h5") or args.src.endswith("hdf5"):
        fmt = "hdf5"
    
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

    generate_plots(args, fmt, properties)    
