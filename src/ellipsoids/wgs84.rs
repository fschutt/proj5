use ellipsoid::Ellipsoid;

pub const WGS_84_ELLIPSOID: Ellipsoid = Ellipsoid {
    a: 6_378_137.0,
    b: 6_356_752.314245,
    offset: 0.0,
    rotation: 0.0,
    scaling: 0.0,
};
