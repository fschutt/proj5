//! UTM projection

use prelude::*;

#[derive(Copy, Clone)]
pub struct UTMSystem;

impl ToLonLat for UTMSystem {
    fn to_lon_lat<E: Ellipsoid>(&self, source: CoordinateBuf<E>) -> LonLatBuf<E>
    {
        // todo: reproject here
        LonLatBuf {
            data: source.data,
            ellipsoid: source.ellipsoid,
        }
    }
}

impl FromLonLat for UTMSystem {
    fn from_lon_lat<E: Ellipsoid>(&self, source: LonLatBuf<E>) -> CoordinateBuf<E>
    {
        // todo: reproject here
        CoordinateBuf {
            data: source.data,
            crs: Box::new(UTMSystem),
            ellipsoid: source.ellipsoid,
        }
    }
}
