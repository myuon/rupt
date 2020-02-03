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

                radience += self.radience(scene, &ray);
            }

            pixels[i as usize] += radience;
        }

        pixels
    }

    fn radience(&self, scene: &Scene, ray: &Ray) -> Color {
        Color::new(0.5, 0.5, 0.5)
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
