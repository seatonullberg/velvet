use std::io::BufRead;
use std::str::FromStr;

use nalgebra::{Matrix3, Vector3};
use vasp_poscar::Poscar;

use velvet_core::prelude::*;

use crate::internal::Float;

/// Returns a [`System`](velvet_core::system::System) object initialized from POSCAR formatted data.
///
/// # Arguments
///
/// * `reader` - File object or buffer to read from.
///
/// # Examples
///
/// ```
/// use velvet_core::system::System;
/// use velvet_external_data::poscar::load_poscar;
///
/// let system = load_poscar("\
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
///     ".as_bytes());
///
/// assert_eq!(system.size(), 2);
/// ```
pub fn load_poscar<R>(reader: R) -> System
where
    R: BufRead,
{
    // Load the poscar file from a reader.
    let poscar = Poscar::from_reader(reader).unwrap();

    // Alias for the system size.
    let size = poscar.num_sites();

    let mut builder = SystemBuilder::new(size);

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
    builder = builder.with_cell(cell);

    // Set system elements if they exist.
    // Panic if they do not exist.
    //
    // TODO: change this panic to a result.
    match poscar.site_symbols() {
        Some(symbols) => {
            let elements: Vec<Element> = symbols.map(|x| Element::from_str(x).unwrap()).collect();
            builder = builder.with_elements(elements)
        }
        None => panic!("POSCAR file is missing site symbols"),
    }

    // Set system positions.
    let positions: Vec<Vector3<Float>> = poscar
        .scaled_cart_positions()
        .iter()
        .map(|x| Vector3::new(x[0] as Float, x[1] as Float, x[2] as Float))
        .collect();
    builder = builder.with_positions(positions);

    // Set system velocities if they exist.
    if let Some(vels) = poscar.cart_velocities() {
        let velocities: Vec<Vector3<Float>> = vels
            .iter()
            .map(|x| Vector3::new(x[0] as Float, x[1] as Float, x[2] as Float))
            .collect();
        builder = builder.with_velocities(velocities);
    }

    // Finish building and return the system.
    builder.build()
}
