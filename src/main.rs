mod renderer;
mod wrapper;

use renderer::*;
use wrapper::{
    color::Color,
    vec::{V3, V3U},
};

fn cornell_box() -> renderer::Scene {
    let width = 100.0;
    let height = 82.0;
    let depth = 250.0;

    renderer::Scene::new(vec![
        // left
        renderer::Object {
            figure: renderer::Figure::Rhombus(renderer::Rhombus {
                origin: V3::new(0.0, 0.0, 0.0),
                a: V3::new(0.0, 0.0, depth),
                b: V3::new(0.0, height, 0.0),
            }),
            color: Color::new(0.75, 0.25, 0.25),
            ..Default::default()
        },
        // right
        renderer::Object {
            figure: renderer::Figure::Rhombus(renderer::Rhombus {
                origin: V3::new(width, 0.0, 0.0),
                a: V3::new(0.0, 0.0, depth),
                b: V3::new(0.0, height, 0.0),
            }),
            color: Color::new(0.25, 0.25, 0.75),
            ..Default::default()
        },
        // front
        renderer::Object {
            figure: renderer::Figure::Rhombus(renderer::Rhombus {
                origin: V3::new(0.0, 0.0, 0.0),
                a: V3::new(width, 0.0, 0.0),
                b: V3::new(0.0, height, 0.0),
            }),
            color: Color::new(0.75, 0.75, 0.75),
            reflection: Reflection::Phong(renderer::PhongParameter {
                diffuse_reflectivity: 0.25,
                specular_reflectivity: 0.5,
                exponent: 50,
            }),
            ..Default::default()
        },
        // back
        renderer::Object {
            figure: renderer::Figure::Rhombus(renderer::Rhombus {
                origin: V3::new(0.0, 0.0, depth),
                a: V3::new(width, 0.0, 0.0),
                b: V3::new(0.0, height, 0.0),
            }),
            color: Color::new(0.75, 0.75, 0.75),
            ..Default::default()
        },
        // bottom
        renderer::Object {
            figure: renderer::Figure::Rhombus(renderer::Rhombus {
                origin: V3::new(0.0, height, 0.0),
                a: V3::new(width, 0.0, 0.0),
                b: V3::new(0.0, 0.0, depth),
            }),
            color: Color::new(0.75, 0.75, 0.75),
            ..Default::default()
        },
        // top
        renderer::Object {
            figure: renderer::Figure::Rhombus(renderer::Rhombus {
                origin: V3::new(0.0, 0.0, 0.0),
                a: V3::new(width, 0.0, 0.0),
                b: V3::new(0.0, 0.0, depth),
            }),
            color: Color::new(0.75, 0.75, 0.75),
            ..Default::default()
        },
        // sphere 1
        renderer::Object {
            figure: renderer::Figure::Sphere(renderer::Sphere {
                radius: 20.0,
                center: V3::new(65.0, 20.0, 20.0),
            }),
            color: Color::new(0.25, 0.75, 0.25),
            ..Default::default()
        },
        // sphere 2
        renderer::Object {
            figure: renderer::Figure::Sphere(renderer::Sphere {
                radius: 16.5,
                center: V3::new(27.0, 16.5, 47.0),
            }),
            color: Color::new(0.99, 0.99, 0.99),
            reflection: Reflection::Specular,
            ..Default::default()
        },
        // sphere 3
        renderer::Object {
            figure: renderer::Figure::Sphere(renderer::Sphere {
                radius: 16.5,
                center: V3::new(77.0, 16.5, 78.0),
            }),
            color: Color::new(0.99, 0.99, 0.99),
            reflection: Reflection::Refraction,
            ..Default::default()
        },
        // light
        renderer::Object {
            figure: renderer::Figure::Rhombus(renderer::Rhombus {
                origin: V3::new(50.0 - 7.5, height - 1.0, 81.6 - 7.5),
                a: V3::new(15.0, 0.0, 0.0),
                b: V3::new(0.0, 0.0, 15.0),
            }),
            emission: Color::new(50.0, 50.0, 50.0),
            ..Default::default()
        },
    ])
}

