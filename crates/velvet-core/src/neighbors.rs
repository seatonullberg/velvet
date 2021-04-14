//! Data structures to improve the efficiency of evaluating short-range interactions.

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::internal::Float;
use crate::system::particle::ParticleType;
use crate::system::System;

#[derive(Clone, Debug)]
pub struct NeighborList {
    pub cutoff: Float,
    particle_types: Option<(ParticleType, ParticleType)>,
    possible_indices: Vec<(usize, usize)>,
    current_indices: Vec<(usize, usize)>,
}

impl NeighborList {
    pub fn new(
        cutoff: Float,
        particle_types: Option<(ParticleType, ParticleType)>,
    ) -> NeighborList {
        NeighborList {
            cutoff,
            particle_types,
            possible_indices: Vec::new(),
            current_indices: Vec::new(),
        }
    }

    pub fn setup(&mut self, system: &System) {
        self.possible_indices = Vec::with_capacity(system.size * system.size);
        for i in 0..system.size {
            let pt_i = system.particle_types[system.particle_type_map[i]];
            for j in (i + 1)..system.size {
                let pt_j = system.particle_types[system.particle_type_map[j]];
                match self.particle_types {
                    Some(particle_types) => {
                        if (pt_i, pt_j) == particle_types {
                            self.possible_indices.push((i, j))
                        } else if (pt_j, pt_i) == particle_types {
                            self.possible_indices.push((j, i))
                        }
                    }
                    None => self.possible_indices.push((i, j)),
                }
            }
        }
        self.possible_indices.shrink_to_fit();
    }

    // TODO: implement rayon version
    pub fn update(&mut self, system: &System) {
        self.current_indices = self
            .possible_indices
            .iter()
            .filter(|(i, j)| {
                let pos_i = system.positions[*i];
                let pos_j = system.positions[*j];
                let r = system.cell.distance(&pos_i, &pos_j);
                r < self.cutoff
            })
            .copied()
            .collect()
    }

    pub fn indices(&self) -> &Vec<(usize, usize)> {
        &self.current_indices
    }
}
