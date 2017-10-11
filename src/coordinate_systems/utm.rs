//! UTM projection

use prelude::*;

#[derive(Copy, Clone)]
pub struct UTMSystem;

impl ToLonLat for UTMSystem {
    fn to_lon_lat<UTMSystem, E: Ellipsoid>(source: CoordinateBuf<UTMSystem, E>) -> LonLatBuf<E>
    {
        // todo: reproject here
        LonLatBuf {
            data: source.data,
            ellipsoid: source.ellipsoid,
        }
    }
}

impl FromLonLat for UTMSystem {
    fn from_lon_lat<UTMSystem, E: Ellipsoid>(source: LonLatBuf<E>) -> CoordinateBuf<UTMSystem, E>
    {
        // todo: reproject here
        CoordinateBuf {
            data: source.data,
            crs: UTMSystem,
            ellipsoid: source.ellipsoid,
        }
    }
}
