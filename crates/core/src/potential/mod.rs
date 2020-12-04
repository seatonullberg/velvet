pub mod pair;

use std::collections::HashMap;

pub trait Potential {
    fn setup(&mut self, params: &HashMap<&'static str, f32>);
}

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
