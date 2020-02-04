use crate::renderer::Reflection;
use crate::wrapper::{
    color::Color,
    ray::Ray,
    vec::{V3, V3U},
};

#[derive(Clone, Debug, PartialEq)]
pub struct HitRecord {
    pub distance: f64,
    pub position: V3,
    pub normal: V3U,
}

#[derive(Clone, Default)]
pub struct Sphere {
    pub center: V3,
    pub radius: f64,
    pub emission: Color,
    pub color: Color,
    pub reflection: Reflection,
}

const EPS: f64 = 0.0001;

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

        Some(HitRecord {
            distance: dist,
            position: pos,
            normal,
        })
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
            // 内側から衝突した場合でも法線は常に中心から外向きであることに注意
            normal: V3U::from_v3_unsafe(V3::new(0.0, 0.0, 1.0)),
            position: V3::new(0.0, 0.0, 10.0),
        }
    );
}
