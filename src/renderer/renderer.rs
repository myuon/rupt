use crate::renderer::Scene;
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
    // samples per pixel
    pub spp: i32,
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
const EPS: f64 = 0.0001;

impl Renderer {
    pub fn render(&self, world: &WorldSetting, scene: &Scene) -> Vec<Color> {
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

                    radience += self.radience(scene, &ray, 0);
                }

                radience.scale(1.0 / self.spp as f64)
            })
            .collect_into_vec(&mut pixels);

        pixels
    }

    fn radience(&self, scene: &Scene, ray: &Ray, depth: i32) -> Color {
        if let Some((hit, target)) = scene.intersect(ray) {
            // Russian Roulette
            let r = rand::random::<f64>();
            let mut q = 0.5;

            if depth < DEPTH_MIN {
                q = 1.0;
            } else if r > q || depth >= DEPTH_LIMIT {
                return target.emission;
            }

            // orienting_normal
            let w = if hit.normal.dot(&ray.dir) < 0.0 {
                hit.normal
            } else {
                hit.normal.neg()
            };
            let u = if w.x().abs() > EPS {
                V3U::from_v3(V3U::unit_y().as_v3().cross(w.as_v3()))
            } else {
                V3U::from_v3(V3U::unit_x().as_v3().cross(w.as_v3()))
            };
            let v = w.as_v3().cross(u.as_v3());

            // hemisphere sampling
            let r1 = 2.0 * std::f64::consts::PI * rand::random::<f64>();
            let r2 = rand::random::<f64>();
            let r2s = r2.sqrt();
            let next_dir = V3U::from_v3(
                u.scale(r1.cos() * r2s) + v.scale(r1.sin() * r2s) + w.scale((1.0 - r2).sqrt()),
            );
            let next_radience = self.radience(
                scene,
                &Ray {
                    origin: hit.position,
                    dir: next_dir,
                },
                depth + 1,
            );

            target.emission + target.color.scale(1.0 / q).blend(next_radience)
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
        use std::io::Write;

        let mut file = File::create(file_path)?;
        write!(file, "P3\n{} {}\n255\n", self.width, self.height)?;

        let colors = self.render(world, scene);
        for c in colors {
            let (r, g, b) = c.as_rgb();
            write!(file, "{} {} {}\n", r, g, b)?;
        }

        Ok(())
    }
}
