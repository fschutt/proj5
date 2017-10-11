//! Proj5, a GPU-accellerated, multithreaded and safe coordinate
//! projection library based on proj4

mod coordinate_systems;
mod coordinate_buf;
mod traits;
mod lonlat;
mod ellipsoids;

pub use traits::{
    Ellipsoid,
    ToLonLat,
    FromLonLat,
    Crs,
};

pub use lonlat::LonLatBuf;
pub use coordinate_buf::CoordinateBuf;

pub use coordinate_systems::merc::MercatorSystem;
pub use coordinate_systems::utm::UTMSystem;
/* other coordinate systems go here */

pub use ellipsoids::wgs84::WGS84Ellipsoid;
/* other ellipsoids go here */
