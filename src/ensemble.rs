#[derive(Clone, Copy, Debug)]
pub enum Ensemble {
    /// Microcanonical ensemble.
    ///
    /// Simulates an adiabatic process with no heat exchange.
    NVE,
    /// Canonical ensemble.
    ///
    /// Simulates constant temperature molecular dynamics.
    NVT,
    /// Isothermal-isobaric ensemble.
    ///
    /// Simulates constant pressure and temperature conditions.
    NPT,
}
