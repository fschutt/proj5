use traits::Ellipsoid;

#[derive(Debug, Copy, Clone)]
pub struct WGS84Ellipsoid;

impl Ellipsoid for WGS84Ellipsoid {
    #[inline]
    fn get_a(&self) -> f64 { 6_378_137.0 }
    #[inline]
    fn get_b(&self) -> f64 { 6_356_752.314245 }
    #[inline]
    fn get_offset(&self) -> f64 { 0.0 }
    #[inline]
    fn get_rotation(&self) -> f64 { 0.0 }
    #[inline]
    fn get_scaling(&self) -> f64 { 0.0 }
}
