#[derive(Debug, Clone)]
pub struct Gene {
    pub source_is_hidden: bool,
    pub source_discriminant: u16,
    pub source_instance: u16,
    pub target_is_output: bool,
    pub target_discriminant: u16,
    pub target_instance: u16,
    pub weight: f32,
}
