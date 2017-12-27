use ellipsoid::Ellipsoid;

#[cfg(target_arch = "wasm32")]
use alloc::Vec;

/// A special version of a CoordinateBuf
/// Every CRS that implements the `Crs` trait can project in and out of a `LonLatBuf`
#[repr(C)]
pub struct LonLatBuf {
    /// The actual coordinates in (lon, lat) format
    pub data: Vec<(f64, f64)>,
    /// The ellipsoid used for the coordinates
    pub ellipsoid: Ellipsoid,
}

impl LonLatBuf {
    /// Reprojects from the current ellipsoid to the target ellipsoid by reprojecting
    /// the points on the different spheroids. Does nothing if the ellipsoids are the same.
    ///
    /// TODO: THIS DOES NOT WORK IF THE `target_ellipsoid` != `source_ellipsoid`.
    pub(crate) fn project_to_ellipsoid(&mut self, target_ellipsoid: Ellipsoid) {
        // TODO !!!

        if self.ellipsoid != target_ellipsoid {
            // panic!("Transforming between ellipsoids is currently not implemented!");
        }
    }
}
