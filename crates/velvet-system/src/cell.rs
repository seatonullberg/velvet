//! Bounding box of the simulation environment.

use nalgebra::{Matrix3, Vector3};

use velvet_internals::float::Float;

/// Bounding box of the simulation environment.
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    matrix: Matrix3<Float>,
    inv_matrix: Matrix3<Float>,
}

impl Cell {
    /// Constructs a [`Cell`] from triclinic lattice parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    ///
    /// // orthorhombic cell
    /// let cell = Cell::triclinic(1.0, 2.0, 3.0, 90.0, 90.0, 90.0);
    /// assert_eq!(cell.a(), 1.0);
    /// assert_eq!(cell.b(), 2.0);
    /// assert_eq!(cell.c(), 3.0);
    /// ```
    pub fn triclinic(
        a: Float,
        b: Float,
        c: Float,
        alpha: Float,
        beta: Float,
        gamma: Float,
    ) -> Cell {
        let matrix = cell_matrix(a, b, c, alpha, beta, gamma);
        let inv_matrix = matrix.try_inverse().unwrap();
        Cell { matrix, inv_matrix }
    }

    /// Constructs a [`Cell`] from cubic lattice parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    ///
    /// let a0 = 4.0;
    /// let cell = Cell::cubic(a0);
    /// assert_eq!(cell.a(), a0);
    /// assert_eq!(cell.b(), a0);
    /// assert_eq!(cell.c(), a0);
    /// ```
    pub fn cubic(a: Float) -> Cell {
        let matrix = cell_matrix(a, a, a, 90.0, 90.0, 90.0);
        let inv_matrix = matrix.try_inverse().unwrap();
        Cell { matrix, inv_matrix }
    }

