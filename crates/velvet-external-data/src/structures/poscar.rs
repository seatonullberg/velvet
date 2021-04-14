use std::collections::HashSet;
use std::ops::Not;
use std::str::FromStr;

use nalgebra::{Matrix3, Vector3};
use velvet_core::prelude::*;

use crate::internal::Float;
use crate::structures::StructureFormat;

/// VASP's structure format.
///
/// # Examples
///
/// Construct a [`System`](velvet_core::system::System) from POSCAR formatted data.
/// ```
/// use velvet_external_data::prelude::*;
///
/// let system = Poscar.parse_system_from_reader("\
///     Cubic BN
///     3.57
///     0.0 0.5 0.5
///     0.5 0.0 0.5
///     0.5 0.5 0.0
///     B N
///     1 1
///     Direct
///     0.00 0.00 0.00
///     0.25 0.25 0.25
/// ".as_bytes());
///
/// assert_eq!(system.size, 2);
/// ```
pub struct Poscar;

impl StructureFormat for Poscar {
    fn write_str_from_system(&self, system: &System) -> &str {
        unimplemented!()
    }

    fn parse_system_from_reader<T: std::io::Read>(&self, reader: T) -> System {
        let buf = std::io::BufReader::new(reader);
        let poscar = vasp_poscar::Poscar::from_reader(buf).unwrap();

        // Alias for the system size.
        let size = poscar.num_sites();

        // Set system cell.
        let vecs = poscar.scaled_lattice_vectors();
        let matrix: Matrix3<Float> = Matrix3::new(
            vecs[0][0] as Float,
            vecs[1][0] as Float,
            vecs[2][0] as Float,
            vecs[0][1] as Float,
            vecs[1][1] as Float,
            vecs[2][1] as Float,
            vecs[0][2] as Float,
            vecs[1][2] as Float,
            vecs[2][2] as Float,
        );
        let cell = Cell::from_matrix(matrix);

        let particle_types: Vec<ParticleType> = match poscar.group_symbols() {
            Some(symbols) => symbols.fold(Vec::new(), |mut accumulator, symbol| {
                let element = Element::from_str(symbol).unwrap();
                let pt = ParticleType::from_element(element);
                accumulator.push(pt);
                accumulator
            }),
            None => panic!("Missing particle types."),
        };

        let particle_type_map: Vec<usize> = match poscar.site_symbols() {
            Some(symbols) => {
                let mut id = 0;
                let mut scanned_symbols = HashSet::new();
                symbols
                    .map(|symbol| {
                        // TODO: refactor this mess
                        if scanned_symbols.is_empty() {
                            scanned_symbols.insert(symbol);
                            id
                        } else {
                            if scanned_symbols.contains(symbol).not() {
                                scanned_symbols.insert(symbol);
                                id += 1;
                            }
                            id
                        }
                    })
                    .collect()
            }
            None => panic!("Missing particle type map."),
        };

        // Set system positions.
        let positions: Vec<Vector3<Float>> = poscar
            .scaled_cart_positions()
            .iter()
            .map(|x| Vector3::new(x[0] as Float, x[1] as Float, x[2] as Float))
            .collect();

        let velocities: Vec<Vector3<Float>> = match poscar.cart_velocities() {
            Some(vels) => vels
                .iter()
                .map(|x| Vector3::new(x[0] as Float, x[1] as Float, x[2] as Float))
                .collect(),
            None => vec![Vector3::zeros(); positions.len()],
        };

        System {
            size,
            cell,
            particle_types,
            particle_type_map,
            positions,
            velocities,
        }
    }
}
