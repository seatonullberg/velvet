use nalgebra::{Matrix3, Vector3};

#[derive(Clone, Debug)]
pub struct Cell {
    matrix: Matrix3<f32>,
    inv_matrix: Matrix3<f32>,
}

impl Cell {
    pub fn new(a: f32, b: f32, c: f32, alpha: f32, beta: f32, gamma: f32) -> Cell {
        let cos_alpha = alpha.to_radians().cos();
        let cos_beta = beta.to_radians().cos();
        let (sin_gamma, cos_gamma) = gamma.to_radians().sin_cos();

        let b_x = b * cos_gamma;
        let b_y = b * sin_gamma;

        let c_x = c * cos_beta;
        let c_y = c * (cos_alpha - cos_beta * cos_gamma) / sin_gamma;
        let c_z = f32::sqrt(c * c - c_y * c_y - c_x * c_x);

        let matrix = Matrix3::new(a, b_x, c_x, 0.0, b_y, c_y, 0.0, 0.0, c_z);
        let inv_matrix = matrix.try_inverse().unwrap();

        Cell { matrix, inv_matrix }
    }

    pub fn a(&self) -> f32 {
        self.a_vector().norm()
    }

    pub fn b(&self) -> f32 {
        self.b_vector().norm()
    }

    pub fn c(&self) -> f32 {
        self.c_vector().norm()
    }

    pub fn alpha(&self) -> f32 {
        let b = self.b_vector();
        let c = self.c_vector();
        b.angle(&c).to_degrees()
    }

    pub fn beta(&self) -> f32 {
        let a = self.a_vector();
        let c = self.c_vector();
        a.angle(&c).to_degrees()
    }

    pub fn gamma(&self) -> f32 {
        let a = self.a_vector();
        let b = self.b_vector();
        a.angle(&b).to_degrees()
    }

    pub fn a_vector(&self) -> Vector3<f32> {
        Vector3::new(
            self.matrix[(0, 0)],
            self.matrix[(1, 0)],
            self.matrix[(2, 0)],
        )
    }

    pub fn b_vector(&self) -> Vector3<f32> {
        Vector3::new(
            self.matrix[(0, 1)],
            self.matrix[(1, 1)],
            self.matrix[(2, 1)],
        )
    }

    pub fn c_vector(&self) -> Vector3<f32> {
        Vector3::new(
            self.matrix[(0, 2)],
            self.matrix[(1, 2)],
            self.matrix[(2, 2)],
        )
    }

    pub fn fractional(&self, cartesian: &Vector3<f32>) -> Vector3<f32> {
        self.inv_matrix * cartesian
    }

    pub fn cartesian(&self, fractional: &Vector3<f32>) -> Vector3<f32> {
        self.matrix * fractional
    }

    pub fn wrap_vector(&self, vector: &mut Vector3<f32>) {
        let mut fractional = self.fractional(vector);
        fractional[0] -= f32::floor(fractional[0]);
        fractional[1] -= f32::floor(fractional[1]);
        fractional[2] -= f32::floor(fractional[2]);
        *vector = self.cartesian(&fractional);
    }

    pub fn vector_image(&self, vector: &mut Vector3<f32>) {
        let mut fractional = self.fractional(vector);
        fractional[0] -= f32::round(fractional[0]);
        fractional[1] -= f32::round(fractional[1]);
        fractional[2] -= f32::round(fractional[2]);
        *vector = self.cartesian(&fractional);
    }

    pub fn direction(&self, v1: &Vector3<f32>, v2: &Vector3<f32>) -> Vector3<f32> {
        let mut d = v2 - v1;
        self.vector_image(&mut d);
        d.normalize()
    }

    pub fn distance(&self, v1: &Vector3<f32>, v2: &Vector3<f32>) -> f32 {
        let mut d = v2 - v1;
        self.vector_image(&mut d);
        d.norm()
    }

    pub fn angle(&self, v1: &Vector3<f32>, v2: &Vector3<f32>, v3: &Vector3<f32>) -> f32 {
        let mut v12 = v1 - v2;
        self.vector_image(&mut v12);
        let mut v32 = v3 - v2;
        self.vector_image(&mut v32);
        f32::acos(v12.dot(&v32) / (v12.norm() * v32.norm()))
    }

