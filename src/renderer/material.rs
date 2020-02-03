#[derive(Clone)]
pub enum Material {
    Diffuse,
    Metal,
    Glass,
}

impl Default for Material {
    fn default() -> Self {
        Material::Diffuse
    }
}
