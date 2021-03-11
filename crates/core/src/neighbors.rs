#[cfg(feature = "rayon")]
use rayon::prelude::*;

use serde::{Deserialize, Serialize};

use crate::internal::Float;
use crate::system::species::Specie;
use crate::system::System;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NeighborList {
    pub cutoff: Float,
    thickness: Float,
    species: Option<(Specie, Specie)>,
    possible_indices: Vec<(usize, usize)>,
    current_indices: Vec<(usize, usize)>,
}

impl NeighborList {
    pub fn new(cutoff: Float, thickness: Float, species: Option<(Specie, Specie)>) -> NeighborList {
        NeighborList {
            cutoff,
            thickness,
            species,
            possible_indices: Vec::new(),
            current_indices: Vec::new(),
        }
    }

    pub fn setup(&mut self, system: &System) {
        self.possible_indices = Vec::with_capacity(system.size * system.size);
        for i in 0..system.size {
            let sp_i = system.species[&system.specie_ids[i]];
            for j in (i + 1)..system.size {
                let sp_j = system.species[&system.specie_ids[j]];
                match self.species {
                    Some(species) => {
                        if (sp_i, sp_j) == species {
                            self.possible_indices.push((i, j))
                        } else if (sp_j, sp_i) == species {
                            self.possible_indices.push((j, i))
                        }
                    }
                    None => self.possible_indices.push((i, j)),
                }
            }
        }
        self.possible_indices.shrink_to_fit();
    }

    #[cfg(not(feature = "rayon"))]
    pub fn update(&mut self, system: &System) {
        self.current_indices = self
            .possible_indices
            .iter()
            .filter(|(i, j)| {
                let pos_i = system.positions[*i];
                let pos_j = system.positions[*j];
                let r = system.cell.distance(&pos_i, &pos_j);
                r < self.cutoff + self.thickness
            })
            .copied()
            .collect()
    }

    #[cfg(feature = "rayon")]
    pub fn update(&mut self, system: &System) {
        self.current_indices = self
            .possible_indices
            .par_iter()
            .filter(|(i, j)| {
                let pos_i = system.positions[*i];
                let pos_j = system.positions[*j];
                let r = system.cell.distance(&pos_i, &pos_j);
                r < self.cutoff + self.thickness
            })
            .copied()
            .collect()
    }

    pub fn indices(&self) -> &Vec<(usize, usize)> {
        &self.current_indices
    }
}
