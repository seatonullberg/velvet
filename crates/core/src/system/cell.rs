//! Bounding box of the simulation environment.

use nalgebra::{Matrix3, Vector3};
use serde::{Deserialize, Serialize};

use crate::internal::Float;

/// Matrix representation of a 3D bounding box.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
    matrix: Matrix3<Float>,
    inv_matrix: Matrix3<Float>,
}

impl Cell {
    /// Returns a new cell initialized from triclinic lattice parameters.
    ///
    /// # Arguments
    ///
    /// * `a` - Length of the `a` vector
    /// * `b` - Length of the `b` vector
    /// * `c` - Length of the `c` vector
    /// * `alpha` - Angle betwen the `b` and `c` vectors (degrees)
    /// * `beta` - Angle between the `a` and `c` vectors (degrees)
    /// * `gamma` - Angle between the `a` and `b` vectors (degrees)
    pub fn triclinic(a: Float, b: Float, c: Float, alpha: Float, beta: Float, gamma: Float) -> Cell {
        let cos_alpha = alpha.to_radians().cos();
        let cos_beta = beta.to_radians().cos();
        let (sin_gamma, cos_gamma) = gamma.to_radians().sin_cos();

        let b_x = b * cos_gamma;
        let b_y = b * sin_gamma;

        let c_x = c * cos_beta;
        let c_y = c * (cos_alpha - cos_beta * cos_gamma) / sin_gamma;
        let c_z = Float::sqrt(c * c - c_y * c_y - c_x * c_x);

        let matrix = Matrix3::new(a, b_x, c_x, 0.0, b_y, c_y, 0.0, 0.0, c_z);
        let inv_matrix = matrix.try_inverse().unwrap();

        Cell { matrix, inv_matrix }
    }

    /// Returns a new cell initialized from a 3x3 matrix.
    pub fn from_matrix(matrix: Matrix3<Float>) -> Cell {
        let inv_matrix = matrix.try_inverse().unwrap();
        Cell { matrix, inv_matrix }
    }

    /// Returns the magnitude of the 'a' vector.
    pub fn a(&self) -> Float {
        self.a_vector().norm()
    }

    /// Returns the magnitude of the 'b' vector.
    pub fn b(&self) -> Float {
        self.b_vector().norm()
    }

    /// Returns the magnitude of the 'c' vector.
    pub fn c(&self) -> Float {
        self.c_vector().norm()
    }

    /// Return the angle between 'b' and 'c' in degrees.
    pub fn alpha(&self) -> Float {
        let b = self.b_vector();
        let c = self.c_vector();
        b.angle(&c).to_degrees()
    }

    /// Returns the angle between 'a' and 'c' in degrees.
    pub fn beta(&self) -> Float {
        let a = self.a_vector();
        let c = self.c_vector();
        a.angle(&c).to_degrees()
    }

    /// Returns the angle between 'a' and 'b' in degrees.
    pub fn gamma(&self) -> Float {
        let a = self.a_vector();
        let b = self.b_vector();
        a.angle(&b).to_degrees()
    }

    /// Returns the 'a' vector.
    pub fn a_vector(&self) -> Vector3<Float> {
        Vector3::new(
            self.matrix[(0, 0)],
            self.matrix[(1, 0)],
            self.matrix[(2, 0)],
        )
    }

    /// Returns the 'b' vector.
    pub fn b_vector(&self) -> Vector3<Float> {
        Vector3::new(
            self.matrix[(0, 1)],
            self.matrix[(1, 1)],
            self.matrix[(2, 1)],
        )
    }

    /// Returns the 'c' vector.
    pub fn c_vector(&self) -> Vector3<Float> {
        Vector3::new(
            self.matrix[(0, 2)],
            self.matrix[(1, 2)],
            self.matrix[(2, 2)],
        )
    }

    /// Converts a cartesian position to a fractional position.
    pub fn fractional(&self, cartesian: &Vector3<Float>) -> Vector3<Float> {
        self.inv_matrix * cartesian
    }

    /// Converts a fractional position to a cartesian position.
    pub fn cartesian(&self, fractional: &Vector3<Float>) -> Vector3<Float> {
        self.matrix * fractional
    }

    /// Wraps a position vector into the cell obeying periodic boundary conditions.
    pub fn wrap_vector(&self, vector: &mut Vector3<Float>) {
        let mut fractional = self.fractional(vector);
        fractional[0] -= Float::floor(fractional[0]);
        fractional[1] -= Float::floor(fractional[1]);
        fractional[2] -= Float::floor(fractional[2]);
        *vector = self.cartesian(&fractional);
    }

    /// Finds the image of a position vector in the cell obeying periodic boundary conditions.
    pub fn vector_image(&self, vector: &mut Vector3<Float>) {
        let mut fractional = self.fractional(vector);
        fractional[0] -= Float::round(fractional[0]);
        fractional[1] -= Float::round(fractional[1]);
        fractional[2] -= Float::round(fractional[2]);
        *vector = self.cartesian(&fractional);
    }

    /// Returns the unit vector path between `v1` and `v2` obeying periodic boundary conditions.
    pub fn direction(&self, v1: &Vector3<Float>, v2: &Vector3<Float>) -> Vector3<Float> {
        let mut d = v2 - v1;
        self.vector_image(&mut d);
        d.normalize()
    }

    /// Returns the distance between `v1` and `v2` obeying periodic boundary conditions.
    pub fn distance(&self, v1: &Vector3<Float>, v2: &Vector3<Float>) -> Float {
        let mut d = v2 - v1;
        self.vector_image(&mut d);
        d.norm()
    }

