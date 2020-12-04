use crate::potential::Potential;

trait AnglePotential {}

impl<T: Potential<Args = AngleArgs>> AnglePotential for T {}

#[derive(Copy, Clone, Debug)]
pub struct AngleArgs {
    /// Angle between a triplet of atoms.
    theta: f32,
}

/// Pair potential meta data.
#[derive(Copy, Clone, Debug)]
pub struct AngleMeta {
    /// Applicable angle kind.
    kind: usize,
}
