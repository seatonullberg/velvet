import h5py
import matplotlib.pyplot as plt
import numpy as np

from matplotlib.ticker import FormatStrFormatter

TIMESTEPS = 100000
OUTPUT_INTERVAL = 50
OUTPUT_FILENAME = "nvt.h5"

if __name__ == "__main__":
    h5file = h5py.File(OUTPUT_FILENAME)
    logged_steps = [i for i in range(TIMESTEPS) if i % OUTPUT_INTERVAL == 0 or i == TIMESTEPS - 1]
    energies = [h5file["{}".format(i)]["temperature"][0] for i in logged_steps]
    
    fig, ax = plt.subplots()
    ax.plot(logged_steps, energies, linewidth=0.5)
    ax.set_xlim((0, TIMESTEPS))

    ax.set_xlabel("Timesteps")
    ax.set_ylabel("Temperature (K)")
    ax.yaxis.set_major_formatter(FormatStrFormatter("%.2f"))

    plt.grid(True)
    plt.tight_layout()
    plt.savefig("nvt.png", dpi=300)
