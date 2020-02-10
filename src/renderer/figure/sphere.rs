use crate::renderer::HitRecord;
use crate::wrapper::{
    ray::Ray,
    vec::{V3, V3U},
};

const EPS: f64 = 0.0001;

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Sphere {
    pub center: V3,
    pub radius: f64,
}

impl Sphere {
    pub fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = self.center.clone() - ray.origin.clone();
        let b = oc.dot(&ray.dir.clone().as_v3());
        let det = b * b - oc.dot(&oc) + self.radius * self.radius;

        if det < 0.0 {
            return None;
        }

        let sq_det = det.sqrt();
        let sol1 = b - sq_det;
        let sol2 = b + sq_det;

        if sol1 < EPS && sol2 < EPS {
            return None;
        }

        let dist = if sol1 > EPS { sol1 } else { sol2 };
        let pos = ray.extend_at(dist);
        let normal = V3U::from_v3(pos.clone() - self.center.clone());
        let orienting_normal = normal.flip_if_close(&ray.dir);

        Some(HitRecord {
            distance: dist,
            position: pos,
            normal: orienting_normal,
            is_into: normal.dot(&orienting_normal) > 0.0,
        })
    }

    pub fn sample(&self) -> (V3, V3U) {
        loop {
            let x = rand::random::<f64>();
            let y = rand::random::<f64>();
            let z = rand::random::<f64>();
            let v = V3::new(x, y, z);

            if v.len_square() <= 1.0 {
                return (
                    v.scale(self.radius) + self.center,
                    V3U::from_v3(v - self.center),
                );
            }
        }
    }

    pub fn pdf(&self) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI * self.radius * self.radius)
    }
}

#[test]
fn intersect_sphere_example() {
    use crate::wrapper::vec::V3U;

    let sphere = Sphere {
        radius: 1.0,
        center: V3::new(0.0, 5.0, 0.0),
        ..Default::default()
    };

    let hit = sphere.intersect(&Ray {
        dir: V3U::unit_y(),
        origin: V3::zero(),
    });
    assert!(hit.is_some());
    assert_eq!(
        hit.unwrap(),
        HitRecord {
            distance: 4.0,
            normal: V3U::from_v3_unsafe(V3::new(0.0, -1.0, 0.0)),
            position: V3::new(0.0, 4.0, 0.0),
            is_into: true,
        }
    );
}

#[test]
fn intersect_sphere_from_interior() {
    use crate::wrapper::vec::V3U;

    let sphere = Sphere {
        radius: 10.0,
        center: V3::new(0.0, 0.0, 0.0),
        ..Default::default()
    };

    let hit = sphere.intersect(&Ray {
        dir: V3U::unit_z(),
        origin: V3::zero(),
    });
    assert!(hit.is_some());
    assert_eq!(
        hit.unwrap(),
        HitRecord {
            distance: 10.0,
            normal: V3U::from_v3_unsafe(V3::new(0.0, 0.0, -1.0)),
            position: V3::new(0.0, 0.0, 10.0),
            is_into: false,
        }
    );
}
