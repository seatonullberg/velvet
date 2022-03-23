use velvet_internals::float::Float;
use velvet_system::species::Species;
use velvet_system::System;

// Controls the neighbor search radius beyond that of the cutoff radius.
// There are much more robust ways to do this, but for now I am using this arbitrary scaling factor.
const RADIAL_SCALING_PARAMETER: Float = 1.25;

#[derive(Clone, Debug)]
pub struct NeighborList<const N: usize> {
    pub cutoff: Float,
    current_neighbors: Vec<[usize; N]>,
    possible_neighbors: Vec<[usize; N]>,
}

impl NeighborList<2> {
    pub fn new(cutoff: Float) -> Self {
        let current_neighbors = Vec::new();
        let possible_neighbors = Vec::new();
        NeighborList {
            cutoff,
            current_neighbors,
            possible_neighbors,
        }
    }

    pub fn setup(&mut self, system: &System) {
        self.possible_neighbors = get_possible_pairs(system);
        self.current_neighbors = get_current_pairs(system, &self.possible_neighbors, self.cutoff);
    }

    pub fn setup_with_species(&mut self, system: &System, species_i: Species, species_j: Species) {
        self.possible_neighbors = get_possible_pairs(system)
            .into_iter()
            .filter(|[i, j]| {
                let a = system.species[*i] == species_i && system.species[*j] == species_j;
                let b = system.species[*j] == species_i && system.species[*i] == species_j;
                a || b
            })
            .collect();
        self.current_neighbors = get_current_pairs(system, &self.possible_neighbors, self.cutoff);
    }

    pub fn update(&mut self, system: &System) {
        self.current_neighbors = get_current_pairs(system, &self.possible_neighbors, self.cutoff);
    }

    pub fn iter(&self) -> impl Iterator<Item = &[usize; 2]> {
        self.current_neighbors.iter()
    }
}

fn get_possible_pairs(system: &System) -> Vec<[usize; 2]> {
    let mut possible_indices: Vec<[usize; 2]> = Vec::with_capacity(system.n_atoms.pow(2));
    for i in 0..system.n_atoms {
        for j in (i + 1)..system.n_atoms {
            possible_indices.push([i, j]);
        }
    }
    possible_indices.shrink_to_fit();
    possible_indices
}

fn get_current_pairs(system: &System, possible_pairs: &[[usize; 2]], cutoff: Float) -> Vec<[usize; 2]> {
    possible_pairs
        .iter()
        .filter(|[i, j]| {
            let pos_i = system.positions[*i];
            let pos_j = system.positions[*j];
            system.cell.distance(&pos_i, &pos_j) < cutoff * RADIAL_SCALING_PARAMETER
        })
        .copied()
        .collect()
}
