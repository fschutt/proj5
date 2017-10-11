//! Ellipsoid module

use coordinate_buf::CoordinateBuf;
use lonlat_buf::LonLatBuf;

// The general idea is: If Coordinate-reference-system (CRS) A can project in CRS B
// and CRS B can project into CRS C, then it should be possible to project from 
// CRS A to CRS C by doing two conversions: CRS A (Original) -> CRS B (Base) -> CRS C (Target)

// However, this conversion must adhere to the following:
//
// - The ellipsoid must be the same for the projection of (A -> B) and (B -> C).
// - The units of measurement must be the same for (A -> B) and (B -> C)

// To realistically do this, we need an "easy" CRS B, that all other CRS can agree to
// project in-and-out. A good coordinate system would be (latitude, longitude). However,
// you'd also need to make sure that the ellipsoids are the same:

pub trait Ellipsoid: Copy + Clone {
    /// Get semi-major axis A in meter
    #[inline]
    fn get_a(&self) -> f64;
    /// Get semi-minor axis B in meter
    #[inline]
    fn get_b(&self) -> f64;
    /// Get the offset in relation to an earth-centric coordinate system
    #[inline]
    fn get_offset(&self) -> f64;
    /// Get the rotation in relation to an earth-centric coordinate system
    #[inline]
    fn get_rotation(&self) -> f64;
    /// Get the scaling in relation to an earth-centric coordinate system
    #[inline]
    fn get_scaling(&self) -> f64;
    
    /// Compare two ellipsoids, todo: use std traits
    #[inline]
    fn compare<T: Ellipsoid>(&self, other: &T) -> bool {
        self.get_a() == other.get_a() &&
        self.get_b() == other.get_b() &&
        self.get_offset() == other.get_offset() &&
        self.get_rotation() == other.get_rotation() &&
        self.get_scaling() == other.get_scaling()
    }
}

pub trait ToLonLat {
    fn to_lon_lat<C: ToLonLat + FromLonLat, E: Ellipsoid>(data: CoordinateBuf<C, E>) -> LonLatBuf<E>;
}

pub trait FromLonLat {
    fn from_lon_lat<C: ToLonLat + FromLonLat, E: Ellipsoid>(data: LonLatBuf<E>) -> CoordinateBuf<C, E>;
}

impl<T> Crs for T where T: ToLonLat + FromLonLat { }

pub trait Crs: ToLonLat + FromLonLat {
    fn project_to<CA: Crs, CB: Crs, EA: Ellipsoid, EB: Ellipsoid>
        (data: CoordinateBuf<CA, EA>, other_crs: &CB, other_ellipsoid: &EB) -> CoordinateBuf<CB, EB> {
        let temp = ToLonLat::to_lon_lat::<CA, EA>(data);
        FromLonLat::from_lon_lat::<CB, EB>(temp.project_to_ellipsoid(*other_ellipsoid))
    }
}
