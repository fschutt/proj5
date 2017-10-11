use traits::{Ellipsoid, ToLonLat, FromLonLat, ToFromLonLat, Crs};
use lonlat_buf::LonLatBuf;

pub struct CoordinateBuf<E: Ellipsoid> {
    pub data: Vec<(f64, f64)>,
    pub crs: Box<Crs>,
    pub ellipsoid: E,
}

/*
impl<CA, E> ToLonLat for CoordinateBuf<CA, E> where CA: Crs, E: Ellipsoid {
    fn to_lon_lat<C: Crs, EA: Ellipsoid>(&self, data: CoordinateBuf<C, EA>) -> LonLatBuf<EA> {
        self.crs.to_lon_lat(data)
    }
}

impl<CA, E> FromLonLat for CoordinateBuf<CA, E> where CA: Crs, E: Ellipsoid {
    fn from_lon_lat<C: Crs, EA: Ellipsoid>(&self, data: LonLatBuf<EA>) -> CoordinateBuf<C, EA> {
        self.crs.from_lon_lat(data)
    }
}

impl<CA, EA> Crs for CoordinateBuf<CA, EA> where CA: Crs, EA: Ellipsoid {
    fn project_to<CB: Crs, EB: Ellipsoid>(self, other_crs: &CB, other_ellipsoid: &EB) -> CoordinateBuf<CB, EB> {
        let temp = self.crs.clone().to_lon_lat::<CA, EA>(self);
        other_crs.from_lon_lat::<CB, EB>(temp.project_to_ellipsoid(*other_ellipsoid))
    }
}
*/
