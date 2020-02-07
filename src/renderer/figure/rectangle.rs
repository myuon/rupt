use crate::renderer::HitRecord;
use crate::wrapper::{
    ray::Ray,
    vec::{V3, V3U},
};

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Rhombus {
    pub a: V3,
    pub b: V3,
    pub origin: V3,
}

impl Rhombus {
    pub fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let normal = self.a.cross(self.b);
        let t = (self.origin - ray.origin).dot(&normal) / ray.dir.as_v3().dot(&normal);
        let p = ray.extend_at(t);
        if !self.has(&p) {
            return None;
        }

        Some(HitRecord {
            distance: t,
            position: p,
            normal: V3U::from_v3(normal),
        })
    }

    // pが同一平面上の点であることは仮定している
    pub fn has(&self, p: &V3) -> bool {
        let crosses = [
            self.a.cross(*p - self.origin),
            self.b.cross(*p - self.origin - self.a),
            (*p - self.origin - self.b).cross(self.a),
            (*p - self.origin).cross(self.b),
        ];

        crosses[0].dot(&crosses[1]) > 0.0
            && crosses[0].dot(&crosses[2]) > 0.0
            && crosses[0].dot(&crosses[3]) > 0.0
    }

    pub fn sample(&self) -> (V3, V3U) {
        let x = rand::random::<f64>();
        let y = rand::random::<f64>();

        (
            self.origin + self.a.scale(x) + self.b.scale(y),
            V3U::from_v3(self.a.cross(self.b)),
        )
    }

    pub fn pdf(&self) -> f64 {
        1.0 / (self.a.cross(self.b).len())
    }

    // order: x0, x1, y0, y1
    pub fn polygon(&self) -> [V3; 4] {
        [
            self.origin,
            self.origin + self.a,
            self.origin + self.b,
            self.origin + self.a + self.b,
        ]
    }
}

#[test]
fn intersect_rhombus_example() {
    use crate::wrapper::vec::V3U;

    let rect = Rhombus {
        origin: V3::new(0.0, 5.0, 0.0),
        a: V3::new(10.0, 0.0, 0.0),
        b: V3::new(0.0, 0.0, 20.0),
    };

    assert_eq!(
        rect.polygon(),
        [
            V3::new(0.0, 5.0, 0.0),
            V3::new(10.0, 5.0, 0.0),
            V3::new(0.0, 5.0, 20.0),
            V3::new(10.0, 5.0, 20.0),
        ]
    );

    let hit = rect.intersect(&Ray {
        dir: V3U::unit_y(),
        origin: V3::new(5.0, 0.0, 10.0),
    });
    assert!(hit.is_some());
    assert_eq!(
        hit.unwrap(),
        HitRecord {
            distance: 5.0,
            normal: V3U::from_v3_unsafe(V3::new(0.0, -1.0, 0.0)),
            position: V3::new(5.0, 5.0, 10.0),
        }
    );

    assert!(rect
        .intersect(&Ray {
            dir: V3U::unit_y(),
            origin: V3::new(12.0, 0.0, 5.0),
        })
        .is_none());

    assert!(rect
        .intersect(&Ray {
            dir: V3U::unit_y(),
            origin: V3::new(8.0, 0.0, 5.0),
        })
        .is_some());

    assert!(rect
        .intersect(&Ray {
            dir: V3U::unit_y(),
            origin: V3::new(5.0, 0.0, 37.0),
        })
        .is_none());

    assert!(rect
        .intersect(&Ray {
            dir: V3U::unit_y(),
            origin: V3::new(5.0, 0.0, 10.0),
        })
        .is_some());
}

#[cfg(test)]
impl quickcheck::Arbitrary for Rhombus {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        Rhombus {
            a: quickcheck::Arbitrary::arbitrary(g),
            b: quickcheck::Arbitrary::arbitrary(g),
            origin: quickcheck::Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn sample_in_rhombus(rect: Rhombus) -> bool {
        let (p, _) = rect.sample();
        rect.has(&p)
    }
}
