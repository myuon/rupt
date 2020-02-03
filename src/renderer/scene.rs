use crate::renderer::Sphere;
use crate::wrapper::ray::Ray;

#[derive(Clone)]
pub struct Scene {
    pub objects: Vec<Sphere>,
}

impl Scene {
    /// Finds the closest object
    pub fn intersect(&self, ray: &Ray) -> Option<&Sphere> {
        let mut dist = std::f64::MAX;
        let mut object_index: i32 = -1;

        // 線形探索
        for i in 0..self.objects.len() {
            if let Some(hit) = self.objects[i].intersect(ray) {
                if hit.distance < dist {
                    dist = hit.distance;
                    object_index = i as i32;
                }
            }
        }

        if object_index != -1 {
            Some(&self.objects[object_index as usize])
        } else {
            None
        }
    }
}
