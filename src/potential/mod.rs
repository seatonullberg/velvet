pub mod pair;

use crate::energy::EnergyEvaluator;
use crate::force::ForceEvaluator;

pub trait Potential: EnergyEvaluator + ForceEvaluator {}
