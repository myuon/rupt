#[derive(Clone, Default)]
pub struct Color(f32, f32, f32);

impl Color {
    pub fn as_rgb(self) -> (u8, u8, u8) {
        (
            (self.0.min(1.0) * 255.0) as u8,
            (self.1.min(1.0) * 255.0) as u8,
            (self.2.min(1.0) * 255.0) as u8,
        )
    }

    pub fn black() -> Self {
        Color(0.0, 0.0, 0.0)
    }
}

#[test]
fn color_white() {
    assert_eq!(Color(1.0, 1.0, 1.0).as_rgb(), (255, 255, 255));
}
