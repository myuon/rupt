use std::ops::{Add, AddAssign};

#[derive(Clone, Default, Copy, Debug, PartialEq, PartialOrd)]
pub struct Color(f64, f64, f64);

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Color(x, y, z)
    }

    pub fn as_rgb(self) -> (u8, u8, u8) {
        (
            (self.0.min(1.0) * 255.0) as u8,
            (self.1.min(1.0) * 255.0) as u8,
            (self.2.min(1.0) * 255.0) as u8,
        )
    }

    pub fn gamma_correction(self, gamma: f64) -> Self {
        Color(
            self.0.powf(1.0 / gamma),
            self.1.powf(1.0 / gamma),
            self.2.powf(1.0 / gamma),
        )
    }

    pub fn black() -> Self {
        Color(0.0, 0.0, 0.0)
    }

    pub fn scale(self, scaler: f64) -> Color {
        Color(self.0 * scaler, self.1 * scaler, self.2 * scaler)
    }

    pub fn blend(self, other: Self) -> Self {
        Color(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }

    pub fn map(self, f: impl Fn(f64) -> f64) -> Self {
        Color(f(self.0), f(self.1), f(self.2))
    }

    pub fn brightness(&self) -> f64 {
        (self.0 + self.1 + self.2) / 3.0
    }

    pub fn luminance(&self) -> f64 {
        self.0 * 0.2126 + self.1 * 0.7152 + self.2 * 0.0722
    }

    pub fn adjust_luminance(self, new_luminance: f64) -> Self {
        if self.luminance() < 0.0001 && new_luminance < 0.0001 {
            self
        } else {
            self.map(|v| v * new_luminance / self.luminance())
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Self) -> Self {
        Color(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

#[test]
fn color_white() {
    assert_eq!(Color(1.0, 1.0, 1.0).as_rgb(), (255, 255, 255));
}
