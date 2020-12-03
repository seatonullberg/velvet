#[derive(Clone, Copy, Debug)]
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
