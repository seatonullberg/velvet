//! Generic representation of a query of the system's indices.

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use std::marker::PhantomData;

use crate::internal::Float;
use crate::system::species::Species;
use crate::system::System;

/// Generic representation of a query of the system's indices.
pub struct Selection<SFn, SArgs, UFn, UArgs, const N: usize> {
    possible_indices: Vec<[usize; N]>,
    current_indices: Vec<[usize; N]>,
    setup_func: SFn,
    setup_args: PhantomData<SArgs>,
    update_func: UFn,
    update_args: PhantomData<UArgs>,
}

impl<SFn, SArgs, UFn, UArgs, const N: usize> Selection<SFn, SArgs, UFn, UArgs, N>
where
    SFn: Fn(&System, SArgs) -> Vec<[usize; N]>,
    UFn: Fn(&System, &[[usize; N]], UArgs) -> Vec<[usize; N]>,
{
    /// Returns a new [`Selection`] from the provided `setup` and `update` functions.
    pub fn new(setup: SFn, update: UFn) -> Selection<SFn, SArgs, UFn, UArgs, N> {
        Selection {
            possible_indices: Vec::new(),
            current_indices: Vec::new(),
            setup_func: setup,
            setup_args: PhantomData,
            update_func: update,
            update_args: PhantomData,
        }
    }

    /// Initializes the set of possible indices to search on subsequent calls to `update`.
    pub fn setup(&mut self, system: &System, args: SArgs) {
        self.possible_indices = (self.setup_func)(system, args)
    }

    /// Updates the selection.
    pub fn update(&mut self, system: &System, args: UArgs) {
        self.current_indices = (self.update_func)(system, &self.possible_indices, args)
    }

    /// Returns an iterator over the selection's current indices.
    pub fn indices(&self) -> impl Iterator<Item = &[usize; N]> {
        self.current_indices.iter()
    }

    /// Returns a parallel iterator over the selection's current indices.
    #[cfg(feature = "rayon")]
    pub fn par_indices(&self) -> impl ParallelIterator<Item = &[usize; N]> {
        self.current_indices.par_iter()
    }
}

// This function should not be used in the public API but must be exported for integration testing purposes.
#[doc(hidden)]
pub fn setup_pairs_by_species(
    system: &System,
    species: (Species, Species),
) -> Vec<[usize; 2]> {
    let mut possible_indices: Vec<[usize; 2]> = Vec::with_capacity(system.size.pow(2));
    for i in 0..system.size {
        let species_i = system.species[i];
        for j in (i + 1)..system.size {
            let species_j = system.species[j];
            if (species_i, species_j) == species {
                possible_indices.push([i, j]);
            } else if (species_j, species_i) == species {
                possible_indices.push([j, i]);
            }
        }
    }
    possible_indices.shrink_to_fit();
    possible_indices
}

// This function should not be used in the public API but must be exported for integration testing purposes.
#[doc(hidden)]
pub fn setup_pairs_with_charge(system: &System, _: ()) -> Vec<[usize; 2]> {
    let mut possible_indices: Vec<[usize; 2]> = Vec::with_capacity(system.size.pow(2));
    for i in 0..system.size {
        let species_i = system.species[i];
        for j in (i + 1)..system.size {
            let species_j = system.species[j];
            if species_i.charge().abs() > Float::EPSILON || species_j.charge().abs() > Float::EPSILON {
                possible_indices.push([i, j]);
            }
        }
    }
    possible_indices.shrink_to_fit();
    possible_indices
}

// This function should not be used in the public API but must be exported for integration testing purposes.
#[doc(hidden)]
pub fn update_pairs_by_cutoff_radius(
    system: &System,
    indices: &[[usize; 2]],
    cutoff: Float,
) -> Vec<[usize; 2]> {
    indices
        .iter()
        .filter(|[i, j]| {
            let pos_i = system.positions[*i];
            let pos_j = system.positions[*j];
            let r = system.cell.distance(&pos_i, &pos_j);
            r < cutoff
        })
        .copied()
        .collect()
}
