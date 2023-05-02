//! Bounding box of the simulation environment.

use crate::errors::SystemInitializationError;
use nalgebra::{Matrix3, Vector3};

/// Bounding box of the simulation environment.
#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    matrix: Matrix3<f64>,
    inv_matrix: Matrix3<f64>,
}

fn cell_matrix(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> Matrix3<f64> {
    let cos_alpha = alpha.to_radians().cos();
    let cos_beta = beta.to_radians().cos();
    let (sin_gamma, cos_gamma) = gamma.to_radians().sin_cos();

    let b_x = b * cos_gamma;
    let b_y = b * sin_gamma;

    let c_x = c * cos_beta;
    let c_y = c * (cos_alpha - cos_beta * cos_gamma) / sin_gamma;
    let c_z = f64::sqrt(c * c - c_y * c_y - c_x * c_x);

    let mut matrix: Matrix3<f64> = Matrix3::zeros();
    matrix[(0, 0)] = a;
    matrix[(0, 1)] = b_x;
    matrix[(0, 2)] = c_x;
    matrix[(1, 1)] = b_y;
    matrix[(1, 2)] = c_y;
    matrix[(2, 2)] = c_z;
    matrix
}

impl TryFrom<Matrix3<f64>> for Cell {
    type Error = SystemInitializationError;

    fn try_from(value: Matrix3<f64>) -> Result<Self, Self::Error> {
        let matrix = value;
        match matrix.try_inverse() {
            None => {
                let err = Self::Error::InvalidCellMatrix(matrix);
                Err(err)
            }
            Some(inv_matrix) => Ok(Cell { matrix, inv_matrix }),
        }
    }
}

impl Cell {
    /// Constructs a cell from triclinic lattice parameters.
    /// The angle parameters must be given in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::assert_relative_eq;
    /// # use velvet::errors::SystemInitializationError;
    /// use velvet::system::cell::Cell;
    ///
    /// // Turquoise unit cell from AMCSD: http://rruff.geo.arizona.edu/AMS/minerals/Turquoise.
    /// let cell = Cell::triclinic(
    ///     7.424, 7.629, 9.910,
    ///     68.61, 69.71, 65.05,
    /// )?;
    /// assert_relative_eq!(cell.volume(), 461.3, epsilon = 0.1);
    /// # Ok::<(), SystemInitializationError>(())
    /// ```
    ///
    /// # Errors
    /// - [`SystemInitializationError::InvalidCellMatrix`] if the given lattice parameters produce a
    /// [non-invertable matrix](https://en.wikipedia.org/wiki/Invertible_matrix).
    pub fn triclinic(
        a: f64,
        b: f64,
        c: f64,
        alpha: f64,
        beta: f64,
        gamma: f64,
    ) -> Result<Cell, SystemInitializationError> {
        let matrix = cell_matrix(a, b, c, alpha, beta, gamma);
        Cell::try_from(matrix)
    }

    /// Constructs a cell from orthorhombic lattice parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::assert_relative_eq;
    /// use velvet::system::cell::Cell;
    ///
    /// // Olivine unit cell from AMCSD: http://rruff.geo.arizona.edu/AMS/minerals/Olivine.
    /// let cell = Cell::orthorhombic(4.779, 10.277, 5.995);
    /// assert_relative_eq!(cell.volume(), 294.4, epsilon = 0.1);
    /// ```
    pub fn orthorhombic(a: f64, b: f64, c: f64) -> Cell {
        let matrix = cell_matrix(a, b, c, 90.0, 90.0, 90.0);
        // Orthorhombic lattice parameters will always produce an invertable matrix so `unwrap` will never panic here.
        Cell::try_from(matrix).unwrap()
    }

    /// Constructs a cell from cubic lattice parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::assert_relative_eq;
    /// use velvet::system::cell::Cell;
    ///
    /// // Iron unit cell from AMCSD: http://rruff.geo.arizona.edu/AMS/minerals/Iron.
    /// let cell = Cell::cubic(2.866);
    /// assert_relative_eq!(cell.volume(), 23.5, epsilon = 0.1);
    /// ```
    pub fn cubic(a: f64) -> Cell {
        let matrix = cell_matrix(a, a, a, 90.0, 90.0, 90.0);
        // Cubic lattice parameters will always produce an invertable matrix so `unwrap` will never panic here.
        Cell::try_from(matrix).unwrap()
    }

    /// Returns the magnitude of the 'a' vector.
    pub fn a(&self) -> f64 {
        self.a_vector().norm()
    }

    /// Returns the magnitude of the 'b' vector.
    pub fn b(&self) -> f64 {
        self.b_vector().norm()
    }

    /// Returns the magnitude of the 'c' vector.
    pub fn c(&self) -> f64 {
        self.c_vector().norm()
    }

    /// Return the angle between 'b' and 'c' in degrees.
    pub fn alpha(&self) -> f64 {
        let b = self.b_vector();
        let c = self.c_vector();
        b.angle(&c).to_degrees()
    }

    /// Returns the angle between 'a' and 'c' in degrees.
    pub fn beta(&self) -> f64 {
        let a = self.a_vector();
        let c = self.c_vector();
        a.angle(&c).to_degrees()
    }

    /// Returns the angle between 'a' and 'b' in degrees.
    pub fn gamma(&self) -> f64 {
        let a = self.a_vector();
        let b = self.b_vector();
        a.angle(&b).to_degrees()
    }

    /// Returns the 'a' vector.
    pub fn a_vector(&self) -> Vector3<f64> {
        let mut vec: Vector3<f64> = Vector3::zeros();
        vec[0] = self.matrix[(0, 0)];
        vec[1] = self.matrix[(1, 0)];
        vec[2] = self.matrix[(2, 0)];
        vec
    }

    /// Returns the 'b' vector.
    pub fn b_vector(&self) -> Vector3<f64> {
        let mut vec: Vector3<f64> = Vector3::zeros();
        vec[0] = self.matrix[(0, 1)];
        vec[1] = self.matrix[(1, 1)];
        vec[2] = self.matrix[(2, 1)];
        vec
    }

    /// Returns the 'c' vector.
    pub fn c_vector(&self) -> Vector3<f64> {
        let mut vec: Vector3<f64> = Vector3::zeros();
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
    /// use nalgebra::Vector3;
    /// use velvet::system::cell::Cell;
    ///
    /// let cell = Cell::cubic(5.0);
    /// let cartesian = Vector3::new(0.0, 2.5, 0.0);
    /// let fractional = cell.fractional(&cartesian);
    /// assert_eq!(fractional[1], 0.5);
    /// ```
    pub fn fractional(&self, cartesian: &Vector3<f64>) -> Vector3<f64> {
        self.inv_matrix * cartesian
    }

    /// Converts a fractional position to a cartesian position.
    ///
    /// # Examples
    ///
    /// ```
    /// use nalgebra::Vector3;
    /// use velvet::system::cell::Cell;
    ///
    /// let cell = Cell::cubic(5.0);
    /// let fractional = Vector3::new(0.0, 0.5, 0.0);
    /// let cartesian = cell.cartesian(&fractional);
    /// assert_eq!(cartesian[1], 2.5);
    /// ```
    pub fn cartesian(&self, fractional: &Vector3<f64>) -> Vector3<f64> {
        self.matrix * fractional
    }

    /// Wraps a position vector into the cell obeying periodic boundary conditions.
    /// `vector` must be given in cartesian coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::assert_relative_eq;
    /// use nalgebra::Vector3;
    /// use velvet::system::cell::Cell;
    ///
    /// let cell = Cell::cubic(5.0);
    /// let mut v = Vector3::new(7.0, 0.0, 0.0);
    /// cell.wrap_vector(&mut v);
    /// assert_relative_eq!(v[0], 2.0, epsilon = 1e-5);
    /// ```
    pub fn wrap_vector(&self, vector: &mut Vector3<f64>) {
        let mut fractional = self.fractional(vector);
        fractional[0] -= f64::floor(fractional[0]);
        fractional[1] -= f64::floor(fractional[1]);
        fractional[2] -= f64::floor(fractional[2]);
        *vector = self.cartesian(&fractional);
    }

    /// Finds the image of a position vector in the cell obeying periodic boundary conditions.
    /// `vector` must be given in cartesian coordinates.
    ///  
    /// # Examples
    ///
    /// ```
    /// # use approx::assert_relative_eq;
    /// use nalgebra::Vector3;
    /// use velvet::system::cell::Cell;
    ///
    /// let cell = Cell::cubic(5.0);
    /// let mut v = Vector3::new(4.0, 0.0, 0.0);
    /// cell.vector_image(&mut v);
    /// assert_relative_eq!(v[0], -1.0, epsilon = 1e-5);
    /// ```
    pub fn vector_image(&self, vector: &mut Vector3<f64>) {
        let mut fractional = self.fractional(vector);
        fractional[0] -= f64::round(fractional[0]);
        fractional[1] -= f64::round(fractional[1]);
        fractional[2] -= f64::round(fractional[2]);
        *vector = self.cartesian(&fractional);
    }

    /// Returns the unit vector path between `v1` and `v2` obeying periodic boundary conditions.
    /// `v1` and `v2` must be given in cartesian coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::assert_relative_eq;
    /// use nalgebra::Vector3;
    /// use velvet::system::cell::Cell;
    ///
    /// let cell = Cell::cubic(5.0);
    /// let v1 = Vector3::new(1.0, 2.0, 0.0);
    /// let v2 = Vector3::zeros();
    /// let dir = cell.direction(&v1, &v2);
    /// assert_relative_eq!(dir[0], -0.44721, epsilon = 1e-5);
    /// assert_relative_eq!(dir[1], -0.89443, epsilon = 1e-5);
    /// ```
    pub fn direction(&self, v1: &Vector3<f64>, v2: &Vector3<f64>) -> Vector3<f64> {
        let mut d = v2 - v1;
        self.vector_image(&mut d);
        d.normalize()
    }

    /// Returns the distance between `v1` and `v2` obeying periodic boundary conditions.
    /// `v1` and `v2` must be given in cartesian coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::assert_relative_eq;
    /// use nalgebra::Vector3;
    /// use velvet::system::cell::Cell;
    ///
    /// let cell = Cell::cubic(5.0);
    /// let v1 = Vector3::new(1.0, 2.0, 0.0);
    /// let v2 = Vector3::zeros();
    /// let dist = cell.distance(&v1, &v2);
    /// assert_relative_eq!(dist, 2.23606, epsilon = 1e-5);
    /// ```
    pub fn distance(&self, v1: &Vector3<f64>, v2: &Vector3<f64>) -> f64 {
        let mut d = v2 - v1;
        self.vector_image(&mut d);
        d.norm()
    }

    /// Returns the angle between `v1`, `v2`, and `v3` obeying periodic boundary conditions.
    /// `v1`, `v2`, and `v3` must be given in cartesian coordinates. The returned angle is
    /// in units of degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::assert_relative_eq;
    /// use nalgebra::Vector3;
    /// use velvet::system::cell::Cell;
    ///
    /// let cell = Cell::cubic(5.0);
    /// let v1 = Vector3::new(1.0, 0.0, 0.0);
    /// let v2 = Vector3::zeros();
    /// let v3 = Vector3::new(0.0, 1.0, 0.0);
    /// let angle = cell.angle(&v1, &v2, &v3);
    /// assert_relative_eq!(angle, 90.0, epsilon = 1e-5);
    /// ```
    pub fn angle(&self, v1: &Vector3<f64>, v2: &Vector3<f64>, v3: &Vector3<f64>) -> f64 {
        let mut v12 = v1 - v2;
        self.vector_image(&mut v12);
        let mut v32 = v3 - v2;
        self.vector_image(&mut v32);
        f64::acos(v12.dot(&v32) / (v12.norm() * v32.norm())).to_degrees()
    }

    /// Returns the dihedral angle between `v1`, `v2`, `v3`, and `v4`.
    /// `v1`, `v2`, `v3`, and `v4` must be given in cartesian coordinates.
    /// The returned angle is in units of degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::assert_relative_eq;
    /// use nalgebra::Vector3;
    /// use velvet::system::cell::Cell;
    ///
    /// let cell = Cell::cubic(5.0);
    /// let v1 = Vector3::new(1.0, 0.0, 0.0);
    /// let v2 = Vector3::zeros();
    /// let v3 = Vector3::new(0.0, 1.0, 0.0);
    /// let v4 = Vector3::new(0.0, 0.0, 1.0);
    /// let dihedral = cell.dihedral(&v1, &v2, &v3, &v4);
    /// assert_relative_eq!(dihedral, -90.0, epsilon = 1e-5);
    /// ```
    pub fn dihedral(
        &self,
        v1: &Vector3<f64>,
        v2: &Vector3<f64>,
        v3: &Vector3<f64>,
        v4: &Vector3<f64>,
    ) -> f64 {
        let mut v21 = v2 - v1;
        self.vector_image(&mut v21);
        let mut v32 = v3 - v2;
        self.vector_image(&mut v32);
        let mut v43 = v4 - v3;
        self.vector_image(&mut v43);

        let u = v21.cross(&v32);
        let v = v32.cross(&v43);
        f64::atan2(v32.norm() * v.dot(&v21), u.dot(&v)).to_degrees()
    }

    /// Returns the total volume of the cell.
    pub fn volume(&self) -> f64 {
        (self.a_vector().cross(&self.b_vector())).dot(&self.c_vector())
    }
}
