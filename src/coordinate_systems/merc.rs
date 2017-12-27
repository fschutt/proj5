//! Mercator projection
//!
//! Impelentation taken from:
//! 
//! http://wiki.openstreetmap.org/wiki/Mercator
//!
//! Copyright 2006 Christopher Schmidt

use prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct MercatorSystem;

#[cfg(target_arch = "wasm32")]
use core::f64::consts::PI;
#[cfg(not(target_arch = "wasm32"))]
use std::f64::consts::PI;

#[cfg(target_arch = "wasm32")]
use alloc::Vec;
#[cfg(target_arch = "wasm32")]
use alloc::boxed::Box;

#[inline(always)]
fn pj_phi2(ts: f64, e: f64)
           -> f64
{    
    const HALFPI: f64 = PI / 2.0;
    const TOL: f64 = 0.0000000001;
    
    let eccnth = 0.5 * e;
    let mut phi = HALFPI - 2.0 * ts.atan();

    let mut dphi;
    let mut con;
    let mut iteration_cnt = 15;

    loop {
        con = e * phi.sin();
        dphi = HALFPI - 2.0 * (ts * ((1.0 - con) / (1.0 + con)).powf(eccnth)).atan() - phi;
        phi += dphi; 
        
        if dphi.abs() > TOL && (iteration_cnt - 1) > 0 { break; } else { iteration_cnt -= 1; }
    }
    
    return phi;
}

/// `temp = ellipsoid.b / ellipsoid.a`
#[inline(always)]
fn lat_to_mercator_y(mut lat: f64, ellipsoid_a: f64, temp: f64) -> f64 {

    if lat > 89.5  { lat = 89.5; }
    if lat < -89.5 { lat = -89.5; }

    let es = 1.0 - (temp * temp);
    let eccent = es.sqrt();
    let phi = lat.to_radians();
    let sinphi = phi.sin();
    let con = eccent * sinphi;
    let com = 0.5 * eccent;
    let con = (1.0 - con) / (1.0 + con).powf(com);
    let ts = (0.5 * (PI * 0.5 - phi)).tan() / con;
    let y = 0.0 - ellipsoid_a * ts.ln();

    return y;
}

#[inline(always)]
fn merc_x_to_lon(x: f64, ellipsoid_a: f64)
                 -> f64
{
    (x / ellipsoid_a).to_degrees()
}

/// `temp = ellipsoid.a / ellipsoid.b`
/// `e = (1.0 - (temp * temp)).sqrt()`
#[inline(always)]
fn merc_y_to_lat(y: f64, ellipsoid_a: f64, e: f64)
                 -> f64
{
    pj_phi2((0.0 - (y / ellipsoid_a)).exp(), e).to_degrees()
}

impl ToLonLat for MercatorSystem {
    fn to_lon_lat(&self, mut data: Vec<(f64, f64)>, ellipsoid: &Ellipsoid, strategy: &mut MultithreadingStrategy)
                  -> LonLatBuf
    {
        let temp = ellipsoid.b / ellipsoid.a;
        let e = (1.0 - (temp * temp)).sqrt();
        
        match *strategy {
            SingleCore => {
                for &mut (ref mut x, ref mut y) in data.iter_mut() {
                    *x = merc_x_to_lon(*x, ellipsoid.a);
                    *y = merc_y_to_lat(*y, ellipsoid.b, e);
                }
            },
            #[cfg(not(target_arch = "wasm32"))]
            MultiCore(ref mut thread_pool) => {
                thread_pool.scoped(|scoped| {
                    for &mut (ref mut x, ref mut y) in data.iter_mut() {
                        scoped.execute(move || {
                            *x = merc_x_to_lon(*x, ellipsoid.a);
                            *y = merc_y_to_lat(*y, ellipsoid.b, e);
                        });
                    }
                });
            },
            #[cfg(not(target_arch = "wasm32"))]
            _ => unimplemented!("Multithreading methods other than SingleCore and MultiCore are not yet implemented!"),          
        }
        
        LonLatBuf {
            data: data,
            ellipsoid: *ellipsoid,
        }
    }
}

impl FromLonLat for MercatorSystem {

    fn from_lon_lat(&self, mut data: Vec<(f64, f64)>, ellipsoid: &Ellipsoid, strategy: &mut MultithreadingStrategy)
                    -> CoordinateBuf
    {        
        let temp = ellipsoid.b / ellipsoid.a;

        // TODO: copy-pasted! bad!
        match *strategy {
            SingleCore => {
                for &mut (ref mut lon, ref mut lat) in data.iter_mut() {
                    *lon = ellipsoid.a * lon.to_radians();
                    *lat = lat_to_mercator_y(*lat, ellipsoid.a, temp);
                }
            },
            #[cfg(not(target_arch = "wasm32"))]
            MultiCore(ref mut thread_pool) => {
                thread_pool.scoped(|scoped| {
                    // Create references to each element in the vector ...
                    for &mut (ref mut lon, ref mut lat) in &mut data {
                        // ... and add 1 to it in a seperate thread
                        scoped.execute(move || {
                            *lon = ellipsoid.a * lon.to_radians();
                            *lat = lat_to_mercator_y(*lat, ellipsoid.a, temp);
                        });
                    }
                });
            },
            #[cfg(not(target_arch = "wasm32"))]
            _ => unimplemented!("Multithreading methods other than SingleCore and MultiCore are not yet implemented!"),          
        }

        CoordinateBuf {
            data: data,
            crs: Box::new(MercatorSystem),
            ellipsoid: *ellipsoid,
        }
    }
}

