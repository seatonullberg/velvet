#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Element {
    H,
    He,
    Li,
    F,
    Ar,
}

impl Element {
    pub fn mass(&self) -> f32 {
        match self {
            Element::H => 1.008,
            Element::He => 4.0026,
            Element::Li => 6.94,
            Element::F => 18.998,
            Element::Ar => 39.948,
        }
    }
}
