use traits::Ellipsoid;

pub struct LonLatBuf<E: Ellipsoid> {
    pub data: Vec<(f64, f64)>,
    pub ellipsoid: E,
}

impl<E: Ellipsoid> LonLatBuf<E> {
    // Project data from Ellipsoid A to Ellipsoid B
    pub fn project_to_ellipsoid<F: Ellipsoid>(self, ellipsoid: F) -> LonLatBuf<F> {
        // TODO !!!

        if !self.ellipsoid.compare(&ellipsoid) {
            panic!("Transforming between ellipsoids is currently not implemented!");
        }
        
        LonLatBuf {
            data: self.data,
            ellipsoid: ellipsoid,
        }
    }
}
