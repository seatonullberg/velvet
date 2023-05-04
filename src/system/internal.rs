use crate::errors::SystemInitializationError;

use std::path::Path;

use chemfiles::{Frame, Trajectory};

// Loads a frame from a trajectory file or returns an error if unsuccessful.
pub fn load_frame_from_trajectory_file<'a, P, S>(
    path: P,
    format: S,
    step: usize,
) -> Result<Frame, SystemInitializationError>
where
    P: AsRef<Path>,
    S: Into<&'a str>,
{
    // Load a trajectory object from file.
    let mut trajectory = match Trajectory::open_with_format(path, 'r', format) {
        Ok(trajectory) => trajectory,
        Err(err) => return Err(SystemInitializationError::InvalidTrajectoryFile(err)),
    };
    // Read a frame from the trajectory or return an error if the read was unsuccessful.
    let mut frame = Frame::new();
    trajectory.read_step(step, &mut frame).map_or_else(
        |err| Err(SystemInitializationError::InvalidTrajectoryFile(err)),
        |_| Ok(frame),
    )
}

#[cfg(test)]
mod tests {
    use super::load_frame_from_trajectory_file;
    use crate::internal::get_resource_filepath;

    // Check that the `load_frame_from_trajectory_file` function works for a valid lammps data file.
    #[test]
    fn load_frame_from_trajectory_file_valid_lammps_data_file() {
        let path = get_resource_filepath("water.lmp");
        let frame = load_frame_from_trajectory_file(path, "LAMMPS Data", 0).unwrap();
        assert_eq!(frame.size(), 300);
    }
}
