use prelude::*;

/// Source of the given coordinates
/// This is needed so we can reproject LatLon directly to the target CRS
/// without any intermediate steps.
pub enum CoordinateSource {
    CoordinateBuf(Box<CoordinateBuf>),
    LonLatBuf(Box<LonLatBuf>),
}

impl CoordinateSource {
    pub fn project(self, target: &mut CoordinateBuf, strategy: &mut MultithreadingStrategy)
    {
        let (mut temp, source_ellipsoid) = match self {
            CoordinateSource::CoordinateBuf(buf) => {
                let buf = *buf;
                (buf.crs.to_lon_lat(buf.data, &buf.ellipsoid, strategy),
                 buf.ellipsoid)
            },
            CoordinateSource::LonLatBuf(buf) => {
                let ellipsoid = buf.ellipsoid;
                let buf = *buf;
                (buf, ellipsoid)
            },
        };

        if source_ellipsoid != target.ellipsoid {
            temp.project_to_ellipsoid(target.ellipsoid);
        }

        let result = target.crs.from_lon_lat(temp.data, &temp.ellipsoid, strategy);
        *target = result;
    }
}

/// A buffer of coordinates. Units are arbitrary.
pub struct CoordinateBuf {
    /// The actual coordinates, in (x, y) format
    pub data: Vec<(f64, f64)>,
    /// The coordinate reference system
    pub crs: Box<Crs>,
    /// The ellipsoid that is used in this CRS.
    pub ellipsoid: Ellipsoid,
}

