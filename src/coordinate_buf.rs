use traits::Crs;
use traits::Ellipsoid;

#[derive(Debug, Clone)]
pub struct CoordinateBuf<C: Crs, E: Ellipsoid> {
    pub data: Vec<(f64, f64)>,
    pub crs: C,
    pub ellipsoid: E,
}
