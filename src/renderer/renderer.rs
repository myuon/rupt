use crate::renderer::{picture::Picture, Scene};
use crate::wrapper::{
    color::Color,
    ray::Ray,
    vec::{V3, V3U},
};
use rayon::prelude::*;
use std::fs::File;

pub struct RendererOption {
    pub enable_mis: bool,
    pub mis_power_heuristic: i32,
}

pub struct Renderer {
    pub width: i32,
    pub height: i32,
    pub spp: i32,   // samples per pixel
    pub gamma: f64, // for gamma correction
    pub option: RendererOption,
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

                    radience += self.radience(scene, ray, 0, 1.0);
                }

                radience.scale(1.0 / self.spp as f64)
            })
            .collect_into_vec(&mut pixels);

        Picture::new(pixels)
    }

    fn radience(&self, scene: &Scene, ray: Ray, depth: i32, bsdf_pdf: f64) -> Color {
        let mut depth = depth;
        let mut ray = ray;
        let mut rad = Color::black();
        let mut path_weight = 1.0;
        let mut path_color = Color::new(1.0, 1.0, 1.0);

        while let Some((hit, target)) = scene.intersect(&ray) {
            if target.emission > Color::black() {
                rad += target.emission.scale(path_weight).blend(path_color);
            }

            if self.option.enable_mis && target.reflection.is_nee_target() {
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
                        let bsdf_pdf = target.reflection.nee_bsdf_weight(&ray, &hit, shadow_dir)
                            * cos_light
                            / dist2;
                        let mis_weight = sample.pdf_value.powi(self.option.mis_power_heuristic)
                            / (sample.pdf_value.powi(self.option.mis_power_heuristic)
                                + bsdf_pdf.powi(self.option.mis_power_heuristic));

                        rad += (target.color)
                            .blend(light.emission)
                            .scale(target.reflection.nee_bsdf_weight(&ray, &hit, shadow_dir))
                            .scale(g / sample.pdf_value)
                            .scale(mis_weight)
                            .scale(path_weight)
                            .blend(path_color);
                    }
                }

                // BSDF Sampling (MIS weight)
                if target.emission > Color::black() {
                    // 単位をBSDFのpdfに合わせる
                    let light_pdf = target.area_pdf() * (ray.origin - hit.position).len_square()
                        / (ray.dir.dot(&hit.normal).cos().abs());
                    let mis_weight = bsdf_pdf.powi(self.option.mis_power_heuristic)
                        / (bsdf_pdf.powi(self.option.mis_power_heuristic)
                            + light_pdf.powi(self.option.mis_power_heuristic));

                    rad += (target.emission)
                        .scale(mis_weight)
                        .scale(path_weight)
                        .blend(path_color);
                }
            }

            // Russian Roulette
            let r = rand::random::<f64>();
            let mut rr_threshould = 0.5;

            if depth < DEPTH_MIN {
                rr_threshould = 1.0;
            } else if r > rr_threshould || depth >= DEPTH_LIMIT {
                break;
            }

            // 反射
            let reflected = target.reflection.reflected(&ray, &hit);
            path_weight = path_weight * reflected.contribution;
            path_color = path_color
                .blend(target.color)
                .scale(reflected.weight / rr_threshould);
            ray = reflected.ray;
            depth += 1;
        }

        rad
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
