use ellipsoid::Ellipsoid;

pub struct LonLatBuf {
    pub data: Vec<(f64, f64)>,
    pub ellipsoid: Ellipsoid,
}

impl LonLatBuf {
    /// Reprojects from the current ellipsoid to the target ellipsoid by reprojecting
    /// the points on the different spheroids. Does nothing if the ellipsoids are the same.
    pub fn project_to_ellipsoid(&mut self, target_ellipsoid: Ellipsoid) {
        // TODO !!!

        if self.ellipsoid != target_ellipsoid {
            panic!("Transforming between ellipsoids is currently not implemented!");
        }
    }
}
