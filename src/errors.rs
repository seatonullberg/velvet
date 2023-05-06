//! Errors that are specific to Velvet.

use crate::potentials::pair::group::MixingStrategy;

use nalgebra::Matrix3;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SystemInitializationError {
    #[error("missing or improperly formatted trajectory file")]
    InvalidTrajectoryFile(#[from] chemfiles::Error),
    #[error("no atom type found for one or more atoms in frame")]
    MissingAtomType,
    #[error("found atom type `{found:?}` which does not match any user-provided atom types `{expected:?}`")]
    InvalidAtomType {
        expected: Vec<String>,
        found: String,
    },
    #[error("the matrix `{0}` cannot be inverted")]
    InvalidCellMatrix(Matrix3<f64>),
    #[error("no cell found in frame")]
    MissingCell,
}

#[derive(Error, Debug)]
pub enum PotentialsInitializationError {
    #[error("mixing strategy `{strategy:?}` is invalid in this context: {msg:?}")]
    InvalidMixingStrategy {
        strategy: MixingStrategy,
        msg: String,
    },
    #[error(
        "found parameter names `{found:?}` which do not match the expected names `{expected:?}`"
    )]
    IncompatibleParameters {
        expected: Vec<String>,
        found: Vec<String>,
    },
}
