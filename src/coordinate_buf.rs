use traits::Crs;
use ellipsoid::Ellipsoid;

pub struct CoordinateBuf {
    pub data: Vec<(f64, f64)>,
    pub crs: Box<Crs>,
    pub ellipsoid: Ellipsoid,
}

