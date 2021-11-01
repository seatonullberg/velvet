use velvet_internals::float::Float;
use velvet_system::species::Species;
use velvet_system::System;

#[derive(Clone, Debug)]
pub struct NeighborList {
    pub cutoff: Float,
    current_pairs: Vec<(usize, usize)>,
    possible_pairs: Vec<(usize, usize)>,
}

impl NeighborList {
    pub fn new(cutoff: Float) -> Self {
        let current_pairs = Vec::new();
        let possible_pairs = Vec::new();
        NeighborList {
            cutoff,
            current_pairs,
            possible_pairs,
        }
    }

    pub fn setup(&mut self, system: &System) {
        self.possible_pairs = get_all_possible_pairs(system);
        self.current_pairs = get_neighbors(self.cutoff, &self.possible_pairs, system)
    }

    pub fn setup_with_species(&mut self, species: &(Species, Species), system: &System) {
        self.possible_pairs = get_all_possible_pairs(system)
            .into_iter()
            .filter(|(i, j)| {
                let species_i = system.species[*i];
                let species_j = system.species[*j];
                let condition_a = &(species_i, species_j) == species;
                let condition_b = &(species_j, species_i) == species;
                condition_a || condition_b
            })
            .collect();
        self.current_pairs = get_neighbors(self.cutoff, &self.possible_pairs, system)
    }

    pub fn update(&mut self, system: &System) {
        self.current_pairs = get_neighbors(self.cutoff, &self.possible_pairs, system)
    }

    pub fn iter(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.current_pairs.iter()
    }
}

fn get_all_possible_pairs(system: &System) -> Vec<(usize, usize)> {
    let mut possible_indices: Vec<(usize, usize)> = Vec::with_capacity(system.n_atoms.pow(2));
    for i in 0..system.n_atoms {
        for j in (i + 1)..system.n_atoms {
            possible_indices.push((i, j));
        }
    }
    possible_indices.shrink_to_fit();
    possible_indices
}

fn get_neighbors(cutoff: Float, pairs: &[(usize, usize)], system: &System) -> Vec<(usize, usize)> {
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
