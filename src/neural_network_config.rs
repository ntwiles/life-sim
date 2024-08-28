pub struct NeuralNetworkConfig {
    pub hidden_layer_width: usize,
    pub hidden_layer_depth: usize,

    pub mutation_rate: f32,

    pub connection_mutation_rate: f32,
    pub connection_mutation_magnitude: f32,

    pub structural_mutation_rate: f32,
}
