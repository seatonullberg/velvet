use nalgebra::{Matrix3, Vector3};

#[derive(Clone, Debug, Default)]
pub struct Atom {
    pub symbol: String,
    pub charge: f32,
    pub mass: f32,
    pub force: Vector3<f32>,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
}

pub struct Bounds {
    pub matrix: Matrix3<f32>,
    pub periodicity: Vector3<bool>,
}

pub struct SimulationCell {
    pub atoms: Vec<Atom>,
    pub bounds: Bounds,
}

impl SimulationCell {
    /// Returns the distance between atoms at indices `a` and `b`.
    pub fn distance_between(&self, a: usize, b: usize) -> f32 {
        self.vector_between(a, b).norm()
    }

    /// Returns the unit vector pointing from the atom at index `a` to the atom at index `b`
    pub fn direction_between(&self, a: usize, b: usize) -> Vector3<f32> {
        let v = self.vector_between(a, b);
        v / v.norm()
    }

    /// Returns the vector pointing from the atom at index `a` to the atom at index `b`
    pub fn vector_between(&self, a: usize, b: usize) -> Vector3<f32> {
        let a_pos = self.wrap(a);
        let b_pos = self.wrap(b);
        let mut dist: Vector3<f32> = Vector3::zeros();

        for i in 0..3 {
            let mut a_coord = a_pos[i];
            let mut b_coord = b_pos[i];
            let mag = self.bounds.matrix.row(i).norm();
            if self.bounds.periodicity[i] {
                if a_coord > mag / 2.0 {
                    a_coord = mag - a_coord;
                }
                if b_coord > mag / 2.0 {
                    b_coord = mag - b_coord;
                }
            }
            dist[i] = a_coord - b_coord;
        }
        dist
    }

    /// Returns the position of the atom at index `a` as if it were wrapped back into the cell.
    fn wrap(&self, a: usize) -> Vector3<f32> {
        let pos = self.atoms[a].position;
        let mut res: Vector3<f32> = Vector3::zeros();

        for i in 0..3 {
            let mut coord = pos[i];
            let mag = self.bounds.matrix.row(i).norm();
            while coord > mag {
                coord -= mag;
            }
            while coord < 0.0 {
                coord += mag;
            }
            res[i] = coord;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::simcell::{Atom, Bounds, SimulationCell};
    use nalgebra::{Matrix3, Vector3};

    fn get_nonperiodic_simulation_cell() -> SimulationCell {
        let mut atoms = vec![Atom::default(), Atom::default()];
        atoms[0].position = Vector3::new(-0.25, 1.25, 0.5);
        atoms[1].position = Vector3::new(0.25, 0.75, 0.0);
        let bounds = Bounds {
            matrix: Matrix3::identity(),
            periodicity: Vector3::new(false, false, false),
        };
        SimulationCell {atoms, bounds}
    }

    fn get_periodic_simulation_cell() -> SimulationCell {
        let mut cell = get_nonperiodic_simulation_cell();
        cell.bounds.periodicity = Vector3::new(true, true, true);
        cell
    }

    #[test]
    fn nonperiodic_distance() {
        let cell = get_nonperiodic_simulation_cell();
        let res = cell.distance_between(0, 1);
        let target = 0.8660254;
        assert_eq!(res, target);
    }

    #[test]
    fn periodic_distance() {
        let cell = get_periodic_simulation_cell();
        let res = cell.distance_between(0, 1);
        let target = 0.5;
        assert_eq!(res, target);
    }
}