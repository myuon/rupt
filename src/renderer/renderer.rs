use crate::renderer::{picture::Picture, Scene};
use crate::wrapper::{
    color::Color,
    ray::Ray,
    vec::{V3, V3U},
};
use rayon::prelude::*;
use std::fs::File;

pub struct Renderer {
    pub width: i32,
    pub height: i32,
    pub spp: i32,   // samples per pixel
    pub gamma: f64, // for gamma correction
}

pub struct Camera {
    pub position: V3,
    pub dir: V3U,
    pub up: V3U,
}

pub struct Screen {
    pub width: f64,
    pub height: f64,
    pub dist: f64,
}

pub struct WorldSetting {
    pub camera: Camera,
    pub screen: Screen,
}

const DEPTH_LIMIT: i32 = 64;
const DEPTH_MIN: i32 = 5;

impl Renderer {
    pub fn render(&self, world: &WorldSetting, scene: &Scene) -> Picture {
        let screen_x = (world.camera.dir.as_v3())
            .cross(world.camera.up.as_v3())
            .normalize()
            .scale(world.screen.width);
        let screen_y = screen_x
            .cross(world.camera.dir.as_v3())
            .normalize()
            .scale(world.screen.height);
        let screen_center = world.camera.position + world.camera.dir.scale(world.screen.dist);

        let mut pixels = Vec::with_capacity((self.width * self.height) as usize);

        (0..self.width * self.height)
            .into_par_iter()
            .map(move |i| {
                let mut radience = Color::black();
                for _ in 0..self.spp {
                    let x = (i % self.width) as f64;
                    let y = (self.height - i / self.width - 1) as f64;

                    let r1 = rand::random::<f64>();
                    let r2 = rand::random::<f64>();

                    let screen_position = screen_center
                        + screen_x.scale((r1 + x) / self.width as f64 - 0.5)
                        + screen_y.scale((r2 + y) / self.height as f64 - 0.5);
                    let ray = Ray {
                        origin: world.camera.position,
                        dir: V3U::from_v3(screen_position - world.camera.position),
                    };

                    radience += self.radience(scene, &ray, 0, 1.0);
                }

                radience.scale(1.0 / self.spp as f64)
            })
            .collect_into_vec(&mut pixels);

        Picture::new(pixels)
    }

    fn radience(&self, scene: &Scene, ray: &Ray, depth: i32, bsdf_pdf: f64) -> Color {
        if let Some((hit, target)) = scene.intersect(ray) {
            // Russian Roulette
            let r = rand::random::<f64>();
            let mut q = 0.5;

            if depth < DEPTH_MIN {
                q = 1.0;
            } else if r > q || depth >= DEPTH_LIMIT {
                return target.emission;
            }

            let mut rad = Color::black();

            // NEE (MIS weight)
            if let Some((sample, light)) = scene.sample_on_lights() {
                // 衝突点から光源点への向き
                let shadow_dir = V3U::from_v3(sample.point - hit.position);
                let object = scene
                    .intersect(&Ray {
                        origin: hit.position,
                        dir: shadow_dir,
                    })
                    .unwrap()
                    .1;
                if object == light {
                    let cos_light = shadow_dir.dot(&sample.normal).abs();
                    let dist2 = (sample.point - hit.position).len_square();

                    // 幾何項
                    let g = shadow_dir.dot(&hit.normal).abs() * cos_light / dist2;
                    // 単位をlightのpdfに合わせる
                    let bsdf_pdf = target.reflection.nee_bsdf_weight(ray, &hit, shadow_dir)
                        * cos_light
                        / dist2;
                    let mis_weight = sample.pdf_value / (sample.pdf_value + bsdf_pdf);

                    rad += (target.color)
                        .scale(target.reflection.nee_bsdf_weight(ray, &hit, shadow_dir))
                        .blend(light.emission)
                        .scale(g / (q * sample.pdf_value))
                        .scale(mis_weight);
                }
            }

            // BSDF Sampling (MIS weight)
            if target.emission > Color::black() {
                // 単位をBSDFのpdfに合わせる
                let light_pdf = target.area_pdf() * (ray.origin - hit.position).len_square()
                    / (ray.dir.dot(&hit.normal).cos().abs());
                let mis_weight = bsdf_pdf / (bsdf_pdf + light_pdf);

                rad += target.emission.scale(mis_weight);
            }

            // 反射
            let reflected = target.reflection.reflected(ray, &hit);
            let next_radience = self
                .radience(scene, &reflected.ray, depth + 1, reflected.pdf_value)
                .scale(reflected.contribution);

            rad += target.emission
                + (target.color)
                    .scale(reflected.weight / q)
                    .blend(next_radience);

            rad
        } else {
            // 背景色
            Color::new(0.0, 0.5, 0.75)
        }
    }

    pub fn write_ppm(
        &self,
        file_path: &str,
        world: &WorldSetting,
        scene: &Scene,
    ) -> std::io::Result<()> {
        use std::io::{BufWriter, Write};

        let mut picture = self.render(world, scene);
        picture.tone_map();
        picture.correct_gamma(self.gamma);

        let mut file = BufWriter::new(File::create(file_path)?);
        write!(file, "P3\n{} {}\n255\n", self.width, self.height)?;

        for c in picture.as_vec() {
            let (r, g, b) = c.as_rgb();
            write!(file, "{} {} {}\n", r, g, b)?;
        }

        Ok(())
    }
}
