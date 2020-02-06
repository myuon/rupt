use crate::wrapper::vec::{V3, V3U};

#[derive(Clone, Debug, PartialEq)]
pub struct HitRecord {
    pub distance: f64,
    pub position: V3,
    pub normal: V3U,
}

mod sphere;
pub use sphere::*;
