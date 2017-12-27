//! Ellipsoid module

use coordinate_buf::CoordinateBuf;
use lonlat_buf::LonLatBuf;
use prelude::*;

#[cfg(target_arch = "wasm32")]
use alloc::boxed::Box;
#[cfg(target_arch = "wasm32")]
use alloc::Vec;

#[cfg(not(target_arch = "wasm32"))]
use std::fmt::Debug;
#[cfg(target_arch = "wasm32")]
use core::fmt::Debug;

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

pub trait ToLonLat
{
    fn to_lon_lat(&self, data: Vec<(f64, f64)>, ellipsoid: &Ellipsoid, strategy: &mut MultithreadingStrategy)
                  -> LonLatBuf;
}

pub trait FromLonLat
{
    fn from_lon_lat(&self, data: Vec<(f64, f64)>, ellipsoid: &Ellipsoid, strategy: &mut MultithreadingStrategy)
                    -> CoordinateBuf;
}

pub trait Crs: ToLonLat + FromLonLat + Debug {
    fn clone(&self) -> Box<Crs>;
}

impl<T> Crs for T where T: ToLonLat + FromLonLat + Debug + Clone + 'static {
    fn clone(&self) -> Box<Crs> {
        Box::new((*self).clone())
    }
}