fn mis_example() -> renderer::Scene {
    renderer::Scene::new(vec![
        renderer::Object {
            figure: renderer::Figure::Sphere(renderer::Sphere {
                center: V3::new(0.0, 0.0, 0.0),
                radius: 5000.0,
            }),
            color: Color::new(0.75, 0.75, 0.75),
            ..Default::default()
        },
        renderer::Object {
            figure: renderer::Figure::Rhombus(renderer::Rhombus {
                origin: V3::new(-100.0, 90.0, -100.0),
                a: V3::new(300.0, 0.0, 0.0),
                b: V3::new(0.0, -25.0, 10.0),
            }),
            color: Color::new(0.75, 0.75, 0.75),
            reflection: Reflection::Phong(renderer::PhongParameter {
                diffuse_reflectivity: 0.0,
                specular_reflectivity: 1.0,
                exponent: 1000,
            }),
            ..Default::default()
        },
        renderer::Object {
            figure: renderer::Figure::Rhombus(renderer::Rhombus {
                origin: V3::new(-100.0, 60.0, -100.0),
                a: V3::new(300.0, 0.0, 0.0),
                b: V3::new(0.0, -20.0, 15.0),
            }),
            color: Color::new(0.75, 0.75, 0.75),
            reflection: Reflection::Phong(renderer::PhongParameter {
                diffuse_reflectivity: 0.0,
                specular_reflectivity: 1.0,
                exponent: 250,
            }),
            ..Default::default()
        },
        renderer::Object {
            figure: renderer::Figure::Rhombus(renderer::Rhombus {
                origin: V3::new(-100.0, 20.0, -100.0),
                a: V3::new(300.0, 0.0, 0.0),
                b: V3::new(0.0, -15.0, 20.0),
            }),
            color: Color::new(0.75, 0.75, 0.75),
            reflection: Reflection::Phong(renderer::PhongParameter {
                diffuse_reflectivity: 0.0,
                specular_reflectivity: 1.0,
                exponent: 100,
            }),
            ..Default::default()
        },
        renderer::Object {
            figure: renderer::Figure::Rhombus(renderer::Rhombus {
                origin: V3::new(-100.0, -20.0, -80.0),
                a: V3::new(300.0, 0.0, 0.0),
                b: V3::new(0.0, -10.0, 25.0),
            }),
            color: Color::new(0.75, 0.75, 0.75),
            reflection: Reflection::Phong(renderer::PhongParameter {
                diffuse_reflectivity: 0.0,
                specular_reflectivity: 1.0,
                exponent: 15,
            }),
            ..Default::default()
        },
        renderer::Object {
            figure: renderer::Figure::Sphere(renderer::Sphere {
                center: V3::new(-40.0, 110.0, -10.0),
                radius: 0.5,
            }),
            emission: Color::new(9000.0, 0.5, 0.5),
            ..Default::default()
        },
        renderer::Object {
            figure: renderer::Figure::Sphere(renderer::Sphere {
                center: V3::new(10.0, 110.0, -10.0),
                radius: 2.0,
            }),
            emission: Color::new(100.0, 100.0, 0.5),
            ..Default::default()
        },
        renderer::Object {
            figure: renderer::Figure::Sphere(renderer::Sphere {
                center: V3::new(70.0, 110.0, -10.0),
                radius: 10.0,
            }),
            emission: Color::new(1.0, 2.0, 1.0),
            ..Default::default()
        },
        renderer::Object {
            figure: renderer::Figure::Sphere(renderer::Sphere {
                center: V3::new(150.0, 110.0, -10.0),
                radius: 25.0,
            }),
            emission: Color::new(1.0, 3.0, 4.0),
            ..Default::default()
        },
    ])
}

fn main() {
    let scene = cornell_box();
    //let scene = mis_example();
    let option = RendererOption {
        enable_mis: option_env!("ENABLE_MIS")
            .map(|r| r.parse::<bool>().unwrap())
            .unwrap_or(true),
        mis_power_heuristic: 2,
        enable_mis_debug_mode: option_env!("ENABLE_MIS_DEBUG_MODE")
            .map(|r| r.parse::<bool>().unwrap())
            .unwrap_or(false),
    };

    let renderer = Renderer {
        width: 640,
        height: 480,
        spp: 16,
        gamma: 2.2,
        option,
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
