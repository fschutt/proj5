//! UTM projection

use prelude::*;

/// UTM System
#[derive(Debug, Copy, Clone)]
pub struct UTMSystem {
    pub utm_zone: u8,
}

pub mod utils {

    use prelude::*;

    pub const UTM_SCALE_FACTOR: f64 = 0.9996;
    pub const FALSE_EASTING: f64 = 500000.0;
    pub const FALSE_NORTHING: f64 = 10000000.0;
    
    /// Calculates the UTM zone this longitude falls in
    /// Handles exceptions for Norway / Svalbard
    /// For a visual representation: https://upload.wikimedia.org/wikipedia/commons/a/a5/UTM-Zone.svg
    ///
    /// Inputs: Longitude, in degrees
    ///         Latitude, in degrees
    ///
    /// Returns: UTM Zone (0 to 60)
    ///
    #[allow(non_snake_case)]
    pub fn get_utm_zone(lon: f64, lat: f64) -> u8 {

        let mut zone = ((lon + 180.0) / 6.0).floor() + 1.0;

        if lat > 56.0 && lat < 64.0 {
            // Zone V, Norway
            if lon > 3.0 && lon < 6.0 { zone += 1.0; }

        } else if lat > 72.0 && lat < 84.0 {
            // Zone X, Svalbard
            if lon > 6.0 && lon < 9.0         { zone -= 1.0; }
            if lon > 9.0 && lon < 12.0        { zone += 1.0; }
            if lon > 18.0 && lon < 21.0       { zone -= 1.0; }
            if lon > 21.0 && lon < 24.0       { zone += 1.0; }
            if lon > 30.0 && lon < 33.0       { zone -= 1.0; }
            if lon > 33.0 && lon < 36.0       { zone += 1.0; }
        }
        
        return zone as u8;
    }


    /// Computes the ellipsoidal distance from the equator to a point at a
    /// given latitude.
    ///
    /// Reference: Hoffmann-Wellenhof, B., Lichtenegger, H., and Collins, J.,
    /// GPS: Theory and Practice, 3rd ed.  New York: Springer-Verlag Wien, 1994.
    ///
    /// Inputs:
    /// lat - Latitude of the point, in radians
    /// ellipsoid - The ellipsoid to use
    ///
    /// Returns:
    /// The ellipsoidal distance of the point from the equator, in meters.
    ///
    #[allow(non_snake_case)]
    pub fn arc_length_of_meridian(lat: f64, ellipsoid: &Ellipsoid) -> f64 {

        // Precalculate n
        let n = (ellipsoid.a - ellipsoid.b) / (ellipsoid.a + ellipsoid.b);

        // Precalculate alpha
        let alpha = ((ellipsoid.a + ellipsoid.b) / 2.0) *
            (1.0 + (n.powi(2) / 4.0) + (n.powi(4) / 64.0));

        // Precalculate beta
        let beta = (-3.0 * n / 2.0) + (9.0 * n.powi(3) / 16.0) + (-3.0 * n.powi(5) / 32.0);

        // Precalculate gamma
        let gamma = (15.0 * n.powi(2) / 16.0) + (-15.0 * n.powi(4) / 32.0);

        // Precalculate delta
        let delta = (-35.0 * n.powi(3) / 48.0) + (105.0 * n.powi(5) / 256.0);

        // Precalculate epsilon
        let epsilon = 315.0 * n.powi(4) / 512.0;

        // Now calculate the sum of the series and return
        alpha *
            (lat + (beta * (2.0 * lat).sin()) + (gamma * (4.0 * lat).sin()) +
             (delta * (6.0 * lat).sin()) + (epsilon * (8.0 * lat).sin()))
    }


    /// Determines the central meridian for the given UTM zone.
    ///
    /// Inputs:
    /// zone - An integer value designating the UTM zone, range [1,60].
    ///
    /// Returns:
    /// The central meridian for the given UTM zone, in degrees
    /// If the given zone is out of range, the value will wrap around (zone 61 becomes zone 1)
    ///
    /// Range of the central meridian is the radian equivalent of [-177,+177].
    ///
    #[inline]
    pub fn utm_central_meridian(zone: u8) -> f64 {
        (-183.0 + (zone as f64 * 6.0)).to_radians()
    }

