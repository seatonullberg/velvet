/// Defines the interface of a pairwise interatomic potential.
pub trait PairPotential {
    /// Returns the potential energy of an atom separated from another by a distance `r`. 
    fn energy(&self, r: f32) -> f32;
    /// Returns the magnitude of the force acting on an atom by another separated by a distance `r`.
    fn force(&self, r: f32) -> f32;
}

