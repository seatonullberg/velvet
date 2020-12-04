pub mod angle;
pub mod pair;

use std::collections::HashMap;

pub trait Potential {
    type Args;

    fn keys(&self) -> Vec<&'static str>;
    fn setup(&mut self, params: &HashMap<&'static str, f32>);

    fn energy(&self, args: &Self::Args) -> f32;
    fn force(&self, args: &Self::Args) -> f32;
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
