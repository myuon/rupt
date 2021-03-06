use crate::renderer::HitRecord;
use crate::wrapper::{ray::Ray, vec::V3U};

#[derive(Clone, PartialEq, Debug)]
pub struct PhongParameter {
    pub diffuse_reflectivity: f64,
    pub specular_reflectivity: f64,
    pub exponent: i32,
}

impl PhongParameter {
    pub fn bsdf(&self, cosine_value: f64) -> f64 {
        self.diffuse_reflectivity * self.diffuse_pdf(cosine_value)
            + self.specular_reflectivity * self.specular_pdf(cosine_value)
    }

    pub fn diffuse_pdf(&self, _cosine_value: f64) -> f64 {
        1.0 / std::f64::consts::PI
    }

    pub fn specular_pdf(&self, cosine_value: f64) -> f64 {
        (self.exponent as f64 + 2.0) * cosine_value.powi(self.exponent)
            / (2.0 * std::f64::consts::PI)
    }
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

#[derive(Default)]
pub struct Reflected {
    pub ray: Ray,
    pub contribution: f64,
    pub weight: f64, // BSDF*cos(θ)/PDFの値(ただし反射率は別で計算される(データの持ち方の都合上…))
    pub pdf_value: f64, // 計算に使用したPDFの値、MISのweight計算用
}

impl Reflected {
    pub fn new(ray: Ray, weight: f64, pdf_value: f64) -> Reflected {
        Reflected {
            ray,
            contribution: 1.0,
            weight,
            pdf_value,
        }
    }
}

impl Reflection {
    pub fn is_nee_target(&self) -> bool {
        use Reflection::*;

        match self {
            Diffuse => true,
            Phong(_) => true,
            _ => false,
        }
    }

    // NEE用に3点経路上のBSDFを計算する(反射率は除く)
    pub fn nee_bsdf_weight(&self, ray: &Ray, hit: &HitRecord, light_dir: V3U) -> f64 {
        use Reflection::*;

        match self {
            Diffuse => 1.0 / std::f64::consts::PI,
            Phong(params) => params.bsdf(hit.reflected_dir(ray.dir).dot(&light_dir)),
            Specular => 0.0,
            Refraction => 0.0,
            _ => unimplemented!(),
        }
    }

    pub fn reflected(&self, ray: &Ray, hit: &HitRecord) -> Reflected {
        let specular_ray = Ray {
            origin: hit.position,
            dir: hit.reflected_dir(ray.dir),
        };

        // 反射面に対する半球座標系
        let w = hit.normal;
        let u = if w.x().abs() > EPS {
            V3U::from_v3(V3U::unit_y().as_v3().cross(w.as_v3()))
        } else {
            V3U::from_v3(V3U::unit_x().as_v3().cross(w.as_v3()))
        };
        let v = w.as_v3().cross(u.as_v3());

        let diffuse_ray = {
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
            Reflection::Diffuse => {
                // Diffuseでは入射角のcosine値/πに沿ったimportance samplingを行っているのでそれがpdfとなる
                // Diffuse面でのBSDFはρ/πでpdfはcos(θ)/πなのでweight = BSDF・cos(θ)/ρ・pdf = 1
                Reflected::new(diffuse_ray, 1.0, ray.dir.dot(&hit.normal).abs())
            }
            Reflection::Specular => {
                // specular面の場合はデルタ関数が出てくるがここでは適当な巨大な数にしておく
                Reflected::new(
                    specular_ray,
                    1.0,
                    1000000.0 / ray.dir.dot(&hit.normal).abs(),
                )
            }
            Reflection::Glossy(r) => {
                let mut specular_ray_mut = specular_ray;
                specular_ray_mut.dir =
                    V3U::from_v3(specular_ray_mut.dir.as_v3() + diffuse_ray.dir.as_v3().scale(*r));

                let specular_angle_cosine = specular_ray_mut.dir.dot(&hit.normal).abs();

                // これはウソ(scatter rayの選び方が適当なのでpdfよくわからないがspecular rayとのcosine値から適当に計算しておく)
                Reflected::new(
                    specular_ray_mut,
                    1.0 / specular_angle_cosine,
                    1.0 / specular_angle_cosine,
                )
            }
            Reflection::Refraction => {
                let nc = 1.0; // 真空の屈折率
                let nt = 1.5; // このオブジェクトの屈折率
                let nnt = if hit.is_into { nc / nt } else { nt / nc };
                let d = ray.dir.dot(&hit.normal);
                let cos2t = 1.0 - nnt * nnt * (1.0 - d * d);

                // 全反射
                if cos2t < 0.0 {
                    return Reflected::new(
                        specular_ray,
                        1.0,
                        1000000.0 / ray.dir.dot(&hit.normal).abs(),
                    );
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
                        weight: 1.0 / q,
                        pdf_value: 1000000.0 / ray.dir.dot(&hit.normal).abs(),
                    }
                } else {
                    Reflected {
                        ray: refraction_ray,
                        contribution: tr,
                        weight: 1.0 / (1.0 - q),
                        pdf_value: 1000000.0 / ray.dir.dot(&hit.normal).abs(),
                    }
                }
            }
            Reflection::Phong(params) => {
                // diffuseをとるかspecularをとるかをランダムに決定する(contributionがない場合もある)
                if params.diffuse_reflectivity + params.specular_reflectivity > 1.0 {
                    unreachable!()
                }

                let xi = rand::random::<f64>();
                if xi < params.diffuse_reflectivity {
                    Reflected {
                        ray: diffuse_ray,
                        contribution: params.diffuse_reflectivity,
                        weight: 1.0,
                        pdf_value: ray.dir.dot(&hit.normal).abs(),
                    }
                } else if xi < params.diffuse_reflectivity + params.specular_reflectivity {
                    // specular lobe sampling
                    let r1 = 2.0 * std::f64::consts::PI * rand::random::<f64>();
                    let r2 = rand::random::<f64>();
                    let t = r2.powf(1.0 / (params.exponent as f64 + 1.0));

                    let phong_reflect_dir = V3U::from_v3(
                        u.scale(r1.cos() * (1.0 - t).sqrt())
                            + v.scale(r1.sin() * (1.0 - t).sqrt())
                            + w.scale(t.sqrt()),
                    );

                    Reflected {
                        ray: Ray {
                            origin: hit.position,
                            dir: phong_reflect_dir,
                        },
                        contribution: params.specular_reflectivity,
                        weight: phong_reflect_dir.dot(&hit.normal),
                        pdf_value: params.specular_pdf(r2),
                    }
                } else {
                    Reflected {
                        contribution: 0.0,
                        weight: 0.0,
                        ..Default::default()
                    }
                }
            }
        }
    }
}
