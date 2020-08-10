use crate::system::System;

use nalgebra::Vector3;

// NOTE: positions a and b are assumed to be within the bounding box
pub fn distance(system: &System, a: &Vector3<f32>, b: &Vector3<f32>) -> f32 {
    let mut dist: Vector3<f32> = Vector3::zeros();
    for i in 0..3 {
        let mut a_elem = a[i];
        let mut b_elem = b[i];
        let mag = system.basis.row(i).norm();
        if system.periodicity[i] {
            // periodic boundaries cut the effective span in half
            if a[i] > mag / 2.0 {
                a_elem = mag - a[i];
            }
            if b[i] > mag / 2.0 {
                b_elem = mag - b[i];
            }
        }
        dist[i] = a_elem - b_elem
    }
    dist.norm()
}

#[cfg(test)]
mod tests {
    use crate::distance::distance;
    use crate::ensemble::Ensemble;
    use crate::system::System;
    use nalgebra::{Matrix3, Vector3};

    fn get_test_system() -> System {
        System {
            atoms: Vec::new(),
            basis: Matrix3::identity(),
            ensemble: Ensemble::NVE,
            n_threads: 1,
            n_timesteps: 1,
            periodicity: Vector3::new(false, false, false),
            timestep: 1.0,
        }
    }

    fn get_test_points() -> (Vector3<f32>, Vector3<f32>) {
        (Vector3::zeros(), Vector3::new(0.25, 0.5, 0.75))
    }

    #[test]
    fn nonperiodic_distance() {
        let system = get_test_system();
        let (a, b) = get_test_points();
        let res = distance(&system, &a, &b);
        assert_eq!(res, 0.935414347)
    }

    #[test]
    fn semiperiodic_distance() {
        let mut system = get_test_system();
        system.periodicity = Vector3::new(false, false, true);
        let (a, b) = get_test_points();
        let res = distance(&system, &a, &b);
        assert_eq!(res, 0.612372436);
    }

    #[test]
    fn periodic_distance() {
        let mut system = get_test_system();
        system.periodicity = Vector3::new(true, true, true);
        let (a, b) = get_test_points();
        let res = distance(&system, &a, &b);
        assert_eq!(res, 0.612372436);
    }
}
