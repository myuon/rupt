use crate::wrapper::color::Color;

pub struct Picture {
    pixels: Vec<Color>,
}

impl Picture {
    pub fn new(pixels: Vec<Color>) -> Picture {
        Picture { pixels }
    }

    pub fn as_vec(self) -> Vec<Color> {
        self.pixels
    }

    pub fn correct_gamma(&mut self, gamma: f64) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = self.pixels[i].gamma_correction(gamma);
        }
    }

    pub fn tone_map(&mut self) {
        let max_white = self.get_max_white();
        let reinhard_ext =
            |c: Color| c.map(|v| v * (1.0 + v / (max_white * max_white)) / (1.0 + v));

        for i in 0..self.pixels.len() {
            let c = self.pixels[i];
            self.pixels[i] = reinhard_ext(c);
        }
    }

    fn get_max_white(&self) -> f64 {
        // find maximum
        self.pixels
            .iter()
            .fold(0.0 / 0.0, |m, v| v.brightness().max(m))
    }
}
