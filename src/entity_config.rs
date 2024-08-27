pub struct EntityConfig {
    pub start_count: u32,
    pub child_count: u32,

    // TODO: Move these to NeuralNetworkConfig.
    pub mutation_rate: f32,
    pub mutation_magnitude: f32,
}
