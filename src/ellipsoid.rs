
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ellipsoid {
    /// Semi-major axis A in meter
    pub a: f64,
    /// Semi-minor axis B in meter
    pub b: f64,
    /// Offset in relation to an earth-centric coordinate system
    pub offset: f64,
    /// Get the rotation in relation to an earth-centric coordinate system
    pub rotation: f64,
    /// Get the scaling in relation to an earth-centric coordinate system
    pub scaling: f64,
}