    /// Computes the footpoint latitude for use in converting transverse
    /// Mercator coordinates to ellipsoidal coordinates.
    ///
    /// Reference: Hoffmann-Wellenhof, B., Lichtenegger, H., and Collins, J.,
    /// GPS: Theory and Practice, 3rd ed.  New York: Springer-Verlag Wien, 1994.
    ///
    /// Inputs:
    /// y - The UTM northing coordinate, in meters.
    ///
    /// Returns:
    /// The footpoint latitude, in radians.
    ///
    #[allow(non_snake_case)]
    pub fn footpoint_latitude(y: f64, ellipsoid: &Ellipsoid) -> f64 {

        // Precalculate n (Eq. 10.18)
        let n = (ellipsoid.a - ellipsoid.b) / (ellipsoid.a + ellipsoid.b);

        // Precalculate alpha_ (Eq. 10.22)
        // (Same as alpha in Eq. 10.17)
        let alpha_ = ((ellipsoid.a + ellipsoid.b) / 2.0) *
            (1.0 + (n.powi(2) / 4.0) + (n.powi(4) / 64.0));

        // Precalculate y_ (Eq. 10.23)
        let y_ = y / alpha_;

        // Precalculate beta_ (Eq. 10.22)
        let beta_ = (3.0 * n / 2.0) + (-27.0 * n.powi(3) / 32.0) + (269.0 * n.powi(5) / 512.0);

        // Precalculate gamma_ (Eq. 10.22)
        let gamma_ = (21.0 * n.powi(2) / 16.0) + (-55.0 * n.powi(4) / 32.0);

        // Precalculate delta_ (Eq. 10.22)
        let delta_ = (151.0 * n.powi(3) / 96.0) + (-417.0 * n.powi(5) / 128.0);

        // Precalculate epsilon_ (Eq. 10.22)
        let epsilon_ = 1097.0 * n.powi(4) / 512.0;

        // Now calculate the sum of the series (Eq. 10.21)
        y_ + (beta_ * (2.0 * y_).sin()) + (gamma_ * (4.0 * y_).sin()) + (delta_ * (6.0 * y_).sin()) +
            (epsilon_ * (8.0 * y_).sin())
    }

