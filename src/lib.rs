//! Proj5, a GPU-accellerated, multithreaded and safe coordinate
//! projection library based on proj4

mod coordinate_systems;
mod coordinate_buf;
mod traits;
mod lonlat_buf;
mod ellipsoid;

pub use traits::{
    ToLonLat,
    FromLonLat,
    Crs,
};

pub use ellipsoid::*;
pub use lonlat_buf::LonLatBuf;
pub use coordinate_buf::CoordinateBuf;

pub mod crs {
    pub use coordinate_systems::merc::MercatorSystem as MercatorSystem;
    pub use coordinate_systems::utm::UTMSystem as UTMSystem;
    // other coordinate systems go here

    // utility functions, specific to certain coordinate systems
    pub mod utils {
        pub mod utm {
            pub use coordinate_systems::utm::*;
        }
    }
}

// prelude for easy importing
pub mod prelude {
    pub use traits::*;
    pub use lonlat_buf::LonLatBuf;
    pub use coordinate_buf::CoordinateBuf;
    pub use ellipsoid::Ellipsoid;
}
