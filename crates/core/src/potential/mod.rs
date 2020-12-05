pub mod pair;

pub trait Potential: Clone + Copy + Send + Sync {}

/// Restrictions which can be applied to a potential.
#[derive(Clone, Copy, Debug)]
pub enum Restriction {
    /// No restrictions.
    None,
    /// Applies only to atoms in separate molecules.
    Intermolecular,
    /// Applies only to atoms within the same molecule.
    Intramolecular,
}