    /// Constructs a [`Cell`] from a 3x3 matrix.
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
        let mut vec: Vector3<Float> = Vector3::zeros();
        vec[0] = self.matrix[(0, 0)];
        vec[1] = self.matrix[(1, 0)];
        vec[2] = self.matrix[(2, 0)];
        vec
    }

    /// Returns the 'b' vector.
    pub fn b_vector(&self) -> Vector3<Float> {
        let mut vec: Vector3<Float> = Vector3::zeros();
        vec[0] = self.matrix[(0, 1)];
        vec[1] = self.matrix[(1, 1)];
        vec[2] = self.matrix[(2, 1)];
        vec
    }

    /// Returns the 'c' vector.
    pub fn c_vector(&self) -> Vector3<Float> {
        let mut vec: Vector3<Float> = Vector3::zeros();
        vec[0] = self.matrix[(0, 2)];
        vec[1] = self.matrix[(1, 2)];
        vec[2] = self.matrix[(2, 2)];
        vec
    }

    /// Converts a cartesian position to a fractional position.
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    /// use nalgebra::Vector3;
    /// use approx::*;
    ///
    /// let cell = Cell::cubic(4.0);
    /// let cart = Vector3::new(2.0, 2.0, 2.0);
    /// let frac = cell.fractional(&cart);
    /// assert_relative_eq!(frac[0], 0.5);
    /// assert_relative_eq!(frac[1], 0.5);
    /// assert_relative_eq!(frac[2], 0.5);
    /// ```
    pub fn fractional(&self, cartesian: &Vector3<Float>) -> Vector3<Float> {
        self.inv_matrix * cartesian
    }

    /// Converts a fractional position to a cartesian position.
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    /// use nalgebra::Vector3;
    /// use approx::*;
    ///
    /// let cell = Cell::cubic(4.0);
    /// let frac = Vector3::new(0.5, 0.5, 0.5);
    /// let cart = cell.cartesian(&frac);
    /// assert_relative_eq!(cart[0], 2.0);
    /// assert_relative_eq!(cart[1], 2.0);
    /// assert_relative_eq!(cart[2], 2.0);
    /// ```
    pub fn cartesian(&self, fractional: &Vector3<Float>) -> Vector3<Float> {
        self.matrix * fractional
    }

    /// Wraps a position vector into the cell obeying periodic boundary conditions.
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    /// use nalgebra::Vector3;
    /// use approx::*;
    ///
    /// let cell = Cell::cubic(4.0);
    /// let mut vec = Vector3::new(1.0, 5.0, 1.0);
    /// cell.wrap_vector(&mut vec);
    /// assert_relative_eq!(vec[0], 1.0, epsilon=1e-6);
    /// assert_relative_eq!(vec[1], 1.0, epsilon=1e-6);
    /// assert_relative_eq!(vec[2], 1.0, epsilon=1e-6);
    /// ```
    pub fn wrap_vector(&self, vector: &mut Vector3<Float>) {
        let mut fractional = self.fractional(vector);
        fractional[0] -= Float::floor(fractional[0]);
        fractional[1] -= Float::floor(fractional[1]);
        fractional[2] -= Float::floor(fractional[2]);
        *vector = self.cartesian(&fractional);
    }

    /// Finds the image of a position vector in the cell obeying periodic boundary conditions.
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    /// use nalgebra::Vector3;
    /// use approx::*;
    ///
    /// let cell = Cell::cubic(4.0);
    /// let mut vec = Vector3::new(1.0, 3.0, 1.0);
    /// cell.vector_image(&mut vec);
    /// assert_relative_eq!(vec[0], 1.0, epsilon=1e-6);
    /// assert_relative_eq!(vec[1], -1.0, epsilon=1e-6);
    /// assert_relative_eq!(vec[2], 1.0, epsilon=1e-6);
    /// ```
    pub fn vector_image(&self, vector: &mut Vector3<Float>) {
        let mut fractional = self.fractional(vector);
        fractional[0] -= Float::round(fractional[0]);
        fractional[1] -= Float::round(fractional[1]);
        fractional[2] -= Float::round(fractional[2]);
        *vector = self.cartesian(&fractional);
    }

    /// Returns the unit vector path between `v1` and `v2` obeying periodic boundary conditions.
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    /// use nalgebra::Vector3;
    /// use approx::*;
    ///
    /// let cell = Cell::cubic(4.0);
    /// let v1 = Vector3::new(0.0, 0.0, 0.0);
    /// let v2 = Vector3::new(1.5, 0.0, 0.0);
    /// let dir = cell.direction(&v1, &v2);
    /// assert_relative_eq!(dir[0], 1.0, epsilon=1e-6);
    /// assert_relative_eq!(dir[1], 0.0, epsilon=1e-6);
    /// assert_relative_eq!(dir[2], 0.0, epsilon=1e-6);
    /// ```
    pub fn direction(&self, v1: &Vector3<Float>, v2: &Vector3<Float>) -> Vector3<Float> {
        let mut d = v2 - v1;
        self.vector_image(&mut d);
        d.normalize()
    }

    /// Returns the distance between `v1` and `v2` obeying periodic boundary conditions.
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    /// use nalgebra::Vector3;
    /// use approx::*;
    ///
    /// let cell = Cell::cubic(4.0);
    /// let v1 = Vector3::new(0.0, 0.0, 0.0);
    /// let v2 = Vector3::new(1.5, 0.0, 0.0);
    /// let dist = cell.distance(&v1, &v2);
    /// assert_relative_eq!(dist, 1.5, epsilon=1e-6);
    /// ```
    pub fn distance(&self, v1: &Vector3<Float>, v2: &Vector3<Float>) -> Float {
        let mut d = v2 - v1;
        self.vector_image(&mut d);
        d.norm()
    }

    /// Returns the angle between `v1`, `v2` and `v3` obeying periodic boundary conditions.
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    /// use nalgebra::Vector3;
    /// use approx::*;
    ///
    /// let cell = Cell::cubic(4.0);
    /// let v1 = Vector3::new(0.0, 0.0, 0.0);
    /// let v2 = Vector3::new(1.0, 0.0, 0.0);
    /// let v3 = Vector3::new(1.5, 0.0, 0.0);
    /// let angle = cell.angle(&v1, &v2, &v3);
    /// assert_relative_eq!(angle, 3.14159, epsilon=1e-5);
    /// ```
    pub fn angle(&self, v1: &Vector3<Float>, v2: &Vector3<Float>, v3: &Vector3<Float>) -> Float {
        let mut v12 = v1 - v2;
        self.vector_image(&mut v12);
        let mut v32 = v3 - v2;
        self.vector_image(&mut v32);
        Float::acos(v12.dot(&v32) / (v12.norm() * v32.norm()))
    }

    /// Returns the dihedral angle between `v1`, `v2`, `v3`, and `v4`.
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    /// use nalgebra::Vector3;
    /// use approx::*;
    ///
    /// let cell = Cell::cubic(4.0);
    /// let v1 = Vector3::new(0.0, 0.0, 0.0);
    /// let v2 = Vector3::new(0.5, 0.0, 0.0);
    /// let v3 = Vector3::new(1.0, 0.5, 0.0);
    /// let v4 = Vector3::new(1.5, 0.5, 0.0);
    /// let angle = cell.dihedral(&v1, &v2, &v3, &v4);
    /// assert_relative_eq!(angle, 3.14159, epsilon=1e-5);
    /// ```
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

    /// Returns the total volume of the cell.
    ///
    /// # Examples
    ///
    /// ```
    /// use velvet_system::prelude::*;
    /// use approx::*;
    ///
    /// let cell = Cell::cubic(4.0);
    /// assert_relative_eq!(cell.volume(), 64.0);
    /// ```
    pub fn volume(&self) -> Float {
        (self.a_vector().cross(&self.b_vector())).dot(&self.c_vector())
    }
}

