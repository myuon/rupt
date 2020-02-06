use crate::renderer::HitRecord;
use crate::wrapper::{
    ray::Ray,
    vec::{V3, V3U},
};

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Rectangle {
    pub center: V3,
    pub up: V3U,
    pub normal: V3U,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let t = (self.center.dot(&self.normal.as_v3()) - ray.origin.dot(&self.normal.as_v3()))
            / ray.dir.dot(&self.normal);
        let p = ray.extend_at(t);
        if !self.has(&p) {
            return None;
        }

        Some(HitRecord {
            distance: t,
            position: p,
            normal: self.normal,
        })
    }

    pub fn has(&self, p: &V3) -> bool {
        let across = V3U::from_v3(self.up.as_v3().cross(self.normal.as_v3()));
        let w = (*p - self.center).dot(&across.as_v3());
        let h = (*p - self.center).dot(&self.up.as_v3());

        -self.width / 2.0 <= w
            && w <= self.width / 2.0
            && -self.height / 2.0 <= h
            && h <= self.height / 2.0
    }

    pub fn sample(&self) -> (V3, V3U) {
        // random over [-1.0,1.0]
        let x = rand::random::<f64>() * 2.0 - 1.0;
        let y = rand::random::<f64>();
        let across = self.up.as_v3().cross(self.normal.as_v3());

        (
            self.center
                + self.up.scale((self.height / 2.0) * y)
                + across.scale((self.width / 2.0) * x),
            self.normal,
        )
    }

    pub fn pdf(&self) -> f64 {
        1.0 / (self.width * self.height)
    }

    // order: x0, x1, y0, y1
    pub fn polygon(&self) -> [V3; 4] {
        let across = self.up.as_v3().cross(self.normal.as_v3());

        [
            self.center + across.scale(self.width / 2.0) + self.up.scale(self.height / 2.0),
            self.center - across.scale(self.width / 2.0) + self.up.scale(self.height / 2.0),
            self.center + across.scale(self.width / 2.0) - self.up.scale(self.height / 2.0),
            self.center - across.scale(self.width / 2.0) - self.up.scale(self.height / 2.0),
        ]
    }
}

#[test]
fn intersect_rectangle_example() {
    use crate::wrapper::vec::V3U;

    let rect = Rectangle {
        center: V3::new(0.0, 5.0, 0.0),
        up: V3U::unit_x(),
        normal: V3U::unit_y(),
        width: 10.0,
        height: 20.0,
        ..Default::default()
    };

    assert_eq!(
        rect.polygon(),
        [
            V3::new(10.0, 5.0, 5.0),
            V3::new(10.0, 5.0, -5.0),
            V3::new(-10.0, 5.0, 5.0),
            V3::new(-10.0, 5.0, -5.0),
        ]
    );

    let hit = rect.intersect(&Ray {
        dir: V3U::unit_y(),
        origin: V3::zero(),
    });
    assert!(hit.is_some());
    assert_eq!(
        hit.unwrap(),
        HitRecord {
            distance: 5.0,
            normal: V3U::from_v3_unsafe(V3::new(0.0, 1.0, 0.0)),
            position: V3::new(0.0, 5.0, 0.0),
        }
    );

    assert!(rect
        .intersect(&Ray {
            dir: V3U::unit_y(),
            origin: V3::new(12.0, 0.0, 0.0),
        })
        .is_none());

    assert!(rect
        .intersect(&Ray {
            dir: V3U::unit_y(),
            origin: V3::new(8.0, 0.0, 0.0),
        })
        .is_some());

    assert!(rect
        .intersect(&Ray {
            dir: V3U::unit_y(),
            origin: V3::new(0.0, 0.0, 7.0),
        })
        .is_none());

    assert!(rect
        .intersect(&Ray {
            dir: V3U::unit_y(),
            origin: V3::new(0.0, 0.0, 3.0),
        })
        .is_some());
}

#[cfg(test)]
impl quickcheck::Arbitrary for Rectangle {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let as_u64 = |x: u64| x as f64;

        Rectangle {
            center: quickcheck::Arbitrary::arbitrary(g),
            up: quickcheck::Arbitrary::arbitrary(g),
            normal: quickcheck::Arbitrary::arbitrary(g),
            width: as_u64(quickcheck::Arbitrary::arbitrary(g)) + 1.0,
            height: as_u64(quickcheck::Arbitrary::arbitrary(g)) + 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn sample_in_rectangle(rect: Rectangle) -> bool {
        let (p, _) = rect.sample();
        rect.has(&p)
    }
}
