use crate::renderer::{picture::Picture, reflection::Reflection, Scene};
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

                    radience += self.radience(scene, &ray, 0, true);
                }

                radience.scale(1.0 / self.spp as f64)
            })
            .collect_into_vec(&mut pixels);

        Picture::new(pixels)
    }

    fn radience(&self, scene: &Scene, ray: &Ray, depth: i32, is_previous_specular: bool) -> Color {
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

            // NEE
            if let Some((sample_point, sample_point_normal, light)) = scene.sample_on_lights() {
                let shadow_dir = V3U::from_v3(sample_point - hit.position);
                // 反射面がDiffuseでないとき(= Specular, Refraction)のときは寄与を計算しない
                // 本来はBSDFを考慮すべき
                let object = scene
                    .intersect(&Ray {
                        origin: hit.position,
                        dir: shadow_dir,
                    })
                    .unwrap()
                    .1;
                if object == light && target.reflection.is_nee_target() {
                    // BSDFはDiffuse面の場合は等しくρ/π
                    let fs = target.color.scale(1.0 / std::f64::consts::PI);
                    let pa = light.pdf();
                    // 幾何項
                    let g = shadow_dir.dot(&hit.normal).abs()
                        * shadow_dir.neg().dot(&sample_point_normal).abs()
                        / (sample_point - hit.position).len_square();
                    rad += fs.blend(light.emission).scale(g / pa).scale(1.0 / q);
                }
            }

            // 光源からの寄与を二重に計算しないように、光源に当たった場合は寄与を計算しない(specular面での反射を除く)
            if target.emission > Color::black() {
                return if is_previous_specular {
                    target.emission
                } else {
                    Color::black()
                };
            }

            // 反射
            let reflected = target.reflection.reflected(ray, &hit);
            let next_radience = self
                .radience(
                    scene,
                    &reflected.ray,
                    depth + 1,
                    // specular面などのNEEの対象でないものの場合は特別扱いする
                    !target.reflection.is_nee_target(),
                )
                .scale(reflected.contribution);

            rad += target.emission
                + target
                    .color
                    .scale(1.0 / (q * reflected.rr_prob))
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