    /// Returns the angle between `v1`, `v2` and `v3` obeying periodic boundary conditions.
    pub fn angle(&self, v1: &Vector3<Float>, v2: &Vector3<Float>, v3: &Vector3<Float>) -> Float {
        let mut v12 = v1 - v2;
        self.vector_image(&mut v12);
        let mut v32 = v3 - v2;
        self.vector_image(&mut v32);
        Float::acos(v12.dot(&v32) / (v12.norm() * v32.norm()))
    }

    /// Returns the dihedral angle between `v1`, `v2`, `v3`, and `v4`.
    pub fn dihedral(
        &self,
        v1: &Vector3<Float>,
        v2: &Vector3<Float>,
        v3: &Vector3<Float>,
        v4: &Vector3<Float>,
    ) -> Float {
        let mut v21 = v2 - v1;
        self.vector_image(&mut v21);
        let mut v32 = v3 - v2;
        self.vector_image(&mut v32);
        let mut v43 = v4 - v3;
        self.vector_image(&mut v43);

        let u = v21.cross(&v32);
        let v = v32.cross(&v43);
        Float::atan2(v32.norm() * v.dot(&v21), u.dot(&v))
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;
    use approx::*;
    use nalgebra::Vector3;
    use crate::constants::PI;
    use crate::internal::Float;

    #[test]
    fn new() {
        let cell = Cell::triclinic(3.0, 4.0, 5.0, 80.0, 90.0, 110.0);
        assert_eq!(cell.a_vector(), Vector3::new(3.0, 0.0, 0.0));
        assert_eq!(cell.b_vector()[2], 0.0);

        assert_relative_eq!(cell.a(), 3.0);
        assert_relative_eq!(cell.b(), 4.0);
        assert_relative_eq!(cell.c(), 5.0);

        assert_relative_eq!(cell.alpha(), 80.0);
        assert_relative_eq!(cell.beta(), 90.0);
        assert_relative_eq!(cell.gamma(), 110.0);
    }

    #[test]
    fn fractional_cartesian() {
        let cell = Cell::triclinic(5.0, 6.0, 3.6, 90.0, 53.0, 77.0);
        let tests = vec![Vector3::new(0.0, 10.0, 4.0), Vector3::new(-5.0, 12.0, 4.9)];

        for test in &tests {
            let res = cell.cartesian(&cell.fractional(test));
            assert_relative_eq!((test - &res).norm(), 0.0, epsilon = 1e-5);
        }
    }

    #[test]
    fn wrap_vector() {
        let cell = Cell::triclinic(3.0, 4.0, 5.0, 90.0, 90.0, 90.0);
        let mut v = Vector3::new(1.0, 1.5, 6.0);
        cell.wrap_vector(&mut v);
        let res = Vector3::new(1.0, 1.5, 1.0);
        assert_relative_eq!((v - &res).norm(), 0.0, epsilon = 1e-5);
    }

    #[test]
    fn vector_image() {
        let cell = Cell::triclinic(3.0, 4.0, 5.0, 90.0, 90.0, 90.0);
        let mut v = Vector3::new(1.0, 1.5, 6.0);
        cell.vector_image(&mut v);
        let res = Vector3::new(1.0, 1.5, 1.0);
        assert_relative_eq!((v - &res).norm(), 0.0, epsilon = 1e-5);
    }

    #[test]
    fn distance() {
        let cell = Cell::triclinic(3.0, 4.0, 5.0, 90.0, 90.0, 90.0);
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 2.0, 6.0);
        assert_relative_eq!(cell.distance(&v1, &v2), Float::sqrt(6.0));
        let cell = Cell::triclinic(1.0, 1.0, 1.0, 90.0, 90.0, 90.0);
        let v1 = Vector3::new(0.1, 0.0, 0.0);
        let v2 = Vector3::new(0.9, 0.0, 0.0);
        assert_relative_eq!(cell.distance(&v1, &v2), 0.2);
    }

    #[test]
    fn angle() {
        let cell = Cell::triclinic(3.0, 4.0, 5.0, 90.0, 90.0, 90.0);
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 0.0, 0.0);
        let c = Vector3::new(0.0, 1.0, 0.0);
        assert_relative_eq!(cell.angle(&a, &b, &c), PI / 2.0);
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 0.0, 0.0);
        let c = Vector3::new(Float::cos(1.877), Float::sin(1.877), 0.0);
        assert_relative_eq!(cell.angle(&a, &b, &c), 1.877);
    }

    #[test]
    fn direction() {
        let cell = Cell::triclinic(1.0, 1.0, 1.0, 90.0, 90.0, 90.0);
        let v1 = Vector3::new(0.5, 0.5, 0.5);
        let v2 = Vector3::new(0.5, 0.5, 1.1);
        let res = cell.direction(&v1, &v2);
        assert_relative_eq!(res[0], 0.0, epsilon = 1e-5);
        assert_relative_eq!(res[1], 0.0, epsilon = 1e-5);
        assert_relative_eq!(res[2], -1.0);
    }

    #[test]
    fn dihedral() {
        let cell = Cell::triclinic(3.0, 4.0, 5.0, 90.0, 90.0, 90.0);
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 0.0, 0.0);
        let v3 = Vector3::new(1.0, 1.0, 0.0);
        let v4 = Vector3::new(2.0, 1.0, 0.0);
        assert_relative_eq!(cell.dihedral(&v1, &v2, &v3, &v4), PI, epsilon=1e-6);
        let v1 = Vector3::new(1.241, 0.444, 0.349);
        let v2 = Vector3::new(-0.011, -0.441, 0.333);
        let v3 = Vector3::new(-1.176, 0.296, -0.332);
        let v4 = Vector3::new(-1.396, 1.211, 0.219);
        assert_relative_eq!(cell.dihedral(&v1, &v2, &v3, &v4), -1.045379, epsilon=1e-6);
    }
}
