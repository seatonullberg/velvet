use velvet_internals::float::Float;
use velvet_system::species::Species;
use velvet_system::System;

pub struct NeighborList {
    cutoff: Float,
    current_pairs: Vec<(usize, usize)>,
    possible_pairs: Vec<(usize, usize)>,
}

impl NeighborList {
    pub fn new(cutoff: Float, system: &System) -> Self {
        let possible_pairs: Vec<(usize, usize)> = (0..system.n_atoms - 2)
            .into_iter()
            .zip((1..system.n_atoms - 1).into_iter())
            .collect();
        let current_pairs = get_neighbors(cutoff, possible_pairs, system);
        NeighborList {
            cutoff,
            current_pairs,
            possible_pairs,
        }
    }

    pub fn between_species(cutoff: Float, system: &System, species: (Species, Species)) -> Self {
        let possible_pairs: Vec<(usize, usize)> = (0..system.n_atoms - 2)
            .into_iter()
            .zip((1..system.n_atoms - 1).into_iter())
            .filter(|(i, j)| {
                (system.species[*i] == species.0 && system.species[*j] == species.1)
                    || (system.species[*j] == species.0 && system.species[*i] == species.1)
            })
            .collect();
        let current_pairs = get_neighbors(cutoff, possible_pairs, system);
        NeighborList {
            cutoff,
            current_pairs,
            possible_pairs,
        }
    }

    pub fn update(&mut self, system: &System) {
        self.current_pairs = get_neighbors(self.cutoff, self.possible_pairs, system)
    }

    pub fn iter(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.current_pairs.iter()
    }
}

fn get_neighbors(
    cutoff: Float,
    pairs: Vec<(usize, usize)>,
    system: &System,
) -> Vec<(usize, usize)> {
    pairs
        .iter()
        .filter(|(i, j)| {
            let pos_i = system.positions[*i];
            let pos_j = system.positions[*j];
            system.cell.distance(&pos_i, &pos_j) < cutoff
        })
        .copied()
        .collect()
}
