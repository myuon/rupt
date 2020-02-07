use crate::renderer::Reflection;
use crate::wrapper::{
    color::Color,
    ray::Ray,
    vec::{V3, V3U},
};

#[derive(Clone, Debug, PartialEq)]
pub struct HitRecord {
    pub distance: f64,
    pub position: V3,
    pub normal: V3U,
}

mod rectangle;
mod sphere;

pub use rectangle::*;
pub use sphere::*;

#[derive(Clone, PartialEq, Default)]
pub struct Object {
    pub figure: Figure,
    pub emission: Color,
    pub color: Color,
    pub reflection: Reflection,
}

#[derive(Clone, PartialEq)]
pub enum Figure {
    Sphere(Sphere),
    Rhombus(Rhombus),
}

impl Default for Figure {
    fn default() -> Self {
        Figure::Sphere(Default::default())
    }
}

impl Object {
    pub fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        use Figure::*;

        match &self.figure {
            Rhombus(r) => r.intersect(ray),
            Sphere(r) => r.intersect(ray),
        }
    }

    pub fn sample(&self) -> (V3, V3U) {
        use Figure::*;

        match &self.figure {
            Rhombus(r) => r.sample(),
            Sphere(r) => r.sample(),
        }
    }

    pub fn pdf(&self) -> f64 {
        use Figure::*;

        match &self.figure {
            Rhombus(r) => r.pdf(),
            Sphere(r) => r.pdf(),
        }
    }
}
