# Integrators

 The beating heart of any molecular dynamics simulation is its integrator.
 When we evolve our system over a number of time steps, what really happens mathematically is just an integration of the system's equations of motion.
More specifically, given that molecular dynamics is an application of classical physics, we integrate Newton's equations of motion.

In the simplest case, I'm sure we can all recall Newton's equation for the force acting on an object.

\\[
F = \frac {dp} {dt} = ma
\\]

This can be extended to a system of \\(n\\) interacting particles.

\\[
\frac {dp_i} {dt} = F_E + \sum_{i = 0}^n \sum_{j \ne i}^n F_{ij}
\\]

Where each particle \\(i\\) is subject to any external forces \\(F_E\\) and the force \\(F_{ij}\\) from particle \\(j\\) acting on particle \\(i\\).
In the context of molecular dynamics, the \\(F_{ij}\\) term is known as an interatomic potential.
[Chapter 4](../chapter_04/index.html) is dedicated to the many forms that this term can take.
For now, it is sufficient to say that we can define \\(F_{ij}\\) as a function of the distance between particles \\(i\\) and \\(j\\).

We have established that the forces acting on each particle can be calculated in terms of the particles' positions, but how can we use this to propagate the system in time? There are multiple strategies to achieve this, but the general procedure is as follows.

1) Calculate the __position__ at the next timestep from the current velocity and acceleration.
2) Calculate the __acceleration__ at the next timestep by evaluating an interatomic potential over the updated positions.
3) Calculate the __velocity__ at the next timestep from the updated accelerations.
4) Repeat for a number of timesteps.

The following sections explore Velvet's available integrators.
