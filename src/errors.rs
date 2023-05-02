//! Errors that are specific to Velvet.

use nalgebra::Matrix3;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SystemInitializationError {
    #[error("missing or improperly formatted trajectory file")]
    InvalidTrajectoryFile(#[from] chemfiles::Error),
    #[error("no atom types found in frame or supplied by user")]
    MissingAtomTypes,
    #[error("found atom type `{found:?}` which does not match any user-provided atom types `{expected:?}`")]
    InvalidAtomType {
        expected: Vec<String>,
        found: String,
    },
    #[error("the matrix `{0}` cannot be inverted")]
    InvalidCellMatrix(Matrix3<f64>),
}
