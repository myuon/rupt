use crate::renderer::{HitRecord, Sphere};
use crate::wrapper::ray::Ray;

#[derive(Clone)]
pub struct Scene {
    pub objects: Vec<Sphere>,
}

impl Scene {
    /// Finds the closest object
    pub fn intersect(&self, ray: &Ray) -> Option<(HitRecord, &Sphere)> {
        let mut dist = std::f64::MAX;
        let mut result = None;

        // 線形探索
        for obj in &self.objects {
            if let Some(hit) = obj.intersect(ray) {
                if hit.distance < dist {
                    dist = hit.distance;
                    result = Some((hit, obj))
                }
            }
        }

        result
    }
}