    /// Function that converts a single X-Y value from UTM to lon / lat
    /// Returns: (lon, lat)
    #[inline]
    #[allow(non_snake_case)]
    pub fn utm_xy_to_lonlat(x: f64, y: f64, ellipsoid: &Ellipsoid, utm_zone: u8)
                            -> (f64, f64)
    {   
        let mut cur_x = x;
        let mut cur_y = y;
        
        // If in southern hemisphere, adjust y accordingly to be negative
        cur_y -= FALSE_NORTHING;
        cur_y /= UTM_SCALE_FACTOR;

        cur_x -= FALSE_EASTING;
        cur_x /= UTM_SCALE_FACTOR;

        // lambda0 is in radiansd
        let lambda0 = utm_central_meridian(utm_zone);

        // Get the value of phif, the footpoint latitude.
        let phif = footpoint_latitude(cur_y, &ellipsoid);

        // Precalculate ep2
        let ep2 = (ellipsoid.a.powi(2) - ellipsoid.b.powi(2)) / ellipsoid.b.powi(2);

        // Precalculate cos (phif)
        let cf = phif.cos();

        // Precalculate nuf2
        let nuf2 = ep2 * cf.powi(2);

        // Precalculate Nf and initialize Nfpow
        let Nf = ellipsoid.a.powi(2) / (ellipsoid.b * (1.0 + nuf2).sqrt());
        let mut Nfpow = Nf;

        // Precalculate tf
        let tf = phif.tan();
        let tf2 = tf.powi(2);
        let tf4 = tf2.powi(2);

        // Precalculate fractional coefficients for x**n in the equations
        // below to simplify the expressions for latitude and longitude.
        let x1frac = 1.0 / (Nfpow * cf);

        Nfpow *= Nf; /* now equals Nf**2) */
        let x2frac = tf / (2.0 * Nfpow);

        Nfpow *= Nf; /* now equals Nf**3) */
        let x3frac = 1.0 / (6.0 * Nfpow * cf);

        Nfpow *= Nf; /* now equals Nf**4) */
        let x4frac = tf / (24.0 * Nfpow);

        Nfpow *= Nf; /* now equals Nf**5) */
        let x5frac = 1.0 / (120.0 * Nfpow * cf);

        Nfpow *= Nf; /* now equals Nf**6) */
        let x6frac = tf / (720.0 * Nfpow);

        Nfpow *= Nf; /* now equals Nf**7) */
        let x7frac = 1.0 / (5040.0 * Nfpow * cf);

        Nfpow *= Nf; /* now equals Nf**8) */
        let x8frac = tf / (40320.0 * Nfpow);

        // Precalculate polynomial coefficients for x**n.
        // -- x**1 does not have a polynomial coefficient.
        let x2poly = -1.0 - nuf2;

        let x3poly = -1.0 - 2.0 * tf2 - nuf2;

        let x4poly = 5.0 + 3.0 * tf2 + 6.0 * nuf2 - 6.0 * tf2 * nuf2 - 3.0 * (nuf2 * nuf2) -
            9.0 * tf2 * (nuf2 * nuf2);

        let x5poly = 5.0 + 28.0 * tf2 + 24.0 * tf4 + 6.0 * nuf2 + 8.0 * tf2 * nuf2;

        let x6poly = -61.0 - 90.0 * tf2 - 45.0 * tf4 - 107.0 * nuf2 + 162.0 * tf2 * nuf2;

        let x7poly = -61.0 - 662.0 * tf2 - 1320.0 * tf4 - 720.0 * (tf4 * tf2);

        let x8poly = 1385.0 + 3633.0 * tf2 + 4095.0 * tf4 + 1575.0 * (tf4 * tf2);

        // Calculate latitude
        let lat = phif + x2frac * x2poly * (cur_x * cur_x) + x4frac * x4poly * x.powi(4) +
            x6frac * x6poly * x.powi(6) + x8frac * x8poly * x.powi(8);

        // Calculate longitude
        let lon = lambda0 + x1frac * cur_x + x3frac * x3poly * x.powi(3) + x5frac * x5poly * x.powi(5) +
            x7frac * x7poly * x.powi(7);

        (lon.to_degrees(), lat.to_degrees())
    }

