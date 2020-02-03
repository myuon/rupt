mod renderer;
mod wrapper;

use renderer::{Reflection, Renderer};
use wrapper::{color::Color, vec::V3};

fn cornell_box() -> renderer::Scene {
    renderer::Scene {
        objects: vec![
            // left
            renderer::Sphere {
                radius: 10000.0,
                center: V3::new(10000.0 + 1.0, 40.8, 81.6),
                color: Color::new(0.75, 0.25, 0.25),
                ..Default::default()
            },
            // right
            renderer::Sphere {
                radius: 10000.0,
                center: V3::new(-10000.0 + 99.0, 40.8, 81.6),
                color: Color::new(0.25, 0.25, 0.75),
                ..Default::default()
            },
            // front
            renderer::Sphere {
                radius: 10000.0,
                center: V3::new(50.0, 40.8, 10000.0),
                color: Color::new(0.75, 0.75, 0.75),
                ..Default::default()
            },
            // back
            renderer::Sphere {
                radius: 10000.0,
                center: V3::new(50.0, 40.8, -10000.0 + 250.0),
                ..Default::default()
            },
            // bottom
            renderer::Sphere {
                radius: 10000.0,
                center: V3::new(50.0, 10000.0, 81.6),
                color: Color::new(0.75, 0.75, 0.75),
                ..Default::default()
            },
            // top
            renderer::Sphere {
                radius: 10000.0,
                center: V3::new(50.0, -10000.0 + 81.6, 81.6),
                color: Color::new(0.75, 0.75, 0.75),
                ..Default::default()
            },
            // sphere 1
            renderer::Sphere {
                radius: 20.0,
                center: V3::new(65.0, 20.0, 20.0),
                color: Color::new(0.25, 0.75, 0.25),
                ..Default::default()
            },
            // sphere 2
            renderer::Sphere {
                radius: 16.5,
                center: V3::new(27.0, 16.5, 47.0),
                color: Color::new(0.99, 0.99, 0.99),
                reflection: Reflection::Specular,
                ..Default::default()
            },
            // sphere 3
            renderer::Sphere {
                radius: 16.5,
                center: V3::new(77.0, 16.5, 78.0),
                color: Color::new(0.99, 0.99, 0.99),
                reflection: Reflection::Refraction,
                ..Default::default()
            },
            // light
            renderer::Sphere {
                radius: 15.0,
                center: V3::new(50.0, 90.0, 81.6),
                emission: Color::new(36.0, 36.0, 36.0),
                ..Default::default()
            },
        ],
    }
}

fn main() {
    let scene = cornell_box();
    let renderer = Renderer {
        width: 400,
        height: 300,
        spp: 10,
    };

    renderer.write_ppm("out.ppm", &scene).unwrap();
}
