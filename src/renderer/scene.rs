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
        for i in 0..self.objects.len() {
            if let Some(hit) = self.objects[i].intersect(ray) {
                if hit.distance < dist {
                    dist = hit.distance;
                    result = Some((hit, &self.objects[i]))
                }
            }
        }

        result
    }
}
