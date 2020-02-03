#[derive(Clone)]
pub enum Reflection {
    Diffuse,
    Specular,
    Refraction,
}

impl Default for Reflection {
    fn default() -> Self {
        Reflection::Diffuse
    }
}

impl Reflection {
    fn pdf(&self, cosine_value: f64) -> f64 {
        match self {
            Reflection::Diffuse => cosine_value / std::f64::consts::PI,
            _ => unimplemented!(),
        }
    }
}
