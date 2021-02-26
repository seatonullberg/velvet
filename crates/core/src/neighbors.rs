use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::internal::Float;
use crate::system::species::Specie;
use crate::system::System;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NeighborList {
    cutoff: Float,
    species: Option<(Specie, Specie)>,
    possible_pairs: Vec<(usize, usize)>,
    current_pairs: Vec<(usize, usize)>,
}

impl NeighborList {
    pub fn new(cutoff: Float, species: Option<(Specie, Specie)>) -> NeighborList {
        NeighborList {
            cutoff,
            species,
            possible_pairs: Vec::new(),
            current_pairs: Vec::new(),
        }
    }

    pub fn setup(&mut self, system: &System) {
        self.possible_pairs = Vec::with_capacity(system.size * system.size);
        for i in 0..system.size {
            let sp_i = system.species[&system.specie_ids[i]];
            for j in (i + 1)..system.size {
                let sp_j = system.species[&system.specie_ids[j]];
                match self.species {
                    Some(species) => {
                        if (sp_i, sp_j) == species {
                            self.possible_pairs.push((i, j))
                        } else if (sp_j, sp_i) == species {
                            self.possible_pairs.push((j, i))
                        }
                    }
                    None => self.possible_pairs.push((i, j)),
                }
            }
        }
        self.possible_pairs.shrink_to_fit();
        self.update(system)
    }

    pub fn update(&mut self, system: &System) {
        self.current_pairs = self
            .possible_pairs
            .iter()
            .copied()
            .filter(|(i, j)| {
                let pos_i = system.positions[*i];
                let pos_j = system.positions[*j];
                let r = system.cell.distance(&pos_i, &pos_j);
                r < self.cutoff
            })
            .collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.current_pairs.iter()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Neighbors {
    neighbor_lists: Vec<NeighborList>,
    pairs: HashMap<(usize, usize), usize>,
}

impl Neighbors {
    pub fn setup(&mut self, system: &System) {
        self.neighbor_lists
            .iter_mut()
            .for_each(|nl| nl.setup(system));
        self.update(system)
    }

    pub fn update(&mut self, system: &System) {
        self.neighbor_lists
            .iter_mut()
            .for_each(|nl| nl.update(system));

        let pairs = &mut self.pairs;
        self.neighbor_lists
            .iter()
            .enumerate()
            .for_each(|(index, nl)| {
                nl.iter().for_each(|(i, j)| {
                    pairs.insert((*i, *j), index);
                })
            });
    }

    pub fn iter(&self) -> impl Iterator<Item = (&(usize, usize), &usize)> {
        self.pairs.iter()
    }
}

pub struct NeighborsBuilder {
    neighbor_lists: Vec<NeighborList>,
    pairs: HashMap<(usize, usize), usize>,
}

impl NeighborsBuilder {
    pub fn new() -> NeighborsBuilder {
        NeighborsBuilder {
            neighbor_lists: Vec::new(),
            pairs: HashMap::new(),
        }
    }

    pub fn with_neighbor_list(mut self, neighbor_list: NeighborList) -> NeighborsBuilder {
        self.neighbor_lists.push(neighbor_list);
        self
    }

    pub fn build(self) -> Neighbors {
        Neighbors {
            neighbor_lists: self.neighbor_lists,
            pairs: self.pairs,
        }
    }
}

impl Default for NeighborsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
