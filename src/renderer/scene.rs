use crate::renderer::{HitRecord, Sphere};
use crate::wrapper::{
    color::Color,
    ray::Ray,
    vec::{V3, V3U},
};

#[derive(Clone)]
pub struct Scene {
    objects: Vec<Sphere>,
    lights: Vec<usize>,
}

impl Scene {
    pub fn new(objects: Vec<Sphere>) -> Self {
        let light_indices = objects
            .iter()
            .enumerate()
            .filter(|(_, obj)| obj.emission > Color::black())
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        Scene {
            objects,
            lights: light_indices,
        }
    }

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

    // returns point, normal, reference to object
    pub fn sample_on_lights(&self) -> Option<(V3, V3U, &Sphere)> {
        if self.lights.is_empty() {
            return None;
        }

        let i = rand::random::<usize>() % self.lights.len();
        let (p, n) = self.objects[self.lights[i]].sample();
        Some((p, n, &self.objects[self.lights[i]]))
    }
}
