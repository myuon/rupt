use crate::wrapper::vec::*;

pub struct Ray {
    pub origin: V3,
    pub dir: V3U,
}

impl Ray {
    pub fn extend_at(&self, scaler: f64) -> V3 {
        self.origin.clone() + self.dir.clone().as_v3().scale(scaler)
    }
}
