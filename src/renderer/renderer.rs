use crate::renderer::Scene;
use crate::wrapper::{
    color::Color,
    ray::Ray,
    vec::{V3, V3U},
};
use std::fs::File;

pub struct Renderer {
    pub width: i32,
    pub height: i32,
    // samples per pixel
    pub spp: i32,
}

const DEPTH_LIMIT: i32 = 50;
const EPS: f64 = 0.0001;

impl Renderer {
    pub fn render(&self, scene: &Scene) -> Vec<Color> {
        let mut pixels = std::iter::repeat(Color::black())
            .take((self.width * self.height) as usize)
            .collect::<Vec<_>>();

        for i in 0..(self.width * self.height) {
            let mut radience = Color::black();
            for _ in 0..self.spp {
                let ray = Ray {
                    origin: V3::new(50.0, 52.0, 220.0),
                    dir: V3U::from_v3(V3::new(0.0, -0.04, -1.0)),
                };

                radience += self.radience(scene, &ray, 0);
            }

            pixels[i as usize] += radience;
        }

        pixels
    }

    fn radience(&self, scene: &Scene, ray: &Ray, depth: i32) -> Color {
        if let Some((hit, target)) = scene.intersect(ray) {
            let mut radience = Color::black();

            // Russian Roulette
            let r = rand::random::<f64>();
            let q = 0.5;

            let emission = target.emission;
            if r > q || depth >= DEPTH_LIMIT {
                radience += emission;
                return radience;
            }

            // orienting_normal
            let w = hit.normal;
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

            target.color.scale(1.0 / q).blend(next_radience)
        } else {
            Color::new(0.0, 0.5, 0.75)
        }
    }

    pub fn write_ppm(&self, file_path: &str, scene: &Scene) -> std::io::Result<()> {
        use std::io::Write;

        let mut file = File::create(file_path)?;
        write!(file, "P3\n{} {}\n255\n", self.width, self.height)?;

        let colors = self.render(scene);
        for c in colors {
            let (r, g, b) = c.as_rgb();
            write!(file, "{} {} {}\n", r, g, b)?;
        }

        Ok(())
    }
}
