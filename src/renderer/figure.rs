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
    // this is guaranteed to be an orienting normal
    pub normal: V3U,
    // 光がオブジェクトの中に入る動きかそうでないかの判断
    // これは実質Sphereでしか動かない(がRhombusでRefractionしないと思うので別にそれでも良い気がする…)
    pub is_into: bool,
}

impl HitRecord {
    pub fn reflected_dir(&self, incoming: V3U) -> V3U {
        V3U::from_v3(incoming.as_v3() - self.normal.scale(2.0 * self.normal.dot(&incoming)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SampleRecord {
    pub point: V3,
    pub normal: V3U,
    pub pdf_value: f64,
}

mod rhombus;
mod sphere;

pub use rhombus::*;
pub use sphere::*;

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Object {
    pub figure: Figure,
    pub emission: Color,
    pub color: Color,
    pub reflection: Reflection,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Figure {
    Sphere(Sphere),
    Rhombus(Rhombus),
    Figures(Vec<Figure>),
}

impl Figure {
    pub fn parallelepiped(origin: V3, a: V3, b: V3, c: V3) -> Figure {
        Figure::Figures(vec![
            Figure::Rhombus(Rhombus { origin, a, b }),
            Figure::Rhombus(Rhombus { origin, a, b: c }),
            Figure::Rhombus(Rhombus { origin, a: b, b: c }),
            Figure::Rhombus(Rhombus {
                origin: origin + a,
                a: b,
                b: c,
            }),
            Figure::Rhombus(Rhombus {
                origin: origin + b,
                a,
                b: c,
            }),
            Figure::Rhombus(Rhombus {
                origin: origin + c,
                a,
                b,
            }),
        ])
    }
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
            Figures(figs) => {
                let mut min = std::f64::MAX;
                let mut result = None;

                for fig in figs {
                    if let Some(hit) = (Object {
                        figure: fig.clone(),
                        emission: self.emission,
                        color: self.color,
                        reflection: self.reflection.clone(),
                    }
                    .intersect(ray))
                    {
                        if hit.distance < min {
                            min = hit.distance;
                            result = Some(hit);
                        }
                    }
                }

                result
            }
        }
    }

    pub fn sample(&self) -> SampleRecord {
        use Figure::*;

        match &self.figure {
            Rhombus(r) => r.sample(),
            Sphere(r) => r.sample(),
            Figures(figs) => {
                let i = rand::random::<usize>() % figs.len();
                Object {
                    figure: figs[i].clone(),
                    emission: self.emission,
                    color: self.color,
                    reflection: self.reflection.clone(),
                }
                .sample()
            }
        }
    }

    pub fn bsdf(&self, specular_angle_cosine: f64) -> Color {
        use Reflection::*;

        let k = match &self.reflection {
            // BSDFはDiffuse面の場合は等しくρ/π
            Diffuse => 1.0 / std::f64::consts::PI,
            Phong(params) => {
                (params.diffuse_reflectivity / std::f64::consts::PI)
                    + (params.specular_reflectivity
                        * (params.exponent as f64 + 2.0)
                        * specular_angle_cosine.powi(params.exponent))
                        / (2.0 * std::f64::consts::PI)
            }
            _ => 1.0,
        };

        self.color.scale(k)
    }
}
