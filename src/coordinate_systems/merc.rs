//! Mercator projection

use prelude::*;

pub struct MercatorSystem;

/*
/// JS implementation of accurate Mercator projection, 
/// to be used at larger scales
/// Copyright 2006 Christopher Schmidt 
/// see: http://wiki.openstreetmap.org/wiki/Mercator
/// 

var sm_a = 6378137.0;
var sm_b = 6356752.314;

/// Converts degrees to radians.
function DegToRad(deg) { return (deg / 180.0 * pi) }

/// Converts radians to degrees.
function RadToDeg(rad) { return (rad / pi * 180.0) }

/// Converts a longitude to meter using the mercator projection
function ToMercatorX(lon) {
return sm_a * DegToRad(lon);
}

/// Converts a longitude to meter using the mercator projection
function FromMercatorX(x) {
    return RadToDeg(x / sm_a);
}

/// Converts a latitude to meter using the mercator projection
function ToMercatorY(lat) {

    if (lat > 89.5) {
        lat = 89.5;
    }

    if (lat < -89.5) {
        lat = -89.5;
    }

    var temp = sm_b / sm_a;
    var es = 1.0 - (temp * temp);
    var eccent = Math.sqrt(es);
    var phi = DegToRad(lat);
    var sinphi = Math.sin(phi);
    var con = eccent * sinphi;
    var com = .5 * eccent;
    con = Math.pow((1.0-con)/(1.0+con), com);
    var ts = Math.tan(.5 * (Math.PI*0.5 - phi))/con;
    var y = 0 - sm_a * Math.log(ts);

    return y;
}

/// Converts a longitude to meter using the mercator projection
function FromMercatorY(y) {

var temp = sm_b / sm_a;
var e = Math.sqrt(1.0 - (temp * temp));
var lat = RadToDeg(PjPhi2(Math.exp( 0 - (y / sm_a)), e));

return lat;
}

/// Taken from http://wiki.openstreetmap.org/wiki/Mercator - used in reverse mercator function
function PjPhi2(ts, e) {

    var N_ITER=15;
    var HALFPI=Math.PI/2;
    
    var TOL=0.0000000001;
    var eccnth, phi, con, dphi;
    var i;
    var eccnth = .5 * e;
    phi = HALFPI - 2. * Math.atan (ts);
    i = N_ITER;

    do {
        con = e * Math.sin (phi);
        dphi = HALFPI - 2. * Math.atan (ts * Math.pow((1. - con) / (1. + con), eccnth)) - phi;
        phi += dphi;
        
    } while ( Math.abs(dphi)>TOL && --i);

    return phi;
}

/// Wrapper: calculates meter in mercator from a lon and lat pair
function LatLonToMercatorXY(lat, lon) {
    return [ToMercatorX(lon),ToMercatorY(lat)];
}

/// Wrapper: calculates lat and lon using the mercator projection from an x and y pair
function MercatorXYToLatLon(x, y) {
    return [FromMercatorX(x), FromMercatorY(y)];
}

 */

fn pj_phi2(ts: f64, e: f64)
           -> f64
{    
    const HALFPI: f64 = ::std::f64::consts::PI / 2.0;
    const TOL: f64 = 0.0000000001;
    
    let eccnth = 0.5 * e;
    let mut phi = HALFPI - 2.0 * ts.atan();

    let mut dphi;
    let mut con;
    
    loop {
        con = e * phi.sin();
        dphi = HALFPI - 2.0 * (ts * ((1.0 - con) / (1.0 + con)).powf(eccnth)).atan() - phi;
        phi += dphi; 
        
        if dphi.abs() > TOL { break; }
    }
    
    return phi;
}

fn lat_to_mercator_y(mut lat: f64, ellipsoid: &Ellipsoid) -> f64 {

    if lat > 89.5  { lat = 89.5; }
    if lat < -89.5 { lat = -89.5; }

    let temp = ellipsoid.b / ellipsoid.a;
    let es = 1.0 - (temp * temp);
    let eccent = es.sqrt();
    let phi = lat.to_radians();
    let sinphi = phi.sin();
    let con = eccent * sinphi;
    let com = 0.5 * eccent;
    let con = (1.0 - con) / (1.0 + con).powf(com);
    let ts = (0.5 * (::std::f64::consts::PI * 0.5 - phi)).tan() / con;
    let y = 0.0 - ellipsoid.a * ts.ln();

    return y;
}

impl ToLonLat for MercatorSystem {
    fn to_lon_lat(&self, mut data: Vec<(f64, f64)>, ellipsoid: Ellipsoid)
                  -> LonLatBuf
    {
        for &mut (ref mut x, ref mut y) in data.iter_mut() {
            let lon = (*x / ellipsoid.a).to_degrees();
            let lat = {
                let temp = ellipsoid.b / ellipsoid.a;
                let e = (1.0 - (temp * temp)).sqrt();
                pj_phi2((0.0 - (*y / ellipsoid.a)).exp(), e).to_degrees()
            };

            *x = lon;
            *y = lat;
        }

        LonLatBuf {
            data: data,
            ellipsoid: ellipsoid,
        }
    }
}

impl FromLonLat for MercatorSystem {

    fn from_lon_lat(&self, mut data: Vec<(f64, f64)>, ellipsoid: Ellipsoid)
                    -> CoordinateBuf
    {
        for &mut (ref mut lon, ref mut lat) in data.iter_mut() {
            let x = ellipsoid.a * lon.to_radians();
            let y = lat_to_mercator_y(*lat, &ellipsoid);
            
            *lon = x;
            *lat = y;
        }

        CoordinateBuf {
            data: data,
            crs: Box::new(MercatorSystem),
            ellipsoid: ellipsoid,
        }
    }
}