fn cell_matrix(
    a: Float,
    b: Float,
    c: Float,
    alpha: Float,
    beta: Float,
    gamma: Float,
) -> Matrix3<Float> {
    let cos_alpha = alpha.to_radians().cos();
    let cos_beta = beta.to_radians().cos();
    let (sin_gamma, cos_gamma) = gamma.to_radians().sin_cos();

    let b_x = b * cos_gamma;
    let b_y = b * sin_gamma;

    let c_x = c * cos_beta;
    let c_y = c * (cos_alpha - cos_beta * cos_gamma) / sin_gamma;
    let c_z = Float::sqrt(c * c - c_y * c_y - c_x * c_x);

    let mut matrix: Matrix3<Float> = Matrix3::zeros();
    matrix[(0, 0)] = a;
    matrix[(0, 1)] = b_x;
    matrix[(0, 2)] = c_x;
    matrix[(1, 1)] = b_y;
    matrix[(1, 2)] = c_y;
    matrix[(2, 2)] = c_z;
    matrix
}

#[cfg(test)]
mod tests {
    use super::Cell;
    use approx::*;
    use nalgebra::Vector3;
    use velvet_internals::consts::PI;
    use velvet_internals::float::Float;

    #[test]
    fn triclinic() {
        // Kyanite parameters: http://database.iem.ac.ru/mincryst/s_carta.php?KYANITE+2426
        let cell = Cell::triclinic(7.12, 7.8479, 5.5738, 89.9740, 101.1170, 106.0);
        assert_relative_eq!(cell.volume(), 293.31, epsilon = 0.01)
    }

    #[test]
    fn cubic() {
        let cell = Cell::cubic(4.0);
        assert_relative_eq!(cell.volume(), 64.0);
    }

    #[test]
    fn cartesian_to_fractional() {
        let cell = Cell::cubic(4.0);
        let fractional_position: Vector3<Float> = Vector3::from_element(0.1);
        let cartesian_position: Vector3<Float> = Vector3::from_element(0.4);
        assert_relative_eq!(fractional_position, cell.fractional(&cartesian_position));
    }

    #[test]
    fn fractional_to_cartesian() {
        let cell = Cell::cubic(4.0);
        let fractional_position: Vector3<Float> = Vector3::from_element(0.1);
        let cartesian_position: Vector3<Float> = Vector3::from_element(0.4);
        assert_relative_eq!(cartesian_position, cell.cartesian(&fractional_position));
    }

    #[test]
    fn wrap_vector() {
        let cell = Cell::cubic(4.0);
        let mut vec: Vector3<Float> = Vector3::from_element(7.0);
        cell.wrap_vector(&mut vec);
        let res: Vector3<Float> = Vector3::from_element(3.0);
        assert_relative_eq!((vec - &res).norm(), 0.0, epsilon = 1e-5)
    }

    #[test]
    fn vector_image() {
        let cell = Cell::cubic(4.0);
        let mut vec: Vector3<Float> = Vector3::from_element(7.0);
        cell.vector_image(&mut vec);
        let res: Vector3<Float> = Vector3::from_element(-1.0);
        assert_relative_eq!((vec - &res).norm(), 0.0, epsilon = 1e-5)
    }

    #[test]
    fn direction() {
        let cell = Cell::cubic(4.0);
        let v1: Vector3<Float> = Vector3::zeros();
        let v2: Vector3<Float> = Vector3::from_element(7.0);
        let res: Vector3<Float> = Vector3::from_element(-1.0 / Float::sqrt(3.0));
        assert_relative_eq!((cell.direction(&v1, &v2) - res).norm(), 0.0, epsilon = 1e-5)
    }

    #[test]
    fn distance() {
        let cell = Cell::cubic(4.0);
        let v1: Vector3<Float> = Vector3::zeros();
        let v2: Vector3<Float> = Vector3::from_element(7.0);
        let res: Float = Float::sqrt(3.0);
        assert_relative_eq!((cell.distance(&v1, &v2) - res), 0.0, epsilon = 1e-5)
    }

    #[test]
    fn angle() {
        let cell = Cell::cubic(4.0);
        let mut v1: Vector3<Float> = Vector3::zeros();
        let v2: Vector3<Float> = Vector3::zeros();
        let mut v3: Vector3<Float> = Vector3::zeros();
        v1[0] = 1.0;
        v3[1] = 1.0;
        assert_relative_eq!(cell.angle(&v1, &v2, &v3), PI / 2.0);
    }

    #[test]
    fn dihedral() {
        let cell = Cell::cubic(4.0);
        let v1: Vector3<Float> = Vector3::zeros();
        let mut v2: Vector3<Float> = Vector3::zeros();
        let mut v3: Vector3<Float> = Vector3::zeros();
        let mut v4: Vector3<Float> = Vector3::zeros();
        v2[0] = 1.0;
        v3[0] = 1.0;
        v3[1] = 1.0;
        v4[0] = 2.0;
        v4[1] = 1.0;
        assert_relative_eq!(cell.dihedral(&v1, &v2, &v3, &v4), PI)
    }
}
