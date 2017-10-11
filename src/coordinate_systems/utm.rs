//! UTM projection

use prelude::*;

#[derive(Copy, Clone)]
pub struct UTMSystem;

impl ToLonLat for UTMSystem {
    fn to_lon_lat(&self, data: Vec<(f64, f64)>, ellipsoid: Ellipsoid)
                  -> LonLatBuf
    {
        // todo: reproject here
        LonLatBuf {
            data: data,
            ellipsoid: ellipsoid,
        }
    }
}

impl FromLonLat for UTMSystem {
    fn from_lon_lat(&self, data: Vec<(f64, f64)>, ellipsoid: Ellipsoid)
                    -> CoordinateBuf
    {
        // todo: reproject here
        CoordinateBuf {
            data: data,
            crs: Box::new(UTMSystem),
            ellipsoid: ellipsoid,
        }
    }
}
