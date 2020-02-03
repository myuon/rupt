use std::ops::{Add, Sub};

#[derive(Default, PartialEq, PartialOrd, Clone, Debug, Copy)]
pub struct V3(f64, f64, f64);

impl V3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        V3(x, y, z)
    }

    pub fn scale(self, k: f64) -> Self {
        V3(self.0 * k, self.1 * k, self.2 * k)
    }

    pub fn len_square(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn len(&self) -> f64 {
        self.len_square().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(self, other: Self) -> Self {
        V3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn normalize(self) -> Self {
        let s = self.len();
        self.scale(1.0 / s)
    }

    pub fn multiply(self, other: Self) -> Self {
        V3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }

    pub fn zero() -> Self {
        V3(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }
}

impl Add<V3> for V3 {
    type Output = V3;

    fn add(self, other: Self) -> Self {
        V3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub<V3> for V3 {
    type Output = V3;

    fn sub(self, other: Self) -> Self {
        V3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[derive(Default, PartialEq, PartialOrd, Clone, Debug, Copy)]
pub struct V3U(V3);

impl V3U {
    pub fn from_v3(v: V3) -> Self {
        V3U(v.normalize())
    }

    pub fn from_v3_unsafe(v: V3) -> Self {
        V3U(v)
    }

    pub fn as_v3(self) -> V3 {
        self.0
    }

    pub fn unit_x() -> Self {
        V3U::from_v3_unsafe(V3(1.0, 0.0, 0.0))
    }

    pub fn unit_y() -> Self {
        V3U::from_v3_unsafe(V3(0.0, 1.0, 0.0))
    }

    pub fn unit_z() -> Self {
        V3U::from_v3_unsafe(V3(0.0, 0.0, 1.0))
    }

    pub fn scale(self, scaler: f64) -> V3 {
        self.as_v3().scale(scaler)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.as_v3().dot(&other.as_v3())
    }

    pub fn x(&self) -> f64 {
        self.0.x()
    }

    pub fn y(&self) -> f64 {
        self.0.y()
    }

    pub fn z(&self) -> f64 {
        self.0.z()
    }

    pub fn neg(self) -> Self {
        V3U(self.0.scale(-1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for V3 {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let (x, y, z) = Arbitrary::arbitrary(g);
            V3(x, y, z)
        }
    }

    #[quickcheck]
    fn cross_product_perpendicularity(v1: V3, v2: V3) -> bool {
        let v1_clone = v1.clone();
        let v2_clone = v2.clone();
        let c = v1.cross(v2);
        c.dot(&v1_clone).abs() <= 0.01 && c.dot(&v2_clone).abs() <= 0.01
    }
}
