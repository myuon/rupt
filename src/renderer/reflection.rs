use crate::renderer::HitRecord;
use crate::wrapper::{ray::Ray, vec::V3U};

#[derive(Clone)]
pub enum Reflection {
    Diffuse,
    Specular,
    Refraction,
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
    pub fn reflected(&self, ray: &Ray, hit: &HitRecord) -> Reflected {
        let orienting_normal = if hit.normal.dot(&ray.dir) < 0.0 {
            hit.normal
        } else {
            hit.normal.neg()
        };

        let specular_ray = Ray {
            origin: hit.position,
            dir: V3U::from_v3(ray.dir.as_v3() - hit.normal.scale(2.0 * hit.normal.dot(&ray.dir))),
        };

        match self {
            Reflection::Diffuse => {
                let w = orienting_normal;
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

                Reflected::new(Ray {
                    origin: hit.position,
                    dir: V3U::from_v3(
                        u.scale(r1.cos() * r2s)
                            + v.scale(r1.sin() * r2s)
                            + w.scale((1.0 - r2).sqrt()),
                    ),
                })
            }
            Reflection::Specular => Reflected::new(specular_ray),
            Reflection::Refraction => {
                let is_into = hit.normal.dot(&orienting_normal) > 0.0;

                let nc = 1.0; // 真空の屈折率
                let nt = 1.5; // このオブジェクトの屈折率
                let nnt = if is_into { nc / nt } else { nt / nc };
                let d = ray.dir.dot(&orienting_normal);
                let cos2t = 1.0 - nnt * nnt * (1.0 - d * d);

                // 全反射
                if cos2t < 0.0 {
                    return Reflected::new(specular_ray);
                }

                let refraction_ray = Ray {
                    origin: hit.position,
                    dir: V3U::from_v3(
                        ray.dir.scale(nnt)
                            - hit.normal.scale(
                                (if is_into { 1.0 } else { -1.0 }) * (d * nnt + cos2t.sqrt()),
                            ),
                    ),
                };

                // Schlickの近似
                // Fr(t) = F0 + (1 - F0)(1 - cos t)^5
                let a = nt - nc;
                let b = nt + nc;
                let r0 = (a * a) / (b * b);
                let c = 1.0
                    - (if is_into {
                        -d
                    } else {
                        refraction_ray.dir.dot(&orienting_normal.neg())
                    });
                let nnt2 = nnt.powf(2.0);

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
        }
    }
}