    /// Function that converts a single lon-lat pair from (lon / lat) to UTM (x, y)
    /// Returns: (x, y)
    #[inline]
    #[allow(non_snake_case)]
    pub fn lonlat_to_utm_xy(lon: f64, lat: f64, ellipsoid: &Ellipsoid, utm_zone: u8)
                            -> (f64, f64)
    {        
        let cur_lon = lon.to_radians();
        let cur_lat = lat.to_radians();

        let lambda0 = utm_central_meridian(utm_zone);

        // Precalculate ep2
        let ep2 = (ellipsoid.a.powi(2) - ellipsoid.b.powi(2)) / ellipsoid.b.powi(2);

        // Precalculate nu2
        let nu2 = ep2 * lat.cos().powi(2);

        // Precalculate N
        let N = ellipsoid.a.powi(2) / (ellipsoid.b * (1.0 + nu2).sqrt());

        // Precalculate t
        let t = lat.tan();
        let t2 = t * t;

        // Precalculate l
        let l = cur_lon - lambda0;

        // Precalculate coefficients for l**n in the equations below
        // so a normal human being can read the expressions for easting
        // and northing
        // -- l**1 and l**2 have coefficients of 1.0 */
        let l3coef = 1.0 - t2 + nu2;

        let l4coef = 5.0 - t2 + 9.0 * nu2 + 4.0 * (nu2 * nu2);

        let l5coef = 5.0 - 18.0 * t2 + (t2 * t2) + 14.0 * nu2 - 58.0 * t2 * nu2;

        let l6coef = 61.0 - 58.0 * t2 + (t2 * t2) + 270.0 * nu2 - 330.0 * t2 * nu2;

        let l7coef = 61.0 - 479.0 * t2 + 179.0 * (t2 * t2) - (t2 * t2 * t2);

        let l8coef = 1385.0 - 3111.0 * t2 + 543.0 * (t2 * t2) - (t2 * t2 * t2);

        // Calculate easting
        let mut x = N * lat.cos() * l + (N / 6.0 * lat.cos().powi(3) * l3coef * l.powi(3)) +
            (N / 120.0 * lat.cos().powi(5) * l5coef * l.powi(5)) +
            (N / 5040.0 * lat.cos().powi(7) * l7coef * l.powi(7));

        // Calculate northing
        let mut y = arc_length_of_meridian(cur_lat, &ellipsoid) +
            (t / 2.0 * N * lat.cos().powi(2) * l.powi(2)) +
            (t / 24.0 * N * lat.cos().powi(4) * l4coef * l.powi(4)) +
            (t / 720.0 * N * lat.cos().powi(6) * l6coef * l.powi(6)) +
            (t / 40320.0 * N * lat.cos().powi(8) * l8coef * l.powi(8));

        // Adjust easting and northing for UTM system
        x = (x * UTM_SCALE_FACTOR) + FALSE_EASTING;
        y = y * UTM_SCALE_FACTOR;

        if y > 0.0 {
            y = y + FALSE_NORTHING;
        }

        (x, y)        
    }
}

impl ToLonLat for UTMSystem {
    fn to_lon_lat(&self, mut data: Vec<(f64, f64)>, ellipsoid: &Ellipsoid, strategy: &mut MultithreadingStrategy)
                  -> LonLatBuf
    {
        let zone = self.utm_zone;
        
        match *strategy {
            SingleCore => {
                for &mut (ref mut x, ref mut y) in data.iter_mut() {
                    let (lon, lat) = utils::utm_xy_to_lonlat(*x, *y, ellipsoid, zone);
                    *x = lon; *y = lat;  
                }
            },
            MultiCore(ref mut thread_pool) => {
                thread_pool.scoped(|scoped| {
                    for &mut (ref mut x, ref mut y) in data.iter_mut() {
                        scoped.execute(move || {
                            let (lon, lat) = utils::utm_xy_to_lonlat(*x, *y, ellipsoid, zone);
                            *x = lon; *y = lat;
                        });
                    }
                });
            },
            _ => unimplemented!("Multithreading methods other than SingleCore and MultiCore are not yet implemented!"),          
        }
        
        LonLatBuf {
            data: data,
            ellipsoid: *ellipsoid,
        }
    }
}

impl FromLonLat for UTMSystem {
    fn from_lon_lat(&self, mut data: Vec<(f64, f64)>, ellipsoid: &Ellipsoid, strategy: &mut MultithreadingStrategy)
                    -> CoordinateBuf
    {
        let zone = self.utm_zone;
        
        match *strategy {
            SingleCore => {
                for &mut (ref mut lon, ref mut lat) in data.iter_mut() {
                    let (x, y) = utils::lonlat_to_utm_xy(*lon, *lat, ellipsoid, zone);
                    *lon = x; *lat = y;  
                }
            },
            MultiCore(ref mut thread_pool) => {
                thread_pool.scoped(|scoped| {
                    for &mut (ref mut lon, ref mut lat) in data.iter_mut() {
                        scoped.execute(move || {
                            let (x, y) = utils::lonlat_to_utm_xy(*lon, *lat, ellipsoid, zone);
                            *lon = x; *lat = y;
                        });
                    }
                });
            },
            _ => unimplemented!("Multithreading methods other than SingleCore and MultiCore are not yet implemented!"),          
        }
                
        CoordinateBuf {
            data: data,
            crs: Box::new(UTMSystem {
                utm_zone: self.utm_zone,
            }),
            ellipsoid: *ellipsoid,
        }
    }
}
