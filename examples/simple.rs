extern crate proj5;

use proj5::prelude::*;

fn main() {

    // -- single threaded

    // warning: PROJ.5 can currently not reproject between different ellipsoids!
    // using different ellipsoids will panic!
    let ellipsoid = WGS_1984_ELLIPSOID;

    let source_coordinates = CoordinateSource::CoordinateBuf(Box::new(
        CoordinateBuf {
            data: vec![(377299.0, 1483035.0)],
            crs: Box::new(UTMSystem {
                utm_zone: 48,
            }),
            ellipsoid: ellipsoid,
        }
    ));

    let mut target_coordinates = CoordinateSource::CoordinateBuf(Box::new(
        CoordinateBuf {
            data: Vec::new(),
            crs: Box::new(MercatorSystem),
            ellipsoid: ellipsoid,
        }
    ));

    let mut strategy = MultithreadingStrategy::SingleCore;
    source_coordinates.project(&mut target_coordinates, &mut strategy);

    println!("first batch of coordinates: {:#?}", target_coordinates.get_data_ref());

    // -- multithreaded

    // The MultithreadingStrategy has to be only initialized once
    // and can be reused throughout multiple projections.
    #[cfg(all(not(target_arch = "wasm32"), feature = "scoped_threadpool"))]
    let mut strategy = MultithreadingStrategy::MultiCore(ThreadPool::new(2));
    #[cfg(not(feature = "scoped_threadpool"))]
    let mut strategy = MultithreadingStrategy::SingleCore;

    let lon_lat_coordinates = CoordinateSource::LonLatBuf(Box::new(
        LonLatBuf {
            data: vec![(-174.726563, -66.086990), (16.171875, 49.386186), (-99.492188, 42.557395)],
            ellipsoid: ellipsoid,
        }
    ));

    // reuse the original coordinate buf
    lon_lat_coordinates.project(&mut target_coordinates, &mut strategy);

    println!("second batch of coordinates: {:#?}", target_coordinates.get_data_ref());
}
