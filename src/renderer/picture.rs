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
        let max_white = self.get_max_luminance();
        let reinhard_ext = |c: Color| {
            let lumi = c.luminance();
            c.adjust_luminance(lumi * (1.0 + (lumi / (max_white * max_white))) / (1.0 + lumi))
        };

        for i in 0..self.pixels.len() {
            let c = self.pixels[i];
            self.pixels[i] = reinhard_ext(c);
        }
    }

    fn get_max_luminance(&self) -> f64 {
        // find maximum
        self.pixels
            .iter()
            .fold(0.0 / 0.0, |m, v| v.luminance().max(m))
    }
}