    pub fn dihedral(
        &self,
        v1: &Vector3<f32>,
        v2: &Vector3<f32>,
        v3: &Vector3<f32>,
        v4: &Vector3<f32>,
    ) -> f32 {
        let mut v21 = v2 - v1;
        self.vector_image(&mut v21);
        let mut v32 = v3 - v2;
        self.vector_image(&mut v32);
        let mut v43 = v4 - v3;
        self.vector_image(&mut v43);

        let u = v21.cross(&v32);
        let v = v32.cross(&v43);
        return f32::atan2(v32.norm() * v.dot(&v21), u.dot(&v));
    }
}

#[cfg(test)]
mod tests {
    use crate::system::cell::Cell;
    use nalgebra::Vector3;

    #[test]
    fn new() {
        let cell = Cell::new(3.0, 4.0, 5.0, 80.0, 90.0, 110.0);
        assert_eq!(cell.a_vector(), Vector3::new(3.0, 0.0, 0.0));
        assert_eq!(cell.b_vector()[2], 0.0);

        assert!(f32::abs(cell.a() - 3.0) < 1e-5);
        assert!(f32::abs(cell.b() - 4.0) < 1e-5);
        assert!(f32::abs(cell.c() - 5.0) < 1e-5);

        assert!(f32::abs(cell.alpha() - 80.0) < 1e-5);
        assert!(f32::abs(cell.beta() - 90.0) < 1e-5);
        assert!(f32::abs(cell.gamma() - 110.0) < 1e-5);
    }

    #[test]
    fn fractional_cartesian() {
        let cell = Cell::new(5.0, 6.0, 3.6, 90.0, 53.0, 77.0);
        let tests = vec![Vector3::new(0.0, 10.0, 4.0), Vector3::new(-5.0, 12.0, 4.9)];

        for test in &tests {
            let res = cell.cartesian(&cell.fractional(test));
            assert!(f32::abs((test - &res).norm()) < 1e-5)
        }
    }

    #[test]
    fn wrap_vector() {
        let cell = Cell::new(3.0, 4.0, 5.0, 90.0, 90.0, 90.0);
        let mut v = Vector3::new(1.0, 1.5, 6.0);
        cell.wrap_vector(&mut v);
        let res = Vector3::new(1.0, 1.5, 1.0);
        assert!(f32::abs((v - res).norm()) < 1e-5);
    }

    #[test]
    fn vector_image() {
        let cell = Cell::new(3.0, 4.0, 5.0, 90.0, 90.0, 90.0);
        let mut v = Vector3::new(1.0, 1.5, 6.0);
        cell.vector_image(&mut v);
        let res = Vector3::new(1.0, 1.5, 1.0);
        assert!(f32::abs((v - res).norm()) < 1e-5);
    }

    #[test]
    fn distance() {
        let cell = Cell::new(3.0, 4.0, 5.0, 90.0, 90.0, 90.0);
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 2.0, 6.0);
        assert_eq!(cell.distance(&v1, &v2), f32::sqrt(6.0));
    }

    #[test]
    fn angle() {
        let cell = Cell::new(3.0, 4.0, 5.0, 90.0, 90.0, 90.0);
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 0.0, 0.0);
        let c = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(cell.angle(&a, &b, &c), std::f32::consts::PI / 2.0);
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 0.0, 0.0);
        let c = Vector3::new(f32::cos(1.877), f32::sin(1.877), 0.0);
        assert_eq!(cell.angle(&a, &b, &c), 1.877);
    }

    #[test]
    fn direction() {
        let cell = Cell::new(3.0, 4.0, 5.0, 90.0, 90.0, 90.0);
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 2.0, 6.0);
        let res = cell.direction(&v1, &v2);
        assert_eq!(res.norm(), 1.0);
    }

    #[test]
    fn dihedral() {
        let cell = Cell::new(3.0, 4.0, 5.0, 90.0, 90.0, 90.0);
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 0.0, 0.0);
        let v3 = Vector3::new(1.0, 1.0, 0.0);
        let v4 = Vector3::new(2.0, 1.0, 0.0);
        assert_eq!(cell.dihedral(&v1, &v2, &v3, &v4), std::f32::consts::PI);
        let v1 = Vector3::new(1.241, 0.444, 0.349);
        let v2 = Vector3::new(-0.011, -0.441, 0.333);
        let v3 = Vector3::new(-1.176, 0.296, -0.332);
        let v4 = Vector3::new(-1.396, 1.211, 0.219);
        assert_eq!(cell.dihedral(&v1, &v2, &v3, &v4), -1.045379);
    }
}
