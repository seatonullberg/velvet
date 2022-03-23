use chemfiles::{Frame, Trajectory};
use nalgebra::{Matrix3, Vector3};
use std::path::Path;
use velvet_internals::float::Float;
use velvet_system::cell::Cell;
use velvet_system::species::Species;
use velvet_system::System;

/// Shared behavior to construct an object from a chemfiles supported file format.
/// The list of available formats can be found [here](https://chemfiles.org/chemfiles/latest/formats.html#list-of-supported-formats).
pub trait FromChemfile {
    /// Constructs an object from a file with automatic format detection.
    fn from_chemfile<P>(path: P) -> Self
    where
        P: AsRef<Path>;

    /// Constructs an object from a file with manual format specification.
    fn from_chemfile_with_format<'a, P, S>(path: P, format: S) -> Self
    where
        P: AsRef<Path>,
        S: Into<&'a str>;
}

impl FromChemfile for System {
    fn from_chemfile<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let mut trajectory = Trajectory::open(path, 'r').unwrap();
        new_system_from_trajectory(&mut trajectory)
    }

    fn from_chemfile_with_format<'a, P, S>(path: P, format: S) -> Self
    where
        P: AsRef<Path>,
        S: Into<&'a str>,
    {
        let mut trajectory = Trajectory::open_with_format(path, 'r', format).unwrap();
        new_system_from_trajectory(&mut trajectory)
    }
}

fn new_system_from_trajectory(trajectory: &mut Trajectory) -> System {
    // Initialize an empty frame to read the trajectory into.
    let mut frame = Frame::new();
    trajectory.read(&mut frame).unwrap();

    // Process the frame's properties.
    let n_atoms = frame.size();
    let positions: Vec<Vector3<Float>> = get_rows(frame.positions());
    let velocities: Vec<Vector3<Float>>;
    if frame.has_velocities() {
        velocities = get_rows(frame.velocities());
    } else {
        velocities = vec![Vector3::zeros(); n_atoms];
    }

    // Create a Species for each atom in the frame.
    let species: Vec<Species> = Vec::from_iter((0..n_atoms).into_iter().map(|i| {
        let atom = frame.atom(i);
        Species::new(
            atom.atomic_type(),
            atom.mass() as Float,
            atom.charge() as Float,
        )
    }));

    // Extract a topology from the frame and process bonding information.
    let topology = frame.topology();
    let bonds = topology.bonds();
    let angles = topology.angles();
    let dihedrals = topology.dihedrals();

    // Process the bounds of the simulation cell.
    let cell = Cell::from_matrix(get_matrix(&frame.cell().matrix()));

    // Return the initialized System.
    System {
        n_atoms,
        cell,
        species,
        positions,
        velocities,
        bonds,
        angles,
        dihedrals,
    }
}

fn get_rows(data: &[[f64; 3]]) -> Vec<Vector3<Float>> {
    data.iter()
        .map(|[x, y, z]| {
            let mut vec: Vector3<Float> = Vector3::zeros();
            vec[0] = *x as Float;
            vec[1] = *y as Float;
            vec[2] = *z as Float;
            vec
        })
        .collect()
}

fn get_matrix(data: &[[f64; 3]; 3]) -> Matrix3<Float> {
    let mut matrix: Matrix3<Float> = Matrix3::zeros();
    matrix[(0, 0)] = data[0][0] as Float;
    matrix[(0, 1)] = data[0][1] as Float;
    matrix[(0, 2)] = data[0][2] as Float;
    matrix[(1, 0)] = data[1][0] as Float;
    matrix[(1, 1)] = data[1][1] as Float;
    matrix[(1, 2)] = data[1][2] as Float;
    matrix[(2, 0)] = data[2][0] as Float;
    matrix[(2, 1)] = data[2][1] as Float;
    matrix[(2, 2)] = data[2][2] as Float;
    matrix
}

#[cfg(test)]
mod tests {
    use crate::FromChemfile;
    use approx::*;
    use velvet_system::elements::Element;
    use velvet_system::species::Species;
    use velvet_system::System;
    use velvet_test_utils::resources_path;

    #[test]
    fn argon() {
        let system = System::from_chemfile(resources_path("argon.xyz"));
        assert_eq!(system.n_atoms, 1000);
        assert_relative_eq!(system.cell.a(), 100.0, epsilon = 1e-5);
        assert_relative_eq!(system.cell.b(), 100.0, epsilon = 1e-5);
        assert_relative_eq!(system.cell.c(), 100.0, epsilon = 1e-5);
        let argon = Species::from_element(&Element::Ar);
        system
            .species
            .into_iter()
            .for_each(|species| assert_eq!(species, argon));
    }

    #[test]
    fn argon_xenon() {
        let system = System::from_chemfile(resources_path("argon-xenon.xyz"));
        assert_eq!(system.n_atoms, 2000);
        assert_relative_eq!(system.cell.a(), 50.0, epsilon = 1e-5);
        assert_relative_eq!(system.cell.b(), 50.0, epsilon = 1e-5);
        assert_relative_eq!(system.cell.c(), 50.0, epsilon = 1e-5);
        let argon = Species::from_element(&Element::Ar);
        let xenon = Species::from_element(&Element::Xe);
        (0..1000)
            .into_iter()
            .for_each(|i| assert_eq!(system.species[i], argon));
        (1000..2000)
            .into_iter()
            .for_each(|i| assert_eq!(system.species[i], xenon));
    }

    #[test]
    fn zirconia() {
        let system = System::from_chemfile(resources_path("zirconia.cif"));
        assert_eq!(system.n_atoms, 750);
        assert_relative_eq!(system.cell.a(), 18.004, epsilon = 1e-5);
        assert_relative_eq!(system.cell.b(), 18.004, epsilon = 1e-5);
        assert_relative_eq!(system.cell.c(), 25.896499, epsilon = 1e-5);
        let oxygen = Species::from_element(&Element::O);
        let zirconium = Species::from_element(&Element::Zr);
        (0..500)
            .into_iter()
            .for_each(|i| assert_eq!(system.species[i], oxygen));
        (500..750)
            .into_iter()
            .for_each(|i| assert_eq!(system.species[i], zirconium));
    }
}
