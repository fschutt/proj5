use traits::Ellipsoid;

pub struct LonLatSystem<E: Ellipsoid> {
    pub ellipsoid: E,
}
