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
