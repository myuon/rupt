mod renderer;
mod wrapper;

use renderer::*;
use wrapper::{
    color::Color,
    vec::{V3, V3U},
};

fn cornell_box() -> renderer::Scene {
    renderer::Scene::new(vec![
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
            emission: Color::new(5.0, 5.0, 5.0),
            ..Default::default()
        },
    ])
}

fn main() {
    let scene = cornell_box();
    let renderer = Renderer {
        width: 640,
        height: 480,
        spp: 16,
        gamma: 2.2,
    };
    let world = WorldSetting {
        camera: Camera {
            position: V3::new(50.0, 52.0, 220.0),
            dir: V3U::from_v3(V3::new(0.0, -0.04, -1.0)),
            up: V3U::unit_y(),
        },
        screen: Screen {
            width: 30.0 * renderer.width as f64 / renderer.height as f64,
            height: 30.0,
            dist: 40.0,
        },
    };

    renderer.write_ppm("out.ppm", &world, &scene).unwrap();
}
