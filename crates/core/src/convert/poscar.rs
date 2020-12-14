use std::convert::AsRef;
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;

use nalgebra::{Matrix3, Vector3};
use vasp_poscar::Poscar;

use crate::system::cell::Cell;
use crate::system::elements::Element;
use crate::system::{System, SystemBuilder};

pub fn load_poscar<P>(path: P) -> System
where
    P: AsRef<Path>,
{
    // load the poscar file
    let poscar = Poscar::from_path(path).unwrap();

    // system size alias
    let size = poscar.num_sites();

    // initialize the system builder
    let mut builder = SystemBuilder::new(size);

    // set system cell
    let vecs = poscar.scaled_lattice_vectors();
    let matrix: Matrix3<f32> = Matrix3::new(
        vecs[0][0] as f32,
        vecs[1][0] as f32,
        vecs[2][0] as f32,
        vecs[0][1] as f32,
        vecs[1][1] as f32,
        vecs[1][1] as f32,
        vecs[0][2] as f32,
        vecs[1][2] as f32,
        vecs[2][2] as f32,
    );
    let cell = Cell::from_matrix(matrix);
    builder.with_cell(cell);

    // set system elements if they exist
    match poscar.site_symbols() {
        Some(symbols) => {
            let elements: Vec<Element> =
                Vec::from_iter(symbols.map(|x| Element::from_str(x).unwrap()));
            builder.with_elements(elements);
        }
        None => panic!("POSCAR file is missing site symbols"),
    }

    // set the system positions
    let positions: Vec<Vector3<f32>> = Vec::from_iter(
        poscar
            .scaled_cart_positions()
            .iter()
            .map(|x| Vector3::new(x[0] as f32, x[1] as f32, x[2] as f32)),
    );
    builder.with_positions(positions);

    // set the system velocities if they exist
    match poscar.cart_velocities() {
        Some(vels) => {
            let velocities: Vec<Vector3<f32>> = Vec::from_iter(
                vels.iter()
                    .map(|x| Vector3::new(x[0] as f32, x[1] as f32, x[2] as f32)),
            );
            builder.with_velocities(velocities);
        }
        None => {}
    };

    // finish building and return the system
    builder.finish()
}

#[cfg(test)]
mod tests {
    use super::load_poscar;

    #[test]
    fn argon() {
        // TODO: replace this
        let path = format!(
            "{}/../../resources/test/argon.poscar",
            env!("CARGO_MANIFEST_DIR"),
        );
        let sys = load_poscar(path);
        assert_eq!(sys.size(), 108);
    }
}
