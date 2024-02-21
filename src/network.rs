use std::collections::HashMap;

use super::input_neuron::{InputNeuron, InputNeuronKind};
use super::output_neuron::{OutputNeuron, OutputNeuronKind};

#[derive(Debug)]
pub struct NeuralNetwork {
    pub input_layer: Vec<InputNeuron>,
    pub output_layer: Vec<OutputNeuron>,
    pub connections: HashMap<(usize, usize), f32>,
}

impl NeuralNetwork {
    pub fn new(connections: HashMap<(usize, usize), f32>, neuron_fire_threshold: f32) -> Self {
        Self {
            input_layer: vec![
                InputNeuron::new(InputNeuronKind::Random),
                InputNeuron::new(InputNeuronKind::Time),
            ],
            output_layer: vec![
                OutputNeuron::new(OutputNeuronKind::MoveRandom, neuron_fire_threshold),
                OutputNeuron::new(OutputNeuronKind::MoveUp, neuron_fire_threshold),
                OutputNeuron::new(OutputNeuronKind::MoveDown, neuron_fire_threshold),
                OutputNeuron::new(OutputNeuronKind::MoveLeft, neuron_fire_threshold),
                OutputNeuron::new(OutputNeuronKind::MoveRight, neuron_fire_threshold),
                OutputNeuron::new(OutputNeuronKind::Stay, neuron_fire_threshold),
            ],
            connections,
        }
    }

    pub fn decide(&mut self, generation_time: f32) -> Vec<OutputNeuronKind> {
        let mut decisions = Vec::new();

        for (i, input) in self.input_layer.iter().enumerate() {
            let raw_signal = input.update(generation_time);

            for ((input, output), weight) in &self.connections {
                if *input != i {
                    continue;
                }

                let output = &mut self.output_layer[*output];
                output.update(raw_signal * weight);
            }
        }

        for output in &mut self.output_layer {
            if output.fire() {
                decisions.push(output.kind());
            }
        }

        decisions
    }
}
