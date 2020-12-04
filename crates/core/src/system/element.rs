use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Element {
    H,
    He,
    Li,
}

impl Element {
    pub fn mass(&self) -> f32 {
        match self {
            Element::H => 1.008,
            Element::He => 4.0026,
            Element::Li => 6.94,
        }
    }
}

// TODO: implement TryFrom &str
