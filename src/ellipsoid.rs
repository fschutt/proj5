//! Definitions of common Ellipsoids

/// Ellipsoid struct, to be instantiated with known values
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Ellipsoid {
    /// Semi-major axis A in meter
    pub a: f64,
    /// Semi-minor axis B in meter
    pub b: f64,
    /// Inverse flattening, can be calculated from a and b
    pub flattening: f64,
    /// English name
    pub nice_name: &'static str,
    /// Usage region in English, just for completeness (from Wikipedia)
    pub usage_region: &'static str,
    /// Offset in relation to an earth-centric coordinate system
    pub offset: f64,
    /// Get the rotation in relation to an earth-centric coordinate system
    pub rotation: f64,
    /// Get the scaling in relation to an earth-centric coordinate system
    pub scaling: f64,
}

// Note: It doesn't really make sense to put ellipsoids in seperate files,
// they are too simple. So we just put them here.

/// The 24 standard ellipsoids
pub const ELLIPSOIDS: [&'static Ellipsoid; 24] = [
    &MAUPERTUIS_1738_ELLIPSOID,
    &PLESSIS_1817_ELLIPSOID,
    &EVEREST_1830_ELLIPSOID,
    &EVEREST_1967_MODIFIED_ELLIPSOID,
    &EVEREST_1967_DEFINITION_ELLIPSOID,
    &AIRY_1830_ELLIPSOID,
    &BESSEL_1841_ELLIPSOID,
    &CLARKE_1866_ELLIPSOID,
    &CLARKE_1878_ELLIPSOID,
    &CLARKE_1880_ELLIPSOID,
    &HELMERT_1906_ELLIPSOID,
    &HAYFORD_1910_ELLIPSOID,
    &INTERNATIONAL_1924_ELLIPSOID,
    &KRASSOVSKY_1940_ELLIPSOID,
    &WGS_1966_ELLIPSOID,
    &AUSTRALIAN_1966_ELLIPSOID,
    &NEW_INTERNATIONAL_1967_ELLIPSOID,
    &GRS_1967_ELLIPSOID,
    &SOUTH_AMERICAN_1969_ELLIPSOID,
    &WGS_1972_ELLIPSOID,
    &GRS_1980_ELLIPSOID,
    &WGS_1984_ELLIPSOID,
    &IERS_1989_ELLIPSOID,
    &IERS_2003_ELLIPSOID,
];

/// Maupertuis (1738)
pub const MAUPERTUIS_1738_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_397_300.0,
    b: 6_363_806.283,
    flattening: 191.0,
    usage_region: "France",
    nice_name: "Maupertuis (1738)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Plessis (1817)
pub const PLESSIS_1817_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_376_523.0,
    b: 6_355_862.9333,
    flattening: 308.64,
    usage_region: "France",
    nice_name: "Plessis (1817)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Everest (1830)
pub const EVEREST_1830_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_377_299.365,
    b: 6_356_098.359,
    flattening: 300.80172554,
    usage_region: "India",
    nice_name: "Everest (1830)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Everest 1830 Modified (1967)
pub const EVEREST_1967_MODIFIED_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_377_304.063,
    b: 6_356_103.0390,
    flattening: 300.8017,
    usage_region: "West Malaysia & Singapore",
    nice_name: "Everest 1830 Modified (1967)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Everest 1830 (1967 Definition)
pub const EVEREST_1967_DEFINITION_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_377_298.556,
    b: 6_356_097.550,
    flattening: 300.8017,
    usage_region: "Brunei & East Malaysia",
    nice_name: "Everest 1830 Modified (1967)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Airy (1830)
pub const AIRY_1830_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_377_563.396,
    b: 6_356_256.909,
    flattening: 299.3249646,
    usage_region: "Britain",
    nice_name: "Airy (1830)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Bessel (1841)
pub const BESSEL_1841_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_377_397.155,
    b: 6_356_078.963,
    flattening: 299.1528128,
    usage_region: "Europe, Japan",
    nice_name: "Bessel (1841)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Clarke (1866)
pub const CLARKE_1866_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_206.4,
    b: 6_356_583.8,
    flattening: 294.9786982,
    usage_region: "North America",
    nice_name: "Clarke (1866)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Clarke (1878)
pub const CLARKE_1878_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_190.0,
    b: 6_356_456.0,
    flattening: 293.4659980,
    usage_region: "North America",
    nice_name: "Clarke (1878)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Clarke (1880)
pub const CLARKE_1880_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_249.145,
    b: 6_356_514.870,
    flattening: 293.465,
    usage_region: "France, Africa",
    nice_name: "Clarke (1880)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Helmert (1906)
pub const HELMERT_1906_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_200.0,
    b: 6_356_818.17,
    flattening: 298.3,
    usage_region: "",
    nice_name: "Helmert (1906)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Hayford (1910)
pub const HAYFORD_1910_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_388.0,
    b: 6_356_911.946,
    flattening: 297.0,
    usage_region: "USA",
    nice_name: "Hayford (1910)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// International (1924)
pub const INTERNATIONAL_1924_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_388.0,
    b: 6_356_911.946,
    flattening: 297.0,
    usage_region: "Europe",
    nice_name: "International (1924)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Krassovsky (1940)
pub const KRASSOVSKY_1940_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_245.0,
    b: 6_356_863.019,
    flattening: 298.3,
    usage_region: "USSR, Russia, Romania",
    nice_name: "Krassovsky (1940)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// WGS66 (1966)
pub const WGS_1966_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_145.0,
    b: 6_356_759.769,
    flattening: 298.25,
    usage_region: "USA / Department of Defense",
    nice_name: "WGS66 (1966)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// Australian National (1966)
pub const AUSTRALIAN_1966_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_160.0,
    b: 6_356_774.719,
    flattening: 298.25,
    usage_region: "Australia",
    nice_name: "Australian National (1966)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// New International (1967)
pub const NEW_INTERNATIONAL_1967_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_157.5,
    b: 6_356_772.2,
    flattening: 298.24961539,
    usage_region: "",
    nice_name: "New International (1967)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// GRS-67 (1967)
pub const GRS_1967_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_160.0,
    b: 6_356_774.516,
    flattening: 298.247167427,
    usage_region: "",
    nice_name: "GRS-67 (1967)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// South American (1969)
pub const SOUTH_AMERICAN_1969_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_160.0,
    b: 6_356_774.719,
    flattening: 298.25,
    usage_region: "South America",
    nice_name: "South American (1969)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// WGS-72 (1972)
pub const WGS_1972_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_135.0,
    b: 6_356_750.52,
    flattening: 298.26,
    usage_region: "USA / Department of Defense",
    nice_name: "WGS-72 (1972)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// GRS-80 (1979)
pub const GRS_1980_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_137.0,
    b: 6_356_752.3141,
    flattening: 298.257222101,
    usage_region: "Global ITRS",
    nice_name: "GRS-80 (1979)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// WGS-84 (1984)
pub const WGS_1984_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_137.0,
    b: 6_356_752.314245,
    flattening: 298.257223563,
    usage_region: "Global/GPS",
    nice_name: "WGS-84 (1984)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// IERS (1989)
pub const IERS_1989_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_136.0,
    b: 6_356_751.302,
    flattening: 298.257,
    usage_region: "",
    nice_name: "IERS (1989)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};

/// IERS (2003)
pub const IERS_2003_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_136.6,
    b: 6_356_751.9,
    flattening: 298.25642,
    usage_region: "",
    nice_name: "IERS (2003)",
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};
