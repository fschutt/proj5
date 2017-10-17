use prelude::*;

/// A buffer of coordinates. Units are arbitrary.
pub struct CoordinateBuf {
    /// The actual coordinates, in (x, y) format
    pub data: Vec<(f64, f64)>,
    /// The coordinate reference system
    pub crs: Box<Crs>,
    /// The ellipsoid that is used in this CRS.
    pub ellipsoid: Ellipsoid,
}

/// Source of the given coordinates
/// This is needed so we can reproject LatLon directly to the target CRS
/// without any intermediate steps.
pub enum CoordinateSource {
    CoordinateBuf(Box<CoordinateBuf>),
    LonLatBuf(Box<LonLatBuf>),
}

impl CoordinateSource
{    
    /// Project coordinates from `self` to `target`.
    pub fn project(self, target: &mut CoordinateSource, strategy: &mut MultithreadingStrategy)
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

        let result = match *target {
            CoordinateSource::CoordinateBuf(ref mut buf) => {
                if source_ellipsoid != buf.ellipsoid {
                    temp.project_to_ellipsoid(buf.ellipsoid);
                }
                
                CoordinateSource::CoordinateBuf(
                    Box::new(buf.crs.from_lon_lat(temp.data, &temp.ellipsoid, strategy))
                )
            },
            CoordinateSource::LonLatBuf(ref mut buf) => {
                if source_ellipsoid != buf.ellipsoid {
                    temp.project_to_ellipsoid(buf.ellipsoid);
                }
                CoordinateSource::LonLatBuf(Box::new(temp))
            },
        };

        *target = result;
    }

    /// Consume the buffer and extract the data
    pub fn into_data(self)
                    -> Vec<(f64, f64)>
    {
        match self {
            CoordinateSource::CoordinateBuf(buf) => buf.data,
            CoordinateSource::LonLatBuf(buf) => buf.data,
        }
    }

    /// Get the data as a reference
    pub fn get_data_ref(&self)
                    -> &Vec<(f64, f64)>
    {
        match *self {
            CoordinateSource::CoordinateBuf(ref buf) => &buf.data,
            CoordinateSource::LonLatBuf(ref buf) => &buf.data,
        }
    }

    /// Get the data as a mutable reference
    pub fn get_data_ref_mut(&mut self)
                    -> &mut Vec<(f64, f64)>
    {
        match *self {
            CoordinateSource::CoordinateBuf(ref mut buf) => &mut buf.data,
            CoordinateSource::LonLatBuf(ref mut buf) => &mut buf.data,
        }
    }

    /// Get the ellipsoid (by value)
    pub fn get_ellipsoid(&self)
                         -> Ellipsoid
    {
        match *self {
            CoordinateSource::CoordinateBuf(ref buf) => buf.ellipsoid,
            CoordinateSource::LonLatBuf(ref buf) => buf.ellipsoid,
        }
    }
}

