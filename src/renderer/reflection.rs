use crate::renderer::HitRecord;
use crate::wrapper::{ray::Ray, vec::V3U};

#[derive(Clone, PartialEq, Debug)]
pub struct PhongParameter {
    pub diffuse_reflectivity: f64,
    pub specular_reflectivity: f64,
    pub exponent: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Reflection {
    Diffuse,
    Specular,
    Refraction,
    Glossy(f64),           // primitive glossy surface
    Phong(PhongParameter), // glossy surface based on Phong model
}

impl Default for Reflection {
    fn default() -> Self {
        Reflection::Diffuse
    }
}

const EPS: f64 = 0.0001;

pub struct Reflected {
    pub ray: Ray,
    pub contribution: f64,
    pub rr_prob: f64,
}

impl Reflected {
    pub fn new(ray: Ray) -> Reflected {
        Reflected {
            ray,
            contribution: 1.0,
            rr_prob: 1.0,
        }
    }
}

impl Reflection {
    pub fn is_nee_target(&self) -> bool {
        use Reflection::*;

        match self {
            Diffuse => true,
            _ => false,
        }
    }

    pub fn reflected(&self, ray: &Ray, hit: &HitRecord) -> Reflected {
        let specular_ray = Ray {
            origin: hit.position,
            dir: hit.reflected_dir(ray.dir),
        };

        let diffuse_ray = {
            let w = hit.normal;
            let u = if w.x().abs() > EPS {
                V3U::from_v3(V3U::unit_y().as_v3().cross(w.as_v3()))
            } else {
                V3U::from_v3(V3U::unit_x().as_v3().cross(w.as_v3()))
            };
            let v = w.as_v3().cross(u.as_v3());

            // 半球に沿ったimportance sampling
            let r1 = 2.0 * std::f64::consts::PI * rand::random::<f64>();
            let r2 = rand::random::<f64>();
            let r2s = r2.sqrt();

            Ray {
                origin: hit.position,
                dir: V3U::from_v3(
                    u.scale(r1.cos() * r2s) + v.scale(r1.sin() * r2s) + w.scale((1.0 - r2).sqrt()),
                ),
            }
        };

        match self {
            Reflection::Diffuse => Reflected::new(diffuse_ray),
            Reflection::Specular => Reflected::new(specular_ray),
            Reflection::Glossy(r) => {
                let mut specular_ray_mut = specular_ray;
                specular_ray_mut.dir =
                    V3U::from_v3(specular_ray_mut.dir.as_v3() + diffuse_ray.dir.as_v3().scale(*r));

                Reflected::new(specular_ray_mut)
            }
            Reflection::Refraction => {
                let nc = 1.0; // 真空の屈折率
                let nt = 1.5; // このオブジェクトの屈折率
                let nnt = if hit.is_into { nc / nt } else { nt / nc };
                let d = ray.dir.dot(&hit.normal);
                let cos2t = 1.0 - nnt * nnt * (1.0 - d * d);

                // 全反射
                if cos2t < 0.0 {
                    return Reflected::new(specular_ray);
                }

                let refraction_ray = Ray {
                    origin: hit.position,
                    dir: V3U::from_v3(
                        ray.dir.scale(nnt) - hit.normal.scale(d * nnt + cos2t.sqrt()),
                    ),
                };

                // Schlickの近似
                // Fr(t) = F0 + (1 - F0)(1 - cos t)^5
                let a = nt - nc;
                let b = nt + nc;
                let r0 = (a * a) / (b * b);
                let c = 1.0
                    - (if hit.is_into {
                        -d
                    } else {
                        refraction_ray.dir.dot(&hit.normal.neg())
                    });

                // 反射光の寄与
                let re = r0 + (1.0 - r0) * c.powf(5.0);
                // 屈折光の寄与
                let tr = (1.0 - re) * nnt.powf(2.0);

                // Russian Roulette
                let q = 0.25 + 0.5 * re;
                let r = rand::random::<f64>();

                // 反射
                if r < q {
                    Reflected {
                        ray: specular_ray,
                        contribution: re,
                        rr_prob: q,
                    }
                } else {
                    Reflected {
                        ray: refraction_ray,
                        contribution: tr,
                        rr_prob: 1.0 - q,
                    }
                }
            }
            Reflection::Phong(_) => unimplemented!(),
        }
    }
}
