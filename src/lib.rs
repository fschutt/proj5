//! # proj5
//!
//! PROJ.5 is a Rust-based alternative to the established coordinate projection
//! library PROJ.4 (which is written in C). PROJ.5 aims to make coordinate transformations
//! more type-safe (instead of relying on projection strings) and multi-threaded
//! (using multiple threads as well as generated OpenCL code).
//!
//! While it is a large undertaking rewriting such a well-established library,
//! the speed benefits from multithreading, vectorization and batched transformation
//! are reason enough to at least try.
//!
//! I've written this library because I saw various GIS / projection libraries
//! floating around on github, but no centralized Rust-projection library.
//!
//! **Note:** This library is a work-in-progress and is by no means battle-tested.
//! It's just a collection of projection formulas from different authors,
//! ported to Rust with a type-safe interface.
//!
//! **Important:** Currently, there is no reprojection between ellipsoids yet.
//!
//! **Also important:** Coordinates are always horizonal, then vertical. (LonLat instead of LatLon)
//!
//! PROJ.5 defines the 24 standard ellipsoids (such as WGS84, Bessel, etc.),
//! but you can make your own ellipsoids.
//!
//! ## Usage
//!
//! ```rust
//! extern crate proj5;
//!
//! use proj5::prelude::*;
//!
//! fn main() {
//!
//!     //! warning: PROJ.5 can currently not reproject between different ellipsoids!
//!     //! using different ellipsoids will panic!
//!     let ellipsoid = WGS_1984_ELLIPSOID;
//!
//!     let source_coordinates = CoordinateSource::CoordinateBuf(Box::new(
//!         CoordinateBuf {
//!             data: vec![(377299.0, 1483035.0)],
//!             crs: Box::new(UTMSystem {
//!                 utm_zone: 48,
//!             }),
//!             ellipsoid: ellipsoid,
//!         }
//!     ));
//!
//!     let mut target_coordinates = CoordinateSource::CoordinateBuf(Box::new(
//!         CoordinateBuf {
//!             data: Vec::new(),
//!             crs: Box::new(MercatorSystem),
//!             ellipsoid: ellipsoid,
//!         }
//!     ));
//!
//!     let mut strategy = MultithreadingStrategy::SingleCore;
//!     source_coordinates.project(&mut target_coordinates, &mut strategy);
//!
//!     println!("first batch of coordinates: {:#?}", target_coordinates.get_data_ref());
//! }
//!
//! ```
//!
//! ## Performance
//!
//! Performance is dependent on the chosen `MultithreadingStrategy`. Of course,
//! a multithreaded transform will always be faster than a single-threaded one.
//! When working with coordinates, X and Y are often calculated seperately,
//! which is why this library does not work with vectors (i.e. libraries such
//! as `nalgebra`. The transformations are not linear, which is why vectors
//! are in this case useless.
//!
//! PROJ.5 uses two virtual function calls per (batched) transformation. It
//! is highly, HIGHLY recommended to **batch** your coordinates, whenever
//! you can. PROJ.5 uses double precision for calculation.
//!
//!
//! ## Design
//!
//! Projecting from any projection and any ellipsoid into any other
//! projection and ellipsoid would result in
//! `(number of projections) ^ (number of ellipoids) ^ 2`
//! conversions. This is not realistically possible. Instead,
//! what PROJ.5 does is the following conversion:
//!
//! ```ignore
//! +-----------------+    +------------------+
//! |(1)              |    |(2)               |
//! |Source CRS       |    |Longitude / Latit.|
//! |Source Ellipsoid +-v->+Source Ellipsoid  |
//! |Source Units     |    |lon/lat (degrees) |
//! |                 |    |                  |
//! +-----------------+    +--------+---------+
//!                                 |
//!                                 |
//! +-----------------+    +--------+---------+
//! |(4)              |    |(3)               |
//! |Target CRS       |    |Longitude / Latit.|
//! |Target Ellipsoid +<-v-+Target Ellipsoid  |
//! |Target Units     |    |lon/lat (degrees) |
//! |                 |    |                  |
//! +-----------------+    +------------------+
//!
//! ```
//!
//! The arrows marked with `v` require a virtual function call,
//! in order to lookup the implementation of the given coordinate system.
//!
//! In order to implement your own CRS, you have to implement the `ToLatLon` and `FromLatLon` traits.
//! The required trait `Crs` is then automatically implemented for you.
//!
//! ```rust,ignore
//! impl ToLatLon for MyCoordinateSystem {
//!    fn to_lon_lat(&self, mut data: Vec<(f64, f64)>, ellipsoid: Ellipsoid)
//!               -> LonLatBuf
//!    { ... }
//! }
//!
//! impl FromLatLon for MyCoordinateSystem {
//!     fn from_lon_lat(&self, mut data: Vec<(f64, f64)>, ellipsoid: Ellipsoid)
//!                     -> CoordinateBuf
//!     { ... }
//! }
//! ```
//!
//! This way, every coordinate system can talk to every other coordinate system.
//!


#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", feature(alloc, lang_items, core_float, core_intrinsics))]
// #[cfg(target_arch = "wasm32")]
// #[feature(alloc, lang_items, core_float, core_intrinsics)]

#[cfg(target_arch = "wasm32")]
extern crate alloc;

#[cfg(not(target_arch = "wasm32"))]
extern crate scoped_threadpool;

#[cfg(target_arch = "wasm32")]
mod math;

mod coordinate_systems;
mod coordinate_buf;
mod traits;
mod lonlat_buf;
mod ellipsoid;
mod multithreading;

pub use traits::{
    ToLonLat,
    FromLonLat,
    Crs,
};

#[cfg(not(target_arch = "wasm32"))]
pub use scoped_threadpool::Pool as ThreadPool;

pub use multithreading::MultithreadingStrategy;
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
            pub use coordinate_systems::utm::utils::*;
        }
    }
}

// prelude for easy importing
pub mod prelude {
    pub use traits::*;
    pub use lonlat_buf::LonLatBuf;
    pub use coordinate_buf::CoordinateBuf;
    pub use ellipsoid::*;
    pub use crs::*;
    #[cfg(not(target_arch = "wasm32"))]
    pub use ThreadPool;
    pub use multithreading::MultithreadingStrategy;
    pub use multithreading::MultithreadingStrategy::*;
    pub use coordinate_buf::CoordinateSource;
}

#[cfg(target_arch = "wasm32")]
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

#[cfg(target_arch = "wasm32")]
#[lang = "eh_personality"] extern fn eh_personality() {}